use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use near_sdk::{
    env, 
    near_bindgen,
    json_types::{
        U128,
        U64,
    }
};
// use near_sdk::collections::Vector;
// use near_sdk::serde::{Deserialize, Serialize};


use utils::{ MEME_KEY, XCC_GAS, MIN_ACCOUNT_BALANCE, MAX_COMMENT_LENGTH, AccountId, Category };
use model::{ Comment, Vote, Meme, Donation, TrieState };


near_sdk::setup_alloc!();

// Making model public so it can be imported by test modules
pub mod model;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    /// Vectors and PersistentSets trie storage
    pub trie_state: TrieState,
}

#[near_bindgen]
impl Contract{
    #[init]
    pub fn new(
        title: String, 
        data: String, 
        category: Category,
    ) -> Self {
        // Contract may only be initialized once
        assert_contract_is_not_initialized();

        // storing meme metadata requires some storage staking (balance locked to offset cost of data storage)
        assert!(
            env::attached_deposit() >= MIN_ACCOUNT_BALANCE,
            "Minimum account balance must be attached to initialize this contract (3 NEAR)",
        );

        // title has to be at least 1 character
        assert!(title.len() > 0, "Meme title may not be blank");

        // create the meme using incoming metadata
        Meme::create(title, data, category);

        Contract::default()
    }

    /// Return the meme.
    pub fn get_meme(&self) -> Meme {
        assert_contract_is_initialized();
        Meme::get()
    }

    // ----------------------------------------------------------------------------
    // Voting
    // ----------------------------------------------------------------------------

    /// Register a single vote, up or down, for the meme.
    pub fn vote(&mut self, value: i8) {
        assert_contract_is_initialized();
        assert_eq!(
            env::signer_account_id(),
            env::predecessor_account_id(),
            "Users must vote directly",
        );
        assert!((value == 1) || (value == -1), "Invalid vote, must be -1 or 1");

        // register the vote.
        self.batch_vote(value, Some(false));
    }

    /// Register a batched vote where several votes are captured together.
    pub fn batch_vote(&mut self, value: i8, is_batch: Option<bool>) {
        let is_batch = is_batch.unwrap_or(true);
        // register the vote
        if is_batch {
            assert_eq!(
                env::predecessor_account_id(),
                env::current_account_id(),
                "Batch votes may only be made by the Meme account",
            );
        }

        let voter: String = match is_batch {
            true => { format!("batch-{}", env::predecessor_account_id())},
            false => { format!("{}", env::predecessor_account_id())},
        };

        let trie_state = &mut self.trie_state;

        Meme::add_vote(trie_state, voter, value);
    }

    /// Get a list of recent votes
    pub fn get_recent_votes(&mut self) -> Vec<Vote> {
        assert_contract_is_initialized();
        Meme::recent_votes(&mut self.trie_state, None)
    }

    /// Get the current vote score
    pub fn get_vote_score(&self) -> i32 {
        assert_contract_is_initialized();
        return Meme::get().vote_score
    }

    // ----------------------------------------------------------------------------
    // Comments
    // ----------------------------------------------------------------------------

    /// Add a comment.
    pub fn add_comment(&mut self, text: String) {
        assert_contract_is_initialized();
        assert_eq!(
            env::signer_account_id(),
            env::predecessor_account_id(),
            "Users must comment directly",
        );
        assert_reasonable_comment_length(&text);

        Meme::add_comment(&mut self.trie_state, text);
    }

    /// Get a list of recent comments
    pub fn get_recent_comments(&mut self) -> Vec<Comment> {
        assert_contract_is_initialized();
        Meme::recent_comments(&mut self.trie_state, None)
    }

    // ----------------------------------------------------------------------------
    // Donations
    // ----------------------------------------------------------------------------

