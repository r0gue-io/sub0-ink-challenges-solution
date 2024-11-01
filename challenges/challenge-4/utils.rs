use super::*;
use superdao::SuperdaoRef as Superdao;

pub fn call_superdao(address: AccountId) -> Superdao {
    ink::env::call::FromAccountId::from_account_id(address)
}
