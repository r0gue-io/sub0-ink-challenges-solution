import { Box, Button, Text } from '@chakra-ui/react';
import { useApp } from '@/providers/AppProvider.tsx';
import { txToaster } from '@/utils/txToaster.tsx';
import { useContractQuery, useContractTx } from 'typink';

interface VoterInfoProps {
  address: string;
}

export function VoterInfo({ address }: VoterInfoProps) {
  const { minidao5Contract: contract } = useApp();

  const { data: isVoter, refresh } = useContractQuery({ contract, fn: 'hasVoter', args: [address] });
  const registerVoterTx = useContractTx(contract, 'registerVoter');
  const deregisterVoterTx = useContractTx(contract, 'deregisterVoter');

  const registerVoter = async () => {
    const toaster = txToaster('Signing transaction...');
    try {
      await registerVoterTx.signAndSend({
        callback: ({ status }) => {
          if (status.type === 'BestChainBlockIncluded') {
            refresh();
          }

          console.log(status);
          toaster.updateTxStatus(status);
        },
      });
    } catch (e: any) {
      console.error(e);
      toaster.onError(e);
    }
  };

  const deregisterVoter = async () => {
    const toaster = txToaster('Signing transaction...');
    try {
      await deregisterVoterTx.signAndSend({
        callback: ({ status }) => {
          if (status.type === 'BestChainBlockIncluded') {
            refresh();
          }

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
    <>
      {isVoter ? (
        <Box mt={2}>
          <Text>You're a voter</Text>
          <Button mt={2} size='sm' onClick={deregisterVoter} isLoading={deregisterVoterTx.isInProgress}>
            Deregister
          </Button>
        </Box>
      ) : (
        <Box mt={2}>
          <Text>You're not a voter, register now!</Text>
          <Button mt={2} size='sm' onClick={registerVoter} isLoading={registerVoterTx.isInProgress}>
            Register as voter
          </Button>
        </Box>
      )}
    </>
  );
}
