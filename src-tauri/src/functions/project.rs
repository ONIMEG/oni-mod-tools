use std::{
  fs::{self, File, OpenOptions, read_to_string},
  io::{BufReader, BufWriter},
  path::PathBuf,
  iter
};
use std::io::{Write};
use std::path::Path;
use anyhow::{Result, Error as AnyError, anyhow};
use xml::reader::{EventReader, XmlEvent};
use serde::{Deserialize, Serialize};
use zip::ZipArchive;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProjectInfo {
  pub root: PathBuf,
  pub project_name: String,
  pub solution_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectItem {
  pub path: PathBuf,
  pub name: String,
  pub prop: Project
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavedSolutionItem {
  pub path: PathBuf,
  pub name: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Project {
  #[serde(rename = "PropertyGroup")]
  pub property_group: PropertyGroup
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PropertyGroup {
  #[serde(rename = "AssemblyTitle")]
  pub assembly_title: String,
  #[serde(rename = "FileVersion")]
  pub file_version: String,
  #[serde(rename = "RootNamespace")]
  pub root_namespace: String,
  #[serde(rename = "Description")]
  pub description: String,
  #[serde(rename = "AssemblyVersion")]
  pub assembly_version: String,
  #[serde(rename = "LastWorkingBuild")]
  pub last_working_build: u32,
  #[serde(rename = "Platforms")]
  pub platforms: String,
}

impl Project {
  pub fn new(csproj_name: String) -> Project {
    Project{
      property_group: PropertyGroup{
        assembly_title: csproj_name.clone(),
        file_version: String::from("1.0.0"),
        root_namespace: csproj_name.clone(),
        description: String::from("缺氧模组"),
        assembly_version: String::from("1.0.0"),
        last_working_build: 526233,
        platforms: String::from("Vanilla;Mergedown"),
      }
    }
  }
}

impl ProjectItem {
  pub fn new(path: PathBuf, name: String, prop: Project) -> ProjectItem {
    ProjectItem{
      path,
      name,
      prop
    }
  }
}

impl SavedSolutionItem {
  pub fn new(path: PathBuf, name: String) -> SavedSolutionItem {
    SavedSolutionItem{
      path,
      name
    }
  }
}

const CS_GUID: &str = "9A19103F-16F7-4668-BE54-9A1E7A4F7556";
const PROJECT_ITEM: &str = r#"Project("{$[a]}") = "$[b]", "$[b]\$[b].csproj", "{$[c]}"
EndProject
"#;

fn save_solution_item(info: &CreateProjectInfo) -> Result<(), AnyError> {
  let path = Path::new("solutions.json");
  let mut saved_solutions:Vec<SavedSolutionItem> = vec![];
  if path.exists() {
    saved_solutions = serde_json::from_str::<Vec<SavedSolutionItem>>(&read_to_string(path)?)?;
  }
  saved_solutions.push(SavedSolutionItem::new(info.root.clone(), info.solution_name.clone()));
  let content = serde_json::to_string(&saved_solutions)?;
  fs::write(path, content)?;
  Ok(())
}

pub fn create_project(info: CreateProjectInfo) -> Result<(), AnyError> {
  save_solution_item(&info)?;
  let solution_path = info.root.join(&info.solution_name);
  if solution_path.exists() {
    return Err(anyhow!("项目文件夹已存在"));
  }
  // 创建存放项目的文件夹
  fs::create_dir(&solution_path)?;
  // 将压缩包解压
  let file = File::open("./resources/build.zip")?;
  let mut archive = ZipArchive::new(BufReader::new(file))?;
  for i in 0..archive.len() {
    let mut file = archive.by_index(i)?;
    let out_path = solution_path.join(file.mangled_name());
    if (&*file.name()).ends_with('/') {
      fs::create_dir_all(&out_path)?;
    } else {
      if let Some(p) = out_path.parent() {
        fs::create_dir_all(p)?;
      }
      let mut out_file = BufWriter::new(File::create(&out_path)?);
      std::io::copy(&mut file, &mut out_file)?;
    }
  }
  // 重命名 solution
  let target_sln = solution_path.join(format!("{}.sln",&info.solution_name));
  fs::rename(solution_path.join("solution"), &target_sln)?;
  // 修改 solution guid
  let mut content = read_to_string(&target_sln)?;
  content = content.replace("$[guid]", &*guid());
  fs::write(&(&target_sln), content)?;
  // 创建 csproj 项目
  let csproj_path = solution_path.join(&info.project_name);
  fs::create_dir(&csproj_path)?;
  create_csproj(
    csproj_path.join(format!("{}.csproj", info.project_name)),
    Project::new(info.project_name.clone()))?;
  add_csproj_to_sln(target_sln, &info.project_name)?;
  Ok(())
}

fn guid() ->String{
  return  Uuid::new_v4().as_hyphenated().to_string().to_uppercase().to_string()
}

pub fn create_csproj(target_path: PathBuf, new_info:Project)->Result<(), AnyError>{
  let mut new_csproj_xml = serde_xml_rs::to_string(&new_info)?;
  new_csproj_xml = format_xml(new_csproj_xml)?;
  new_csproj_xml = new_csproj_xml.replace(
    "<Project>",
    "<Project Sdk=\"Microsoft.NET.Sdk\">");
  fs::write(&target_path, new_csproj_xml)?;
  let parent = target_path.parent().unwrap().parent().expect("没找到根目录");
  let admin = parent.join(".admin");
  let mut csproj_s:Vec<ProjectItem> = vec![];
  if !admin.exists() {
    csproj_s.push(ProjectItem::new(target_path,new_info.property_group.assembly_title.clone(),new_info.clone()));
    let content = serde_json::to_string(&csproj_s)?;
    fs::write(admin,content)?;
    return Ok(());
  }
  let mut content = read_to_string(&admin)?;
  csproj_s = serde_json::from_str(&content.as_str())?;
  csproj_s.push(ProjectItem::new(target_path,new_info.property_group.assembly_title.clone(),new_info.clone()));
  content = serde_json::to_string(&csproj_s)?;
  fs::write(admin,content)?;
  Ok(())
}

pub fn add_new_project(create_project_info: CreateProjectInfo, project:Project) -> Result<(), AnyError> {
  let solution_path = create_project_info.root.join(&create_project_info.solution_name);
  let csproj_path = solution_path.join(&create_project_info.project_name);
  let target_sln = solution_path.join(format!("{}.sln",&create_project_info.solution_name));
  println!("{:#?}\n{:#?}\n{:#?}",&solution_path,&csproj_path,&target_sln);
  fs::create_dir(&csproj_path)?;
  create_csproj(
    csproj_path.join(format!("{}.csproj",create_project_info.project_name)),
    project
  )?;
  add_csproj_to_sln(target_sln, &create_project_info.project_name)?;
  Ok(())
}

// 在 solution 添加 project 字段
fn add_csproj_to_sln(target_sln: PathBuf, csproj_name: &String) -> Result<(), AnyError> {
  let project_item = PROJECT_ITEM
    .replace("$[a]", CS_GUID)
    .replace("$[b]", csproj_name)
    .replace("$[c]", &*guid());
  let mut sln_file = OpenOptions::new()
    .write(true)
    .append(true)
    .create(false)
    .open(&target_sln)?;
  sln_file.write_all(project_item.as_bytes())?;
  Ok(())
}

// 对 csproj 的 xml 进行格式化
fn format_xml(xml_string: String) -> Result<String, AnyError> {
  let input = BufReader::new(xml_string.as_bytes());
  let mut formatted_xml = String::new();
  let parser = EventReader::new(input);
  let mut depth = 0;
  for event in parser {
    match event {
      Ok(XmlEvent::StartElement { name, .. }) => {
        formatted_xml += &iter::repeat("  ")
          .take(depth)
          .collect::<String>();
        if depth == 2 {
          formatted_xml += &format!("<{}>", name);
        } else {
          formatted_xml += &format!("<{}>\n", name);
        }
        depth += 1;
      }
      Ok(XmlEvent::EndElement { name }) => {
        depth -= 1;
        if depth < 2 {
          formatted_xml += &iter::repeat("  ")
            .take(depth)
            .collect::<String>();
        }
        formatted_xml += &format!("</{}>\n", name);
      }
      Ok(XmlEvent::Characters(text)) => {
        formatted_xml += &format!("{}", text.trim());
      }
      Ok(_) => {}
      Err(e) => return Err(anyhow!("格式化 xml 失败：{}",e.to_string())),
    }
  }
  Ok(formatted_xml)
}

pub fn get_csproj_list(solution_item: SavedSolutionItem) -> Result<String, AnyError> {
  let admin = solution_item.path.join(solution_item.name).join(".admin");
  return Ok(read_to_string(admin)?);
}

#[cfg(test)]
mod tests {
  use std::path::PathBuf;
  use super::*;

  #[test]
  fn create_project_test() {
    let mut path = PathBuf::new();
    path = path.join("target/TestSolution");
    if path.exists() {
      fs::remove_dir_all(path).expect("文件夹移除失败？？？");
    }
    let info = CreateProjectInfo {
      root: PathBuf::from("target"),
      solution_name: String::from("TestSolution"),
      project_name: String::from("test"),
    };
    let result = create_project(info);
    assert_eq!(result.is_ok(), true)
  }

  #[test]
  fn create_csproj_test() {
    let project = Project ::new(String::from("test"));
    let result = create_csproj(
      PathBuf::from("target/debug/test.csproj"),project);
    assert_eq!(result.is_ok(), true)
  }

  #[test]
  fn create_new_project_test() {
    let create_project_info = CreateProjectInfo {
      root: PathBuf::from("target"),
      solution_name: String::from("TestSolution"),
      project_name: String::from("New_test"),
    };
    let project = Project::new(String::from("New_test"));
    let result = add_new_project(create_project_info, project);
    assert_eq!(result.is_ok(), true)
  }
}