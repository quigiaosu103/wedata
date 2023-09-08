
pub mod event;
use std::vec;
use near_sdk::{near_bindgen, collections::{UnorderedMap, LookupMap}, AccountId, Balance, borsh::BorshSerialize, env::{self}, PanicOnDefault, Promise, serde::{Deserialize, Serialize}, ext_contract, json_types::U128};
use near_sdk::borsh::{self, BorshDeserialize};
use event::*;
use std::convert::{TryFrom, TryInto};

pub type ECID = String;
pub type DecryptedKey = String;
pub const ONE_NEAR: u128 = 1_000_000_000_000_000_000_000_000; 

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
    pub key_cid: String,
    pub pub_key: String
}


#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Metadata {
    pub state: State,
    pub title: String,
    pub tags: String,
    pub owner: AccountId,
    pub cid_encrypted: ECID,
    pub price: Balance,
    pub size: String,
    pub list_access: Vec<AccountId>,
    pub is_active: bool,
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
    access_by_data: LookupMap<ECID, DataValue>, //the account_id in a dataset
    waiting_list: LookupMap<AccountId, Vec<(AccountId, ECID, String)>>,
    data_unconfirmed: LookupMap<ECID, AccountId>,
    ft_per_account: LookupMap<AccountId, Balance>
}


pub trait Function {
    fn new()-> Self;
    fn get_data_by_id(&self, data_id: ECID) -> Metadata;
    fn new_meta_data(&mut self, title_given: String, tags_given: String, cid_encrypted_given: ECID, size: String) -> Metadata;
    //chuyển trạng thái của data private-> public  và set price
    fn set_state(&mut self, state: State, cid: ECID, price: Balance) -> Metadata;
    fn access_to_data(&mut self, cid: ECID, user_id: AccountId, access_key: String, pub_key: String);
    fn purchase(&mut self, encrypted_cid: ECID, access_key: String, contract_id: AccountId) -> Metadata;
    fn payment(&self,sender: AccountId, receiver: AccountId, deposit: Balance);
    fn is_accessed(&self, encrypted_id: ECID, user_id: AccountId) -> bool;
    //lấy những data đã được public (xuất hiện trên marketplace)
    fn get_published_data(&self, user: AccountId) -> Vec<Metadata> ;
    //lấy những data mà user đã mua
    fn get_accessed_data_by_user(&self, user_id: AccountId) -> Vec<Metadata>;
    //Những data của người upload lên
    fn get_data_by_owner(&self, owner_account_id: AccountId)-> Vec<Metadata>;
    //những user đã mua data đó
    fn get_user_by_data(&self, encrypted_cid: ECID) -> Vec<AccountId>;
    //xét điều kiện và trả về accountId của người được quyền access và key
    fn get_data_value(&self, encrypted_cid: ECID, user_id: AccountId) -> DataValue;
    fn update_access_data(&mut self,account_id: AccountId, cid: ECID);
    fn update_data_share_address(&mut self,account_id: AccountId, cid: ECID) ;
    fn update_total_publish(&mut self,account_id: AccountId);
    fn replace_metadata_by_key(&mut self, key_to_replace: u32, new_metadata: Metadata);
    fn store_waiting_list(&mut self, buyer: AccountId, data: Metadata, pub_key: String);
    fn confirm_transaction(&mut self, buyer_id: AccountId, encrypted_cid: ECID, is_access: bool, access_key: String, contract_id: AccountId, pub_key: String) -> Metadata;
    fn get_waiting_list(&self, owner: AccountId) -> Vec<(AccountId, ECID, String)>;
    fn cross_call(&self, contract_id: AccountId, sender: AccountId, receiver: AccountId, amount: Balance);
    fn get_balance_of(&self, account_id: AccountId) -> i128;
    fn set_balance_of(&mut self, account_id: AccountId, new_balance: Balance);
    fn update_balance(&mut self, sender_id: &AccountId, receiver_id: &AccountId, amount: U128);
    fn register(&mut self, account_id: AccountId);
}

