import { createContext, useContext } from 'react';
import { ContractId } from '@/contracts/deployments.ts';
import { Challenge5ContractContractApi } from '@/contracts/types/challenge5-contract';
import { SuperdaoContractApi } from '@/contracts/types/superdao';
import { Props } from '@/types.ts';
import { Contract } from 'dedot/contracts';
import { useContract } from 'typink';

interface AppContextProps {
  superDaoContract?: Contract<SuperdaoContractApi>;
  minidao5Contract?: Contract<Challenge5ContractContractApi>;
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
  return <AppContext.Provider value={{ superDaoContract, minidao5Contract }}>{children}</AppContext.Provider>;
}
