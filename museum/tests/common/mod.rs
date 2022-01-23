use near_sdk::{
    Balance,
    BlockHeight,
    EpochHeight,
    Gas,
    PublicKey,
    StorageUsage,
    test_utils::{VMContextBuilder},
    testing_env,
    json_types::ValidAccountId, env::signer_account_id, AccountId,
};


// == CONFIG VALUES ============================================================
// Using functions because the compiler doesn't like the idea of global const strings

#[allow(non_snake_case)]
pub fn NAME() -> String { String::from("usain") }
#[allow(non_snake_case)]
pub fn MUSEUM_ACCOUNT_ID() -> String { String::from("museum") }
#[allow(non_snake_case)]
pub fn OWNER_ACCOUNT_ID() -> String { String::from("alice") }
#[allow(non_snake_case)]
pub fn CONTRIBUTOR_ACCOUNT_ID() -> String { String::from("bob") }


// == HELPER FUNCTIONS =========================================================

// pub fn current_account_id(&mut self, account_id: ValidAccountId) -> &mut Self
// pub fn signer_account_id(&mut self, account_id: ValidAccountId) -> &mut Self
// pub fn signer_account_pk(&mut self, pk: PublicKey) -> &mut Self
// pub fn predecessor_account_id( &mut self, account_id: ValidAccountId) -> &mut Self
// pub fn block_index(&mut self, block_index: BlockHeight) -> &mut Self
// pub fn block_timestamp(&mut self, block_timestamp: u64) -> &mut Self
// pub fn epoch_height(&mut self, epoch_height: EpochHeight) -> &mut Self
// pub fn account_balance(&mut self, amount: Balance) -> &mut Self
// pub fn account_locked_balance(&mut self, amount: Balance) -> &mut Self
// pub fn storage_usage(&mut self, usage: StorageUsage) -> &mut Self
// pub fn attached_deposit(&mut self, amount: Balance) -> &mut Self
// pub fn prepaid_gas(&mut self, gas: Gas) -> &mut Self
// pub fn random_seed(&mut self, seed: Vec<u8>) -> &mut Self
// pub fn is_view(&mut self, is_view: bool) -> &mut Self
// pub fn build(&self) -> VMContext'

// current_account_id
// signer_account_id
// signer_account_pk
// predecessor_account_id
// block_index
// block_timestamp
// epoch_height
// account_balance
// account_locked_balance
// storage_usage
// attached_deposit
// prepaid_gas
// random_seed
// is_view
// build

fn validate(account_id: String) -> ValidAccountId {
    ValidAccountId::try_from(account_id).unwrap()
}

fn to_account_id(bytes: &[u8]) -> ValidAccountId {
    let mut account_id = bytes;
    let account_id = String::from(String::from_utf8_lossy(&bytes));
    validate(account_id)
}

fn to_vec_u8(bytes: &[u8]) -> Vec<u8> {
    Vec::from(bytes)
}

// We will use little endian for u64 and u128
fn to_u64(bytes: &[u8]) -> u64 {
    // bytes.try_into().unwrap()
    let (int_bytes, _) = bytes.split_at(std::mem::size_of::<u64>());
    // *input = rest;
    u64::from_le_bytes(int_bytes.try_into().unwrap())
}

// We will use little endian for u64 and u128
fn to_u128(bytes: &[u8]) -> u128 {
    // bytes.try_into().unwrap()
    let (int_bytes, _) = bytes.split_at(std::mem::size_of::<u128>());
    // *input = rest;
    u128::from_le_bytes(int_bytes.try_into().unwrap())
}

fn to_bool(bytes: &[u8]) -> bool {
    let (int_bytes, _) = bytes.split_at(std::mem::size_of::<bool>());
    // If  the byte == 1 return true, else false
    let byte = u8::from_le_bytes(int_bytes.try_into().unwrap());
    assert!(byte<=1, "Invalid type for boolean, byte value is ({})", byte);
    byte == 1
}




pub trait VMCB_ext{
    fn bytefy(&self) -> Vec<u8>;

    fn to_account_id(&self) -> ValidAccountId;

    fn to_u64(&self) -> u64;

    fn to_u128(&self) -> u128;