#[ext_contract(ext_ft_contract)]
trait ExtFtContract {
    fn ft_transfer(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId, 
        amount: U128, 
        memo: Option<String>
    );
    fn storage_deposit(account_id: AccountId);
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
            waiting_list: LookupMap::new(b"waiting list".try_to_vec().unwrap()),
            data_unconfirmed: LookupMap::new(b"data unconfirmed".try_to_vec().unwrap()),
            ft_per_account: LookupMap::new(b"ft_per_account".try_to_vec().unwrap()),
        }
    }
    fn register(&mut self, account_id: AccountId) {
        self.ft_per_account.insert(&account_id, &(0 as u128));
        let contract_id: AccountId = "harvardtp_ft.testnet".parse().unwrap();
        ext_ft_contract::ext(contract_id)
        // Attach 1 yoctoNEAR with static GAS equal to the GAS for nft transfer. Also attach an unused GAS weight of 1 by default.
        .with_attached_deposit(10_000_000_000_000_000_000_000)
        .storage_deposit(
            account_id
        );
    }

    fn new_meta_data(&mut self, title_given: String, tags_given: String, cid_encrypted_given: ECID, size: String) -> Metadata {
        let owner = env::signer_account_id();
        let meta_data = Metadata {
            state: State::Private,
            title: title_given.clone(),
            tags: tags_given.clone(),
            owner: owner.clone(),
            cid_encrypted: cid_encrypted_given.clone(),
            price: 0,
            list_access: vec![],
            size,
            is_active: false,
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
        metadata.push_shareaddress(account_id.clone());
        self.data_by_id.insert(&cid, &metadata);
        let data_list = &self.total_published;
        let mut index:u32 = 0;
        for i in 0..data_list.len() {
            let data = data_list.get(&(i as u32)).unwrap_or(Metadata {
                state: State::Private,
                title: "".to_string(),
                tags: "tags_given".to_string(),
                owner: account_id.clone(),
                cid_encrypted: "cid_encrypted_given".to_string(),
                price: 0,
                list_access: vec![],
                size: "1".to_string(),
                is_active: false,
            });
            if data.cid_encrypted == cid {
                index = i as u32;
                break;
            }
        }
        self.total_published.insert(&index, &metadata);

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

    fn access_to_data(&mut self,  cid: ECID, user_id: AccountId, access_key: String, pub_key: String) {
        self.update_data_share_address(user_id.clone(), cid.clone());
        self.access_by_data.insert(&cid, &DataValue { user_id:user_id.to_string(), key_cid: access_key, pub_key });
        self.update_access_data(user_id.clone(), cid.clone());
        self.update_total_publish(user_id);
    }

    fn store_waiting_list(&mut self, buyer: AccountId, data: Metadata, pub_key: String) {
        let id = data.cid_encrypted;
        let owner = data.owner;
        let mut vec_waiting =  self.waiting_list.get(&owner).unwrap_or_else(||Vec::new());
        vec_waiting.push((buyer, id, pub_key));
        self.waiting_list.insert(&owner, &vec_waiting);
    }

    #[payable]
    fn purchase(&mut self, encrypted_cid: ECID, pub_key: String, contract_id: AccountId)-> Metadata {
        let data = self.data_by_id.get(&encrypted_cid).unwrap();
        let buyer_id = env::signer_account_id();
        let buyer_balance = self.get_balance_of(buyer_id.clone());
        assert!(data.price <= buyer_balance as u128, "You dont have enough coin!");
        assert_ne!(buyer_id, data.owner, "You are owner of this data!");
        self.cross_call(contract_id.clone(), buyer_id.clone(), contract_id, data.price * ONE_NEAR);
        self.store_waiting_list(buyer_id.clone(), data, pub_key.clone());
        self.data_unconfirmed.insert(&encrypted_cid, &buyer_id);
        self.data_by_id.get(&encrypted_cid).unwrap()
    }

    fn cross_call(&self, contract_id: AccountId, sender: AccountId, receiver: AccountId, amount: Balance) {
        ext_ft_contract::ext(contract_id)
        // Attach 1 yoctoNEAR with static GAS equal to the GAS for nft transfer. Also attach an unused GAS weight of 1 by default.
        .with_attached_deposit(1)
        .ft_transfer(
            sender,
            receiver, //seller to transfer the FTs to
            U128(amount), //amount to transfer
            Some("Sale from marketplace".to_string()), //memo (to include some context)
        );    
    }

    fn confirm_transaction(&mut self, buyer_id: AccountId, encrypted_cid: ECID, is_access: bool, access_key: String, contract_id: AccountId, pub_key: String) -> Metadata {
        let data = self.get_data_by_id(encrypted_cid.clone());
        let owner = data.owner;
        
        if is_access {
            
            // self.payment(buyer_id.clone(), owner.clone(), data.price);
            self.cross_call(contract_id.clone(), contract_id, owner.clone(), data.price * ONE_NEAR);
            self.access_to_data(encrypted_cid.clone(), buyer_id.clone(), access_key, pub_key);
            self.data_unconfirmed.remove(&encrypted_cid);

        } else {
            self.cross_call(contract_id.clone(), contract_id, buyer_id.clone(), data.price * ONE_NEAR);
            //  self.payment(owner.clone(), buyer_id.clone(), data.price);
        }

        let mut w_list = self.waiting_list.get(&owner.clone()).unwrap();
        let mut index = 0;
        for i in w_list.clone() {
            if i.0 == buyer_id && i.1 == encrypted_cid {
                break;
            }
            index+=1;
        }
        w_list.remove(index);
        self.waiting_list.insert(&owner, &w_list);
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


    fn set_balance_of(&mut self, account_id: AccountId, new_balance: Balance) {
        self.ft_per_account.insert(&account_id, &new_balance);
    }

    //=====================GET FUNCTION=============================

    fn get_balance_of(&self, account_id: AccountId) -> i128 {
        if self.ft_per_account.contains_key(&account_id) {
            return self.ft_per_account.get(&account_id).unwrap() as i128;
        }
        -1
    }

    fn get_data_by_id(&self, data_id: ECID) -> Metadata {
        assert!(self.data_by_id.contains_key(&data_id), "data is not exist!");
        let data = self.data_by_id.get(&data_id).clone().unwrap();
        data
        
    }

    //al data which have been published for market
    fn get_published_data(&self, user: AccountId) -> Vec<Metadata> {
        let mut vec_data: Vec<Metadata> = vec![];
        let published_data = &self.total_published;
        for i in published_data {
            let mut data = i.1;
            if self.data_unconfirmed.contains_key(&data.cid_encrypted) {
                if self.data_unconfirmed.get(&data.cid_encrypted).unwrap() == user {
                    data.is_active = true;
                }
            }
            vec_data.push(data);
        }
        vec_data
    }
    
    //all data which are bought by a user
    fn get_accessed_data_by_user(&self, user_id: AccountId) -> Vec<Metadata> {
        let mut vec_data = vec![];
        let vec_data_id: Vec<ECID> = self.access_by_user.get(&user_id).unwrap_or_else(|| vec![]);
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

    fn get_data_value(&self, encrypted_cid: ECID, user_id: AccountId) -> DataValue {
        let list_access = &self.access_by_data;
        assert_eq!(list_access.get(&encrypted_cid).unwrap().user_id, user_id.to_string(), "You dont have permision to get this data!");
        list_access.get(&encrypted_cid).unwrap()
    }

    fn get_waiting_list(&self, owner: AccountId) -> Vec<(AccountId, ECID, String)> {
        let mut vec_result: Vec<(AccountId, ECID, String)> = vec![];
        let vec_waiting = self.waiting_list.get(&owner).unwrap_or_else(|| Vec::new()); 
        for i in vec_waiting {
            vec_result.push(i);
        }
        vec_result
    }

    fn update_balance(&mut self, sender_id: &AccountId, receiver_id: &AccountId, amount: U128) {
        if self.ft_per_account.contains_key(sender_id) {
            self.ft_per_account.insert(sender_id, &(self.ft_per_account.get(sender_id).unwrap_or(0) - amount.0));
        }

        if self.ft_per_account.contains_key(receiver_id) {
            self.ft_per_account.insert(receiver_id, &(self.ft_per_account.get(sender_id).unwrap_or(0) + amount.0));
        } else {
            self.ft_per_account.insert(receiver_id, &amount.0);
        }

        let payment_info = EventLog { //info of transaction
            standard: "e-comerce-1.0.0".to_string(),
            event: EventLogVariant::Purchase(vec![PurchaseProduct {
                receiver: receiver_id.to_string(),
                sender: sender_id.to_string(),
                amount: amount.0,
                memo: None,
            }])
        };
        //add new checker into checkers of this campaign
        env::log_str(&payment_info.to_string());   
    }
    


}
