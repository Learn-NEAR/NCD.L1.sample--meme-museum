mod common;

use near_sdk::{
    testing_env,
    MockedBlockchain,
};
use near_sdk::{
    test_utils::VMContextBuilder, 
};

use utils::validate;

use common::{NAME, OWNER_ACCOUNT_ID};

use museum::Contract;

#[test]
fn creates_a_new_museum_with_proper_metadata() {
    let mut builder: VMContextBuilder = VMContextBuilder::new();
    builder.predecessor_account_id(validate("museum"));
    builder.attached_deposit(utils::MIN_ACCOUNT_BALANCE);
    testing_env!(builder.build());

    let mut contract = Contract::new(NAME().to_string(), Vec::from([OWNER_ACCOUNT_ID()]));
    let m = contract.get_museum();
    assert_eq!(m.name, NAME().to_string());
    assert_eq!(contract.globals.owners.len(), 1);
}

#[test]
#[should_panic(expected = "Contract is already initialized")]
fn prevents_double_initialization() {
    let mut builder = VMContextBuilder::new();
    builder.predecessor_account_id(validate("museum"));
    builder.attached_deposit(utils::MIN_ACCOUNT_BALANCE);
    testing_env!(builder.build());

    let _contract = Contract::new(NAME().to_string(), Vec::from([OWNER_ACCOUNT_ID()]));
    let _new_contract = Contract::new(NAME().to_string(), Vec::from([OWNER_ACCOUNT_ID()]));
}

#[test]
#[should_panic(expected = "Museum name may not be blank")]
fn require_title_not_to_be_blank() {
    let mut builder: VMContextBuilder = VMContextBuilder::new();
    builder.predecessor_account_id(validate("museum"));
    builder.attached_deposit(utils::MIN_ACCOUNT_BALANCE);
    testing_env!(builder.build());

    let _contract = Contract::new(String::from(""), Vec::from([OWNER_ACCOUNT_ID()]));
}

#[test]
#[should_panic(
    expected = "Minimum account balance must be attached to initialize this contract (3 NEAR)"
)]
fn requires_a_minimum_balance() {
    let mut builder = VMContextBuilder::new();
    builder.predecessor_account_id(validate("museum"));
    testing_env!(builder.build());

    let _contract = Contract::new(NAME().to_string(), Vec::from([OWNER_ACCOUNT_ID()]));
}
