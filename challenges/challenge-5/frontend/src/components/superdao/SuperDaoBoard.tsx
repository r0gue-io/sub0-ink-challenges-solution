import { Box, Tab, TabList, TabPanel, TabPanels, Tabs } from '@chakra-ui/react';
import React from 'react';
import { MembersPanel } from '@/components/superdao/MembersPanel.tsx';
import { ProposalsPanel } from '@/components/superdao/ProposalsPanel.tsx';
import { ContractId } from '@/contracts/deployments.ts';
import { SuperdaoContractApi } from '@/contracts/types/superdao';
import useContract from '@/hooks/useContract.ts';

export function SuperDaoBoard() {
  return (
    <Box>
      <Tabs orientation='vertical'>
        <TabList>
          <Tab>Members</Tab>
          <Tab>Proposals</Tab>
        </TabList>

        <TabPanels>
          <TabPanel pt={0}>
            <MembersPanel />
          </TabPanel>
          <TabPanel pt={0}>
            <ProposalsPanel />
          </TabPanel>
        </TabPanels>
      </Tabs>
    </Box>
  );
}