import minidao5Metadata from '@/contracts/minidao/challenge_5_contract.json';
import superdaoMetadata from '@/contracts/superdao/superdao.json';
import { ContractDeployment } from '@/types.ts';
import { NetworkId } from '@/utils/networks.ts';

export enum ContractId {
  SUPER_DAO = 'SUPER_DAO',
  MINI_DAO_5 = 'MINI_DAO_5',
}

export const deployments: ContractDeployment[] = [
  {
    id: ContractId.SUPER_DAO,
    metadata: superdaoMetadata as any,
    network: NetworkId.POP_TESTNET,
    address: '136WER67N7eEumefDxANeez12fGuoj1amJTPa2rFEmNxWtjU',
  },
  {
    id: ContractId.MINI_DAO_5,
    metadata: minidao5Metadata as any,
    network: NetworkId.POP_TESTNET,
    address: '14r5YvZoreg6YG9d5ywBbHVxBYL9t35uJufNbUDE2eagdysq',
  },
];
