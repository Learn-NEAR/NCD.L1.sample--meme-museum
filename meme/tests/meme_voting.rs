
mod common;


use near_sdk::{
    testing_env,  
    MockedBlockchain,
    // test_utils::VMContextBuilder,
    json_types::ValidAccountId,
};


use common::{
    // doInitialize,
    useContributorAsPredecessor,
    TITLE,
    DATA,
    CATEGORY,
    CREATOR_ACCOUNT_ID,
    setup_meme_voting as setup,
};


use near_sdk::collections::Vector;

use meme::Contract;
use utils::AccountId;



#[test]
fn allow_individuals_to_vote(){
    let mut builder = setup(false);
    useContributorAsPredecessor(&mut builder);
    testing_env!(builder.build());


    let mut contract: Contract = Contract::new(TITLE(), DATA(), CATEGORY());

    assert_eq!(contract.trie_state.votes.len(), 0);

    contract.vote(1);

    assert_eq!(contract.trie_state.votes.len(), 1);
}

#[test]
#[should_panic]
fn prevents_vote_automation_for_individuals() {
    println!("Expected to fail with 'Users must vote directly'");
    testing_env!(setup(false).build());
    let mut contract: Contract = Contract::new(TITLE(), DATA(), CATEGORY());
    contract.vote(1);
}

#[test]
#[should_panic]
fn prevents_any_user_from_voting_more_than_once() {
    println!("Expected to fail with 'Voter has already voted'");
    let mut builder = setup(false);
    useContributorAsPredecessor(&mut builder);
    testing_env!(builder.build());

    let mut contract: Contract = Contract::new(TITLE(), DATA(), CATEGORY());

    // If this fails, the test will be considered successful
    // for that, we need to rely on the other tests to be sure
    contract.vote(1);

    // Should panic here
    contract.vote(1);
}

#[test]
fn allows_groups_to_vote(){
    let mut builder = setup(false);
    builder.signer_account_id(ValidAccountId::try_from(CREATOR_ACCOUNT_ID()).unwrap());
    builder.predecessor_account_id(ValidAccountId::try_from(CREATOR_ACCOUNT_ID()).unwrap());

    testing_env!(builder.build());

    // Initialize contract
    let mut contract: Contract = Contract::new(TITLE(), DATA(), CATEGORY());

    contract.batch_vote(3, None);

    assert_eq!(contract.trie_state.votes.len(), 1);

    // Gets a reference (zero-copy) slice of the unorderedset as a Vector
    let view: &Vector<AccountId> = contract.trie_state.voters.as_vector();

    assert!(view.get(0).unwrap().starts_with("batch-"));
}