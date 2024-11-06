#![cfg_attr(not(feature = "std"), no_std, no_main)]

// # ✒️ Challenge 1: Basics of ink! and setting up a DAO contract
//
// - **Difficulty**: Easy
// - **Submission Criteria:** ink! contract must
//     - Have a constructor accepting a name parameter.
//     - Have a storage field for the DAO name.
//     - Implement the `BasicDao` trait methods.
//     - Unit test for constructor and setting DAO name.
//     - Be built and deployed on Pop Network testnet.
// - **Submission Guidelines:**
//     - Verify with R0GUE DevRel, and post on X.
// - **Prize:** sub0 merch

use ink::prelude::string::String;

#[ink::contract]
mod dao {
    use super::*;

    #[ink(storage)]
    pub struct Dao {
        name: String,
    }

    impl Dao {
        // Constructor that initializes the values for the contract.
        #[ink(constructor)]
        pub fn new(name: String) -> Self {
            Self { name }
        }

        // Constructor that initializes the default values for the contract.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn name(&self) -> String {
            // - Returns the name of the Dao
            self.name.clone()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::dao::Dao;

        #[ink::test]
        fn test_name() {
            let dao = Dao::new(String::from("any name"));
            assert_eq!(dao.name, String::from("any name"));
        }
    }
}
