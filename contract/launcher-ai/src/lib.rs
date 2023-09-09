
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

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ProjectPool {
  pub id: String,
  pub title: String,
  pub description: String,
  pub pool: Balance,
  pub staus: Status, // description
  pub owner: AccountId,
  pub contributors: Vec<Contributor>
}


// Define the contract structure
#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    contract_name: AccountId,
    total_projects: u64,
    projects: UnorderedMap<u64, ProjectPool>,
    project_by_id: LookupMap<String, ProjectPool>,
    projects_by_user: LookupMap<AccountId, Vec<String>>
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
    }
  }

  //tạo project
  pub fn new_project(&mut self, id: String, title: String, description: String, pool: Balance, vec_roles: Vec<Contributor>) -> ProjectPool {
    let owner = env::signer_account_id();
    let project = ProjectPool {
      id: id.clone(),
      title,
      description,
      pool,
      staus: Status::Init,
      owner: owner.clone(),
      contributors: vec_roles,
    };
    assert!(self.project_by_id.contains_key(&id), "project id is already exist!");
    self.project_by_id.insert(&id, &project);
    self.projects.insert(&self.projects.len(), &project);
    if self.projects_by_user.contains_key(&owner) {
      let mut vec = self.projects_by_user.get(&owner).unwrap();
      vec.push(id.clone());
      self.projects_by_user.insert(&owner, &vec);

    }else {
      let mut vec: Vec<String> = vec![];
      vec.push(id);
      self.projects_by_user.insert(&owner, &vec);
    }
    project
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


  
  pub fn contribute(&mut self, project_id: String, user_id: String, role: String, permision: Permision) -> ProjectPool {
    assert!(self.project_by_id.contains_key(&project_id), "Project id is not valid");
    let mut project = self.project_by_id.get(&project_id).unwrap();
    let mut index =0;
    for i in project.clone().contributors {
        if i.role == role {
          break;
        }
        index+=1;
    }
    project.contributors.insert(index, Contributor {
      account_id: user_id,
      role,
      permision,
      description:"".to_string()
    });
    self.update_projects(project.clone());
    project
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

