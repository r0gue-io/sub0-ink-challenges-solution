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

#[ink::contract]
mod dao {
    use ink::{
        prelude::string::String,
        storage::{Mapping, StorageVec},
    };
    use minidao_common::*;

    #[derive(Clone)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, PartialEq, Eq, ink::storage::traits::StorageLayout)
    )]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct BasicProposal {
        pub vote_count: u32,
    }

    #[ink(storage)]
    pub struct Dao {
        name: String,
        voters: StorageVec<AccountId>,
        votes: Mapping<AccountId, u32>,
        proposals: Mapping<u32, BasicProposal>,
        next_proposal_id: u32,
    }

    impl Dao {
        // Constructor that initializes the values for the contract.
        #[ink(constructor)]
        pub fn new(name: String) -> Self {
            Self {
                name,
                voters: StorageVec::new(),
                proposals: Mapping::new(),
                next_proposal_id: 0,
                votes: Mapping::new(),
            }
        }

        #[ink(message)]
        pub fn get_name(&self) -> String {
            // - Returns the name of the Dao
            self.name.clone()
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
        pub fn create_proposal(&mut self) -> Result<(), DaoError> {
            let caller = self.env().caller();
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            if !self.has_voter(caller) {
                return Err(DaoError::VoterNotRegistered);
            }

            // - Success: Create a new proposal that stores `votes` from `voters`
            self.proposals
                .insert(self.next_proposal_id, &BasicProposal { vote_count: 0 });
            self.next_proposal_id = self.next_proposal_id.saturating_add(1);
            Ok(())
        }

        #[ink(message)]
        pub fn remove_proposal(&mut self, proposal_id: u32) -> Result<(), DaoError> {
            let caller = self.env().caller();
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            if !self.has_voter(caller) {
                return Err(DaoError::VoterNotRegistered);
            }

            // - Error: Throw error `DaoError::ProposalDoesNotExist` if the proposal is not created
            if self.proposals.get(&proposal_id).is_none() {
                return Err(DaoError::ProposalDoesNotExist);
            }

            // - Success: Create a new proposal that stores `votes` from `voters`
            self.proposals.remove(proposal_id);
            Ok(())
        }

        #[ink(message)]
        pub fn get_proposal(&self, proposal_id: u32) -> Option<BasicProposal> {
            // - Success: Returns the proposal detail
            self.proposals.get(proposal_id)
        }

        #[ink(message)]
        pub fn vote(&mut self, proposal_id: u32) -> Result<(), DaoError> {
            let caller = self.env().caller();

            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            if !self.has_voter(caller) {
                return Err(DaoError::VoterNotRegistered);
            }
            let mut vote = self.votes.take(caller).unwrap_or_default();

            // - Error: Throw error `Error::ProposalDoesNotExist` if the proposal is not created
            let mut proposal = self
                .proposals
                .take(proposal_id)
                .ok_or(DaoError::ProposalDoesNotExist)
                .unwrap();

            // - Success: Vote on the proposal
            proposal.vote_count = proposal.vote_count.saturating_add(1);
            self.proposals.insert(proposal_id, &proposal);

            vote = vote.saturating_add(1);
            self.votes.insert(caller, &vote);
            Ok(())
        }

        #[ink(message)]
        pub fn vote_count(&self, voter: AccountId) -> u32 {
            // - Returns the number of `votes` a Dao `voter` voted
            self.votes.get(voter).unwrap_or_default()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::dao::Dao;

        #[ink::test]
        fn test_voter_registration() {
            let mut dao = Dao::default();
            let accounts = ink::env::test::default_accounts::<Environment>();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            assert!(dao.register_voter().is_ok());
            assert_eq!(dao.register_voter(), Err(DaoError::VoterAlreadyRegistered));
            assert_eq!(dao.voters.len(), 1);
            assert!(dao.has_voter(accounts.alice));

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            assert!(dao.deregister_voter().is_ok());
            assert_eq!(dao.deregister_voter(), Err(DaoError::VoterNotRegistered));
            assert_eq!(dao.voters.get(0), None);
            assert!(!dao.has_voter(accounts.alice));
        }

        #[ink::test]
        fn test_proposal_management() {
            let mut dao = Dao::default();
            let proposal = 0;
            let accounts = ink::env::test::default_accounts::<Environment>();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            assert!(dao.register_voter().is_ok());
            assert!(dao.create_proposal().is_ok());
            assert_eq!(
                dao.get_proposal(proposal),
                Some(BasicProposal { vote_count: 0 })
            );

            assert!(dao.remove_proposal(proposal).is_ok());
            assert_eq!(dao.get_proposal(proposal), None);
        }

        #[ink::test]
        fn test_vote() {
            let mut dao = Dao::default();
            let proposal = 0;
            let accounts = ink::env::test::default_accounts::<Environment>();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            assert!(dao.register_voter().is_ok());
            assert!(dao.create_proposal().is_ok());
            assert!(dao.vote(proposal).is_ok());
            assert_eq!(
                dao.get_proposal(proposal),
                Some(BasicProposal { vote_count: 1 })
            );
        }
    }
}
