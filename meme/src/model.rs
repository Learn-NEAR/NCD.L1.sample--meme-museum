use near_sdk::{
    env,
    borsh::{
        self,
        BorshDeserialize,
        BorshSerialize,
    },
    collections::{
        Vector,
        UnorderedSet,
    },
    serde::{
        Deserialize,
        Serialize,
    }
};

use utils::{ MEME_KEY, PAGE_SIZE, Category, AccountId, Money, Timestamp };

#[derive(Clone, Default, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Comment {
    created_at: Timestamp,
    author: AccountId,
    text: String,
}

impl Comment {
    pub fn new(text: String) -> Self {
        let created_at: Timestamp = env::block_timestamp();
        let author: AccountId = env::predecessor_account_id();
        Comment{
            created_at,
            author,
            text,
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Vote {
    created_at: Timestamp,
    value: i8,
    voter: AccountId,
}


impl Vote{
    pub fn new(value: i8, voter: AccountId) -> Self {
        let created_at: Timestamp = env::block_timestamp();

        Vote {
            created_at,
            value,
            voter,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Donation {
    pub amount: Money,
    pub donor: AccountId,
    pub created_at: Timestamp,
}

impl Default for Donation{
    fn default() -> Self {
        let amount: Money = env::attached_deposit();
        let donor: AccountId = env::predecessor_account_id();
        let created_at: Timestamp = env::block_timestamp();

        Donation{
            amount,
            donor,
            created_at,
        }
    }
}


#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Meme {
    pub creator: AccountId,
    pub created_at: Timestamp,
    pub vote_score: i32,
    pub total_donations: u128,
    pub title: String,
    pub data: String,
    pub category: Category,
}


impl Meme {
    pub fn new(title: String, data: String, category: Category) -> Self {
        let creator: AccountId = env::predecessor_account_id();
        let created_at: Timestamp = env::block_timestamp();
        let vote_score: i32 = 0;
        let total_donations: u128 = 0;

        Meme{
            creator,
            created_at,
            vote_score,
            total_donations,
            title,
            data,
            category,
        }
    }

    pub fn create(
        title: String, 
        data: String, 
        category: Category,
    ) {
        // data has to have identifier from valid content provider
        assert!(is_valid_meme_data(&data), "Data is not valid, must start with valid 9gag.com URL");

        // save the meme to storage
        let mut meme = Meme::new(title, data, category);

        meme.set();
    }

    pub fn get() -> Self {
        let stored = env::storage_read(MEME_KEY().as_bytes()).unwrap();

        let result: Meme = BorshDeserialize::deserialize(&mut (&stored[..])).unwrap();
        // We have to use borsh to serialize/deserialize this object to/from bytes
        return result
    }

    pub fn set(&mut self) {
        let mut meme: Vec<u8> = Vec::new();
        // self.serialize(&mut meme).unwrap();
        borsh::ser::BorshSerialize::serialize(self, &mut meme).unwrap();
        env::storage_write(MEME_KEY().as_bytes(), &meme[..]);
    }

    // ----------------------------------------------------------------------------
    // Voting
    // ----------------------------------------------------------------------------

    pub fn add_vote(
        trie_state: &mut TrieState,
        voter: String, 
        value: i8,
    ) {
        // allow each account to vote only once
        assert!(!trie_state.voters.contains(&voter), "Voter has already voted");
        // fetch meme from storage
        let mut meme = Self::get();
        // calculate the new score for the meme
        meme.vote_score += value as i32;
        // save it back to storage
        meme.set();
        // remember the voter has voted
        trie_state.voters.insert(&voter);
        // add the new Vote
        trie_state.votes.insert(&Vote::new(value, voter));   
    }

    pub fn get_votes_count(votes: &Vector<Vote>) -> u32 {
        votes.len() as u32
    }

    pub fn recent_votes(trie_state: &mut TrieState, count: Option<i32>) -> Vec<Vote> {
        let result = get_last(&mut trie_state.votes, count);

        result
    }

    // ----------------------------------------------------------------------------
    // Comments
    // ----------------------------------------------------------------------------

    pub fn add_comment(trie_state: &mut TrieState, text: String) {
        trie_state.comments.insert(&Comment::new(text));
    }

    pub fn get_comments_count(trie_state: &mut TrieState) -> u32 {
        trie_state.comments.len() as u32
    }

    pub fn recent_comments(trie_state: &mut TrieState, count: Option<i32>) -> Vec<Comment> {
        let result = get_last(&mut trie_state.comments, count);

        result
    }

    // ----------------------------------------------------------------------------
    // Donations
    // ----------------------------------------------------------------------------

    pub fn add_donation(trie_state: &mut TrieState){
        // fetch meme from storage
        let mut meme = Self::get();
        // record the donation
        meme.total_donations = meme.total_donations + env::attached_deposit();
        // save it back to storage
        meme.set();
        // add the new Donation
        trie_state.donations.insert(&Donation::default());
    }

    pub fn get_donations_count(trie_state: &TrieState) -> u32 {
        trie_state.donations.len() as u32
    }

    pub fn recent_donations(trie_state: &mut TrieState, count: Option<i32>) -> Vec<Donation> {
        let result = get_last(&mut trie_state.donations, count);

        result
    }  
}

/// Handle validation and extraction of meme data
pub fn is_valid_meme_data(data: &str) -> bool {
    return data.starts_with("https://9gag.com")
}


/// get_last is currently not implemented for Vector, 
/// so we're implementing here to avoid typing the same code several times.
pub fn get_last<D: BorshDeserialize + BorshSerialize>(unset: &mut UnorderedSet<D>, count: Option<i32>) -> Vec<D> {
    let count: i32 = count.unwrap_or(PAGE_SIZE as i32);
    let mut result: Vec<D> = Vec::new();

    let mut start: i32 = unset.len() as i32 - count;
    let end: i32 = unset.len() as i32;
    if start < 0{
        start = 0;
    }

    let view: &Vector<D> = unset.as_vector();

    for i in start..end {
        let value = view.get(i as u64).unwrap();
        result.push(value);
    }

    result
}



/// Custom type for storing "global" variables
/// Rust doesn't allow creating global variables that aren't constants, even pointers are not allowed.
/// So we're using references to this instead.
#[derive(BorshDeserialize, BorshSerialize)]
pub struct TrieState{
    // pub comments: Vector<Comment>,// Doesn't implement Serialize
    pub comments: UnorderedSet<Comment>,
    // pub votes: Vector<Vote>, // Doesn't Implement Serialize
    pub votes: UnorderedSet<Vote>,
    pub voters: UnorderedSet<AccountId>,
    // pub donations: Vector<Donation>, // Doesn't Implement Serialize
    pub donations: UnorderedSet<Donation>,
}


/// Default trait is like new() constructor, but it has no args, and 
/// the vm calls it when creating the contract (If it's in the main contract struct).
impl Default for TrieState{
    fn default() -> Self {
        let comments: UnorderedSet<Comment> = UnorderedSet::new("c".as_bytes());
        let votes: UnorderedSet<Vote> = UnorderedSet::new("v".as_bytes());
        let voters: UnorderedSet<AccountId> = UnorderedSet::new("vs".as_bytes());
        let donations: UnorderedSet<Donation> = UnorderedSet::new("d".as_bytes());

        TrieState{
            comments,
            votes,
            voters,
            donations,
        }
    }
}


