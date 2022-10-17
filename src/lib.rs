use std::collections::HashMap;
use borsh::{BorshDeserialize, BorshSerialize};
use smart_contract::{contract, view, action, init, ContractField};


#[contract]
pub struct TestContract {}

#[contract]
impl TestContract {  
    pub fn test() {
        println!("Deploy!");
    }
}

