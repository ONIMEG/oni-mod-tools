use std::path::PathBuf;
use anyhow::{Error as AnyError};
use git2::Repository;
use Result::Ok;

pub const GIT_REPO_NOT_EXIST:u16 = 100;
pub fn is_repo_exist(path: PathBuf) -> Result<String, AnyError> {
    let repo = Repository::open(path);
    return if repo.is_err() {
        Err(AnyError::msg("仓库不存在"))
    } else {
        Ok(String::from("仓库存在"))
    }
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
}