import { Box, Flex, Heading, Spinner, Text } from '@chakra-ui/react';
import React from 'react';
import { Proposal } from '@/components/superdao/Proposal.tsx';
import { useApp } from '@/providers/AppProvider.tsx';
import { useContractQuery } from 'typink';

export function ProposalsPanel() {
  const { superDaoContract: contract } = useApp();
  const {
    data: proposals,
    isLoading,
    refresh: refreshProposals,
  } = useContractQuery({ contract, fn: 'superDaoQueryGetProposals' });

  return (
    <>
      <Heading size='md'>Proposals</Heading>
      <Box mt={4}>
        {isLoading && <Spinner />}
        {proposals && proposals.length === 0 && <Text>No proposals</Text>}
        {proposals && (
          <Flex direction='column' gap={2}>
            {proposals.map(([index, p], idx) => (
              <Proposal
                proposal={p}
                index={index}
                key={idx}
                onExecuted={() => {
                  refreshProposals();
                }}
              />
            ))}
          </Flex>
        )}
      </Box>
    </>
  );
}
