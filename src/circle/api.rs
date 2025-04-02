use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Message {
    pub messages: Vec<MessageItem>,
}

#[derive(Deserialize, Clone)]
pub struct MessageItem {
    pub attestation: String,
    pub message: String,
    #[serde(alias = "eventNonce")]
    pub event_nonce: String,
    #[serde(alias = "cctpVersion")]
    pub cctp_version: u8,
    pub status: String,
}

pub async fn message(transaction_hash: String) -> Message {
    println!(
        "https://iris-api-sandbox.circle.com/v2/messages/0?transactionHash={transaction_hash}"
    );
    reqwest::get(format!(
        "https://iris-api-sandbox.circle.com/v2/messages/0?transactionHash={transaction_hash}"
    ))
    .await
    .unwrap()
    .json()
    .await
    .unwrap()
}
