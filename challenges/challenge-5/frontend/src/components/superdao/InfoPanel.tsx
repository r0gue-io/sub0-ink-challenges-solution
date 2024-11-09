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

export function InfoPanel() {
  const { superDaoContract: contract } = useApp();
  const { data: voteThreshold, isLoading: loadingThreshold } = useContractQuery({
    contract,
    fn: 'superDaoQueryGetVoteThreshold',
  });
  const { data: votingPeriod, isLoading: loadingPeriod } = useContractQuery({
    contract,
    fn: 'superDaoQueryGetVotingPeriod',
  });
  const { data: members, isLoading } = useContractQuery({ contract, fn: 'superDaoQueryGetMembers' });

  return (
    <>
      <Heading size='md'>Basic Info</Heading>
      <Box mt={2}>
        <Box>
          Voting Threshold:{' '}
          <PendingText isLoading={loadingThreshold}>
            <b>{voteThreshold}</b>
          </PendingText>
        </Box>
        <Box>
          Voting Period:{' '}
          <PendingText isLoading={loadingPeriod}>
            <b>{votingPeriod}</b>
          </PendingText>
        </Box>
      </Box>
      <Heading size='md' mt={4}>
        DAO Members
      </Heading>
      <Box mt={2}>
        {isLoading && <Spinner />}
        {members && members.length === 0 && <Text>No members</Text>}
        {members && members.map((member, idx) => <Member address={member.address()} key={idx} />)}
      </Box>
    </>
  );
}
