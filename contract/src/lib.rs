
use near_sdk::{near_bindgen, collections::{UnorderedMap, LookupMap}, AccountId, Balance, borsh::BorshSerialize, env::{self}, PanicOnDefault, Promise, serde::{Deserialize, Serialize}};
use near_sdk::borsh::{self, BorshDeserialize};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]

pub struct Metadata {
    pub state: bool,
    pub title: String,
    pub tags: String,
    pub createby: AccountId,
    pub cid_encrypted: String,
    pub shareadress: Vec<AccountId>,
}

pub type CID = String;

#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    contract_owner: AccountId,
    data_by_owner: LookupMap<AccountId,CID>,
    data_by_id: LookupMap<CID,Metadata>,
    access_by_user: LookupMap<AccountId,Vec<CID>>,
    total_access: UnorderedMap<Metadata>,
}

pub impl Contract {

    pub fn purchase(cid: CID, buyer_id: AccountId) -> Metadata {
        let owner = self.data_by_id.get(&cid).unwrap();
    }

    pub fn transaction() -> Promise {
        Promise::new()
    }
}