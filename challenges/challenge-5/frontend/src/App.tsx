import { Box, Flex, Tab, Tabs, TabPanel, TabList, TabPanels } from '@chakra-ui/react';
import React, { useState } from 'react';
import { useSearchParam } from 'react-use';
import { MiniDao5Board } from '@/components/minidao5/MiniDao5Board.tsx';
import BalanceInsufficientAlert from '@/components/shared/BalanceInsufficientAlert.tsx';
import { BlockInfo } from '@/components/shared/BlockInfo.tsx';
import MainFooter from '@/components/shared/MainFooter';
import MainHeader from '@/components/shared/MainHeader';
import { SuperDaoBoard } from '@/components/superdao/SuperDaoBoard.tsx';

// @ts-ignore
BigInt.prototype.toJSON = function () {
  return this.toString();
};

function App() {
  const tab = useSearchParam('tab');
  const tabIndex = tab ? parseInt(tab) : 0;
  const [index, setIndex] = useState(tabIndex);

  const handleTabsChange = (index: number) => {
    setIndex(index);
    history.pushState({}, '', location.pathname + `?tab=${index}`);
  };

  return (
    <Flex direction='column' minHeight='100vh'>
      <MainHeader />
      <Box maxWidth='container.lg' mx='auto' my={4} px={4} flex={1} w='full'>
        <BalanceInsufficientAlert />
        <BlockInfo />
        <Box>
          <Tabs index={index} onChange={handleTabsChange}>
            <TabList>
              <Tab>SuperDAO</Tab>
              <Tab>MiniDAO (Challenge 5)</Tab>
            </TabList>

            <TabPanels>
              <TabPanel>
                <SuperDaoBoard />
              </TabPanel>
              <TabPanel>
                <MiniDao5Board />
              </TabPanel>
            </TabPanels>
          </Tabs>
        </Box>
      </Box>
      <MainFooter />
    </Flex>
  );
}

export default App;
