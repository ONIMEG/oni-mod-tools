use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::functions::{project};


const SUCCESS:u16 = 200;
const CONVERT_ERROR:u16 = 500;
const CREATE_PROJECT_ERROR:u16 = 501;
const GET_SOLUTION_LIST_ERROR:u16 = 502;
const GET_PROJECT_LIST_ERROR:u16 = 503;

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

pub fn get_project_list(json_solution_item:String) -> String{
  let solution_item = serde_json::from_str(&json_solution_item);
  if !solution_item.is_ok() {
    return ResultBody::convert(CONVERT_ERROR, solution_item.err().unwrap().to_string().as_str())
  }
  let result = project::get_csproj_list(solution_item.unwrap());
  if  !result.is_ok() {
    return ResultBody::convert(GET_PROJECT_LIST_ERROR, result.err().unwrap().to_string().as_str())
  }
  return ResultBody::convert(SUCCESS, result.unwrap().as_str());
}

#[cfg(test)]
mod tests {
  use std::fs;
  use std::path::PathBuf;
  use super::*;

  #[test]
  fn test_return_result_json(){
    let result = ResultBody::convert(SUCCESS,"success");
    println!("{}",result)
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
}
