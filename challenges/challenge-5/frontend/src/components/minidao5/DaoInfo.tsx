import { Box, Text } from '@chakra-ui/react';
import { VoterInfo } from '@/components/minidao5/VoterInfo.tsx';
import { useTypink } from '@/providers/TypinkProvider.tsx';

export function DaoInfo() {
  const { selectedAccount } = useTypink();

  return (
    <Box>
      <Text>
        DAO Name: <b>MINI DAO</b>
      </Text>

      {selectedAccount ? (
        <VoterInfo address={selectedAccount.address} />
      ) : (
        <Text mt={2}>Connect to your wallet to see voter info</Text>
      )}
    </Box>
  );
}
