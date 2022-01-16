use serde::{Deserialize,Serialize};
use toml;
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

#[derive(Deserialize,Serialize,Debug)]
pub struct Token {
    pub access_token: Option<String>,
    pub expires_in: Option<u64>,
}

#[derive(Deserialize,Serialize,Debug)]
pub struct Config {
    pub auth: Auth,
    pub params: Params,
    pub token: Token,
}



pub fn load_conf(path:&String) -> BoxResult<Config> {
    let s = std::fs::read_to_string(path)?;
    let config:Config = toml::from_str(&s)?;
    Ok(config)
}

pub fn save_conf(path:&String,conf:&Config) -> BoxResult<()> {
    let s = toml::to_string_pretty(conf)?;
    std::fs::write(path, s)?;
    Ok(())
}