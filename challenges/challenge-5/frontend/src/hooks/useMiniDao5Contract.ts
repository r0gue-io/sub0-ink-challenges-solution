import { useMemo } from 'react';
import { ContractId } from '@/contracts/deployments.ts';
import { Challenge5ContractContractApi } from '@/contracts/types/challenge5-contract';
import { useTypink } from 'typink';
import { useRawContract } from 'typink/hooks/useRawContract';

export function useMiniDao5Contract(address: string) {
  const { deployments } = useTypink();

  const minidao5Metadata = useMemo(() => {
    return deployments.find((d) => d.id === ContractId.MINI_DAO_5)?.metadata;
  }, [deployments]);

  return useRawContract<Challenge5ContractContractApi>(minidao5Metadata, address);
}
