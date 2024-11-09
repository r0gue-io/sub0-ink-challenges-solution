// Generated by dedot cli

import type { GenericSubstrateApi } from 'dedot/types';
import type { Result } from 'dedot/codecs';
import type {
  GenericConstructorQuery,
  GenericConstructorQueryCall,
  GenericConstructorCallResult,
  ConstructorCallOptions,
  ContractInstantiateResult,
} from 'dedot/contracts';
import type { InkPrimitivesLangError } from './types';

export interface ConstructorQuery<ChainApi extends GenericSubstrateApi> extends GenericConstructorQuery<ChainApi> {
  /**
   *
   * @param {ConstructorCallOptions} options
   *
   * @selector 0x9bae9d5e
   **/
  new: GenericConstructorQueryCall<
    ChainApi,
    (options?: ConstructorCallOptions) => Promise<GenericConstructorCallResult<[], ContractInstantiateResult<ChainApi>>>
  >;
}
