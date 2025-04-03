use cctp_v2_sdk::cctpv2::Cctpv2;
use cctp_v2_sdk::config::{Chain, Env};

#[tokio::main]
async fn main() {
    let destation_address = "0x34435151EaD1Eae702d26EE1274E02DC69E9C641";
    let cctpv2 = Cctpv2::new(Env::Dev);
    /*cctpv2
        .start_burn(
            Chain::Evm,
            Chain::Avalanche,
            100000u128,
            100000u128,
            destation_address,
            10000u128,
            1000,
        )
        .await;*/
    cctpv2
    .receive(
        Chain::Evm,
        Chain::Avalanche,
        "0x0ff7dd975bbf552bebc01924db6d84c05f6bc76deaf6d0232d7c415f47d4f1ac".to_string(),
    )
    .await;
}
