
//new project
//contribute
//set state

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env,  near_bindgen, AccountId, Balance, PanicOnDefault};

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum Status {
  Init,
  Inprogress,
  Testing,
  Beta
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone,PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Permision {
  FullAccess,
  ReadData,
  OnlyView
}


#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone,PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Contributor {
  pub account_id: String,
  pub role: String,
  pub permision: Permision, 
  pub description: String,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone,PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct RequestContributeInfo {
  pub account_id: AccountId,
  pub role: String,
  pub project_title: String,
  pub project_id: String
}



#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ProjectPool {
  pub id: String,
  pub title: String,
  pub description: String,
  pub pool: Balance,
  pub staus: Status, // description
  pub owner: AccountId,
  pub contributors: Vec<Contributor>,
  pub urls: Vec<String>
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FeedBack {
  account_id: AccountId,
  content: String
}

// Define the contract structure
#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    contract_name: AccountId,
    total_projects: u64,
    projects: UnorderedMap<u64, ProjectPool>,
    project_by_id: LookupMap<String, ProjectPool>,
    projects_by_user: LookupMap<AccountId, Vec<String>>,
    feedbacks_by_project: LookupMap<String, Vec<FeedBack>>,
    request_by_owner: LookupMap<AccountId, Vec<RequestContributeInfo>>
}



#[near_bindgen]
impl Contract {
  #[init]
  pub fn new() -> Self {
    Self {
      contract_name: env::signer_account_id(),
      total_projects: 0,
      projects: UnorderedMap::new(b"projects".try_to_vec().unwrap()),
      project_by_id: LookupMap::new(b"project by id".try_to_vec().unwrap()),
      projects_by_user: LookupMap::new(b"projects by user".try_to_vec().unwrap()),
      feedbacks_by_project: LookupMap::new(b"feedbacks by project".try_to_vec().unwrap()),
      request_by_owner: LookupMap::new(b" by project".try_to_vec().unwrap()),
    }
  }

  //tạo project
  pub fn new_project(&mut self, id: String, title: String, description: String, pool: Balance, vec_roles: Vec<Contributor>, urls: Vec<String>) -> ProjectPool {
    let owner = env::signer_account_id();
    let project = ProjectPool {
      id: id.clone(),
      title,
      description,
      pool,
      staus: Status::Init,
      owner: owner.clone(),
      contributors: vec_roles,
      urls
    };
    assert!(!self.project_by_id.contains_key(&id), "project id is already exist!");
    self.project_by_id.insert(&id, &project);
    self.projects.insert(&self.projects.len(), &project);
    if self.projects_by_user.contains_key(&owner) {
      let mut vec = self.projects_by_user.get(&owner).unwrap();
      vec.push(id.clone());
      self.projects_by_user.insert(&owner, &vec);

    }else {
      let mut vec: Vec<String> = vec![];
      vec.push(id.clone());
      self.projects_by_user.insert(&owner, &vec);
    }
    let vec: Vec<FeedBack> = vec![];
    self.feedbacks_by_project.insert(&id, &vec);
    project
  }

  pub fn new_feedback(&mut self, project_id: String, account_id: AccountId, content: String) -> FeedBack {
    let feedback = FeedBack {
      account_id,
      content
    };
    let mut feedbacks = self.feedbacks_by_project.get(&project_id).unwrap();
    feedbacks.push(feedback.clone());
    self.feedbacks_by_project.insert(&project_id, &feedbacks);  
    feedback  
  }

  pub fn get_feedbacks_by_project(&self, project_id: String) -> Vec<FeedBack> {
    let vec_fbs = self.feedbacks_by_project.get(&project_id).unwrap();
    vec_fbs
  }


  pub fn get_project_by_id(&self, id: String) -> ProjectPool{
    assert!(self.project_by_id.contains_key(&id), "id is invalid");
    let prj = self.project_by_id.get(&id).unwrap();  
    prj
  }


  pub fn get_request_by_owner(&self, account_id: AccountId) -> Vec<RequestContributeInfo> {
    let mut vec: Vec<RequestContributeInfo> = vec![];
    let list_request = self.request_by_owner.get(&account_id).unwrap_or_else(|| vec![]);
    for i in list_request {
      vec.push(i);
    } 
    vec
  }

  pub fn get_feedbacks(&self, prj_id: String) -> Vec<FeedBack>  {
    let mut vec: Vec<FeedBack> = vec![];
    let list_feedbacks = self.feedbacks_by_project.get(&prj_id).unwrap();
    for i in list_feedbacks {
      vec.push(i);
    }
    vec
  }

