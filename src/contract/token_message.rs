use crate::config::Config;
use crate::contract::token_message::TOKEN_MESSAGE::TOKEN_MESSAGEInstance;
use alloy::network::{AnyNetwork, EthereumWallet};
use alloy::primitives::{FixedBytes, U256, address};
use alloy::providers::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller,
};
use alloy::providers::{Identity, RootProvider};
use alloy::sol;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    TOKEN_MESSAGE,
    "token_message.abi.json"
);

pub struct TokenMessage {
    token_messageinstance: TOKEN_MESSAGEInstance<
        (),
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
        AnyNetwork,
    >,
}

impl TokenMessage {
    pub fn new(
        contract_address: &str,
        provider: FillProvider<
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
        TokenMessage {
            token_messageinstance: TOKEN_MESSAGE::new(contract_address.parse().unwrap(), provider),
        }
    }

    pub async fn burn(&self, config: &Config) -> Result<String, ()> {
        //alloy_primitives::bits::fixed::FixedBytes<32>,

        let burn = self
            .token_messageinstance
            .depositForBurn(
                U256::from(1000000u128),
                6, //目标链的id
                FixedBytes::from(
                    address!("0x69d7b3de716ea095f15821d77ee9260e2d988d3b").into_word(),
                ),
                (&config.contract.dev.usdc.evm.as_str()).parse().unwrap(),
                FixedBytes::default(),
                U256::try_from(100000u128).unwrap(),
                2000,
            )
            .send()
            .await;

        match burn {
            Ok(val) => {
                println!("burn!");
                let tx_hash = val.inner().tx_hash().to_string();
                println!("burn tx hash:{}", tx_hash);
                Ok(tx_hash)
            }
            Err(e) => {
                println!("Error: {}", e.to_string());
                Err(())
            }
        }
    }
}
