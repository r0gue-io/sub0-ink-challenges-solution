import { Box, Button, Flex, Heading } from '@chakra-ui/react';
import useContractTx from '@/hooks/useContractTx.ts';
import { useApp } from '@/providers/AppProvider.tsx';
import { txToaster } from '@/utils/txToaster.tsx';

export function ProposalsPanel() {
  const { minidao5Contract: contract } = useApp();

  const createCrossChainProposalTx = useContractTx(contract, 'createSuperdaoCrossChainProposal');
  const createContractCallProposalTx = useContractTx(contract, 'createContractCallProposal');

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
    </Box>
  );
}
