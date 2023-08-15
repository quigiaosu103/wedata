
use near_sdk::{near_bindgen, collections::{UnorderedMap, LookupMap}, AccountId, Balance, borsh::BorshSerialize, env::{self}, PanicOnDefault, Promise, serde::{Deserialize, Serialize}};
use near_sdk::borsh::{self, BorshDeserialize};


#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum State {
    Private,
    Public
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Metadata {
    pub state: State,
    pub title: String,
    pub tags: String,
    pub createby: AccountId,
    pub cid_encrypted: String,
    pub shareadress: Vec<AccountId>,
}

pub type CID = String;
pub type DecryptedKey = String;

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    contract_owner: AccountId,
    data_by_owner: LookupMap<AccountId,CID>,
    data_by_id: LookupMap<CID,Metadata>,
    access_by_user: LookupMap<AccountId,Vec<CID>>,
    total_access: UnorderedMap<u32, Metadata>,
    keys_by_account: LookupMap<AccountId, Vec<DecryptedKey>>
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            contract_owner: env::signer_account_id(),
            data_by_owner: LookupMap::new(b"data by user".try_to_vec().unwrap()),
            data_by_id:  LookupMap::new(b"data by id".try_to_vec().unwrap()),
            access_by_user: LookupMap::new(b"access by user".try_to_vec().unwrap()),
            total_access: UnorderedMap::new(b"total access".try_to_vec().unwrap()),
            keys_by_account: LookupMap::new(b"keys by account".try_to_vec().unwrap())
        }
    }
// chỗ này có vấn đề vì mình không biết cái CID
    pub fn get_data_by_id(&self, data_id: CID) -> Metadata {
        let data = self.data_by_id.get(&data_id).clone().unwrap();
        data
        
    }

    pub fn new_meta_data(&mut self, state: State, title_given: String, tags_given: String, createby: AccountId, cid_encrypted_given: CID) -> Metadata {
        let meta_data = Metadata {
            state,
            title: title_given.clone(),
            tags: tags_given.clone(),
            createby: createby.clone(),
            cid_encrypted: cid_encrypted_given.clone(),
            shareadress: vec![]
        };
        self.data_by_id.insert(&cid_encrypted_given,&meta_data);
        self.data_by_owner.insert(&createby,&cid_encrypted_given);
        meta_data 
    }

    
    // #[payable]
    // pub fn purchase(cid: CID, buyer_id: AccountId) -> Metadata {
    //     let owner = self.data_by_id.get(&cid).unwrap();
    // }

    // pub fn transaction() -> Promise {
    //     Promise::new()
    // }
}
