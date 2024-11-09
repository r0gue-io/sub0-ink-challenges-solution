import { Box, Heading, Spinner, Text } from '@chakra-ui/react';
import React from 'react';
import useContractQuery from '@/hooks/useContractQuery.ts';
import { useApp } from '@/providers/AppProvider.tsx';

export function MembersPanel() {
  const { superDaoContract: contract } = useApp();
  const { data: members, isLoading } = useContractQuery({ contract, fn: 'superDaoQueryGetMembers' });

  return (
    <>
      <Heading size='md'>DAO Members</Heading>
      <Box mt={4}>
        {isLoading && <Spinner />}
        {members && members.length === 0 && <Text>No members</Text>}
        {members &&
          members.map((member, idx) => (
            <Box border='1px' borderColor='gray.200' p={4}>
              <Text key={idx}>{member.address()}</Text>
            </Box>
          ))}
      </Box>
    </>
  );
}
