#![cfg_attr(not(feature = "std"), no_std, no_main)]

// # ✒️ Challenge 3: Connect your DAO to the Super DAO with registration and voting
//
// - **Difficulty**: Mid
// - **Submission Criteria:** ink! contract must
//     - Import the Super DAO trait>
//     - Store Super DAO contract address.
//     - Register contract as member of Super DAO - using trait-based contract calling.
//     - Vote on proposals in the Super DAO - using trait-based contract calling.
//     - Create proposals to call another contract - using trait-based contract calling.
//     - E2E test for cross-contract call.
// - **Submission Guidelines:**
//     - Verify with R0GUE DevRel, and post on X.
// - **Prize:** Sub0 Merch & ink! sports towel

use crate::types::*;
use ink::{
    contract_ref,
    prelude::{string::String, vec},
    storage::StorageVec,
};
use superdao_traits::{Call, ContractCall, SuperDao, Vote};

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
        fn test_create_superdao_contract_call_proposal() {
            todo!("Challenge 3");
        }

        #[ink::test]
        fn test_vote_superdao_proposal() {
            todo!("Challenge 3");
        }
    }
}
