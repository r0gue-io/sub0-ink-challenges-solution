#![cfg_attr(not(feature = "std"), no_std, no_main)]

// # ✒️ Challenge 5: Build a UI for your DAO
//
// NOTE: Using this contract to combine the functionalities of challenge 4 contract and challenge 3 contract. Compile and deploy on Pop Network then building a UI for it.
//
// - **Difficulty:** Mid
// - **Submission Criteria:** The UI must support
//     - Registering & viewing members
//     - Voting on and viewing proposals
//     - Viewing overall proposal votes
// - **Submission Guidelines:** Verify with R0GUE or Dedot DevRel, and post on X
// - **Prize:** Sub0 merch & ink! sports towel

use crate::types::*;

use ink::{
    contract_ref,
    prelude::{string::String, vec},
    storage::StorageVec,
    xcm::prelude::*,
};
use superdao_traits::{Call, ChainCall, ContractCall, SuperDao, Vote};

mod types;

#[ink::contract]
mod dao {
    use super::*;

    #[ink(storage)]
    pub struct Dao {
        superdao: contract_ref!(SuperDao),
        name: String,
        voters: StorageVec<AccountId>,
    }

    impl Dao {
        // Constructor that initializes the values for the contract.
        #[ink(constructor)]
        pub fn new(name: String, superdao: AccountId) -> Self {
            // Register your Dao as a member of the Superdao.
            let mut instance = Self {
                name,
                superdao: superdao.into(),
                voters: StorageVec::new(),
            };
            instance.superdao.register_member();
            instance
        }

        #[ink(message)]
        pub fn register_voter(&mut self) -> Result<(), DaoError> {
            let caller = self.env().caller();
            // - Error: Throw error `DaoError::VoterAlreadyRegistered` if the voter is registered
            if self.has_voter(caller) {
                return Err(DaoError::VoterAlreadyRegistered);
            }
            // - Success: Register a new `voter` to the Dao
            self.voters.push(&caller);
            Ok(())
        }

        #[ink(message)]
        pub fn deregister_voter(&mut self) -> Result<(), DaoError> {
            let caller = self.env().caller();
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            if !self.has_voter(caller) {
                return Err(DaoError::VoterNotRegistered);
            }

            // - Success: Deregister a new `voter` from the Dao
            for i in 0..self.voters.len() {
                if let Some(voter) = self.voters.get(i) {
                    if voter == caller {
                        self.voters.clear_at(i);
                    }
                }
            }
            Ok(())
        }

        #[ink(message)]
        pub fn has_voter(&self, voter: AccountId) -> bool {
            let mut registered = false;
            for i in 0..self.voters.len() {
                if let Some(v) = self.voters.get(i) {
                    if v == voter {
                        registered = true;
                        break;
                    }
                }
            }
            registered
        }

        #[ink(message)]
        pub fn create_superdao_cross_chain_proposal(&mut self) -> Result<(), DaoError> {
            let caller = self.env().caller();
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            if !self.has_voter(caller) {
                return Err(DaoError::VoterNotRegistered);
            }

            // - Success: Create a SuperDao proposal to execute a cross-chain message.
            let location = Location::here();
            let msg: Xcm<()> = Xcm::new();
            let call = Call::Chain(ChainCall::new(&location, &msg));
            self.superdao.create_proposal(call.clone());
            Ok(())
        }

        #[ink(message)]
        pub fn create_contract_call_proposal(&mut self) -> Result<(), DaoError> {
            let caller = self.env().caller();
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            if !self.has_voter(caller) {
                return Err(DaoError::VoterNotRegistered);
            }

            // - Success: Create a SuperDao proposal to call a contract method.
            let call = Call::Contract(ContractCall {
                callee: self.env().caller(),
                selector: [0; 4],
                input: vec![],
                transferred_value: 0,
                ref_time_limit: 0,
                allow_reentry: false,
            });
            self.superdao.create_proposal(call.clone());
            Ok(())
        }

        #[ink(message)]
        pub fn vote_proposal(&mut self, proposal_id: u32, vote: bool) -> Result<(), DaoError> {
            let caller = self.env().caller();
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            if !self.has_voter(caller) {
                return Err(DaoError::VoterNotRegistered);
            }

            // - Success: Vote a SuperDao proposal.
            let vote = if vote { Vote::Aye } else { Vote::Nay };
            self.superdao.vote(proposal_id, vote);
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::dao::Dao;

        #[ink::test]
        fn test_vote_superdao_cross_chain_proposal() {
            todo!("Challenge 4");
        }
    }
}
