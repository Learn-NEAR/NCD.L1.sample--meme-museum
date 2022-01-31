mod common;

use near_sdk::{
    MockedBlockchain,
    testing_env, 
    test_utils::VMContextBuilder,
};

use utils::validate;

use common::{do_initialize, CONTRIBUTOR_ACCOUNT_ID, NAME, OWNER_ACCOUNT_ID};


use museum::Contract;

#[test]
fn returns_a_list_of_owners() {
    let contract = do_initialize();

    assert_eq!(contract.get_owner_list().len(), 1);
}

#[test]
fn returns_a_list_of_memes() {
    let mut contract = do_initialize();
    contract.globals.memes.insert(&NAME());
    assert_eq!(contract.get_meme_list().get(0).unwrap(), &NAME());
}

#[test]
fn returns_a_count_of_memes() {
    let mut contract = do_initialize();
    contract.globals.memes.insert(&NAME());
    assert_eq!(contract.get_meme_count(), 1);
}

#[test]
fn allows_users_to_add_remove_themselves_as_contributors() {
    let mut builder: VMContextBuilder = VMContextBuilder::new();
    builder.attached_deposit(utils::MIN_ACCOUNT_BALANCE);
    builder.predecessor_account_id(validate(&CONTRIBUTOR_ACCOUNT_ID()));
    testing_env!(builder.build());


    let mut contract = Contract::new(NAME().to_string(), Vec::from([OWNER_ACCOUNT_ID()]));
    contract.add_myself_as_contributor();

    assert_eq!(contract.globals.contributors.len(), 1);
    assert!(contract
        .globals
        .contributors
        .contains(&CONTRIBUTOR_ACCOUNT_ID()));

    contract.remove_myself_as_contributor();
    assert_eq!(contract.globals.contributors.len(), 0);
}

