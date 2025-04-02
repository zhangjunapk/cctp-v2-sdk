use crate::circle::api::message;
use crate::config::Config;
use crate::contract::message_transmitter::MessageTransmitter;
use crate::contract::token_message::TokenMessage;
use crate::contract::usdc::Usdc;
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

pub struct Cctpv2 {
    config: Config,
    provider: FillProvider<
        JoinFill<
            JoinFill<
                Identity,
                JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
            >,
            WalletFiller<EthereumWallet>,
        >,
        RootProvider<AnyNetwork>,
        AnyNetwork,
    >,
}

impl Cctpv2 {
    pub fn new() -> Self {
        let config = Config::load_config();
        match config {
            Ok(config) => {
                let signer: PrivateKeySigner = config.wallet_private_key.parse().unwrap();
                let wallet = EthereumWallet::from(signer);
                let provider = ProviderBuilder::new()
                    .wallet(wallet)
                    .network::<AnyNetwork>()
                    .on_http(Url::parse(config.evm_endpoint.as_str()).unwrap());
                Cctpv2 { config, provider }
            }
            Err(err) => {
                panic!("加载配置错误:{}", err);
            }
        }
    }

    pub async fn start_burn(&self) {
        println!("new instance:{:?}", &self.config.contract.dev.usdc.evm);
        println!(
            "token message:{:?}",
            &self.config.contract.dev.token_message.evm
        );

        let usdc = Usdc::new(&self.config.contract.dev.usdc.evm, self.provider.clone());
        let token_message = TokenMessage::new(
            &self.config.contract.dev.token_message.evm,
            self.provider.clone(),
        );
        usdc.approve(&self.config).await;
        token_message.burn(&self.config).await.unwrap();
    }

    pub async fn receive(&self, burn_transaction_hash: String) {
        let message_response = message(burn_transaction_hash).await;

        let message_item = message_response.messages.get(0).unwrap();

        if message_item.status != "complete" {
            info!("Message status is not complete, skipping.");
            return;
        }

        let message_transmitter = MessageTransmitter::new(
            &self.config.contract.dev.message_transmitter.evm,
            self.provider.clone(),
        );

        println!("message:{}", message_item.message);
        println!("attestation:{}", message_item.attestation);

        let message_bytes = Bytes::from(message_item.message.clone());
        let attestation_bytes = Bytes::from(message_item.attestation.clone());

        message_transmitter
            .receive_message(message_bytes, attestation_bytes)
            .await;
    }
}
