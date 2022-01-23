use near_sdk::{
    env,
    collections::UnorderedSet,
    borsh::{ self, BorshDeserialize, BorshSerialize },
    // json_types::{ ValidAccountId, },
};

use utils::{ MUSEUM_KEY, AccountId, Timestamp, Category };


#[derive(BorshDeserialize, BorshSerialize)]
pub struct Museum {
    created_at: Timestamp,
    name: String,
}


impl Museum {
    pub fn new(name: String, created_at: Option<Timestamp>) -> Self {
        let created_at: Timestamp = created_at.unwrap_or(env::block_timestamp());

        Museum{
            created_at,
            name,
        }
    }

    // ----------------------------------------------------------------------------
    // Basic functions
    // ----------------------------------------------------------------------------

    pub fn create(globals: &mut Globals, name: String, new_owners: Vec<AccountId>){
        assert!(name.len() > 0, "Museum name may not be blank");

        // save the museum to storage
        let mut itself = Self::new(name, None);
        itself.set();

        // capture owners
        for index in 0..new_owners.len() {
            globals.owners.insert(new_owners.get(index).unwrap());
        }
    }

    pub fn get() -> Museum {
        let stored: Vec<u8> = env::storage_read(MUSEUM_KEY().as_bytes()).unwrap();

        let result: Museum = BorshDeserialize::deserialize(&mut (&stored[..])).unwrap();
        // We have to use borsh to serialize/deserialize this object to/from bytes
        return result
    }

    pub fn set(&mut self) {
        let mut museum: Vec<u8> = Vec::new();
        
        
        borsh::ser::BorshSerialize::serialize(self, &mut museum).unwrap();
        env::storage_write(MUSEUM_KEY().as_bytes(), &museum[..]);
    }
    
    // ----------------------------------------------------------------------------
    // Memes
    // ----------------------------------------------------------------------------

    pub fn add_meme(globals: &mut Globals, account_id: &AccountId) {
        globals.meme.insert(account_id);
    }

    pub fn remove_meme(globals: &mut Globals, account_id: &AccountId) {
        globals.meme.remove(account_id);
    }

    pub fn has_meme(globals: &Globals, account_id: &AccountId) -> bool {
        globals.meme.contains(account_id)
    }

    pub fn get_meme_list(globals: &Globals) -> Vec<String> { 
        let mut result: Vec<String> = Vec::new();

        for meme in globals.meme.as_vector().iter(){
            result.push(meme);
        }

        result
    }

    pub fn get_meme_count(globals: &Globals) -> u32 {
        globals.meme.len() as u32
    }

    // ----------------------------------------------------------------------------
    // Contributors
    // ----------------------------------------------------------------------------

    pub fn add_contributor(globals: &mut Globals, account: &AccountId) {
        globals.contributors.insert(account);
    }

    pub fn remove_contributor(globals: &mut Globals, account: &AccountId) {
        globals.contributors.remove(account);
    }

    pub fn is_contributor(globals: &Globals, account: &AccountId) -> bool {
        globals.contributors.contains(account)
    }

    // ----------------------------------------------------------------------------
    // Owners
    // ----------------------------------------------------------------------------

    pub fn add_owner(globals: &mut Globals, account: &AccountId) {
        globals.owners.insert(account);
    }

    pub fn remove_owner(globals: &mut Globals, account: &AccountId) {
        globals.owners.remove(account);
    }

    pub fn has_owner(globals: &Globals, account: &AccountId) -> bool {
        globals.owners.contains(account)
    }

    pub fn get_owner_list(globals: &Globals) -> Vec<AccountId> {
        let mut result: Vec<AccountId> = Vec::new();

        for owner in globals.owners.as_vector().iter(){
            result.push(owner);
        }

        result
    }
}














#[derive(BorshSerialize)]
pub struct MemeInitArgs {
    pub title: String,
    pub data: String,
    pub category: Category,
}

impl MemeInitArgs{
    pub fn new(title: String, data: String, category: Category) -> Self{
        MemeInitArgs{title,data, category}
    }

    pub fn bytefy(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        borsh::ser::BorshSerialize::serialize(self, &mut result).unwrap();
        result
    }
}


#[derive(BorshSerialize)]
pub struct MemeNameAsArg{
    meme: String,
}


impl MemeNameAsArg{
    pub fn new(meme: String) -> Self{
        MemeNameAsArg{meme}
    }

    pub fn bytefy(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        borsh::ser::BorshSerialize::serialize(self, &mut result).unwrap();
        result
    }
}


#[derive(BorshDeserialize, BorshSerialize)]
pub struct Globals{
    pub meme: UnorderedSet<AccountId>,
    pub contributors: UnorderedSet<AccountId>,
    pub owners: UnorderedSet<AccountId>,
}


impl Default for Globals{
    fn default() -> Self{
        let meme: UnorderedSet<AccountId> = UnorderedSet::new("m".as_bytes());
        let contributors: UnorderedSet<AccountId> = UnorderedSet::new("c".as_bytes());
        let owners: UnorderedSet<AccountId> = UnorderedSet::new("o".as_bytes());

        Globals{
            meme,
            contributors,
            owners,
        }
    }
}
