use cctp_v2_sdk::cctpv2::Cctpv2;
use cctp_v2_sdk::config::{Chain, Env};

#[tokio::main]
async fn main() {
    let destation_address = "0x34435151EaD1Eae702d26EE1274E02DC69E9C641";
    let cctpv2 = Cctpv2::new(Env::Dev);
  /*  cctpv2
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
        Chain::Avalanche,
        "0xdd90fe9afc4e5997911e5df893aa887e232560362383f5f291fba3738d4faecf".to_string(),
    )
    .await;
}
