use near_sdk::json_types::U128;
use near_sdk::{near_bindgen, result_serializer};


use museum::Museum;


#[ext_contract(museum)]
pub trait MuseumExtContract{
    pub fn new(
        name: &str, 
        owners: Vec<String>,
    );

    #[result_serializer(borsh)]
    pub fn get_museum() -> Museum;

    #[result_serializer(borsh)]
    pub fn get_owner_list() -> Vec<String>;

    #[result_serializer(borsh)]
    pub fn get_meme_list() -> Vec<String>;

    pub fn get_meme_count() -> u32;

    pub fn add_myself_as_contributor();

    pub fn remove_myself_as_contributor();

    #[result_serializer(borsh)]
    pub fn add_meme(meme: &str, title: &str, data: &str, category: Category);


    // ------------------------------------------------------------------------
    // owner methods
    // ------------------------------------------------------------------------

    pub fn add_contributor(account: &str);

    pub fn remove_contributor(account: &str);

    pub fn add_owner(account: &str);

    pub fn remove_owner(account: &str);

    pub fn remove_meme(meme: &str);

    pub fn on_meme_removed(meme: &str);
}



// #[near_bindgen]
// pub struct Museum {}

// #[near_bindgen]
// impl Museum {
//     // ------------------------------------------------------------------------
//     // initialization
//     // ------------------------------------------------------------------------

//     pub fn new(name: &str, owners: Vec<&str>) {}

//     // pub fn get_museum() -> Museum
//     pub fn get_museum() {}

//     // ------------------------------------------------------------------------
//     // self-service methods
//     // ------------------------------------------------------------------------

//     // pub fn get_owner_list() -> AccountId[]
//     pub fn get_owner_list() {}

//     // pub fn get_meme_list() -> AccountId[]
//     pub fn get_meme_list() {}

//     // pub fn get_meme_count() -> u32
//     pub fn get_meme_count() {}

//     pub fn add_myself_as_contributor() {}

//     pub fn remove_myself_as_contributor() {}

//     pub fn add_meme(meme: &str, title: &str, data: &str, category: u8) {}

//     // ------------------------------------------------------------------------
//     // owner methods
//     // ------------------------------------------------------------------------

//     pub fn add_contributor(account: &str) {}

//     pub fn remove_contributor(account: &str) {}

//     pub fn add_owner(account: &str) {}

//     pub fn remove_owner(account: &str) {}

//     pub fn remove_meme(meme: &str) {}

//     pub fn on_meme_removed(meme: &str) {}

//     pub fn museum_to_meme_proxy(meme: &str, view_fn: &str) {}
// }
