use crate::contract::message_transmitter::MESSAGE_TRANSMITTER::MESSAGE_TRANSMITTERInstance;
use alloy::network::{AnyNetwork, EthereumWallet};
use alloy::primitives::Bytes;
use alloy::providers::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller,
};
use alloy::providers::{Identity, RootProvider};
use alloy::sol;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    MESSAGE_TRANSMITTER,
    "message_transmitter.abi.json"
);

pub struct MessageTransmitter {
    instance: MESSAGE_TRANSMITTERInstance<
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

impl MessageTransmitter {
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
        MessageTransmitter {
            instance: MESSAGE_TRANSMITTER::new(contract_address.parse().unwrap(), provider),
        }
    }

    pub async fn receive_message(&self, message: Bytes, attestation: Bytes) {
        let result = self
            .instance
            .receiveMessage(message, attestation)
            .send()
            .await;
        match result {
            Ok(res) => {
                println!("Message received: {:?}", res.tx_hash().to_string());
            }
            Err(error) => {
                println!("Message received error: {:?}", error);
            }
        }
    }
}
