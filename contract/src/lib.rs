
pub mod event;
use std::vec;
use near_sdk::{near_bindgen, collections::{UnorderedMap, LookupMap}, AccountId, Balance, borsh::BorshSerialize, env::{self}, PanicOnDefault, Promise, serde::{Deserialize, Serialize}};
use near_sdk::borsh::{self, BorshDeserialize};
use event::*;

pub type ECID = String;
pub type DecryptedKey = String;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum State {
    Private,
    Public
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct DataValue {
    pub public_key: String,
    pub encrypted_cid: ECID,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Metadata {
    pub state: State,
    pub title: String,
    pub tags: String,
    pub owner: AccountId,
    pub cid_encrypted: String,
    pub price: Balance,
    pub list_access: Vec<AccountId>
}




#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    contract_owner: AccountId,
    data_by_owner: LookupMap<AccountId,Vec<ECID>>,
    data_by_id: LookupMap<ECID,Metadata>,
    access_by_user: LookupMap<AccountId,Vec<ECID>>,
    total_access: UnorderedMap<u32, Metadata>,
    keys_by_account: LookupMap<AccountId, Vec<DecryptedKey>>,
    access_by_data: LookupMap<ECID, LookupMap<AccountId, String>> //the account_id and their pub_key in a data

}

pub trait Function {
    fn new()-> Self;
    fn get_data_by_id(&self, data_id: ECID) -> Metadata;
    fn new_meta_data(&mut self, state: State, title_given: String, tags_given: String, owner: AccountId, cid_encrypted_given: ECID) -> Metadata;
    fn set_state(&mut self, state: State, cid: ECID) -> Metadata;
    fn access_to_data(&mut self, cid: ECID, user_id: AccountId, pub_key: String) ;
    fn purchase(&mut self, cid: ECID, buyer_id: AccountId, pub_key: String) -> Metadata;
    fn payment(&self,sender: AccountId, receiver: AccountId, deposit: Balance);
    fn is_accessed(&self, encrypted_id: ECID, user_id: AccountId) -> bool;
    fn get_published_data(&self) -> Vec<Metadata> ;
    fn get_accessed_data_by_user(&self, user_id: AccountId) -> Vec<Metadata>;
    fn get_data_by_owner(&self, owner_account_id: AccountId)-> Vec<Metadata>;
    fn get_user_by_data(&self, encrypted_cid: ECID) -> Vec<AccountId>;
    fn get_data_value(&self, encrypted_cid: ECID) -> DataValue;
}

#[near_bindgen]
impl Function for Contract {
    #[init]
    fn new() -> Self {
        Self {
            contract_owner: env::signer_account_id(),
            data_by_owner: LookupMap::new(b"data by user".try_to_vec().unwrap()),
            data_by_id:  LookupMap::new(b"data by id".try_to_vec().unwrap()),
            access_by_user: LookupMap::new(b"access by user".try_to_vec().unwrap()),
            total_access: UnorderedMap::new(b"total access".try_to_vec().unwrap()),
            keys_by_account: LookupMap::new(b"keys by account".try_to_vec().unwrap()),
            access_by_data: LookupMap::new(b"access_by_data".try_to_vec().unwrap()),
        }
    }
// chỗ này có vấn đề vì mình không biết cái CID

    fn new_meta_data(&mut self, state: State, title_given: String, tags_given: String, owner: AccountId, cid_encrypted_given: ECID) -> Metadata {
        let meta_data = Metadata {
            state,
            title: title_given.clone(),
            tags: tags_given.clone(),
            owner: owner.clone(),
            cid_encrypted: cid_encrypted_given.clone(),
            price: 0,
            list_access: vec![]
        };
        self.data_by_id.insert(&cid_encrypted_given,&meta_data);
        let mut vec_data_by_owner = self.data_by_owner.get(&owner).unwrap_or_else(||vec![]);
        vec_data_by_owner.push(cid_encrypted_given);
        self.data_by_owner.insert(&owner,&vec_data_by_owner);
        meta_data 
    }

    fn set_state(&mut self, state: State, encrypted_cid: ECID) -> Metadata { //set trạng thái public/private cho data
        //kiểm tra điều kiện : owner? 
        self.data_by_id.get(&encrypted_cid).unwrap()
    }
    
    fn access_to_data(&mut self, encrypted_cid: ECID, user_id: AccountId, pub_key: String) { // access 1 user vào data
        //kiểm tra người dùng == owner?, ...
    }

    #[payable]
    fn purchase(&mut self, encrypted_cid: ECID, buyer_id: AccountId, pub_key: String)-> Metadata {
        let data = self.data_by_id.get(&encrypted_cid).unwrap();
        let deposit = env::attached_deposit();
        assert_ne!(env::signer_account_id(), data.owner, "You are owner of this data!");
        assert_eq!(deposit, data.price, "Invalid deposit!");
        self.payment(buyer_id.clone(), data.owner, deposit);
        self.access_to_data(encrypted_cid.clone(), buyer_id, pub_key);
        self.get_data_by_id(encrypted_cid.clone());
        self.data_by_id.get(&encrypted_cid).unwrap()
    }

    fn payment(&self,sender: AccountId, receiver: AccountId, amount: Balance) {
        let payment_info = EventLog { //info of transaction
            standard: "e-comerce-1.0.0".to_string(),
            event: EventLogVariant::Purchase(vec![PurchaseProduct {
                receiver: receiver.to_string(),
                sender: sender.to_string(),
                amount,
                memo: None,
            }])
        };
        //add new checker into checkers of this campaign
        env::log_str(&payment_info.to_string());   
        Promise::new(receiver).transfer(amount);
    }

    fn is_accessed(&self, encrypted_id: ECID, user_id: AccountId) -> bool {
        let data = self.data_by_id.get(&encrypted_id).unwrap();
        if user_id == data.owner {
            return true;
        }
        //query logic...
        true
    }

    //=====================GET FUNCTION=============================

    fn get_data_by_id(&self, data_id: ECID) -> Metadata {
        let data = self.data_by_id.get(&data_id).clone().unwrap();
        data
        
    }

    //al data which have been published for market
    fn get_published_data(&self) -> Vec<Metadata> {
        let mut vec_data: Vec<Metadata> = vec![];
        let published_data = &self.total_access;
        for i in 0..published_data.len() {
            let data = published_data.get(&(i as u32)).unwrap();
            vec_data.push(data);
        }
        vec_data
    }
    
    //all data which are bought by a user
    fn get_accessed_data_by_user(&self, user_id: AccountId) -> Vec<Metadata> {
        let mut vec_data = vec![];
        let vec_data_id: Vec<ECID> = self.keys_by_account.get(&user_id).unwrap_or_else(|| vec![]);
        for i in vec_data_id {
            let data = self.get_data_by_id(i);
            vec_data.push(data);
        }
        vec_data
    }

    //all data which are owned by a particular owner
    fn get_data_by_owner(&self, owner_account_id: AccountId)-> Vec<Metadata> {
        let mut vec_data: Vec<Metadata> = vec![];
        let all_data = self.data_by_owner.get(&owner_account_id).unwrap_or_else(|| vec![]);
        for i in all_data {
            let data = self.get_data_by_id(i);
            vec_data.push(data);
        }
        vec_data
    }

    //all users who bought a particular data
    fn get_user_by_data(&self, encrypted_cid: ECID) -> Vec<AccountId> {
        let mut vec_account: Vec<AccountId> = vec![];
        let vec_id = self.data_by_id.get(&encrypted_cid).unwrap().list_access;
        for id in vec_id {
            vec_account.push(id);
        }
        vec_account
    }

    fn get_data_value(&self, encrypted_cid: ECID) -> DataValue {
        let list_access = self
            .access_by_data
            .get(&encrypted_cid)
            .unwrap_or_else(||LookupMap::new(b"access_by_data".try_to_vec().unwrap()));
        let singer = env::signer_account_id();
        assert!(list_access.contains_key(&singer), "You dont have permision to get this data!");
        DataValue {
            public_key: list_access.get(&env::signer_account_id()).unwrap(),
            encrypted_cid:encrypted_cid 
        }
    }

}
