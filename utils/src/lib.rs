use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    // AccountId,
    json_types::ValidAccountId,
};
// == CONSTANTS ================================================================

// ONE_NEAR = unit of NEAR token in yocto Ⓝ (1e24)
// XCC_GAS = gas for cross-contract calls, ~5 Tgas (teragas = 1e12) per "hop"
// MIN_ACCOUNT_BALANCE = 3 NEAR min to keep account alive via storage staking

// TODO: revist MIN_ACCOUNT_BALANCE after some real data is included b/c this
//  could end up being much higher

pub const ONE_NEAR: u128 = 1_000_000_000_000_000_000_000_000;
pub const XCC_GAS: u128 = 20_000_000_000_000;
pub const MIN_ACCOUNT_BALANCE: u128 = ONE_NEAR * 3;

// common keys for singleton instances and initialization
#[allow(non_snake_case)]
pub fn MEME_KEY() -> String {
    String::from("state")
}
#[allow(non_snake_case)]
pub fn MUSEUM_KEY() -> String {
    String::from("state")
}

// size constraints
pub const PAGE_SIZE: u32 = 10;
pub const MAX_COMMENT_LENGTH: u32 = 500;

// == TYPES =============================================================

// Money in NEAR is just a u128
pub type Money = u128;

// Timestamp in NEAR is a number.
pub type Timestamp = u64;

/// Category for grouping memes
#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Category {
    A,
    B,
    C,
    D,
}

/// An example of what this does:
/// let a = Category::B;
/// // a converted into i8 is 1
/// assert_eq!(a as i8, 1);
///
impl From<i8> for Category {
    fn from(value: i8) -> Self {
        match value {
            0 => Category::A,
            1 => Category::B,
            2 => Category::C,
            4 => Category::D,
            v => panic!("Tried to convert this value {} into category", v),
        }
    }
}

// When testing, we use comparisons for assertions,
// In order to use == we need to implement PartialEq for this type
impl PartialEq for Category {
    fn eq(&self, other: &Category) -> bool {
        match (&self, &other) {
            (Category::A, Category::A) => true,
            (Category::B, Category::B) => true,
            (Category::C, Category::C) => true,
            (Category::D, Category::D) => true,
            (_, _) => false,
        }
    }
}

// == FUNCTIONS ===============================================================

/// Converts Yocto Ⓝ token quantity into the Amount in NEAR, as a String.
#[allow(non_snake_case)]
pub fn asNEAR(amount: u128) -> String {
    format!("{}", amount / ONE_NEAR)
}

/// Converts Amount in NEAR into Yocto Ⓝ token quantity.
#[allow(non_snake_case)]
pub fn toYocto(amount: u128) -> u128 {
    ONE_NEAR * amount
}

/// Validates a string into AccountId format
pub fn validate(account_id: &str) -> ValidAccountId {
    match ValidAccountId::try_from(String::from(account_id)){
        Ok(value) => value,
        Err(err) => {
            panic!("Failed to parse account_id ({}). Error message: ({}).", account_id, err);
        }
    }
}
