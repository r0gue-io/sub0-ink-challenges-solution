import { Box, Text } from '@chakra-ui/react';
import { useEffect, useState } from 'react';
import { useTypink } from '@/providers/TypinkProvider.tsx';
import { DedotClient, PinnedBlock } from 'dedot';

export function BlockInfo() {
  const { client } = useTypink();
  const [bestBlock, setBestBlock] = useState<number>();
  const [finalizedBlock, setFinalizedBlock] = useState<number>();

  useEffect(() => {
    if (!client) {
      setBestBlock(undefined);
      setFinalizedBlock(undefined);
      return;
    }

    let unsub1: any, unsub2: any;

    if (client instanceof DedotClient) {
      unsub1 = client.chainHead.on('bestBlock', (block: PinnedBlock) => {
        setBestBlock(block.number);
      });

      unsub2 = client.chainHead.on('finalizedBlock', (block: PinnedBlock) => {
        setFinalizedBlock(block.number);
      });
    } else {
      client.rpc
        .chain_subscribeNewHeads((newHead) => {
          setBestBlock(newHead.number);
        })
        .then((unsub) => (unsub1 = unsub))
        .catch(console.error);

      client.rpc
        .chain_subscribeFinalizedHeads((newHead) => {
          setFinalizedBlock(newHead.number);
        })
        .then((unsub) => (unsub2 = unsub))
        .catch(console.error);
    }

    return () => {
      unsub1 && unsub1();
      unsub2 && unsub2();
    };
  }, [client]);

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
