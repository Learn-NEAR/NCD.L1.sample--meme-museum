mod models;

// #[allow(unused_imports)]
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    // json_types::{U64}
    env,
    near_bindgen,
    AccountId,
    Gas,
    Promise,
    PromiseIndex,
    PromiseResult,
    serializer, PanicOnDefault,
};


pub use models::{Globals, MemeInitArgs, MemeNameAsArg, Museum};
use utils::{Category, MIN_ACCOUNT_BALANCE, MUSEUM_KEY, XCC_GAS};

near_sdk::setup_alloc!();

// import meme contract bytecode as a Byte Vec
#[allow(non_snake_case)]
fn CODE() -> Vec<u8> {
    include_bytes!("../../target/wasm32-unknown-unknown/release/meme.wasm").to_vec()
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    museum: Museum,
    pub globals: Globals,
}

#[near_bindgen]
impl Contract {
    #[payable]
    #[init]
    #[serializer(borsh)]
    pub fn new(name: String, owners: Vec<AccountId>) -> Self {
        // contract may only be initialized once
        assert!(!is_initialized(), "Contract is already initialized.");

        // storing meme metadata requires some storage staking (balance locked to offset cost of data storage)
        assert!(
            env::attached_deposit() >= MIN_ACCOUNT_BALANCE,
            "Minimum account balance must be attached to initialize this contract (3 NEAR)",
        );

        // Must have at least 1 owner account
        assert!(owners.len() > 0, "Must specify at least 1 owner");

        let mut globals: Globals = Globals::default();
        // create the museum using incoming metadata
        let museum = Museum::create(&mut globals, name, owners);

        env::log("museum was created".as_bytes());

        Contract { 
            museum,
            globals, 
        }
    }

    // We must set serializer here or the compiler will try using borsh
    #[result_serializer(borsh)]
    pub fn get_museum(&mut self) -> Museum {
        assert_contract_is_initialized();
        Museum::get()
    }

    pub fn get_owner_list(&self) -> Vec<AccountId> {
        assert_contract_is_initialized();
        Museum::get_owner_list(&self.globals)
    }

    pub fn get_meme_list(&self) -> Vec<AccountId> {
        assert_contract_is_initialized();
        Museum::get_meme_list(&self.globals)
    }

    pub fn get_meme_count(&self) -> u32 {
        assert_contract_is_initialized();
        Museum::get_meme_count(&self.globals)
    }

    //
    // Manage your status as a contributor
    //

    pub fn add_myself_as_contributor(&mut self) {
        assert_contract_is_initialized();
        Museum::add_contributor(&mut self.globals, &env::predecessor_account_id());
    }

    pub fn remove_myself_as_contributor(&mut self) {
        assert_contract_is_initialized();
        Museum::remove_contributor(&mut self.globals, &env::predecessor_account_id());
    }

    // Add your meme

    pub fn add_meme(&mut self, meme: AccountId, title: String, data: String, category: Category) {
        assert_contract_is_initialized();
        assert_signed_by_contributor_or_owner(&self.globals);

        // storing meme metadata requires some storage staking (balance locked to offset cost of data storage)
        assert!(
            env::attached_deposit() >= MIN_ACCOUNT_BALANCE,
            "Minimum account balance must be attached to initialize a meme (3 NEAR)",
        );

        let account_id = full_account_for(meme.clone().to_string());

        assert!(
            env::is_valid_account_id(account_id.as_bytes()),
            "Meme name must be valid NEAR account name"
        );
        assert!(
            !Museum::has_meme(&self.globals, &account_id),
            "Meme already exists"
        );

        env::log("Attempting to create meme".as_bytes());

        let promise = Promise::new(account_id)
            .create_account()
            .deploy_contract(CODE())
            .add_full_access_key(env::signer_account_pk());

        promise
            .function_call(
                Vec::from("init".as_bytes()),
                MemeInitArgs::new(title, data, category).bytefy(),
                env::attached_deposit(),
                Gas::from(XCC_GAS as u64),
            )
            .then(Promise::new(env::current_account_id()).function_call(
                Vec::from("on_meme_created".as_bytes()),        // method_name: Vec<u8>,
                MemeNameAsArg::new(meme).bytefy(), // arguments: Vec<u8>,
                0,                                 // amount: Balance,
                Gas::from(XCC_GAS as u64),         //gas: Gas,
            ));
    }

