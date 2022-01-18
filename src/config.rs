use std::{io::BufRead, path::PathBuf};

use serde::{Deserialize,Serialize};
use toml;
use uuid;

use super::utils::BoxResult;

#[derive(Deserialize,Serialize,Debug)]
pub struct Auth {
    pub key: String,
    pub secret: String,
}

#[derive(Deserialize,Serialize,Debug)]
pub struct Params {
    pub aue:String,
    pub per:String,
    pub cuid:String,
}

#[derive(Deserialize,Serialize,Debug,Default)]
pub struct Token {
    pub access_token: Option<String>,
    pub expires_in: Option<u64>,
}

#[derive(Deserialize,Serialize,Debug,Default)]
pub struct Config {
    pub auth: Auth,
    pub params: Params,
    pub token: Token,
}

impl Default for Params {
    fn default() -> Self {
        let uid = format!("{}",uuid::Uuid::new_v4());
        Params {
            aue: "6".to_string(),
            per: "0".to_string(),
            cuid: uid,
        }
    }
}

impl Default for Auth {
    fn default() -> Self {
        let mut key = String::new();
        println!("使用baidu-tts 参: https://ai.baidu.com/tech/speech/tts");
        println!("输入baidu-tts应用的key:");
        std::io::stdin().lock().read_line(&mut key).expect("read key error");
        let mut secret = String::new();
        println!("输入baidu-tts应用的secret:");
        std::io::stdin().lock().read_line(&mut secret).expect("read secret error");
        if key.ends_with('\n') {
            key.truncate(key.len() - 1);
        }
        if secret.ends_with('\n') {
            secret.truncate(secret.len() - 1);
        }
        Auth {
            key: key,
            secret: secret
        }
    }
}

pub fn load_conf(path:&PathBuf) -> BoxResult<Config> {
    let s = std::fs::read_to_string(path)?;
    let config:Config = toml::from_str(&s)?;
    Ok(config)
}

pub fn save_conf(path:&PathBuf,conf:&Config) -> BoxResult<()> {
    let s = toml::to_string_pretty(conf)?;
    std::fs::write(path, s)?;
    Ok(())
}