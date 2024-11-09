import { Box, Tab, TabList, TabPanel, TabPanels, Tabs } from '@chakra-ui/react';
import React from 'react';
import { DaoInfo } from '@/components/minidao5/DaoInfo.tsx';
import { ProposalsPanel } from '@/components/minidao5/ProposalsPanel.tsx';

export function MiniDao5Board() {
  // TODO: List on-going proposal, create proposal
  return (
    <Box>
      <Tabs orientation='vertical'>
        <TabList>
          <Tab>Dao Info</Tab>
          <Tab>Proposals</Tab>
        </TabList>

        <TabPanels>
          <TabPanel pt={0}>
            <DaoInfo />
          </TabPanel>
          <TabPanel pt={0}>
            <ProposalsPanel />
          </TabPanel>
        </TabPanels>
      </Tabs>
    </Box>
  );
}
