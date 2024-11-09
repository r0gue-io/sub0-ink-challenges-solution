import { Box, Button, Code, Text } from '@chakra-ui/react';
import React, { useMemo } from 'react';
import type { SuperdaoTraitsProposal } from '@/contracts/types/superdao';
import { useApp } from '@/providers/AppProvider.tsx';
import { Props } from '@/types.ts';
import { txToaster } from '@/utils/txToaster.tsx';
import { useContractTx } from 'typink';
import { useWatchContractQuery } from 'typink/hooks/useContractQuery.ts';

interface ProposalProps extends Props {
  proposal: SuperdaoTraitsProposal;
  index: number;
  onExecuted?: () => void;
}

export function Proposal({ index, proposal, onExecuted }: ProposalProps) {
  const { superDaoContract: contract } = useApp();
  const resolveProposalTx = useContractTx(contract, 'resolveProposal');
  const { data: votes } = useWatchContractQuery({ contract, fn: 'superDaoQueryGetVotes', args: [index] });

  const [ayaVotes, nayVotes] = useMemo<[number, number]>(() => {
    if (!votes || votes.length === 0) return [0, 0];

    const ayeVotes = votes.filter(([_, v]) => v === 'Aye').length;
    const nayVotes = votes.filter(([_, v]) => v === 'Nay').length;

    return [ayeVotes, nayVotes];
  }, [votes]);

  const { bestBlock } = useApp();
  const executable = useMemo(() => {
    return !!bestBlock && bestBlock >= proposal.votingPeriodEnd;
  }, [bestBlock, proposal.votingPeriodEnd]);

  const resolveProposal = async () => {
    const toaster = txToaster('Signing transaction...');
    try {
      await resolveProposalTx.signAndSend({
        args: [index],
        callback: ({ status }) => {
          if (status.type === 'Finalized') {
            onExecuted && onExecuted();
          }

          console.log(status);
          toaster.updateTxStatus(status);
        },
      });
    } catch (error: any) {
      console.error('Failed to resolve proposal:', error);
      toaster.onError(error);
    }
  };

  return (
    <Box border='1px' borderColor='gray.200' p={4}>
      <Text>
        Proposal:{' '}
        <b>
          #{index} - {proposal.call.type}
        </b>
      </Text>
      <Text>
        Voting period end: <b>{proposal.votingPeriodEnd.toString()}</b>
      </Text>
      <Code p={4} mt={2} whiteSpace='pre-wrap'>
        {JSON.stringify(proposal.call.value, null, 2)}
      </Code>
      <Box mt={4}>
        <Text>
          Votes: <b>{ayaVotes}</b> AYE, <b>{nayVotes}</b> NAY
        </Text>
      </Box>
      <Box mt={4}>
        <Button
          isDisabled={!executable}
          colorScheme='green'
          size='sm'
          onClick={resolveProposal}
          isLoading={resolveProposalTx.isInProgress}>
          Execute Proposal
        </Button>
      </Box>
    </Box>
  );
}
