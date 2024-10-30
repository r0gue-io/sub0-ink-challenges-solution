#![cfg_attr(not(feature = "std"), no_std, no_main)]

// # List of challenges
//
// Challenge 1: Basics of ink! and setting up a Dao contract
// Challenge 2: Implement voter registration, proposal management, and voting in your Dao.
// Challenge 3: Connect your DAO to the Super DAO with registration and voting
// Challenge 4: Support creating cross-chain proposals to the Super DAO

use crate::{traits::*, types::*};
use ink::codegen::Env;
use ink::{
    env::{DefaultEnvironment, Environment},
    primitives::AccountId,
    storage::Mapping,
    storage::StorageVec,
};
use superda0_traits::superdao::{Call, Proposal as SuperDaoProposal};

mod traits;
mod types;
mod utils;

#[ink::contract]
mod dao {
    use super::*;

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

        // Constructor that initializes the default values for the contract.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }
    }

    impl NamedDao for Dao {
        #[ink(message)]
        fn name(&self) -> String {
            // - Returns the name of the Dao
            self.name.clone()
        }
    }

    impl BasicDao for Dao {
        #[ink(message)]
        fn register_voter(&mut self) -> Result<(), DaoError> {
            let caller = self.env().caller();
            // - Error: Throw error `DaoError::MemberAlreadyRegistered` if the member is registered
            if self.has_voter(caller) {
                return Err(DaoError::MemberAlreadyRegistered);
            }
            // - Success: Register a new `member` to the Dao
            self.voters.push(&caller);
            Ok(())
        }

        #[ink(message)]
        fn deregister_voter(&mut self) -> Result<(), DaoError> {
            let caller = self.env().caller();
            // - Error: Throw error `DaoError::MemberNotRegistered` if the member is not registered
            if !self.has_voter(caller) {
                return Err(DaoError::MemberNotRegistered);
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
        fn has_voter(&self, voter: AccountId) -> bool {
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
        fn create_proposal(&mut self) -> Result<(), DaoError> {
            let caller = self.env().caller();
            // - Error: Throw error `DaoError::MemberNotRegistered` if the member is not registered
            if !self.has_voter(caller) {
                return Err(DaoError::MemberNotRegistered);
            }

            // - Success: Create a new proposal that stores `votes` from `voters`
            self.proposals
                .insert(self.next_proposal_id, &BasicProposal { vote_count: 0 });
            self.next_proposal_id = self.next_proposal_id.saturating_add(1);
            Ok(())
        }

        #[ink(message)]
        fn remove_proposal(&mut self, proposal_id: u32) -> Result<(), DaoError> {
            let caller = self.env().caller();
            // - Error: Throw error `DaoError::MemberNotRegistered` if the member is not registered
            if !self.has_voter(caller) {
                return Err(DaoError::MemberNotRegistered);
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
        fn get_proposal(&self, proposal_id: u32) -> Option<BasicProposal> {
            // - Success: Returns the proposal detail
            self.proposals.get(proposal_id)
        }

        #[ink(message)]
        fn vote(&mut self, proposal_id: u32) -> Result<(), DaoError> {
            let caller = self.env().caller();

            // - Error: Throw error `DaoError::MemberNotRegistered` if the member is not registered
            if !self.has_voter(caller) {
                return Err(DaoError::MemberNotRegistered);
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
        fn vote_count(&self, voter: AccountId) -> u32 {
            // - Returns the number of `votes` a Dao `voter` voted
            self.votes.get(voter).unwrap_or_default()
        }

        #[ink(message)]
        fn vote_count(&self, voter: AccountId) -> u32 {
            // - Returns the number of `votes` a Dao `voter` voted
            todo!();
        }
    }

    impl SubDao for Dao {
        #[ink(message)]
        fn create_superdao_contract_call_proposal(&mut self) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Success: Create a SuperDao proposal to call a contract method.
            Ok(())
        }

        #[ink(message)]
        fn vote_superdao_proposal(&mut self, proposal_id: u32, vote: bool) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Success: Vote a SuperDao proposal.
            Ok(())
        }

        #[ink(message)]
        fn create_superdao_cross_chain_proposal(&mut self) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Success: Create a SuperDao proposal to execute a cross-chain message.
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::dao::Dao;

        #[ink::test]
        fn mock_test() {
            assert_eq!(1 + 1, 2);
        }

        #[ink::test]
        fn challenge_1_test_name() {
            let dao = Dao::new(String::from("any name"));
            assert_eq!(dao.name, String::from("any name"));
        }

        #[ink::test]
        fn challenge_2_test_voter_registration() {
            let mut dao = Dao::default();
            let accounts = ink::env::test::default_accounts::<Environment>();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            assert!(dao.register_voter().is_ok());
            assert_eq!(dao.register_voter(), Err(DaoError::MemberAlreadyRegistered));
            assert_eq!(dao.voters.len(), 1);
            assert!(dao.has_voter(accounts.alice));

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            assert!(dao.deregister_voter().is_ok());
            assert_eq!(dao.deregister_voter(), Err(DaoError::MemberNotRegistered));
            assert_eq!(dao.voters.get(0), None);
            assert!(!dao.has_voter(accounts.alice));
        }

        #[ink::test]
        fn challenge_2_test_proposal_management() {
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
        fn challenge_2_test_vote() {
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
