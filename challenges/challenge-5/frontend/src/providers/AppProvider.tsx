import { createContext, useContext, useEffect, useState } from 'react';
import { ContractId } from '@/contracts/deployments.ts';
import { Challenge5ContractContractApi } from '@/contracts/types/challenge5-contract';
import { SuperdaoContractApi } from '@/contracts/types/superdao';
import { Props } from '@/types.ts';
import { DedotClient, PinnedBlock } from 'dedot';
import { Contract } from 'dedot/contracts';
import { useContract, useTypink } from 'typink';

interface AppContextProps {
  superDaoContract?: Contract<SuperdaoContractApi>;
  minidao5Contract?: Contract<Challenge5ContractContractApi>;
  bestBlock?: number;
  finalizedBlock?: number;
}

const AppContext = createContext<AppContextProps>({
  superDaoContract: undefined,
  minidao5Contract: undefined,
});

export const useApp = () => {
  return useContext(AppContext);
};

export function AppProvider({ children }: Props) {
  const { contract: superDaoContract } = useContract<SuperdaoContractApi>(ContractId.SUPER_DAO);
  const { contract: minidao5Contract } = useContract<Challenge5ContractContractApi>(ContractId.MINI_DAO_5);
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
    let done = false;

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
        .then((unsub) => {
          if (done) {
            unsub().catch(console.error);
          } else {
            unsub1 = unsub;
          }
        })
        .catch(console.error);

      client.rpc
        .chain_subscribeFinalizedHeads((newHead) => {
          setFinalizedBlock(newHead.number);
        })
        .then((unsub) => {
          if (done) {
            unsub().catch(console.error);
          } else {
            unsub2 = unsub;
          }
        })
        .catch(console.error);
    }

    return () => {
      done = true;
      unsub1 && unsub1();
      unsub2 && unsub2();
    };
  }, [client]);

  return (
    <AppContext.Provider value={{ superDaoContract, minidao5Contract, bestBlock, finalizedBlock }}>
      {children}
    </AppContext.Provider>
  );
}
