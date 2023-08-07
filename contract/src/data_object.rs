use near_sdk::Balance;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::borsh::{self, BorshDeserialize};
use near_sdk::{AccountId, borsh::BorshSerialize};
pub type DataKey  = String;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum AccessType {
    Private,
    Public
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct DataObject {
    pub data_key: DataKey,
    pub title: String,
    pub description: String,
    pub owner: AccountId,
    pub price: Balance,
    pub access_type: AccessType
}

