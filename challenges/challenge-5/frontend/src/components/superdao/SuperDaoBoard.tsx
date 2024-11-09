import { Box, Tab, TabList, TabPanel, TabPanels, Tabs } from '@chakra-ui/react';
import React from 'react';
import { InfoPanel } from '@/components/superdao/InfoPanel.tsx';
import { ProposalsPanel } from '@/components/superdao/ProposalsPanel.tsx';

export function SuperDaoBoard() {
  return (
    <Box>
      <Tabs orientation='vertical' isLazy>
        <TabList>
          <Tab>Info</Tab>
          <Tab>Proposals</Tab>
        </TabList>

        <TabPanels>
          <TabPanel pt={0}>
            <InfoPanel />
          </TabPanel>
          <TabPanel pt={0}>
            <ProposalsPanel />
          </TabPanel>
        </TabPanels>
      </Tabs>
    </Box>
  );
}
