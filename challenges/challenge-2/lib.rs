#![cfg_attr(not(feature = "std"), no_std, no_main)]

// # ✒️ Challenge 2: Implement voter registration, proposal management, and voting in your Dao.
//
// - **Difficulty**: Mid
// - **Submission Criteria:** ink! contract must
//     - Use a storage-optimized data-structure `Mapping` or `StorageVec`
//     - Store registered members, member votes, and proposals to vote on.
//     - Have method to register and de-register members.
//     - Implement the `GovernanceDao` trait methods.
//     - Have methods to create proposals and a method to vote on proposals.
//     - Unit tests for adding members, votes, and proposals.
// - **Submission Guidelines:**
//     - Verify with R0GUE DevRel, and post on X.
// - **Prize:** sub0 merch

use ink::primitives::AccountId;

mod types;

#[ink::contract]
mod dao {
    use super::*;

    #[ink(storage)]
    pub struct Dao {
        value: bool,
    }

    impl Dao {
        // Constructor that initializes the values for the contract.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        // Constructor that initializes the default values for the contract.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn register_voter(&mut self) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::MemberAlreadyRegistered` if the member is registered
            // - Success: Register a new `member` to the Dao
            Ok(())
        }

        #[ink(message)]
        pub fn deregister_voter(&mut self) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::MemberNotRegistered` if the member is not registered
            // - Success: Deregister a new `voter` from the Dao
            Ok(())
        }

        #[ink(message)]
        pub fn has_voter(&self, voter: AccountId) -> bool {
            // - Success: Return if the voter is registered.
            todo!();
        }

        #[ink(message)]
        pub fn create_proposal(&mut self) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::MemberNotRegistered` if the member is not registered
            // - Success: Create a new proposal that stores `votes` from `voters`
            Ok(())
        }

        #[ink(message)]
        pub fn remove_proposal(&mut self, proposal_id: u32) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::MemberNotRegistered` if the member is not registered
            // - Error: Throw error `DaoError::ProposalDoesNotExist` if the proposal is not created
            // - Success: Create a new proposal that stores `votes` from `voters`
            Ok(())
        }

        #[ink(message)]
        pub fn vote(&mut self, proposal_id: u32) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::MemberNotRegistered` if the member is not registered
            // - Error: Throw error `DaoError::ProposalDoesNotExist` if the proposal is not created
            // - Success: Vote on the proposal
            Ok(())
        }

        #[ink(message)]
        pub fn get_proposal(&self, proposal_id: u32) -> Option<BasicProposal> {
            // - Success: Returns the proposal detail
            todo!();
        }

        #[ink(message)]
        pub fn vote_count(&self, voter: AccountId) -> u32 {
            // - Returns the number of `votes` a Dao `voter` voted
            todo!();
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::dao::Dao;

        #[ink::test]
        fn test_voter_registration() {
            todo!("Challenge 2");
        }

        #[ink::test]
        fn test_proposal_management() {
            todo!("Challenge 2");
        }

        #[ink::test]
        fn test_vote() {
            todo!("Challenge 3");
        }
    }
}
