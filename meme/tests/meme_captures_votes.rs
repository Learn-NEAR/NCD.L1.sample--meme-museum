
// mod meme_voting;
mod common;


use near_sdk::{
    testing_env, 
    // VMContext, 
    MockedBlockchain,
    test_utils::VMContextBuilder,
    json_types::ValidAccountId,
};


use common::{
    CREATOR_ACCOUNT_ID, 
    TITLE, 
    DATA, 
    CATEGORY,
    CONTRIBUTOR_ACCOUNT_ID,
    setup_meme_voting as setup,
};

// use super::setup;
use meme::Contract;

// use common::*;

fn setup2() -> Contract{
    let mut builder: VMContextBuilder = setup(false);
    builder.signer_account_id(ValidAccountId::try_from(CREATOR_ACCOUNT_ID()).unwrap());
    builder.predecessor_account_id(ValidAccountId::try_from(CREATOR_ACCOUNT_ID()).unwrap());

    let mut context = builder.build();

    testing_env!(context.clone());
    println!("Initializing");

    // Initialize the contract
    let mut contract: Contract = Contract::new(TITLE(), DATA(), CATEGORY());

    println!("Voting");
    contract.vote(1);

    // let mut builder: VMContextBuilder = setup(false);
    // builder.signer_account_id(ValidAccountId::try_from(CONTRIBUTOR_ACCOUNT_ID()).unwrap());
    // builder.predecessor_account_id(ValidAccountId::try_from(CONTRIBUTOR_ACCOUNT_ID()).unwrap());
    // testing_env!(builder.build());

    context.signer_account_id = CONTRIBUTOR_ACCOUNT_ID();
    context.predecessor_account_id = CONTRIBUTOR_ACCOUNT_ID();
    testing_env!(context);

    println!("Voting");
    contract.vote(1);

    println!("Returning contract");

    contract
}

#[test]
fn captures_all_votes(){
    let contract: Contract = setup2();
    assert_eq!(contract.trie_state.votes.len(), 2);
    assert_eq!(contract.trie_state.voters.len(), 2);
}

#[test]
fn calculates_a_running_vote_score() {
    let contract: Contract = setup2();
    assert_eq!(contract.get_vote_score(), 2);
}

#[test]
fn returns_a_list_of_recent_votes(){
    let mut contract: Contract = setup2();
    assert_eq!(contract.get_recent_votes().len(), 2);
}