    /// Donate tokens to the contract.
    #[payable]
    pub fn donate(&mut self) {
        assert_contract_is_initialized();
        assert_eq!(
            env::signer_account_id(),
            env::predecessor_account_id(),
            "Users must donate directly",
        );

        assert!(env::attached_deposit() > 0, "Donor must attach some money");

        Meme::add_donation(&mut self.trie_state);
    }

    /// Get a list of donations.
    pub fn get_donations_total(&mut self) -> u128 {
        assert_contract_is_initialized();

        return Meme::get().total_donations
    }

    /// Get a list of recent comments
    pub fn get_recent_donations(&mut self) -> Vec<Donation> {
        assert_contract_is_initialized();

        Meme::recent_donations(&mut self.trie_state, None)
    }

    /// Transfer all donations to a specified account.
    pub fn release_donations(&mut self, account: AccountId){
        assert_contract_is_initialized();
        assert_signed_by_creator();

        // transfer funds to provided account and call ourselves back once transfer is complete
        let promise_index: u64 = env::promise_batch_create(account);

        env::promise_batch_action_transfer(
            promise_index,
            Meme::get().total_donations,
        );

        let promise_index = env::promise_batch_then(
            promise_index,// promise_index: PromiseIndex, 
            env::current_account_id(),// account_id: A,
        );

        env::promise_batch_action_function_call(
            promise_index,// promise_index: PromiseIndex, 
            "on_donations_released".as_bytes(),// method_name: &[u8], 
            "{}".as_bytes(),// arguments: &[u8], 
            0,// amount: Balance, 
            XCC_GAS as u64,// gas: Gas,
        );
    }

    /// Callback method invoked once donation release is complete
    pub fn on_donations_released(&self){
        env::log("Donations were released".as_bytes());
    }

}   


// == PRIVATE FUNCTIONS ========================================================
//
// Helper functions that are not part of the contract

/// Manage comment properties.
pub fn assert_reasonable_comment_length(text: &str){
    assert!(
        text.len() < MAX_COMMENT_LENGTH as usize, 
        "Comment is too long, must be less than {}", MAX_COMMENT_LENGTH,
    );
}

/// Indicate whether contract caller is the creator.
pub fn is_creator() -> bool {
    env::predecessor_account_id().eq(&Meme::get().creator)
}

pub fn assert_signed_by_creator() {
    assert!(is_creator(), "This method can only be called by the meme creator");
}

/// Track Whether or not the meme has been initialized
pub fn is_initialized() -> bool {
    env::storage_has_key(MEME_KEY().as_bytes())
}

pub fn assert_contract_is_initialized() {
    assert!(is_initialized(), "Contract must be initialized first.");
}

pub fn assert_contract_is_not_initialized() {
    assert!(!is_initialized(), "Contract can only be initialized once");
}

#[cfg(test)]
mod tests{
    // use super::*;
    // use utils::{ MIN_ACCOUNT_BALANCE};
    

    // use near_sdk::{
    //     testing_env, 
    //     VMContext, 
    //     MockedBlockchain,
    //     test_utils::VMContextBuilder,
    // };

    

    // // == CONFIG VALUES ============================================================

    // #[allow(non_snake_case)]
    // fn TITLE() -> String { String::from("usain refrain")}
    // #[allow(non_snake_case)]
    // fn DATA() -> String { String::from("https://9gag.com/gag/ayMDG8Y") }
    // #[allow(non_snake_case)]
    // fn CATEGORY() -> Category { utils::Category::A }
    // #[allow(non_snake_case)]
    // fn MUSEUM_ACCOUNT_ID() -> String { String::from("museum") }
    // #[allow(non_snake_case)]
    // fn CREATOR_ACCOUNT_ID() -> String { String::from("alice") }
    // #[allow(non_snake_case)]
    // fn CONTRIBUTOR_ACCOUNT_ID() -> String { String::from("bob") }
    
    // // const ATTACHED_DEPOSIT: u128 = ONE_NEAR * 10;

    // // == HELPER FUNCTIONS =========================================================

