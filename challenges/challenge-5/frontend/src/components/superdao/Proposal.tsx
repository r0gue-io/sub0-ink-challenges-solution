import { Box, Code, Text } from '@chakra-ui/react';
import React from 'react';
import type { Superda0TraitsSuperdaoProposal } from '@/contracts/types/superdao';
import { useApp } from '@/providers/AppProvider.tsx';
import { Props } from '@/types.ts';

interface ProposalProps extends Props {
  proposal: Superda0TraitsSuperdaoProposal;
}

export function Proposal({ proposal }: ProposalProps) {
  const { superDaoContract: contract } = useApp();
  // const {} = useContractTx(contract, '');

  return (
    <Box border='1px' borderColor='gray.200' p={4}>
      <Text>
        Proposal Type: <b>{proposal.call.type}</b>
      </Text>
      <Text>
        Voting period end: <b>{proposal.votingPeriodEnd.toString()}</b>
      </Text>
      <Code p={4} mt={2} whiteSpace='pre-wrap'>
        {JSON.stringify(proposal.call.value, null, 2)}
      </Code>
    </Box>
  );
}
