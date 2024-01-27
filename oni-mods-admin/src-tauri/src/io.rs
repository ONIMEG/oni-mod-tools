use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::functions::{info, project};
use crate::functions::git::{FAIL_GET_STATUES, get_statuses};

const SUCCESS:u16 = 200;
const CONVERT_ERROR:u16 = 500;
const CREATE_PROJECT_ERROR:u16 = 501;
const GET_SOLUTION_LIST_ERROR:u16 = 502;
const GET_NEW_VERSION_ERROR:u16 = 504;
const READ_CONFIG_INFO_ERROR:u16 = 505;

#[derive(Debug,Serialize,Deserialize)]
pub struct ResultBody{
  code:u16,
  message: String
}

impl ResultBody {
  pub fn convert(code:u16,message:&str)->String{
    let result = ResultBody{
      code,
      message: String::from(message)
    };
    return serde_json::to_string(&result).unwrap();
  }
}

pub fn create_project(json_create_project_info:String) -> String{
  let create_project_info:Result<project::CreateProjectInfo, _> = serde_json::from_str(
    &json_create_project_info);

  if create_project_info.is_err(){
    return ResultBody::convert(CONVERT_ERROR,
                               create_project_info.err().unwrap().to_string().as_str());
  }

  let create_project_result = project::create_project(create_project_info.unwrap());
  if create_project_result.is_err(){
    return ResultBody::convert(CREATE_PROJECT_ERROR,
                               create_project_result.err().unwrap().to_string().as_str());
  }

  ResultBody::convert(SUCCESS,"ok")
}

pub fn get_solution_list() -> String{
  let path = Path::new("solutions.json");
  if  path.exists() {
    let result = fs::read_to_string(path);
    return if result.is_ok() {
      ResultBody::convert(SUCCESS, result.unwrap().as_str())
    } else {
      ResultBody::convert(GET_SOLUTION_LIST_ERROR, result.err().unwrap().to_string().as_str())
    }
  }
  return ResultBody::convert(SUCCESS,"[]");
}

pub fn add_new_project(json_solution_info:String, json_new_project_info:String) -> String{
  let solution_info:Result<project::CreateProjectInfo, _>= serde_json::from_str(&json_solution_info);
  let new_project_info:Result<project::Project, _> = serde_json::from_str(&json_new_project_info);
  if solution_info.is_err() || new_project_info.is_err() {
    return ResultBody::convert(CONVERT_ERROR, "转换失败");
  }
  let result = project::add_new_project(&solution_info.unwrap(), new_project_info.unwrap());
  if !result.is_ok() {
    return ResultBody::convert(CREATE_PROJECT_ERROR, result.err().unwrap().to_string().as_str());
  }
  return ResultBody::convert(SUCCESS,"ok");
}

pub fn refresh_version() -> String{
  let result = info::refresh_version();
  if !result.is_ok() {
    return ResultBody::convert(GET_NEW_VERSION_ERROR, result.err().unwrap().to_string().as_str());
  }
  return ResultBody::convert(SUCCESS, result.unwrap().to_string().as_str());
}

pub fn get_config_info() -> String{
  let result = info::load_config_file();
  if !result.is_ok() {
    return ResultBody::convert(READ_CONFIG_INFO_ERROR, result.err().unwrap().to_string().as_str());
  }
  return ResultBody::convert(SUCCESS, serde_json::to_string(&result.unwrap()).unwrap().as_str());
}

pub fn update_config_info(new_config_info: String) -> String{
  let config_info:Result<info::ConfigInfo,_> = serde_json::from_str(new_config_info.as_str());
  if config_info.is_err(){
    return ResultBody::convert(CONVERT_ERROR, "转换失败")
  }
  let result = info::save_config_file(&config_info.unwrap());
  if result.is_err() {
    return ResultBody::convert(READ_CONFIG_INFO_ERROR, result.err().unwrap().to_string().as_str());
  }
  return ResultBody::convert(SUCCESS, "保存配置成功");
}

pub fn git_statues(path: String) -> String {
  let repo_path = PathBuf::new().join(path);
  let result = get_statuses(repo_path);
  if !result.is_ok(){
    return ResultBody::convert(FAIL_GET_STATUES, result.err().unwrap().to_string().as_str())
  }
  return ResultBody::convert(SUCCESS, serde_json::to_string(&result.unwrap()).unwrap().as_str())
}

#[cfg(test)]
mod tests {
  use std::fs;
  use std::path::PathBuf;
  use super::*;

  #[test]
  fn test_return_result_json(){
    let result = ResultBody::convert(SUCCESS,"success");
    assert_eq!("{\"code\":200,\"message\":\"success\"}", result);
  }

  #[test]
  fn test_create_project(){
    let mut path = PathBuf::new();
    path = path.join("target/ioTestSolution");
    if path.exists() {
      fs::remove_dir_all(path).expect("文件夹移除失败？？？");
    }
    let proj_info_json = r#"{"root":"target","project_name":"ioTest","solution_name":"ioTestSolution"}"#;
    let result = create_project(proj_info_json.to_string());
    assert_eq!(result,ResultBody::convert(SUCCESS,"ok"));
  }
  #[test]
  fn test_get_latest_version(){
    let result = refresh_version();
    print!("{:?}",result);
  }
  #[test]
  fn test_get_config_info(){
    let result = get_config_info();
    print!("{:?}",result);
  }
}
