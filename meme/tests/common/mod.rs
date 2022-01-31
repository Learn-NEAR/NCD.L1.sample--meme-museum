// This is where help functions for the other tests are stored.
// Cargo understands that /tests/common/mod.rs is a module that can be imported by other tests

// pub use crate::Contract;
pub use meme::Contract;

pub use utils::{Category, MIN_ACCOUNT_BALANCE, validate};

use near_sdk::{
    json_types::U128,
    test_utils::VMContextBuilder,
};

// == CONFIG VALUES ============================================================

#[allow(non_snake_case)]
pub fn TITLE() -> String {
    String::from("usain refrain")
}
#[allow(non_snake_case)]
pub fn DATA() -> String {
    String::from("https://9gag.com/gag/ayMDG8Y")
}
#[allow(non_snake_case)]
pub fn CATEGORY() -> Category {
    utils::Category::A
}
#[allow(non_snake_case)]
pub fn MUSEUM_ACCOUNT_ID() -> String {
    String::from("museum.near")
}
#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn CREATOR_ACCOUNT_ID() -> String {
    String::from("alice.near")
}
#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn CONTRIBUTOR_ACCOUNT_ID() -> String {
    String::from("bob.near")
}
#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn ATTACHED_DEPOSIT() -> U128 {
    U128::from(utils::ONE_NEAR * 10)
}


// == HELPER FUNCTIONS =========================================================

#[allow(non_snake_case)]
pub fn useMuseumAsPredecessor(context: &mut VMContextBuilder) {
    context.predecessor_account_id(validate(&MUSEUM_ACCOUNT_ID()));
}


#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn useContributorAsPredecessor(context: &mut VMContextBuilder) {
    context.predecessor_account_id(validate(&CONTRIBUTOR_ACCOUNT_ID()));
}

#[allow(non_snake_case)]
pub fn attachMinBalance(context: &mut VMContextBuilder) {
    context.attached_deposit(MIN_ACCOUNT_BALANCE);
}

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn doInitialize(is_view: bool) -> VMContextBuilder {
    let mut builder: VMContextBuilder = VMContextBuilder::new();

    builder
        .signer_account_id((String::from("bob.near")).try_into().unwrap())
        .is_view(is_view);

    attachMinBalance(&mut builder);
    useMuseumAsPredecessor(&mut builder);

    // contract
    builder
}

#[allow(dead_code)]
pub fn setup(is_view: bool) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();

    builder
        .signer_account_id(String::from("bob.near").try_into().unwrap())
        .is_view(is_view);

    useMuseumAsPredecessor(&mut builder);

    builder
}

#[allow(dead_code)]
pub fn setup_meme_voting(is_view: bool) -> VMContextBuilder {
    doInitialize(is_view)
}
