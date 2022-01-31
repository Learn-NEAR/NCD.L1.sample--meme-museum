mod common;

use near_sdk::{
    MockedBlockchain,
    testing_env,
};


use common::{
    attachMinBalance,
    setup,
    CATEGORY,
    DATA,
    TITLE,
};

use meme::Contract;


#[test]
fn creates_a_new_meme_with_proper_metadata() {
    let mut builder = setup(false);
    attachMinBalance(&mut builder);
    testing_env!(builder.build());

    let contract: Contract = Contract::new(TITLE(), DATA(), CATEGORY());
    let m = contract.get_meme();

    assert_eq!(m.title, TITLE());
    assert_eq!(m.data, DATA());
    // assert_eq!(m.category, CATEGORY());
    if m.category != CATEGORY() {
        panic!("Category not equal")
    };
    assert_eq!(m.total_donations, 0);
    assert_eq!(m.vote_score, 0);
}

#[test]
#[should_panic]
fn prevents_double_initialization() {
    let mut builder = setup(false);
    attachMinBalance(&mut builder);
    testing_env!(builder.build());

    println!("should panic with 'contract is already initialized'");

    let _contract: Contract = Contract::new(TITLE(), DATA(), CATEGORY());

    let _another_contract: Contract = Contract::new(TITLE(), DATA(), CATEGORY());
}

#[test]
#[should_panic]
fn require_title_not_to_be_blank() {
    let mut builder = setup(false);
    attachMinBalance(&mut builder);
    testing_env!(builder.build());

    println!("Should panic with 'Meme title may not be blank'");

    let _contract: Contract = Contract::new(String::from(""), DATA(), CATEGORY());
}

#[test]
#[should_panic]
fn require_a_minimum_balance() {
    testing_env!(setup(false).build());
    let _contract = Contract::new(TITLE(), DATA(), CATEGORY());
}
