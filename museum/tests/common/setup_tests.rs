use near_sdk::{
    Balance,
    BlockHeight,
    EpochHeight,
    Gas,
    PublicKey,
    StorageUsage,
    test_utils::{VMContextBuilder},
    json_types::ValidAccountId,
};

fn validate(account_id: String) -> ValidAccountId {
    ValidAccountId::try_from(account_id).unwrap()
}


fn to_bool(bytes: &[u8]) -> bool {
    let (int_bytes, _) = bytes.split_at(std::mem::size_of::<bool>());
    // If  the byte == 1 return true, else false
    let byte = u8::from_le_bytes(int_bytes.try_into().unwrap());
    assert!(byte<=1, "Invalid type for boolean, byte value is ({})", byte);
    byte == 1
}

#[allow(non_camel_case_types)]
pub trait VMCB_ext{
    fn to_account_id(&self) -> ValidAccountId;

    fn to_u64(&self) -> u64;

    fn to_u128(&self) -> u128;

    fn to_vec_u8(&self) -> Vec<u8>;

    fn to_bool(&self) -> bool;
}



impl VMCB_ext for usize{
    fn to_u64(&self) -> u64 { *self as u64 }

    fn to_account_id(&self) -> ValidAccountId { panic!("Invalid type for conversion in Macro") }

    fn to_u128(&self) -> u128 { *self as u128 }

    fn to_vec_u8(&self) -> Vec<u8> { panic!("Invalid type for conversion in Macro") }

    fn to_bool(&self) -> bool { *self != 0 }
}


impl VMCB_ext for i32{
    fn to_u64(&self) -> u64 { *self as u64 }

    fn to_account_id(&self) -> ValidAccountId { panic!("Invalid type for conversion in Macro") }

    fn to_u128(&self) -> u128 { *self as u128 }

    fn to_vec_u8(&self) -> Vec<u8> { panic!("Invalid type for conversion in Macro") }

    fn to_bool(&self) -> bool { *self != 0 }
}


impl VMCB_ext for i64{
    fn to_u64(&self) -> u64 { *self as u64 }

    fn to_account_id(&self) -> ValidAccountId { panic!("Invalid type for conversion in Macro") }

    fn to_u128(&self) -> u128 { *self as u128 }

    fn to_vec_u8(&self) -> Vec<u8> { panic!("Invalid type for conversion in Macro") }

    fn to_bool(&self) -> bool { *self != 0 }
}


impl VMCB_ext for u32{
    fn to_u64(&self) -> u64 { *self as u64 }

    fn to_account_id(&self) -> ValidAccountId { panic!("Invalid type for conversion in Macro") }

    fn to_u128(&self) -> u128 { *self as u128 }

    fn to_vec_u8(&self) -> Vec<u8> { panic!("Invalid type for conversion in Macro") }

    fn to_bool(&self) -> bool { *self != 0 }
}


impl VMCB_ext for u64{
    fn to_u64(&self) -> u64 {
        self.clone()
    }

    fn to_account_id(&self) -> ValidAccountId { panic!("Invalid type for conversion in Macro") }

    fn to_u128(&self) -> u128 { *self as u128 }

    fn to_vec_u8(&self) -> Vec<u8> { panic!("Invalid type for conversion in Macro") }

    fn to_bool(&self) -> bool { panic!("Invalid type for conversion in Macro") }
}


impl VMCB_ext for u128{
    fn to_u128(&self) -> u128 {
        self.clone()
    }

    fn to_account_id(&self) -> ValidAccountId { panic!("Invalid type for conversion in Macro") }

    fn to_u64(&self) -> u64 { panic!("Invalid type for conversion in Macro") }

    fn to_vec_u8(&self) -> Vec<u8> { panic!("Invalid type for conversion in Macro") }
    
    fn to_bool(&self) -> bool { panic!("Invalid type for conversion in Macro") }
}

impl VMCB_ext for Vec<u8>{
    fn to_vec_u8(&self) -> Vec<u8> {
        self.clone()
    }

    fn to_account_id(&self) -> ValidAccountId { panic!("Invalid type for conversion in Macro") }

    fn to_u64(&self) -> u64 { panic!("Invalid type for conversion in Macro") }

    fn to_u128(&self) -> u128 { panic!("Invalid type for conversion in Macro") }

    fn to_bool(&self) -> bool { panic!("Invalid type for conversion in Macro") }
}


impl VMCB_ext for String {
    fn to_account_id(&self) -> ValidAccountId {
        validate(self.clone())
    }

    fn to_u64(&self) -> u64 { panic!("Invalid type for conversion in Macro") }

    fn to_u128(&self) -> u128 { panic!("Invalid type for conversion in Macro") }

    fn to_vec_u8(&self) -> Vec<u8> { panic!("Invalid type for conversion in Macro") }

    fn to_bool(&self) -> bool { panic!("Invalid type for conversion in Macro") }
}


impl VMCB_ext for str{
    fn to_account_id(&self) -> ValidAccountId {
        validate(String::from(self))
    }

    fn to_u64(&self) -> u64 { panic!("Invalid type for conversion in Macro") }

    fn to_u128(&self) -> u128 { panic!("Invalid type for conversion in Macro") }

    fn to_vec_u8(&self) -> Vec<u8> { panic!("Invalid type for conversion in Macro") }

    fn to_bool(&self) -> bool { panic!("Invalid type for conversion in Macro") }
}


impl VMCB_ext for &str{
    fn to_account_id(&self) -> ValidAccountId {
        validate(String::from(*self))
    }

    fn to_u64(&self) -> u64 { panic!("Invalid type for conversion in Macro") }

    fn to_u128(&self) -> u128 { panic!("Invalid type for conversion in Macro") }

    fn to_vec_u8(&self) -> Vec<u8> { panic!("Invalid type for conversion in Macro") }

    fn to_bool(&self) -> bool { panic!("Invalid type for conversion in Macro") }
}


impl VMCB_ext for bool{
    fn to_account_id(&self) -> ValidAccountId { panic!("Invalid type for conversion in Macro") }

    fn to_u64(&self) -> u64 { panic!("Invalid type for conversion in Macro") }

    fn to_u128(&self) -> u128 { panic!("Invalid type for conversion in Macro") }

    fn to_vec_u8(&self) -> Vec<u8> { panic!("Invalid type for conversion in Macro") }

    fn to_bool(&self) -> bool { 
        to_bool(& Vec::from([self.clone().into()]))
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


#[macro_export]
macro_rules! setup_tests {

    (@partial_setup ()) => {};

    (@partial_setup $builder: ident,) => {};

    (@partial_setup $builder: ident, $invalid: expr) => {
        // println!("single {}", $invalid);
        compile_error!("Only allow pairs of arguments.");
    };

    (@partial_setup $builder: ident, $name: expr, $arg: expr, $($others: tt)*) => {
        // setup_tests!(@partial_setup $builder, $name, $arg);
        println!("Setting context attribute '{}' to the value '{}'", $name, $arg);
        common::setup_tests::VMCB::new(&$name, $arg).action(&mut $builder);

        setup_tests!(@partial_setup $builder, $($others)*);
    };

    () => {};
    ($name: expr, $arg: expr, $($others: tt)*) => {
        let mut builder = near_sdk::test_utils::VMContextBuilder::new();
        
        setup_tests!(@partial_setup builder, $name, $arg, $($others)*);

        near_sdk::testing_env!(builder.build());
    };
}
