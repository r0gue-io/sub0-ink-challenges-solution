import { Box, Button, Code, Flex, Text } from '@chakra-ui/react';
import React, { useMemo } from 'react';
import type { SuperdaoTraitsProposal, SuperdaoTraitsVote } from '@/contracts/types/superdao';
import { useApp } from '@/providers/AppProvider.tsx';
import { Props } from '@/types.ts';
import { txToaster } from '@/utils/txToaster.tsx';
import { useContractQuery, useContractTx } from 'typink';
import { useWatchContractQuery } from 'typink/hooks/useContractQuery';

interface ProposalProps extends Props {
  proposal: SuperdaoTraitsProposal;
  index: number;
}

export function Proposal({ index, proposal }: ProposalProps) {
  const { minidao5Contract: contract, superDaoContract: superDao } = useApp();
  const voteTx = useContractTx(contract, 'voteProposal');
  const { data: votes } = useWatchContractQuery({ contract: superDao, fn: 'superDaoQueryGetVotes', args: [index] });

  const vote = useMemo<SuperdaoTraitsVote | undefined>(() => {
    if (!votes || votes.length === 0) return;
    const vote = votes.find(([id, _]) => id.address() === contract?.address?.address());
    if (vote) return vote[1];
  }, [votes]);

  const voteProposal = async (aye: boolean) => {
    const toaster = txToaster('Signing transaction...');
    try {
      await voteTx.signAndSend({
        args: [index, aye],
        callback: ({ status }) => {
          console.log(status);
          toaster.updateTxStatus(status);
        },
      });
    } catch (error: any) {
      console.error('Failed to vote proposal:', error);
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
        Last vote: <b>{vote || 'Not voted yet'}</b>
      </Box>
      <Flex mt={4} gap={2}>
        <Button colorScheme='green' size='sm' onClick={() => voteProposal(true)} isLoading={voteTx.isInProgress}>
          Aye
        </Button>
        <Button colorScheme='red' size='sm' onClick={() => voteProposal(false)} isLoading={voteTx.isInProgress}>
          Nay
        </Button>
      </Flex>
    </Box>
  );
}
