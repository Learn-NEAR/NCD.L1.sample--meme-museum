use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen};
// use near_sdk::serde::{Deserialize, Serialize};


near_sdk::setup_alloc!();


#[near_bindgen]
#[derive(Clone, Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    id: u32,
}

#[near_bindgen]
impl Contract{
    pub fn increment(&mut self) -> u32 {
        self.id += 1;
        self.id
    }
}