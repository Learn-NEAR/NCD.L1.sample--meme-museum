mod common;

use near_sdk::{
    testing_env,
    MockedBlockchain,
    AccountId,
};

use utils::validate;

use common::{
    setup_meme_voting as setup,
    useContributorAsPredecessor,
    CATEGORY,
    CREATOR_ACCOUNT_ID,
    DATA,
    TITLE,
};

use near_sdk::collections::Vector;

use meme::Contract;

#[test]
fn allow_individuals_to_vote() {
    let mut builder = setup(false);
    useContributorAsPredecessor(&mut builder);
    testing_env!(builder.build());

    let mut contract: Contract = Contract::new(TITLE(), DATA(), CATEGORY());

    assert_eq!(contract.globals.votes.len(), 0);

    contract.vote(1);

    assert_eq!(contract.globals.votes.len(), 1);
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
fn allows_groups_to_vote() {
    let mut builder = setup(false);
    builder.signer_account_id(validate(&CREATOR_ACCOUNT_ID()));
    builder.predecessor_account_id(validate(&CREATOR_ACCOUNT_ID()));

    testing_env!(builder.build());

    // Initialize contract
    let mut contract: Contract = Contract::new(TITLE(), DATA(), CATEGORY());

    contract.batch_vote(3, None);

    assert_eq!(contract.globals.votes.len(), 1);

    // Gets a reference (zero-copy) slice of the unorderedset as a Vector
    let view: &Vector<AccountId> = contract.globals.voters.as_vector();

    assert!(String::from(view.get(0).unwrap()).starts_with("batch-"));
}
