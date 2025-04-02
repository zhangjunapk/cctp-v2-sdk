use cctp_v2_sdk::cctpv2::Cctpv2;

#[tokio::main]
async fn main() {
    let cctpv2 = Cctpv2::new();
    cctpv2.start_burn().await;
    /*cctpv2
    .receive("0x9b0cc8f5cbf57253ed74f245e81b9d50f2768a040aec1f23c56d664a76dff3f5".to_string())
    .await;*/
}
