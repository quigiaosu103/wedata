use near_sdk::Balance;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::borsh::{self, BorshDeserialize};
use near_sdk::{AccountId, borsh::BorshSerialize};


#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct UserMetaData {
    pub name: String,
    pub email_address: String,
    pub age: u32,
    pub git: String,
    pub location: String,
}
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct User {
    pub id: AccountId,
    pub balance: Balance,
    pub meta_data: UserMetaData
}
