use crate::config::Config;
use alloy::network::{AnyNetwork, EthereumWallet};
use alloy::primitives::U256;
use alloy::providers::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller,
};
use alloy::providers::{Identity, RootProvider};
use alloy::sol;
use std::sync::Arc;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    USDC,
    "usdc.abi.json"
);

pub struct Usdc {
    instance: USDC::USDCInstance<
        (),
        Arc<
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
        >,
        AnyNetwork,
    >,
}

impl Usdc {
    pub fn new(
        contract_address: &str,
        provider:
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
    ) -> Self {
        Usdc {
            instance: USDC::new(
                contract_address.parse().unwrap(),
                provider.to_owned().into(),
            ),
        }
    }

    pub async fn approve(&self, config: &Config) {
        let approve_result = self
            .instance
            .approve(
                (&config.contract.dev.token_message.evm.as_str())
                    .parse()
                    .unwrap(),
                U256::try_from(1000000u128).unwrap(),
            )
            .send()
            .await;

        match approve_result {
            Ok(val) => println!("Approved!"),
            Err(e) => println!("Error: {}", e),
        }
    }
}
