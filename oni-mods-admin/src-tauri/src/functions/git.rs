use std::path::{PathBuf};
use anyhow::{Error as AnyError};
use git2::{Repository, StatusOptions};
use Result::Ok;
use serde::{Deserialize, Serialize};
use tauri::api::dialog::message;


pub const GIT_REPO_NOT_EXIST:u16 = 100;
#[derive(Debug, Serialize, Deserialize)]
pub struct StatuesItem {
    status_type: u8,
    old_path: String,
    new_path: String
}
pub fn is_repo_exist(path: PathBuf) -> Result<String, AnyError> {
    let _repo = Repository::open(path)?;
    Ok(String::from("仓库存在"))
}

pub fn get_statuses(path: PathBuf) -> Result<Vec<StatuesItem>, AnyError>{
    let repo = Repository::open(path)?;
    let mut opt = StatusOptions::new();
    opt.include_untracked(true);
    let statuses = repo.statuses(Some(&mut opt))?;
    let mut status_list:Vec<StatuesItem> =Vec::new() ;
    for entry in statuses.iter() {
        if entry.index_to_workdir().is_none() {
            continue;
        }
        let status_type:u8 = match entry.status() {
            s if s.is_wt_deleted() => 0,
            s if s.is_wt_modified() => 1,
            s if s.is_wt_new() => 2,
            s if s.is_wt_renamed() => 3,
            s if s.is_wt_typechange() => 4,
            _ => continue
        };
        let old_path = String::from(
            entry.index_to_workdir().unwrap().old_file().path().unwrap().to_str().unwrap());
        let new_path = String::from(
            entry.index_to_workdir().unwrap().new_file().path().unwrap().to_str().unwrap());
        let item = StatuesItem {
            status_type,
            old_path,
            new_path
        };
        status_list.push(item);
    }
    Ok(status_list)
}

pub fn add_to_commit(path: PathBuf) -> Result<(),AnyError>{
    let repo = Repository::open(path.clone())?;
    let mut index = repo.index()?;
    let states:Vec<StatuesItem> = get_statuses(path.clone())?;
    let mut need_add:Vec<String> = Vec::new();
    let mut need_update:Vec<String> = Vec::new();
    for item in states{
        match item {
            i if i.status_type == 2 => need_add.push(i.new_path),
            _ => need_update.push(item.new_path)
        }
    }
    index.add_all(need_add.iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.update_all(need_update.iter(),None)?;
    index.write()?;
    Ok(())
}

pub fn commit_change(repo_path: PathBuf, msg: &str) -> Result<(),AnyError>{
    let repo = Repository::open(repo_path.clone())?;
    let states:Vec<StatuesItem> = get_statuses(repo_path.clone())?;
    if !states.is_empty() {
        add_to_commit(repo_path.clone())?;
    }
    let sig = repo.signature()?;
    let tree_id = {
        let mut index = repo.index()?;
        index.write_tree()?
    };
    let tree = repo.find_tree(tree_id)?;
    repo.commit(Some("HEAD"),&sig,&sig,msg,&tree,&[])?;
    Ok(())
}

mod test{
    use super::*;
    #[test]
    fn test_repo(){
        let result = is_repo_exist(PathBuf::new().join("C:\\Users\\26216\\code\\CSharp\\ONI-Mods"));
        print!("{:?}", result);
        assert_eq!(result.is_ok(), true);
        let result = is_repo_exist(PathBuf::new().join("C:\\Users\\26216\\code\\CSharp"));
        print!("{:?}", result);
        assert_eq!(result.is_err(), true);
    }
    #[test]
    fn test_statues(){
        let result = get_statuses(PathBuf::new().join("C:\\Users\\26216\\code\\CSharp\\ONI-Mods"));
        print!("{:?}", result);
        assert_eq!(result.is_ok(), true);
    }
    #[test]
    fn test_add(){
        let result = add_to_commit(PathBuf::new().join("C:\\Users\\26216\\code\\Others\\test_for_rs_git"));
        print!("{:?}", result);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_commit(){
        let result = commit_change(PathBuf::new().join("C:\\Users\\26216\\code\\Others\\test_for_rs_git"),"test commit");
        print!("{:?}", result);
        assert_eq!(result.is_ok(), true);
    }
}