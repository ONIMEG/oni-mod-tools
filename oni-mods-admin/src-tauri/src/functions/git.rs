use std::path::{Path, PathBuf};
use anyhow::{Error as AnyError};
use git2::{Repository, Status, StatusOptions};
use Result::Ok;
use serde::{Deserialize, Serialize};


pub const GIT_REPO_NOT_EXIST:u16 = 100;
#[derive(Debug, Serialize, Deserialize)]
pub struct StatuesItem {
    status_type: u8,
    old_path: String,
    new_path: String
}
pub fn is_repo_exist(path: PathBuf) -> Result<String, AnyError> {
    let repo = Repository::open(path);
    return if repo.is_err() {
        Err(AnyError::msg("仓库不存在"))
    } else {
        Ok(String::from("仓库存在"))
    }
}

pub fn get_statuses(path: PathBuf) -> Result<Vec<StatuesItem>, AnyError>{
    let repo = Repository::open(path);
    if repo.is_err() {
        return Err(AnyError::msg("仓库不存在"))
    }
    let repo = repo.unwrap();
    let mut opt = StatusOptions::new();
    opt.include_untracked(true);
    let statuses = repo.statuses(Some(&mut opt));
    if statuses.is_err() {
        return Err(AnyError::msg("读取状态失败"))
    }
    let statuses = statuses.unwrap();
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
}