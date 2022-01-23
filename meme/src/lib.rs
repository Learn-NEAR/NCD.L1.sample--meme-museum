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
