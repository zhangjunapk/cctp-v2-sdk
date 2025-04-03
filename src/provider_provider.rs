use std::sync::Arc;
use crate::config::{Chain, Config};
use alloy::network::{AnyNetwork, EthereumWallet};
use alloy::providers::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller,
};
use alloy::providers::{Identity, ProviderBuilder, RootProvider};
use alloy::signers::local::PrivateKeySigner;
use reqwest::Url;

pub(crate) struct ProviderProvider {
    pub config: Config,
}

impl ProviderProvider {
    pub fn new(config: Config) -> Self {
        ProviderProvider { config }
    }

    pub fn provider(
        &self,
        chain: &Chain,
    ) -> Option<
        FillProvider<
            JoinFill<
                JoinFill<
                    Identity,
                    JoinFill<
                        GasFiller,
                        JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>,
                    >,
                >,
                WalletFiller<EthereumWallet>,
            >,
            RootProvider<AnyNetwork>,
            AnyNetwork,
        >,
    > {
        let chain_config = self.config.chain_config.get(&chain);
        if let Some(chain_config) = chain_config {
            let signer: PrivateKeySigner = self.config.wallet_private_key.parse().unwrap();
            let wallet = EthereumWallet::from(signer);
            let provider = ProviderBuilder::new()
                .wallet(wallet)
                .network::<AnyNetwork>()
                .on_http(Url::parse(chain_config.endpoint.as_str()).unwrap());
            return Some(provider);
        }
        None
    }
}
