import { Box, Button, Flex, Heading, Spinner, Text } from '@chakra-ui/react';
import React from 'react';
import { Proposal } from '@/components/minidao5/Proposal.tsx';
import { useApp } from '@/providers/AppProvider.tsx';
import { txToaster } from '@/utils/txToaster.tsx';
import { useContractTx } from 'typink';
import { useWatchContractQuery } from 'typink/hooks/useContractQuery.ts';

export function ProposalsPanel() {
  const { minidao5Contract: contract } = useApp();
  const { superDaoContract: superContract } = useApp();

  const createCrossChainProposalTx = useContractTx(contract, 'createSuperdaoCrossChainProposal');
  const createContractCallProposalTx = useContractTx(contract, 'createContractCallProposal');
  const { data: proposals, isLoading } = useWatchContractQuery({
    contract: superContract,
    fn: 'superDaoQueryGetProposals',
  });

  const doCreateProposal = async () => {
    const toaster = txToaster('Signing transaction...');
    try {
      await createCrossChainProposalTx.signAndSend({
        callback: ({ status }) => {
          console.log(status);
          toaster.updateTxStatus(status);
        },
      });
    } catch (e: any) {
      console.error(e);
      toaster.onError(e);
    }
  };

  const doCreateContractCallProposal = async () => {
    const toaster = txToaster('Signing transaction...');
    try {
      await createContractCallProposalTx.signAndSend({
        callback: ({ status }) => {
          console.log(status);
          toaster.updateTxStatus(status);
        },
      });
    } catch (e: any) {
      console.error(e);
      toaster.onError(e);
    }
  };

  return (
    <Box>
      <Heading size='md'>Proposals</Heading>
      <Flex gap={4}>
        <Button mt={4} size='sm' onClick={doCreateProposal} isLoading={createCrossChainProposalTx.isInProgress}>
          Create Cross Chain Proposal
        </Button>

        <Button
          mt={4}
          size='sm'
          onClick={doCreateContractCallProposal}
          isLoading={createContractCallProposalTx.isInProgress}>
          Create Contract Call Proposal
        </Button>
      </Flex>

      <Box mt={4}>
        {isLoading && <Spinner />}
        {proposals && proposals.length === 0 && <Text>No proposals</Text>}
        {proposals && (
          <Flex direction='column' gap={2}>
            {proposals.map(([index, p], idx) => (
              <Proposal proposal={p} index={index} key={idx} />
            ))}
          </Flex>
        )}
      </Box>
    </Box>
  );
}
