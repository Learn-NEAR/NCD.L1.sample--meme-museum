mod common;


// use common::{setup_tests};

use near_sdk::{
    AccountId,
    testing_env,
    MockedBlockchain,
};



use common::{
    use_museum_as_predecessor,
    attach_min_balance,
    NAME,
    setup, 
    OWNER_ACCOUNT_ID,
};

use museum::Contract;

// fn before_each() -> (String, String) {
//     use_museum_as_predecessor()
// }

// #[test]
// fn creates_a_new_museum_with_proper_metadata(){
//     setup!(
//         "s", "s",
//         "else", "else",
//     );



// }


// macro_rules! two_words {
//     () => {};

//     ($single: expr) => {
//         // println!("single {}", $single);
//         compile_error!("Only allow pairs of arguments.");
//     };

//     ($first: expr, $second: expr) => {
//         println!("two {}, {},", $first, $second);
//     };

//     ($first: expr, $second: expr, $($others: tt)*) => {
//         two_words!($first, $second);
//         two_words!($($others)*);
//     };
// }


// #[test]
// fn thingyy() {
//     two_words!("1", "2", "3", "4");
//     two_words!("1", "2", "3", "4", "5", 6);
//     two_words!( 1, String::from("2"));
//     panic!("fff");
// }


fn setupping(){
    
    setup!(
        "predecessor_account_id", "museum", 
        "attached_deposit", utils::MIN_ACCOUNT_BALANCE,
    );
}


#[test]
fn creates_a_new_museum_with_proper_metadata(){
    setup!(
        "predecessor_account_id", "museum", 
        "attached_deposit", utils::MIN_ACCOUNT_BALANCE,
    );

    let mut contract = Contract::new(NAME(), Vec::from([OWNER_ACCOUNT_ID()]));
    let m = contract.get_museum();
    assert_eq!(m.name, NAME());
    assert_eq!(contract.globals.owners.len(), 1);
}


#[test]
#[should_panic(expected="Contract is already initialized")]
fn prevents_double_initialization(){
    setup!(
        "predecessor_account_id", "museum", 
        "attached_deposit", utils::MIN_ACCOUNT_BALANCE,
    );

    let _contract = Contract::new(NAME(), Vec::from([OWNER_ACCOUNT_ID()]));
    let _new_contract = Contract::new(NAME(), Vec::from([OWNER_ACCOUNT_ID()]));
}

#[test]
#[should_panic(expected="Museum name may not be blank")]
fn require_title_not_to_be_blank(){
    setup!(
        "predecessor_account_id", "museum", 
        "attached_deposit", utils::MIN_ACCOUNT_BALANCE,
    );

    let contract = Contract::new("", Vec::from([OWNER_ACCOUNT_ID()]));
}

#[test]
#[should_panic(expected="Minimum account balance must be attached to initialize this contract (3 NEAR)")]
fn requires_a_minimum_balance() {
    setup!(
        "predecessor_account_id", "museum", 
    );

    let contract = Contract::new(NAME(), Vec::from([OWNER_ACCOUNT_ID()]));
}
