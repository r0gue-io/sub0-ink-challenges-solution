import { Box, Text } from '@chakra-ui/react';
import { useApp } from '@/providers/AppProvider.tsx';

export function BlockInfo() {
  const { bestBlock, finalizedBlock } = useApp();

  return (
    <Box>
      <Text>
        Best Block: <b>{bestBlock || '--'}</b>
      </Text>
      <Text>
        Finalized Block:{' '}
        <b>
          {finalizedBlock || '--'} {bestBlock && finalizedBlock && `(${bestBlock - finalizedBlock})`}
        </b>
      </Text>
    </Box>
  );
}
