mod common;

use near_sdk::{
    MockedBlockchain,
    test_utils::VMContextBuilder,
    testing_env,
};

use utils::validate;

use common::{
    setup_meme_voting as setup, CATEGORY, CONTRIBUTOR_ACCOUNT_ID, CREATOR_ACCOUNT_ID, DATA, TITLE,
};

use meme::Contract;

fn setup2() -> Contract {
    let mut builder: VMContextBuilder = setup(false);
    builder.signer_account_id(validate(&CREATOR_ACCOUNT_ID()));
    builder.predecessor_account_id(validate(&CREATOR_ACCOUNT_ID()));

    let context = builder.clone().build();

    testing_env!(context.clone());
    println!("Initializing");

    // Initialize the contract
    let mut contract: Contract = Contract::new(TITLE(), DATA(), CATEGORY());

    println!("Voting");
    contract.vote(1);

    builder.signer_account_id(validate(&CONTRIBUTOR_ACCOUNT_ID()));
    builder.predecessor_account_id(validate(&CONTRIBUTOR_ACCOUNT_ID()));
    testing_env!(builder.build());

    println!("Voting");
    contract.vote(1);

    println!("Returning contract");

    contract
}

#[test]
fn captures_all_votes() {
    let contract: Contract = setup2();
    assert_eq!(contract.globals.votes.len(), 2);
    assert_eq!(contract.globals.voters.len(), 2);
}

#[test]
fn calculates_a_running_vote_score() {
    let contract: Contract = setup2();
    assert_eq!(contract.get_vote_score(), 2);
}

#[test]
fn returns_a_list_of_recent_votes() {
    let mut contract: Contract = setup2();
    assert_eq!(contract.get_recent_votes().len(), 2);
}
