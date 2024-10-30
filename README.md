<div align="center">

# Sub0 `ink!` Challenges


ink! programming language challenges made for the Sub0 Reset Bangkok

</p>

<img height="70px" alt="Polkadot SDK Logo" src="https://github.com/user-attachments/assets/c60b6b92-a263-480c-b297-3535454ad3f6"/>

</div>

## Getting started

Before tackling the challenges, you need to setup your local environment for ink! contract development first.

- Install `pop-cli`: https://learn.onpop.io/cli/installing-pop-cli/

## About the challenges

To tackle the challenges, first fork and clone the repository

```
git clone https://github.com/r0gue-io/sub0-ink-challenges.git
```

To test your contract, run:

```
cargo test
```

---

### ✒️ Challenge 1: Basics of ink! and setting up a DAO contract

- **Difficulty**: Easy
- **Submission Criteria:** ink! contract must
  - Have a constructor accepting a name parameter.
  - Have a storage field for the DAO name.
  - Implement the `BasicDao` trait methods.
  - Unit test for constructor and setting DAO name.
  - Be built and deployed on Pop Network testnet.
- **Submission Guidelines:**
  - All test with prefix `challenge_1__` must pass.
  - Verify with R0GUE DevRel, and post on X.
- **Prize:** sub0 merch

---

### ✒️ Challenge 2: Membership and voting mechanism to the DAO.

- **Difficulty**: Mid
- **Submission Criteria:** ink! contract must
  - Use a storage-optimized data-structure `Mapping` or `StorageVec`
  - Store registered members, member votes, and proposals to vote on.
  - Have method to register and de-register members.
  - Implement the `GovernanceDao` trait methods.
  - Have methods to create proposals and a method to vote on proposals.
  - Unit tests for adding members, votes, and proposals.
- **Submission Guidelines:**
  - All test with prefix `challenge_2__` must pass.
  - Verify with R0GUE DevRel, and post on X.
- **Prize:** sub0 merch

---

### ✒️ Challenge 3:- Connect your DAO to the Super DAO with registration and voting

- **Difficulty**: Mid
- **Submission Criteria:** ink! contract must
  - Import the Super DAO trait>
  - Store Super DAO contract address.
  - Register contract as member of Super DAO - using trait-based contract calling.
  - Vote on proposals in the Super DAO - using trait-based contract calling.
  - Create proposals to call another contract - using trait-based contract calling.
  - E2E test for cross-contract call.
- **Submission Guidelines:**
  - All test with prefix `challenge_3__` must pass.
  - Verify with R0GUE DevRel, and post on X.
- **Prize:** Sub0 Merch & ink! sports towel

---

### ✒️ Challenge 4: Support creating cross-chain proposals to the Super DAO

- **Difficulty**: Advanced
- **Submission Criteria:** ink! contract must
  - Support creating cross-chain proposals to the Super DAO (XCM)
  - A deployed contract on Pop Network Testnet
  - Have a cross-chain proposal successfully executed
- **Submission Guidelines:**
  - All test with prefix `challenge_4__` must pass.
  - Verify with R0GUE DevRel, and post on X.
- **Prize:** Sub0 merch

---

### ✒️ Challenge 5: Build a UI for your DAO

- **Difficulty:** Mid
- **Submission Criteria:** The UI must support
  - Registering & viewing members
  - Voting on and viewing proposals
  - Viewing overall proposal votes
- **Submission Guidelines:** Verify with R0GUE or Dedot DevRel, and post on X
- **Prize:** Sub0 merch & ink! sports towel

---

### ✒️ Challenge 6: Ideate and design a multichain, permissionless solution for Polkadot

- **Difficulty:** Easy
- **Submission Criteria:** A PDF containing
  - A write-up of their proposal
  - What their idea solves, how it works, and team members (if applicable)
- **Submission Guidelines:** Verify with R0GUE DevRel

---

### ✒️ Challenge 7 (Bonus): Use the Pop API to create a fungibles token for token-backed voting

- **Difficulty:** Mid
- **Submission Criteria:** DAO contract must
  - Use the fungibles Pop API to create a new asset, mint the asset, and use the asset for token-backed voting
- **Submission Guidelines:** Verify with R0GUE DevRel, post on X with GitHub link
- **Prize:** Pop ring candy