    fn to_vec_u8(&self) -> Vec<u8>;

    fn to_bool(&self) -> bool;
}

impl VMCB_ext for u64{
    fn bytefy(&self) -> Vec<u8> {
        Vec::from(&self.to_le_bytes()[..])
    }

    fn to_u64(&self) -> u64 {
        to_u64(&self.bytefy())
    }

    fn to_account_id(&self) -> ValidAccountId { panic!("Invalid type for conversion in Macro") }

    fn to_u128(&self) -> u128 { panic!("Invalid type for conversion in Macro") }

    fn to_vec_u8(&self) -> Vec<u8> { panic!("Invalid type for conversion in Macro") }

    fn to_bool(&self) -> bool { panic!("Invalid type for conversion in Macro") }
}

impl VMCB_ext for u128{
    fn bytefy(&self) -> Vec<u8> {
        Vec::from(&self.to_le_bytes()[..])
    }

    fn to_u128(&self) -> u128 {
        to_u128(&self.bytefy())
    }

    fn to_account_id(&self) -> ValidAccountId { panic!("Invalid type for conversion in Macro") }

    fn to_u64(&self) -> u64 { panic!("Invalid type for conversion in Macro") }

    fn to_vec_u8(&self) -> Vec<u8> { panic!("Invalid type for conversion in Macro") }
    
    fn to_bool(&self) -> bool { panic!("Invalid type for conversion in Macro") }
}

impl VMCB_ext for Vec<u8>{
    fn bytefy(&self) -> Vec<u8> {
        self.clone()
    }

    fn to_vec_u8(&self) -> Vec<u8> {
        to_vec_u8(&self.bytefy())
    }

    fn to_account_id(&self) -> ValidAccountId { panic!("Invalid type for conversion in Macro") }

    fn to_u64(&self) -> u64 { panic!("Invalid type for conversion in Macro") }

    fn to_u128(&self) -> u128 { panic!("Invalid type for conversion in Macro") }

    fn to_bool(&self) -> bool { panic!("Invalid type for conversion in Macro") }
}

impl VMCB_ext for String {
    fn bytefy(&self) -> Vec<u8> {
        Vec::from(self.as_bytes())
    }

    fn to_account_id(&self) -> ValidAccountId {
        to_account_id(&self.bytefy())
    }

    fn to_u64(&self) -> u64 { panic!("Invalid type for conversion in Macro") }

    fn to_u128(&self) -> u128 { panic!("Invalid type for conversion in Macro") }

    fn to_vec_u8(&self) -> Vec<u8> { panic!("Invalid type for conversion in Macro") }

    fn to_bool(&self) -> bool { panic!("Invalid type for conversion in Macro") }
}

impl VMCB_ext for str{
    fn bytefy(&self) -> Vec<u8> {
        Vec::from(self.as_bytes())
    }

    fn to_account_id(&self) -> ValidAccountId {
        to_account_id(&self.bytefy())
    }

    fn to_u64(&self) -> u64 { panic!("Invalid type for conversion in Macro") }

    fn to_u128(&self) -> u128 { panic!("Invalid type for conversion in Macro") }

    fn to_vec_u8(&self) -> Vec<u8> { panic!("Invalid type for conversion in Macro") }

    fn to_bool(&self) -> bool { panic!("Invalid type for conversion in Macro") }
}

impl VMCB_ext for bool{
    fn bytefy(&self) -> Vec<u8> {
        Vec::from([self.clone().into()])
    }

    fn to_account_id(&self) -> ValidAccountId { panic!("Invalid type for conversion in Macro") }

    fn to_u64(&self) -> u64 { panic!("Invalid type for conversion in Macro") }

    fn to_u128(&self) -> u128 { panic!("Invalid type for conversion in Macro") }

    fn to_vec_u8(&self) -> Vec<u8> { panic!("Invalid type for conversion in Macro") }

    fn to_bool(&self) -> bool { 
        to_bool(&self.bytefy())
    }

}


