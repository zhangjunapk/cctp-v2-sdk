use serde::de::Error;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

pub enum Env {
    Dev,
    Pro,
}

#[derive(Deserialize, Hash, Eq, PartialEq, Debug,Clone)]
pub enum Chain {
    Evm,
    Avalanche,
    Base,
}
#[derive(Deserialize,Clone)]
pub struct ChainConfig {
    pub id: u32,
    pub endpoint: String,
    pub(crate) contract: Contract,
}

#[derive(Deserialize, Debug,Clone)]
pub struct Contract {
    pub usdc: String,
    pub token_message: String,
    pub message_transmitter: String,
}

#[derive(Deserialize,Clone)]
pub struct Config {
    pub wallet_private_key: String,
    pub chain_config: HashMap<Chain, ChainConfig>,
}

#[derive(Debug, Deserialize,Clone)]
pub struct ContractEnvironmentAddressConfig {
    pub evm: String,
    pub avalanche: String,
    pub base: String,
}

impl Config {
    pub fn load_config(env: Env) -> Result<Config, serde_yaml::Error> {
        let file_name = match env {
            Env::Dev => "config-dev.yaml",
            Env::Pro => "config-pro.yaml",
        };
        let content = fs::read_to_string(file_name).unwrap_or_else(|_| String::new());
        serde_yaml::from_str(&content).map_err(|e| e)
    }
}
