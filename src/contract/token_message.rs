use crate::contract::token_message::TOKEN_MESSAGE::TOKEN_MESSAGEInstance;
use alloy::network::{AnyNetwork, EthereumWallet};
use alloy::primitives::{Address, FixedBytes, U256};
use alloy::providers::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller,
};
use alloy::providers::{Identity, RootProvider};
use alloy::sol;
use std::str::FromStr;

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

    pub async fn burn(
        &self,
        burn_amount: u128,
        destination_domain: u32,
        destination_address: &str,
        usdc_contract_address: String,
        max_fee: u128,
        finality_threshold: u32,
    ) -> Result<String, ()> {
        //alloy_primitives::bits::fixed::FixedBytes<32>,

        let burn = self
            .token_messageinstance
            .depositForBurn(
                U256::from(burn_amount),
                destination_domain, //目标链的id
                FixedBytes::from(Address::from_str(destination_address).unwrap().into_word()),
                usdc_contract_address.parse().unwrap(),
                FixedBytes::default(),
                U256::try_from(max_fee).unwrap(),
                finality_threshold,
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