    // // fn validate(account_id: &str) -> ValidAccountId{
    // //     ValidAccountId::try_from(account_id).unwrap()
    // // }

    // #[allow(non_snake_case)]
    // fn useMuseumAsPredecessor(context: &mut VMContext) {
    //     context.predecessor_account_id = MUSEUM_ACCOUNT_ID();
    // }

    // // fn useContributorAsPredecessor(builder: &mut VMContextBuilder) {
    // //     builder.predecessor_account_id(CONTRIBUTOR_ACCOUNT_ID());
    // // }

    // #[allow(non_snake_case)]
    // fn useContributorAsPredecessor(context: &mut VMContext) {
    //     context.predecessor_account_id = CONTRIBUTOR_ACCOUNT_ID();
    // }

    // #[allow(non_snake_case)]
    // fn attachMinBalance(context: &mut VMContext) {
    //     context.attached_deposit = MIN_ACCOUNT_BALANCE;
    // }

    // #[allow(non_snake_case)]
    // fn doInitialize() -> VMContext {
    //     let builder: VMContextBuilder = VMContextBuilder::new();

    //     let mut context: VMContext = builder.build();
    //     attachMinBalance(&mut context);
    //     useMuseumAsPredecessor(&mut context);
    //     // testing_env!(context);

    //     // let mut contract: Contract = Contract::new(
    //     //     TITLE(),// title: String,
    //     //     DATA(),// data: String,
    //     //     CATEGORY(),// category: Category,
    //     // );

    //     // contract
    //     context
    // }

    // == UNIT TESTS ==============================================================

    // #[cfg(test)]
    // mod meme_initialization{
    //     use super::*;

    //     fn setup() -> VMContext{
    //         let mut context = VMContextBuilder::new().build();
    //         useMuseumAsPredecessor(&mut context);

    //         // testing_env!(context);
    //         context
    //     }

    //     // fn setup_attach(){
    //     //     let mut builder = VMContextBuilder::new();
    //     //     useMuseumAsPredecessor(&mut builder);
    //     //     attachMinBalance(&mut builder);

    //     //     let context: VMContext = builder.build();
    //     //     testing_env!(context);
    //     // }

    //     #[test]
    //     fn creates_a_new_meme_with_proper_metadata() {
    //         let mut context = setup();
    //         attachMinBalance(&mut context);
    //         testing_env!(context);

    //         let _contract: Contract = Contract::new(TITLE(), DATA(), CATEGORY());
    //         let m = Contract::get_meme();

    //         assert_eq!(m.title, TITLE());
    //         assert_eq!(m.data, DATA());
    //         // assert_eq!(m.category, CATEGORY());
    //         if m.category != CATEGORY() { panic!("Category not equal")};
    //         assert_eq!(m.total_donations, 0);
    //         assert_eq!(m.vote_score, 0);
    //     }

    //     #[test]
    //     #[should_panic]
    //     fn prevents_double_initialization(){
    //         let mut context = setup();
    //         attachMinBalance(&mut context);
    //         testing_env!(context);

    //         println!("should panic with 'contract is already initialized'");

    //         let _contract: Contract = Contract::new(TITLE(), DATA(), CATEGORY());

    //         let _another_contract: Contract = Contract::new(TITLE(), DATA(), CATEGORY());
    //     }

    //     #[test]
    //     #[should_panic]
    //     fn require_title_not_to_be_blank() {
    //         let mut context = setup();
    //         attachMinBalance(&mut context);
    //         testing_env!(context);

    //         println!("Should panic with 'Meme title may not be blank'");

    //         let _contract: Contract = Contract::new(String::from(""), DATA(), CATEGORY());
    //     }

    //     #[test]
    //     #[should_panic]
    //     fn require_a_minimum_balance(){
    //         testing_env!(setup());
    //         let _contract = Contract::new(TITLE(), DATA(), CATEGORY());
    //     }    
    // }

