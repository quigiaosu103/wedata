use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::{near_bindgen, AccountId, env, Balance, PanicOnDefault};
mod user;
mod data_object;
use user::{User, UserMetaData};
use data_object::{DataKey, DataObject, AccessType};

#[near_bindgen]
#[derive(PanicOnDefault,BorshDeserialize, BorshSerialize)]
pub struct Contract {
    platform_name: AccountId,
    user_by_id: LookupMap<AccountId, User>,
    data_objects: UnorderedMap<u32, DataObject>,
    data_by_id: LookupMap<DataKey, DataObject>,
    contributor_by_data: LookupMap<DataKey, Vec<User>>,
    total_users: u32,
    total_data_object: u32,
}

pub trait Function {
    fn new() -> Self;
    fn get_signer_account(&mut self)-> User;
    fn check_new_user(&self, id: AccountId)-> bool;
    fn new_user(&mut self) ->User;
    fn get_user_by_id(&self, id: AccountId) -> User;
    fn new_data_object(&mut self, title: String,description: String, data_key: DataKey, owner: AccountId, price: Balance, access_type: AccessType) -> DataObject;
}
#[near_bindgen]
impl Function for Contract {
    #[init]
    fn new() -> Self {
        Self {
            platform_name: env::signer_account_id(),
            user_by_id: LookupMap::new(b"user by id".try_to_vec().unwrap()),
            data_objects: UnorderedMap::new(b"data objects".try_to_vec().unwrap()),
            data_by_id: LookupMap::new(b"data by id".try_to_vec().unwrap()),
            contributor_by_data: LookupMap::new(b"contributor".try_to_vec().unwrap()),
            total_users: 0,
            total_data_object: 0
        }
    }

    fn get_signer_account(&mut self)-> User {  //load account 
        let id: AccountId = env::signer_account_id();
        assert!(self.user_by_id.contains_key(&id));
        let mut user = self.user_by_id.get(&id).unwrap();
        let balance = user.balance/u32::pow(10,24) as u128;
        user.balance = balance;
        user
    }


    fn check_new_user(&self, id: AccountId)-> bool {
        self.user_by_id.contains_key(&id)
    }

    fn new_user(&mut self) ->User{ 
        let id = env::signer_account_id();
        let user = User {
            id: id.clone(),
            balance: env::account_balance(),
            meta_data: UserMetaData {
                name: "".to_string(),
                email_address: "".to_string(),
                git: "".to_string(),
                age: 0,
                location: "".to_string()
            }, 
        };
        let total_users = self.total_users +1;
        self.total_users = total_users;
        self.user_by_id.insert(&id, &user);
        user
    }

    fn get_user_by_id(&self, id: AccountId) -> User {
        let user = self.user_by_id.get(&id).unwrap();
        user.clone()
    }


    //======================DATA===============
    fn new_data_object(&mut self, title: String,description: String, data_key: DataKey, owner: AccountId, price: Balance, access_type: AccessType) -> DataObject {
        assert!(!self.data_by_id.contains_key(&data_key), "Key of data is already exist!");
        let new_data_obj = DataObject{
            data_key: data_key.clone(),
            title,
            description,
            owner,
            price,
            access_type,
        };
        self.data_objects.insert(&self.total_data_object, &new_data_obj);
        self.data_by_id.insert(&data_key, &new_data_obj);
        new_data_obj
    }

}

