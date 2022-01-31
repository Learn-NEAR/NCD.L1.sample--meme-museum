use utils::validate;

use museum::Contract;

use near_sdk::{
    testing_env,
    AccountId, 
    // BlockchainInterface, 
    test_utils::VMContextBuilder,
    MockedBlockchain,
};

// == CONFIG VALUES ============================================================
// Using functions because the compiler doesn't like the idea of global const strings

#[allow(dead_code, non_snake_case)]
pub fn NAME() -> AccountId {
    AccountId::try_from(String::from("usain")).unwrap()
}

#[allow(dead_code, non_snake_case)]
pub fn MUSEUM_ACCOUNT_ID() -> AccountId {
    AccountId::try_from(String::from("museum")).unwrap()
}

#[allow(dead_code, non_snake_case)]
pub fn OWNER_ACCOUNT_ID() -> AccountId {
    AccountId::try_from(String::from("alice")).unwrap()
}

#[allow(dead_code, non_snake_case)]
pub fn CONTRIBUTOR_ACCOUNT_ID() -> AccountId {
    AccountId::try_from(String::from("bob")).unwrap()
}

#[allow(dead_code)]
pub fn do_initialize() -> Contract {
    let mut builder: VMContextBuilder = VMContextBuilder::new();

    builder.attached_deposit(utils::MIN_ACCOUNT_BALANCE);
    builder.predecessor_account_id(validate(&MUSEUM_ACCOUNT_ID()));


    testing_env!(builder.build());

    Contract::new(NAME().to_string(), Vec::from([OWNER_ACCOUNT_ID()]))
}
