import { Box, Text } from '@chakra-ui/react';
import { VoterInfo } from '@/components/minidao5/VoterInfo.tsx';
import PendingText from '@/components/shared/PendingText.tsx';
import { useApp } from '@/providers/AppProvider.tsx';
import { useContractQuery, useTypink } from 'typink';

export function DaoInfo() {
  const { selectedAccount } = useTypink();
  const { minidao5Contract: contract } = useApp();
  const { data: daoName, isLoading } = useContractQuery({ contract, fn: 'getName' });

  return (
    <Box>
      <Box>
        DAO Name:{' '}
        <PendingText isLoading={isLoading}>
          <b>{daoName}</b>
        </PendingText>
      </Box>

      {selectedAccount ? (
        <VoterInfo address={selectedAccount.address} />
      ) : (
        <Text mt={2}>Connect to your wallet to see voter info</Text>
      )}
    </Box>
  );
}
