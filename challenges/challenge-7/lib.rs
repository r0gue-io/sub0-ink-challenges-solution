#![cfg_attr(not(feature = "std"), no_std, no_main)]

// # ✒️ Challenge 7 (Bonus): Use the Pop API to create a fungibles token for token-backed voting
//
// - **Difficulty:** Mid
// - **Submission Criteria:** DAO contract must
//   - Use the fungibles Pop API to create a new asset.
//   - Mint the asset for newly registered voter.
//   - Use the asset for token-backed voting by creating a new storage item to track the `Prevote` of each Superdao `Proposal`.
//   - Registered voter in the Dao will use the minted tokens to vote on the `Prevote`.
//   - If number of approvals in the `Prevote` is more than the disapprovals after the `deadline`, submit the vote to the proposal on Superdao.
// - **Submission Guidelines:** Verify with R0GUE DevRel, post on X with GitHub link
// - **Prize:** Pop ring candy

#[ink::contract]
mod dao {
    use ink::{
        contract_ref,
        prelude::{string::String, vec::Vec},
        storage::{Mapping, StorageVec},
        xcm::prelude::*,
    };
    use minidao_common::*;
    use pop_api::v0::fungibles::traits::Psp22;
    use superdao_traits::{Call, ChainCall, ContractCall, SuperDao, Vote};

    #[derive(Clone, Default)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, PartialEq, Eq, ink::storage::traits::StorageLayout)
    )]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct Prevote {
        pub deadline: BlockNumber,
        pub aye_votes: Vec<(AccountId, Balance)>,
        pub nay_votes: Vec<(AccountId, Balance)>,
    }

    #[ink(storage)]
    pub struct Dao {
        name: String,
        prevotes: Mapping<u32, Prevote>,
        voters: StorageVec<AccountId>,
        token: AccountId,
        superdao: contract_ref!(SuperDao),
    }

    impl Dao {
        // Constructor that initializes the values for the contract.
        #[ink(constructor)]
        pub fn new(name: String, superdao: AccountId, token: AccountId) -> Self {
            // Register your Dao as a member of the Superdao.
            let mut instance = Self {
                name,
                token,
                superdao: superdao.into(),
                voters: StorageVec::new(),
                prevotes: Mapping::new(),
            };
            instance
                .superdao
                .register_member()
                .expect("Failed to register as a Superdao member");
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
            let proposal_id = self.superdao.create_proposal(call.clone())?;
            self.prevotes.insert(
                proposal_id,
                &Prevote {
                    // Prevoting period ends after 3 blocks.
                    deadline: self.env().block_number().saturating_add(3),
                    aye_votes: Vec::default(),
                    nay_votes: Vec::default(),
                },
            );
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
                input: Vec::default(),
                transferred_value: 0,
                ref_time_limit: 0,
                allow_reentry: false,
            });
            let proposal_id = self.superdao.create_proposal(call.clone())?;
            self.prevotes.insert(proposal_id, &Prevote::default());
            Ok(())
        }

        #[ink(message)]
        pub fn submit_prevote(&mut self, proposal_id: u32, approved: bool) -> Result<(), DaoError> {
            let caller = self.env().caller();
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            if !self.has_voter(caller) {
                return Err(DaoError::VoterNotRegistered);
            }

            // - Success: Dao member prevote is recoreded with the current balance of the voter.
            let psp22: contract_ref!(Psp22) = self.token.into();
            match self.prevotes.get(proposal_id) {
                Some(mut prevote) => {
                    let vote = (caller, psp22.balance_of(caller));
                    if approved {
                        prevote.aye_votes.push(vote);
                    } else {
                        prevote.nay_votes.push(vote);
                    }
                }
                None => return Err(DaoError::ProposalDoesNotExist),
            }
            Ok(())
        }

        #[ink(message)]
        pub fn vote_proposal(&mut self, proposal_id: u32) -> Result<(), DaoError> {
            let caller = self.env().caller();
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            if !self.has_voter(caller) {
                return Err(DaoError::VoterNotRegistered);
            }

            // - Error: Throw error `DaoError::ProposalDoesNotExist` if the proposal does not found.
            match self.prevotes.take(proposal_id) {
                Some(prevote) => {
                    // - Error: Vote submitted before the deadline.
                    if prevote.deadline > self.env().block_number() {
                        return Err(DaoError::PrevotePeriodIsNotEnded);
                    }

                    // - Error: Vote is not approved by voters to submit.
                    let total_aye: Balance =
                        prevote.aye_votes.iter().map(|(_, balance)| *balance).sum();
                    let total_nay: Balance =
                        prevote.nay_votes.iter().map(|(_, balance)| *balance).sum();
                    // - Success: Submit a vote to SuperDao proposal.
                    if total_aye < total_nay {
                        self.superdao.vote(proposal_id, Vote::Nay)?;
                    } else {
                        self.superdao.vote(proposal_id, Vote::Aye)?;
                    }
                }
                None => return Err(DaoError::ProposalDoesNotExist),
            }
            Ok(())
        }
    }
}
