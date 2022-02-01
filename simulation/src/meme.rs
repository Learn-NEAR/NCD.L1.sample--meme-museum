use std::fmt::Debug;
use near_sdk::near_bindgen;
use utils::Category;
use meme::Vote;


#[ext_contract(meme)]
pub trait MemeExtContract{
    // ------------------------------------------------------------------------
    // initialization
    // ------------------------------------------------------------------------

    pub fn new(title: &str, data: &str, category: Category);

    pub fn get_meme();

    pub fn vote(value: i8);

    pub fn batch_vote(value: i8, is_batch: bool);

    #[result_serializer(borsh)]
    pub fn get_recent_votes() -> Vec<Vote>;

    pub fn get_vote_score() -> i32;

    // ----------------------------------------------------------------------------
    // Comments
    // ----------------------------------------------------------------------------

    pub fn add_comment(text: &str);

    #[result_serializer(borsh)]
    pub fn get_recent_comments() -> Vec<Comment>;

    // ----------------------------------------------------------------------------
    // Donations
    // ----------------------------------------------------------------------------

    #[payable]
    pub fn donate();

    pub fn get_donations_total() -> U128;

    #[result_serializer(borsh)]
    pub fn get_recent_donations() -> Vec<Donation>;

    pub fn release_donations(account: &str);

    pub fn on_donations_released();
}


// #[near_bindgen]
// pub struct Meme {}

// #[near_bindgen]
// impl Meme {
//     // ------------------------------------------------------------------------
//     // initialization
//     // ------------------------------------------------------------------------

//     pub fn new(title: &str, data: &str, category: u8) {}

//     // pub fn get_meme() -> Meme {}
//     pub fn get_meme() {}

//     // ------------------------------------------------------------------------
//     // voting
//     // ------------------------------------------------------------------------

//     pub fn vote(value: i8) {}

//     pub fn batch_vote(value: i8, is_batch: bool) {}

//     // pub fn get_recent_votes() -> Vec<Vote> {}
//     pub fn get_recent_votes() {}

//     // pub fn get_vote_score() -> i32 {}
//     pub fn get_vote_score() {}

//     // ------------------------------------------------------------------------
//     // comments
//     // ------------------------------------------------------------------------

//     pub fn add_comment(text: &str) {}

//     // pub fn get_recent_comments() -> Array<Comment> {}
//     pub fn get_recent_comments() {}

//     // ------------------------------------------------------------------------
//     // donations
//     // ------------------------------------------------------------------------

//     pub fn donate() {}

//     // pub fn get_donations_total() -> U128 {}
//     pub fn get_donations_total() {}

//     // pub fn get_recent_donations(): Array<Donation> {
//     pub fn get_recent_donations() {}

//     pub fn release_donations(account: &str) {}

//     pub fn on_donations_released() {}
// }
