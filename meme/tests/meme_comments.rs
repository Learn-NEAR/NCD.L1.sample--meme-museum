mod common;

use near_sdk::{
    testing_env,  
    MockedBlockchain,
    test_utils::VMContextBuilder,
    json_types::ValidAccountId,
};


use common::{
    doInitialize,
    TITLE,
    DATA,
    CATEGORY,
    CONTRIBUTOR_ACCOUNT_ID,
    CREATOR_ACCOUNT_ID,
};

use meme::Contract;


fn setup(is_view: bool) -> VMContextBuilder{
    let mut builder: VMContextBuilder = doInitialize(is_view);

    builder.signer_account_id(validate(&CONTRIBUTOR_ACCOUNT_ID()));
    builder.predecessor_account_id(validate(&CONTRIBUTOR_ACCOUNT_ID()));

    builder
}


fn validate(id: &str) -> ValidAccountId {
    ValidAccountId::try_from(id).unwrap()
}


fn contract() -> Contract {
    Contract::new(TITLE(), DATA(), CATEGORY())
}

#[test]
#[should_panic(expected = "Comment is too long, must be less than 500")]
fn rejects_comments_that_are_too_long(){
    testing_env!(setup(false).build());
    #[allow(non_snake_case)]
    let TOO_LONG_TEXT: String = String::from("Lorem ipsum dolor sit amet, consectetur adipisicing elit. Tempore, doloremque. Quod maiores consectetur praesentium, aperiam repellendus facere velit dolorum vel corporis nisi pariatur asperiores animi quibusdam soluta deserunt nam? Repudiandae quidem quos expedita, vero, obcaecati ex, incidunt sequi porro corporis unde omnis ducimus tempora earum excepturi atque ea aliquid aliquam voluptates necessitatibus sit nostrum iure? Velit adipisci hic molestiae iure minima sint illum ex mollitia vitae consequuntur deserunt sit placeat, obcaecati quasi fugit odit aspernatur animi repellendus fugiat at dignissimos nihil!");

    let mut contract: Contract = contract();
    
    contract.add_comment(TOO_LONG_TEXT);
}


#[test]
fn captures_multiple_comments() {
    let mut builder = setup(false);

    builder.signer_account_id(validate(&CREATOR_ACCOUNT_ID()));
    builder.predecessor_account_id(validate(&CREATOR_ACCOUNT_ID()));
    testing_env!(builder.clone().build());

    let mut contract = contract();
    contract.add_comment(String::from("I love this"));

    builder.signer_account_id(validate(&CONTRIBUTOR_ACCOUNT_ID()));
    builder.predecessor_account_id(validate(&CONTRIBUTOR_ACCOUNT_ID()));
    testing_env!(builder.build());

    contract.add_comment(String::from("I don't like it"));

    assert_eq!(contract.get_recent_comments().len(), 2);
}


