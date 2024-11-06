#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub enum DaoError {
    // Voter is already registered in the Dao.
    VoterAlreadyRegistered,
    // Voter is not registered yet..
    VoterNotRegistered,
    // Voter already voted the proposal.
    VoterAlreadyVoted,
    // Proposal does not exist in the Dao.
    ProposalDoesNotExist,
    // No contract address.
    NoContractAddress,
}
