
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
    pub user_id: String,
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

impl  Metadata {
    fn update_state(&mut self, new_state: State) {
        self.state = new_state;
    }
    fn push_shareaddress(&mut self, account_id: AccountId) {
        self.list_access.push(account_id);
    }
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    contract_owner: AccountId,
    data_by_owner: LookupMap<AccountId,Vec<ECID>>,
    data_by_id: LookupMap<ECID,Metadata>,
    access_by_user: LookupMap<AccountId,Vec<ECID>>,
    total_published: UnorderedMap<u32, Metadata>,
    keys_by_account: LookupMap<AccountId, Vec<DecryptedKey>>,
    access_by_data: LookupMap<ECID, AccountId> //the account_id and their pub_key in a data

}

pub trait Function {
    fn new()-> Self;
    fn get_data_by_id(&self, data_id: ECID) -> Metadata;
    fn new_meta_data(&mut self, title_given: String, tags_given: String, cid_encrypted_given: ECID) -> Metadata;
    //chuyển trạng thái của data private-> public  và set price
    fn set_state(&mut self, state: State, cid: ECID, price: Balance) -> Metadata;
    fn access_to_data(&mut self, cid: ECID, user_id: AccountId, pub_key: String) ;
    fn purchase(&mut self, cid: ECID, pub_key: String) -> Metadata;
    fn payment(&self,sender: AccountId, receiver: AccountId, deposit: Balance);
    fn is_accessed(&self, encrypted_id: ECID, user_id: AccountId) -> bool;
    //lấy những data đã được public (xuất hiện trên marketplace)
    fn get_published_data(&self) -> Vec<Metadata> ;
    //lấy những data mà user đã mua
    fn get_accessed_data_by_user(&self, user_id: AccountId) -> Vec<Metadata>;
    //Những data của người upload lên
    fn get_data_by_owner(&self, owner_account_id: AccountId)-> Vec<Metadata>;
    //những user đã mua data đó
    fn get_user_by_data(&self, encrypted_cid: ECID) -> Vec<AccountId>;
    //xét điều kiện và trả về accountId của người được quyền access và key
    fn get_data_value(&self, encrypted_cid: ECID) -> DataValue;
    fn update_access_data(&mut self,account_id: AccountId, cid: ECID);
    fn update_data_share_address(&mut self,account_id: AccountId, cid: ECID) ;
    fn update_total_publish(&mut self,account_id: AccountId);
    fn replace_metadata_by_key(&mut self, key_to_replace: u32, new_metadata: Metadata);
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
            total_published: UnorderedMap::new(b"total access".try_to_vec().unwrap()),
            keys_by_account: LookupMap::new(b"keys by account".try_to_vec().unwrap()),
            access_by_data: LookupMap::new(b"access_by_data".try_to_vec().unwrap()),
        }
    }
// chỗ này có vấn đề vì mình không biết cái CID

    fn new_meta_data(&mut self, title_given: String, tags_given: String, cid_encrypted_given: ECID) -> Metadata {
        let owner = env::signer_account_id();
        let meta_data = Metadata {
            state: State::Private,
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
    

    fn set_state(&mut self, state: State, cid: ECID, price: Balance) -> Metadata { //thay đổi trạng thái data (private/public)
        let mut metadata = self.get_data_by_id(cid.clone());
        metadata.price = price;
        metadata.update_state(state.clone());
        if state == State::Public {
            let new_index = self.total_published.len();
            self.total_published.insert(&(new_index as u32 +1), &metadata);
        }
        self.data_by_id.insert(&cid, &metadata);
        metadata
    }

    fn replace_metadata_by_key(&mut self, key_to_replace: u32, new_metadata: Metadata) {
        let total_access = &mut self.total_published;
        
        // Sử dụng phạm vi ngắn hơn để mượn self.total_access
        total_access.remove(&key_to_replace);
        total_access.insert(&key_to_replace, &new_metadata);
    }

    fn update_access_data(&mut self,account_id: AccountId, cid: ECID) {
        let mut data_list = self.access_by_user.get(&account_id).unwrap_or_else(|| vec![]);
        data_list.push(cid.clone());
        self.access_by_user.insert(&account_id, &data_list);
    }
	
    fn update_data_share_address(&mut self,account_id: AccountId, cid: ECID) {
        let mut metadata = self.get_data_by_id(cid.clone());
        metadata.push_shareaddress(account_id);
        self.data_by_id.remove(&cid);
        self.data_by_id.insert(&cid, &metadata);
    }

    fn update_total_publish(&mut self,account_id: AccountId) {
        let mut key_find = 0; 
        for (key, value) in self.total_published.iter() {
            if value.owner == account_id {
                key_find = key;
                break;
            }
        }

        if let Some(mut metadata) = self.total_published.get(&key_find) {
            metadata.push_shareaddress(account_id);
            self.replace_metadata_by_key(key_find, metadata);
        }

    }

    fn access_to_data(&mut self,  cid: ECID, user_id: AccountId, pub_key: String) {
        assert_ne!(user_id.clone(), env::signer_account_id(), "You are owner of this data");
        // let mut lk_map_data = self.access_by_data.get(&cid).unwrap();
        // lk_map_data.insert(&user_id, &pub_key);
        // self.access_by_data.insert(&cid, &lk_map_data);
        self.access_by_data.insert(&cid, &user_id);
        self.update_access_data(user_id.clone(), cid.clone());
        self.update_data_share_address(user_id.clone(), cid.clone());
        self.update_total_publish(user_id);
    }

    #[payable]
    fn purchase(&mut self, encrypted_cid: ECID, pub_key: String)-> Metadata {
        let data = self.data_by_id.get(&encrypted_cid).unwrap();
        let deposit = env::attached_deposit();
        let buyer_id = env::signer_account_id();
        assert_ne!(env::signer_account_id(), data.owner, "You are owner of this data!");
        assert_eq!(deposit, data.price, "Invalid deposit!");
        self.payment(buyer_id.clone(), data.owner, deposit);
        self.access_to_data(encrypted_cid.clone(), buyer_id, pub_key);
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
        assert!(self.data_by_id.contains_key(&data_id), "data is not exist!");
        let data = self.data_by_id.get(&data_id).clone().unwrap();
        data
        
    }

    //al data which have been published for market
    fn get_published_data(&self) -> Vec<Metadata> {
        let mut vec_data: Vec<Metadata> = vec![];
        let published_data = &self.total_published;
        for i in published_data {
            vec_data.push(i.1);
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
        let list_access = &self.access_by_data;
        let singer = env::signer_account_id();
        assert!(list_access.contains_key(&singer.to_string()), "You dont have permision to get this data!");
        DataValue {
            user_id: singer.to_string(),
            encrypted_cid:encrypted_cid
        }
    }

}
