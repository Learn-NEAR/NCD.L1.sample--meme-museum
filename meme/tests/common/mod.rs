// This is where help functions for the other tests are stored.
// Cargo understands that /tests/common/mod.rs is a module that can be imported by other tests

// pub use crate::Contract;
pub use meme::{
    Contract,
};

pub use utils::{ MIN_ACCOUNT_BALANCE, Category };

use near_sdk::{
    // VMContext, 
    test_utils::VMContextBuilder,
    json_types::{
        U128,
        ValidAccountId,
    }
};



// == CONFIG VALUES ============================================================

#[allow(non_snake_case)]
pub fn TITLE() -> String { String::from("usain refrain")}
#[allow(non_snake_case)]
pub fn DATA() -> String { String::from("https://9gag.com/gag/ayMDG8Y") }
#[allow(non_snake_case)]
pub fn CATEGORY() -> Category { utils::Category::A }
#[allow(non_snake_case)]
pub fn MUSEUM_ACCOUNT_ID() -> String { String::from("museum.near") }
#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn CREATOR_ACCOUNT_ID() -> String { String::from("alice.near") }
#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn CONTRIBUTOR_ACCOUNT_ID() -> String { String::from("bob.near") }
#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn ATTACHED_DEPOSIT() -> U128 { U128::from( utils::ONE_NEAR * 10) }

// const ATTACHED_DEPOSIT: u128 = ONE_NEAR * 10;

// == HELPER FUNCTIONS =========================================================

// fn validate(account_id: &str) -> ValidAccountId{
//     ValidAccountId::try_from(account_id).unwrap()
// }

#[allow(non_snake_case)]
pub fn useMuseumAsPredecessor(context: &mut VMContextBuilder) {
    context.predecessor_account_id(ValidAccountId::try_from(MUSEUM_ACCOUNT_ID()).unwrap());
}

// fn useContributorAsPredecessor(builder: &mut VMContextBuilder) {
//     builder.predecessor_account_id(CONTRIBUTOR_ACCOUNT_ID());
// }

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn useContributorAsPredecessor(context: &mut VMContextBuilder) {
    context.predecessor_account_id(ValidAccountId::try_from(CONTRIBUTOR_ACCOUNT_ID()).unwrap());
}

#[allow(non_snake_case)]
pub fn attachMinBalance(context: &mut VMContextBuilder) {
    context.attached_deposit(MIN_ACCOUNT_BALANCE);
}

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn doInitialize(is_view: bool) -> VMContextBuilder {
    let mut builder: VMContextBuilder = VMContextBuilder::new();

    builder.signer_account_id("bob.near".try_into().unwrap())
        .is_view(is_view);

    

    // let mut context: VMContext = builder.build();
    attachMinBalance(&mut builder);
    useMuseumAsPredecessor(&mut builder);
    // testing_env!(context);

    // let mut contract: Contract = Contract::new(
    //     TITLE(),// title: String,
    //     DATA(),// data: String,
    //     CATEGORY(),// category: Category,
    // );

    // contract
    builder
}

#[allow(dead_code)]
pub fn setup(is_view: bool) -> VMContextBuilder{
    let mut builder = VMContextBuilder::new();
    
    builder.signer_account_id("bob.near".try_into().unwrap())
        .is_view(is_view);

    useMuseumAsPredecessor(&mut builder);

    // testing_env!(context);
    builder
}

#[allow(dead_code)]
pub fn setup_meme_voting(is_view: bool) -> VMContextBuilder {
    doInitialize(is_view)
}
