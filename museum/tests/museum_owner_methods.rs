mod common;

use near_sdk::{
    testing_env,
    MockedBlockchain, 
    test_utils::VMContextBuilder,
};

use utils::validate;

use common::{CONTRIBUTOR_ACCOUNT_ID, NAME, OWNER_ACCOUNT_ID};

use museum::Contract;

fn before_each() {
    let mut builder: VMContextBuilder =VMContextBuilder::new();
    builder.attached_deposit(utils::MIN_ACCOUNT_BALANCE);
    builder.predecessor_account_id(validate(&OWNER_ACCOUNT_ID()));
    testing_env!(builder.build());
}

#[test]
fn allows_owners_to_whitelist_a_contributor() {
    before_each();
    let mut contract = Contract::new(NAME().to_string(), Vec::from([OWNER_ACCOUNT_ID()]));

    contract.add_contributor(CONTRIBUTOR_ACCOUNT_ID());
    assert_eq!(contract.globals.contributors.len(), 1);
}

#[test]
fn allows_owners_to_remove_a_contributor() {
    before_each();
    let mut contract = Contract::new(NAME().to_string(), Vec::from([OWNER_ACCOUNT_ID()]));

    contract.add_contributor(CONTRIBUTOR_ACCOUNT_ID());
    assert_eq!(contract.globals.contributors.len(), 1);

    contract.remove_contributor(CONTRIBUTOR_ACCOUNT_ID());
    assert_eq!(contract.globals.contributors.len(), 0);
}

#[test]
fn allows_owners_to_add_a_new_owner() {
    before_each();
    let mut contract = Contract::new(NAME().to_string(), Vec::from([OWNER_ACCOUNT_ID()]));

    contract.add_owner(CONTRIBUTOR_ACCOUNT_ID());
    assert_eq!(contract.globals.owners.len(), 2);
}

#[test]
fn allows_owners_to_remove_an_owner() {
    before_each();
    let mut contract = Contract::new(NAME().to_string(), Vec::from([OWNER_ACCOUNT_ID()]));

    contract.add_owner(CONTRIBUTOR_ACCOUNT_ID());
    assert_eq!(contract.globals.owners.len(), 2);

    contract.remove_owner(CONTRIBUTOR_ACCOUNT_ID());
    assert_eq!(contract.globals.owners.len(), 1);
}
