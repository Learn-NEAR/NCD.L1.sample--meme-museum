mod common;

use near_sdk::{
    testing_env,  
    MockedBlockchain,
    test_utils::VMContextBuilder,
    json_types::{
        U128,
        // U64,
        ValidAccountId,
    },
};


use common::{
    ATTACHED_DEPOSIT,
    CATEGORY,
    CONTRIBUTOR_ACCOUNT_ID,
    CREATOR_ACCOUNT_ID,
    DATA,
    doInitialize,
    TITLE,
};

use meme::{
    Contract,
};


fn validate(id: &str) -> ValidAccountId {
    ValidAccountId::try_from(id).unwrap()
}

fn contract() -> Contract {
    Contract::new(TITLE(), DATA(), CATEGORY())
}

#[test]
fn captures_donations() {
    let mut builder: VMContextBuilder = doInitialize(false);
    builder.attached_deposit(u128::from(ATTACHED_DEPOSIT()));
    builder.signer_account_id(validate(&CREATOR_ACCOUNT_ID()));
    builder.predecessor_account_id(validate(&CREATOR_ACCOUNT_ID()));
    testing_env!(builder.build());

    let mut contract = contract();
    contract.donate();

    assert_eq!(U128::from(contract.get_meme().total_donations), ATTACHED_DEPOSIT());
}

/// setup for the following tests
fn setup(is_view: bool) -> Contract{
    let mut builder: VMContextBuilder = doInitialize(is_view);
    builder.attached_deposit(u128::from(ATTACHED_DEPOSIT()));
    builder.signer_account_id(validate(&CREATOR_ACCOUNT_ID()));
    builder.predecessor_account_id(validate(&CREATOR_ACCOUNT_ID()));
    testing_env!(builder.clone().build());

    let mut contract = contract();
    contract.donate();

    builder.signer_account_id(validate(&CONTRIBUTOR_ACCOUNT_ID()));
    builder.predecessor_account_id(validate(&CONTRIBUTOR_ACCOUNT_ID()));
    testing_env!(builder.build());
    contract.donate();

    contract
}

#[test]
fn captures_all_donations() {
    let contract = setup(false);
    assert_eq!(contract.trie_state.donations.len(), 2);
}

#[test]
fn calculates_a_running_donations_total() {
    let twice_attached_deposit = U128::from(2 as u128 * u128::from(ATTACHED_DEPOSIT()));
    let mut contract = setup(false);
    assert!(U128(contract.get_donations_total()) == twice_attached_deposit);
}

#[test]
fn returns_a_list_of_recent_donations() {
    let mut contract = setup(false);
    assert!(contract.get_recent_donations().len() == 2);
}

