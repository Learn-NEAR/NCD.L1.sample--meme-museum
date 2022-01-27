pub(crate) mod setup_tests;

// use near_sdk::{
//     // Balance,
//     // BlockHeight,
//     // EpochHeight,
//     // Gas,
//     // PublicKey,
//     // StorageUsage,
//     // MockedBlockchain,
//     // test_utils::{VMContextBuilder},
//     // testing_env,
//     // json_types::ValidAccountId, env::signer_account_id, AccountId,
// };

use utils;
// use setup_tests::setup_tests;
pub use crate::setup_tests as setup;

use museum::Contract;


// == CONFIG VALUES ============================================================
// Using functions because the compiler doesn't like the idea of global const strings

#[allow(non_snake_case)]
pub fn NAME() -> String { String::from("usain") }
#[allow(non_snake_case)]
pub fn MUSEUM_ACCOUNT_ID() -> String { String::from("museum") }
#[allow(non_snake_case)]
pub fn OWNER_ACCOUNT_ID() -> String { String::from("alice") }
#[allow(non_snake_case)]
pub fn CONTRIBUTOR_ACCOUNT_ID() -> String { String::from("bob") }


// == HELPER FUNCTIONS =========================================================

pub fn use_museum_as_predecessor() -> (String, String){
    ( "predecessor_account_id".into(), MUSEUM_ACCOUNT_ID())
}

pub fn use_admin_as_predecessor() -> (String, String) {
    ( "predecessor_account_id".into(), OWNER_ACCOUNT_ID() )
}

pub fn use_contributor_as_predecessor() -> (String, String ){
    ("predecessor_account_id".into() , CONTRIBUTOR_ACCOUNT_ID() ) 
}

pub fn attach_min_balance() -> (String, u128) {
    ("attached_deposit".into() , utils::MIN_ACCOUNT_BALANCE )
}

pub fn do_initialize() -> Contract {

    let name = attach_min_balance();
    let arg = use_museum_as_predecessor();

    // setup!(name, arg,);

    Contract::new(NAME(), vec!(OWNER_ACCOUNT_ID(),))
}


















// #[test]
// fn setting_up() {
//     let name = "attached_deposit";
//     let balance = utils::MIN_ACCOUNT_BALANCE;
//     // attachminbalance
//     setup_tests!((name, balance));
    
// }


