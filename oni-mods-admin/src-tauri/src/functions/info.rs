use std::fs;
use std::fs::read_to_string;
use std::path::{Path};
use reqwest;
use anyhow::{Result, Error as AnyError};
use scraper::Selector;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigInfo {
    pub latest_version: u32,
}

fn get_latest_version() -> Result<u32,AnyError>{
    let url = "https://forums.kleientertainment.com/game-updates/oni-alpha/";
    let response = reqwest::blocking::get(url);
    if response.is_err() {
        return Ok(526233)
    }
    let body = response?.text().unwrap();
    let document = scraper::Html::parse_document(&body);
    let selector = Selector::parse("h3").expect("选择器无法解析");
    let version_txt = document.select(&selector).next().map(|element| element.text().collect::<String>());
    return if let Some(text) = version_txt {
        let str = text.trim().replace("Release", "");
        // TODO 这里的代码还要检查一下是否会直接报错
        Ok(str.trim().parse::<u32>().unwrap())
    } else {
        Ok(526233)
    }
}

pub fn refresh_version() -> Result<u32, AnyError> {
    let latest_version = get_latest_version()?;
    let mut config_info = load_config_file()?;
    if &latest_version > &config_info.latest_version {
        config_info.latest_version = latest_version;
        save_config_file(&config_info)?;
    }
    Ok(latest_version)
}

pub fn load_config_file() -> Result<ConfigInfo, AnyError>{
    let config_path = Path::new("./config.json");
    let mut config_info = ConfigInfo{
        latest_version: 526233,
    };
    if config_path.exists() {
        config_info = serde_json::from_str::<ConfigInfo>(&read_to_string(config_path)?)?;
    } else {
        save_config_file(&config_info)?;
    }
    Ok(config_info)
}

fn save_config_file(config_info:&ConfigInfo) -> Result<(), AnyError>{
    let config_path = Path::new("./config.json");
    let content = serde_json::to_string(&config_info)?;
    fs::write(config_path, &content)?;
    Ok(())
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn save_load_config_info_test(){
        let config_info = load_config_file();
        let new_config_info = ConfigInfo{
            latest_version: 222,
        };
        let result = save_config_file(&new_config_info);
        print!("{:?}{:?}", config_info, result);
        assert_eq!(config_info.is_ok(), true);
        assert_eq!(result.is_ok(), true);
    }
    #[test]
    fn get_latest_version_test(){
        let result = refresh_version();
        println!("{:?}",result);
        assert_eq!(result.is_ok(),true)
    }
}