pub enum VMCB {
    CurrentAccountId(ValidAccountId),
    SignerAccountId(ValidAccountId),
    SignerAccountPK(PublicKey),
    PredecessorAccountId(ValidAccountId),
    BlockIndex(BlockHeight),
    BlockTimestamp(u64),
    EpochHeight(EpochHeight),
    AccountBalance(Balance),
    AccountLockedBalance(Balance),
    StorageUsage(StorageUsage),
    AttachedDeposit(Balance),
    PrepaidGas(Gas),
    RandomSeed(Vec<u8>),
    IsView(bool),
}

impl VMCB{
    pub fn new<D: VMCB_ext> (name: &str, arg: D) -> VMCB{
        match name{
            "current_account_id" => {       VMCB::CurrentAccountId(arg.to_account_id()) },
            "signer_account_id" => {        VMCB::SignerAccountId(arg.to_account_id()) },
            "signer_account_pk" => {        VMCB::SignerAccountPK(arg.to_vec_u8()) },
            "predecessor_account_id" => {   VMCB::PredecessorAccountId(arg.to_account_id()) },
            "block_index" => {              VMCB::BlockIndex(arg.to_u64())},
            "block_timestamp" => {          VMCB::BlockTimestamp(arg.to_u64())},
            "epoch_height" => {             VMCB::EpochHeight(arg.to_u64()) },
            "account_balance" => {          VMCB::AccountBalance(arg.to_u128()) },
            "account_locked_balance" => {   VMCB::AccountLockedBalance(arg.to_u128()) },
            "storage_usage" => {            VMCB::StorageUsage(arg.to_u64())},
            "attached_deposit" => {         VMCB::AttachedDeposit(arg.to_u128()) },
            "prepaid_gas" => {              VMCB::PrepaidGas(arg.to_u64()) },
            "random_seed" => {              VMCB::RandomSeed(arg.to_vec_u8()) },
            "is_view" => {                  VMCB::IsView(arg.to_bool()) },
            other => {
                panic!("Invalid string arg for VMCB {}", other);
            }
        }
    }

    // Consume self to do the action in the builder
    pub fn action(self, builder: &mut VMContextBuilder){
        match self{
            VMCB::CurrentAccountId(value) => { builder.current_account_id(value); },
            VMCB::SignerAccountId(value) => { builder.signer_account_id(value); },
            VMCB::SignerAccountPK(value) => { builder.signer_account_pk(value); },
            VMCB::PredecessorAccountId(value) => { builder.predecessor_account_id(value);},
            VMCB::BlockIndex(value) => { builder.block_index(value); },
            VMCB::BlockTimestamp(value) => { builder.block_timestamp(value); },
            VMCB::EpochHeight(value) => { builder.epoch_height(value); },
            VMCB::AccountBalance(value) => { builder.account_balance(value); },
            VMCB::AccountLockedBalance(value) => { builder.account_locked_balance(value); },
            VMCB::StorageUsage(value) => { builder.storage_usage(value);},
            VMCB::AttachedDeposit(value) => { builder.attached_deposit(value); },
            VMCB::PrepaidGas(value) => { builder.prepaid_gas(value);},
            VMCB::RandomSeed(value) => {builder.random_seed(value);},
            VMCB::IsView(value) => { builder.is_view(value);},
        }
    }
}

// pub fn set_context(
//     current_account_id: Option<String>,
//     signer_account_id: Option<String>,
//     signer_account_pk: Option<PublicKey>,
//     predecessor_account_id: Option<String>,
//     block_index: Option<BlockHeight>,
//     block_timestamp: Option<u64>,
//     epoch_height: Option<EpochHeight>,
//     account_balance: Option<Balance>,
//     account_locked_balance: Option<Balance>,
//     storage_usage: Option<StorageUsage>,
//     attached_deposit: Option<Balance>,
//     prepaid_gas: Option<Gas>,
//     random_seed: Option<Vec<u8>>,
//     is_view: Option<bool>,
// ) {
//     let builder: VMContextBuilder = VMContextBuilder::new();

//     // validating strings
//     let current_account_id: Option<ValidAccountId> = validate(current_account_id);
//     let signer_account_id: Option<ValidAccountId> = validate(signer_account_id);
//     let predecessor_account_id: Option<ValidAccountId> = validate(predecessor_account_id);

    

// }