    pub fn on_meme_created(&mut self, meme: AccountId) {
        assert_ne!(env::promise_results_count(), 0,);
        let results = env::promise_result(0);

        let meme_name: AccountId = full_account_for(meme.to_string());

        let data: Vec<u8> = match results {
            PromiseResult::NotReady => {
                // promise result is not complete
                env::log(format!("Meme creation for [{}] is pending", meme_name).as_bytes());
                return;
            }
            PromiseResult::Successful(value) => {
                // promise result is complete and successful
                env::log(format!("Meme creation for [{}] succeeded", meme_name).as_bytes());
                value
            }
            PromiseResult::Failed => {
                // promise result is complete and failed
                env::log(format!("Meme creation for [{}] failed", meme_name).as_bytes());
                return;
            }
        };

        let meme: AccountId = BorshDeserialize::deserialize(&mut &data[..]).unwrap();
        self.globals.memes.insert(&meme);
    }

    // Governance methods reserved for 101Labs and NEAR admins
    pub fn add_contributor(&mut self, account: AccountId) {
        assert_contract_is_initialized();
        assert_signed_by_owner(&self.globals);

        Museum::add_contributor(&mut self.globals, &account);

        env::log("Contributor was added".as_bytes());
    }

    pub fn remove_contributor(&mut self, account: AccountId) {
        assert_contract_is_initialized();
        assert_signed_by_owner(&self.globals);

        Museum::remove_contributor(&mut self.globals, &account);
    }

    pub fn add_owner(&mut self, account: AccountId) {
        assert_contract_is_initialized();
        assert_signed_by_owner(&self.globals);

        Museum::add_owner(&mut self.globals, &account);
    }

    pub fn remove_owner(&mut self, account: AccountId) {
        assert_contract_is_initialized();
        assert_signed_by_owner(&self.globals);

        Museum::remove_owner(&mut self.globals, &account);
    }

    pub fn remove_meme(&mut self, meme: AccountId) {
        assert_contract_is_initialized();
        assert_signed_by_owner(&self.globals);

        let promise_id: PromiseIndex =
            env::promise_batch_create(&full_account_for(meme.to_string()));

        env::promise_batch_action_delete_account(promise_id, &env::current_account_id());

        let promise_id: PromiseIndex =
            env::promise_batch_then(promise_id, &env::current_account_id());

        env::promise_batch_action_function_call(
            promise_id,
            "on_meme_removed".as_bytes(),
            &MemeNameAsArg::new(meme.clone()).bytefy(),
            0,
            Gas::from(XCC_GAS as u64),
        );
    }

    pub fn on_meme_removed(&mut self, meme: AccountId) {
        // TODO: confirm that promise was successful
        env::log(format!("[{}] was removed", full_account_for(meme.clone().into())).as_bytes());

        Museum::remove_meme(&mut self.globals, &meme);
    }
}

//
// == PRIVATE FUNCTIONS ========================================================
//
// Helper functions that are not part of the contract interface
//
//
// Track whether or not the meme has been initialized.
//

fn is_initialized() -> bool {
    env::storage_has_key(MUSEUM_KEY().as_bytes())
}

fn assert_contract_is_initialized() {
    assert!(is_initialized(), "Contract must be initialized first.");
}

// Indicate whether contract caller is the creator

fn is_owner(globals: &Globals) -> bool {
    Museum::has_owner(globals, &env::predecessor_account_id())  
}

fn is_contributor(globals: &Globals) -> bool {
    Museum::is_contributor(globals, &env::predecessor_account_id())
}

fn assert_signed_by_owner(globals: &Globals) {
    assert!(
        is_owner(globals),
        "This method can only be called by a museum owner",
    );
}

fn assert_signed_by_contributor_or_owner(globals: &Globals) {
    assert!(
        is_contributor(globals) || is_owner(globals),
        "This method can only be called by a museum contributor or owner"
    );
}

fn full_account_for(meme: String) -> AccountId {
    AccountId::try_from(format!("{}.{}", meme, env::current_account_id())).unwrap()
}