    // #[cfg(test)]
    // pub mod meme_voting{
    //     use super::*;
    //     use near_sdk::collections::Vector;
        
    //     pub fn setup() -> VMContext {
    //         doInitialize()
    //     }

    //     #[test]
    //     fn allow_individuals_to_vote(){
    //         let mut context = setup();
    //         useContributorAsPredecessor(&mut context);
    //         testing_env!(context);


    //         let contract: Contract = Contract::new(TITLE(), DATA(), CATEGORY());

    //         assert_eq!(contract.trie_state.votes.len(), 0);

    //         Contract::vote(1);

    //         assert_eq!(contract.trie_state.votes.len(), 1);
    //     }

    //     #[test]
    //     #[should_panic]
    //     fn prevents_vote_automation_for_individuals() {
    //         println!("Expected to fail with 'Users must vote directly'");
    //         testing_env!(setup());
    //         let _contract: Contract = Contract::new(TITLE(), DATA(), CATEGORY());
    //         Contract::vote(1);
    //     }

    //     #[test]
    //     #[should_panic]
    //     fn prevents_any_user_from_voting_more_than_once() {
    //         println!("Expected to fail with 'Voter has already voted'");
    //         let mut context = setup();
    //         useContributorAsPredecessor(&mut context);

    //         let _contract: Contract = Contract::new(TITLE(), DATA(), CATEGORY());

    //         // If this fails, the test will be considered successful
    //         // for that, we need to rely on the other tests to be sure
    //         Contract::vote(1);

    //         // Should panic here
    //         Contract::vote(1);
    //     }

    //     #[test]
    //     fn allows_groups_to_vote(){
    //         let mut context = setup();
    //         context.signer_account_id = CREATOR_ACCOUNT_ID();
    //         context.predecessor_account_id = CREATOR_ACCOUNT_ID();

    //         testing_env!(context);

    //         // Initialize contract
    //         let mut contract: Contract = Contract::new(TITLE(), DATA(), CATEGORY());

    //         contract.batch_vote(3, None);

    //         assert_eq!(contract.trie_state.votes.len(), 1);

    //         // Gets a reference (zero-copy) slice of the unorderedset as a Vector
    //         let view: &Vector<AccountId> = contract.trie_state.voters.as_vector();

    //         assert!(view.get(0).unwrap().starts_with("batch-"));
    //     }

    //     #[cfg(test)]
    //     pub mod meme_captures_votes{
    //         use super::*;

    //         fn setup2() -> Contract{
    //             let mut context: VMContext = setup();
    //             context.signer_account_id = CREATOR_ACCOUNT_ID();
    //             context.predecessor_account_id = CREATOR_ACCOUNT_ID();

    //             testing_env!(context);

    //             // Initialize the contract
    //             let contract: Contract = Contract::new(TITLE(), DATA(), CATEGORY());

    //             Contract::vote(1);

    //             let mut context: VMContext = setup();
    //             context.signer_account_id = CONTRIBUTOR_ACCOUNT_ID();
    //             context.predecessor_account_id = CONTRIBUTOR_ACCOUNT_ID();
    //             testing_env!(context);

    //             Contract::vote(1);

    //             contract
    //         }

    //         #[test]
    //         fn captures_all_votes(){
    //             let contract: Contract = setup2();
    //             assert_eq!(contract.trie_state.votes.len(), 2);
    //             assert_eq!(contract.trie_state.voters.len(), 2);
    //         }

    //         #[test]
    //         fn calculates_a_running_vote_score() {
    //             let _contract: Contract = setup2();
    //             assert_eq!(Contract::get_vote_score(), 2);
    //         }

    //         #[test]
    //         fn returns_a_list_of_recent_votes(){
    //             let mut contract: Contract = setup2();
    //             assert_eq!(contract.get_recent_votes().len(), 2);
    //         }
    //     }
    // }
}