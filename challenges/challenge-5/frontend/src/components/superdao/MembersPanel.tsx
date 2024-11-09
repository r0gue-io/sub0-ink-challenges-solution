import { Box, Heading, Spinner, Text } from '@chakra-ui/react';
import React from 'react';
import PendingText from '@/components/shared/PendingText.tsx';
import { useMiniDao5Contract } from '@/hooks/useMiniDao5Contract.ts';
import { useApp } from '@/providers/AppProvider.tsx';
import { AccountId32 } from 'dedot/codecs';
import { useContractQuery, useTypink } from 'typink';

interface MemberProps {
  address: string;
}

function Member({ address }: MemberProps) {
  const { defaultCaller, selectedAccount } = useTypink();
  const { contract } = useMiniDao5Contract(address);
  const { data: name, isLoading } = useContractQuery({
    contract,
    fn: 'getName',
    options: { caller: selectedAccount?.address || defaultCaller },
  });

  return (
    <Box border='1px' borderColor='gray.200' p={4}>
      <Box>
        Name:{' '}
        <PendingText isLoading={isLoading}>
          <b>{name}</b>
        </PendingText>
      </Box>

      <Text>{address}</Text>
    </Box>
  );
}

export function MembersPanel() {
  const { superDaoContract: contract } = useApp();
  const { data: members, isLoading } = useContractQuery({ contract, fn: 'superDaoQueryGetMembers' });

  return (
    <>
      <Heading size='md'>DAO Members</Heading>
      <Box mt={4}>
        {isLoading && <Spinner />}
        {members && members.length === 0 && <Text>No members</Text>}
        {members && members.map((member, idx) => <Member address={member.address()} key={idx} />)}
      </Box>
    </>
  );
}
