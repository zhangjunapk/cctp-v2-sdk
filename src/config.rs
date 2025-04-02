use serde::Deserialize;
use serde::de::Error;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub evm_endpoint: String,
    pub wallet_private_key: String,
    pub contract: Contract,
}

#[derive(Debug, Deserialize)]
pub struct Contract {
    pub dev: ContractEnvironmentTypeConfig,
}

#[derive(Debug, Deserialize)]
pub struct ContractEnvironmentTypeConfig {
    pub usdc: ContractEnvironmentAddressConfig,
    pub token_message: ContractEnvironmentAddressConfig,
    pub message_transmitter: ContractEnvironmentAddressConfig,
}

#[derive(Debug, Deserialize)]
pub struct ContractEnvironmentAddressConfig {
    pub evm: String,
    pub avax: String,
    pub base: String,
}

impl Config {
    pub fn load_config() -> Result<Config, serde_yaml::Error> {
        let content = fs::read_to_string("config.yaml").unwrap_or_else(|_| String::new());
        serde_yaml::from_str(&content).map_err(|e| e)
    }
}