  //thay đổi trạng thái proj
  pub fn set_status(&mut self, project_id: String, status: Status) -> ProjectPool {
    assert!(self.project_by_id.contains_key(&project_id), "project id is invalid!");
    let mut project = self.project_by_id.get(&project_id).unwrap();
    assert_eq!(project.owner, env::signer_account_id(), "You dont have permision to change status!");
    project.staus = status;
    self.update_projects(project.clone());
    project
  }


  pub fn update_projects(&mut self, project: ProjectPool) {
    self.project_by_id.insert(&project.id, &project);
    for i in 0..self.projects.len() {
      if let Some(prj) = self.projects.get(&i) {
        if prj.id == project.id {
          self.projects.insert(&i, &project);
          break;
        }
      }
    }
  }



  pub fn accept_contribute(&mut self, project_id: String, user_id: String, role: String) -> ProjectPool {
    assert!(self.project_by_id.contains_key(&project_id), "Project id is not valid");
    let mut project = self.project_by_id.get(&project_id).unwrap();
    let mut index =0;
    let contributors = project.clone().contributors;
    let mut new_ctr = Contributor {
      account_id: "".to_string(),
      role: "".to_string(),
      permision: Permision::FullAccess,
      description:"".to_string()
    };
    for i in  contributors{
        if i.role == role {
          new_ctr=i;
          break;
        }
        index+=1;
    }
    if index>0 {
      index-=1;
    }
    project.contributors.remove(index as usize);
    project.contributors.push(Contributor { account_id: user_id, role: new_ctr.role, permision: new_ctr.permision, description: new_ctr.description });
    self.update_projects(project.clone());
    project
  }

  pub fn request_contribute(&mut self, prj_id: String, role: String) {
    let prj = self.project_by_id.get(&prj_id).unwrap();
    if self.request_by_owner.contains_key(&prj.owner) {
      let mut vec = self.request_by_owner.get(&prj.owner).unwrap();
      vec.push(RequestContributeInfo { account_id: env::signer_account_id(), role, project_id: prj_id.clone(), project_title: prj.title});
      self.request_by_owner.insert(&prj.owner, &vec);
    }else {
      let mut vec : Vec<RequestContributeInfo> = vec![];
      vec.push(RequestContributeInfo { account_id: env::signer_account_id(), role, project_id: prj_id, project_title: prj.title});
      self.request_by_owner.insert(&prj.owner, &vec);
    }
  }

  pub fn excute_request(&mut self, prj_id: String, user_id:AccountId, role: String, is_accept: bool) {
    let project = self.project_by_id.get(&prj_id).unwrap();
    assert_eq!(project.owner, env::signer_account_id(), "you do not have permission to excute request!");
    if is_accept {
      self.accept_contribute(prj_id, user_id.to_string(), role.clone());
    }
    let mut vec = self.request_by_owner.get(&project.owner).unwrap();
    let mut index = 0;
      for i in vec.clone() {
        if i.account_id == user_id  && role == i.role {
          break;
        }
        index+=1;
      }
      if index >0 {
        index-=1;
      }
      vec.remove(index);
      self.request_by_owner.insert(&project.owner, &vec);
  }

  

  //=========================================get functions===================================
  pub fn get_all_projects(&self) -> Vec<ProjectPool> {
    let projects = &self.projects;
    let mut vec_projects: Vec<ProjectPool> = vec![];
    for i in 0..projects.len() {
      if let Some(prj) = projects.get(&i) {
        vec_projects.push(prj);
      }
    }
    vec_projects
  }

  pub fn get_project_by_user(&self, user_id: AccountId) -> Vec<ProjectPool> {
    let mut vec_prjs: Vec<ProjectPool> = vec![];
    let project_ids = self.projects_by_user.get(&user_id).unwrap();
    for i in project_ids {
      let prj = self.project_by_id.get(&i).unwrap();
      vec_prjs.push(prj);
    }
    vec_prjs
  }

  pub fn set_role(&self, project_id: String, account_id: String, role: Option<String>, permision: Option<Permision>) ->  ProjectPool {
    assert!(self.project_by_id.contains_key(&project_id), "project id is not valid!");
    let project = self.project_by_id.get(&project_id).unwrap();
    let mut contributors_list = project.clone().contributors;
    let mut index: i32 = -1;
    let mut contributor : Option<Contributor> = None;
    for i in contributors_list.clone() {
      index +=1;
      if i.account_id == account_id {
        contributor = Some(i);
        break
      }
    }
    assert_eq!(contributor, None, "Contributor is not exist!");
    assert!(index == -1, "account id is not exist!");
    let mut contributor = contributor.unwrap();
    if role != None {
        contributor.role = role.unwrap();
    }
    if permision!= None {
      contributor.permision = permision.unwrap();
    }
    contributors_list.insert(index as usize, contributor);
    project
  }


  pub fn test(&self, vec: Vec<Contributor>) -> Vec<Contributor> {
    let vec_r = vec;
    return vec_r;
    
  }

}

