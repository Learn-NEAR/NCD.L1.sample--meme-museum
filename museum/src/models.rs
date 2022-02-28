use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::UnorderedSet,
    env,
    AccountId,
};

use utils::{Category, Timestamp, MUSEUM_KEY};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Museum {
    created_at: Timestamp,
    pub name: String,
}

impl Museum {
    pub fn new(name: String, created_at: Option<Timestamp>) -> Self {
        let created_at: Timestamp = created_at.unwrap_or(env::block_timestamp());

        Museum { created_at, name }
    }

    // ----------------------------------------------------------------------------
    // Basic functions
    // ----------------------------------------------------------------------------

    pub fn create(globals: &mut Globals, name: String, new_owners: Vec<AccountId>) -> Self{
        assert!(name.len() > 0, "Museum name may not be blank");

        // save the museum to storage
        let mut itself = Self::new(name, None);
        itself.set();

        // capture owners
        for index in 0..new_owners.len() {
            globals.owners.insert(new_owners.get(index).unwrap());
        }
        itself
    }

    pub fn get() -> Museum {
        let stored: Vec<u8> = env::storage_read(MUSEUM_KEY().as_bytes()).unwrap();

        let result: Museum = BorshDeserialize::deserialize(&mut (&stored[..])).unwrap();
        // We have to use borsh to serialize/deserialize this object to/from bytes
        return result;
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
        globals.memes.insert(account_id);
    }

    pub fn remove_meme(globals: &mut Globals, account_id: &AccountId) {
        globals.memes.remove(account_id);
    }

    pub fn has_meme(globals: &Globals, account_id: &AccountId) -> bool {
        globals.memes.contains(account_id)
    }

    pub fn get_meme_list(globals: &Globals) -> Vec<AccountId> {
        let mut result: Vec<AccountId> = Vec::new();

        for meme in globals.memes.as_vector().iter() {
            result.push(meme);
        }

        result
    }

    pub fn get_meme_count(globals: &Globals) -> u32 {
        globals.memes.len() as u32
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
        let result = globals.owners.contains(account);


        if !result {
            for i in globals.owners.iter(){
                env::log(format!("Found: {}", i).as_bytes());
            };

            env::log(format!("\n\nTried: {}\n\n", env::predecessor_account_id()).as_bytes());
        }
        result
    }

    pub fn get_owner_list(globals: &Globals) -> Vec<AccountId> {
        let mut result: Vec<AccountId> = Vec::new();

        for owner in globals.owners.as_vector().iter() {
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

impl MemeInitArgs {
    pub fn new(title: String, data: String, category: Category) -> Self {
        MemeInitArgs {
            title,
            data,
            category,
        }
    }

    pub fn bytefy(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        borsh::ser::BorshSerialize::serialize(self, &mut result).unwrap();
        result
    }
}

#[derive(BorshSerialize)]
pub struct MemeNameAsArg {
    meme: AccountId,
}

impl MemeNameAsArg {
    pub fn new(meme: AccountId) -> Self {
        MemeNameAsArg { meme }
    }

    pub fn bytefy(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        borsh::ser::BorshSerialize::serialize(self, &mut result).unwrap();
        result
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Globals {
    pub memes: UnorderedSet<AccountId>,
    pub contributors: UnorderedSet<AccountId>,
    pub owners: UnorderedSet<AccountId>,
}

impl Default for Globals {
    fn default() -> Self {
        let memes: UnorderedSet<AccountId> = UnorderedSet::new("m".as_bytes());
        let contributors: UnorderedSet<AccountId> = UnorderedSet::new("c".as_bytes());
        let owners: UnorderedSet<AccountId> = UnorderedSet::new("o".as_bytes());

        Globals {
            memes,
            contributors,
            owners,
        }
    }
}
