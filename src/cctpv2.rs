use crate::circle::api::message;
use crate::config::{Chain, ChainConfig, Config, Env};
use crate::contract::message_transmitter::MessageTransmitter;
use crate::contract::token_message::TokenMessage;
use crate::contract::usdc::Usdc;
use crate::provider_provider;
use crate::provider_provider::ProviderProvider;
use alloy::network::{AnyNetwork, EthereumWallet};
use alloy::primitives::Bytes;
use alloy::providers::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller,
};
use alloy::providers::{Identity, ProviderBuilder, RootProvider};
use alloy::signers::local::PrivateKeySigner;
use anyhow::anyhow;
use log::info;
use reqwest::Url;
use std::str::FromStr;
use std::sync::Arc;

pub struct Cctpv2 {
    config: Config,
    provider_provider: ProviderProvider,
}

impl Cctpv2 {
    pub fn new(env: Env) -> Self {
        let config = Config::load_config(env);
        match config {
            Ok(config) => Cctpv2 {
                config: config.clone(),
                provider_provider: ProviderProvider {
                    config: config.clone(),
                },
            },
            Err(err) => {
                panic!("加载配置错误:{}", err);
            }
        }
    }

    pub async fn start_burn(
        &self,
        source_chain: Chain,
        destination_chain: Chain,
        approve_amount: u128,
        burn_amount: u128,
        destination_address: &str,
        max_fee: u128,
        finality_threshold: u32,
    ) {
        let desc_chain_config = self.config.chain_config.get(&destination_chain).unwrap();

        let chain_config = self.config.chain_config.get(&source_chain);

        let provider = self.provider_provider.provider(&source_chain);

        if let Some(chain_config) = chain_config {
            if let Some(provider) = provider {
                let usdc = Usdc::new(&*chain_config.contract.usdc, provider.clone());
                let token_message =
                    TokenMessage::new(&*chain_config.contract.token_message, provider.clone());
                usdc.approve(&*chain_config.contract.token_message, approve_amount)
                    .await;
                token_message
                    .burn(
                        burn_amount,
                        desc_chain_config.id,
                        destination_address,
                        chain_config.contract.usdc.clone().to_string(),
                        max_fee,
                        finality_threshold,
                    )
                    .await
                    .unwrap();
            }
        }
    }

    pub async fn receive(
        &self,
        source_chain: Chain,
        desc_chain: Chain,
        burn_transaction_hash: String,
    ) {
        let source_chain_config = self.config.chain_config.get(&source_chain);

        let desc_chain_config = self.config.chain_config.get(&desc_chain);

        let provider = self.provider_provider.provider(&desc_chain);

        if let Some(provider) = provider {
            if let Some(dest_chain_config) = desc_chain_config {
                if let Some(source_chain_config) = source_chain_config {
                    let message_response =
                        message(source_chain_config.id, burn_transaction_hash).await;

                    let message_item = message_response.messages.get(0).unwrap();

                    if message_item.status != "complete" {
                        info!("Message status is not complete, skipping.");
                        return;
                    }

                    let message_transmitter = MessageTransmitter::new(
                        &dest_chain_config.contract.message_transmitter,
                        provider,
                    );
                    println!("message:{}", message_item.message);
                    println!("attestation:{}", message_item.attestation);

                    let message_bytes =
                        Bytes::from_str(message_item.message.clone().as_str()).unwrap();
                    let attestation_bytes =
                        Bytes::from_str(message_item.attestation.clone().as_str()).unwrap();

                    message_transmitter
                        .receive_message(message_bytes, attestation_bytes)
                        .await;
                }
            }
        }
    }
}
