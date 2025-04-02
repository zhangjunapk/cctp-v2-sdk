use cctp_v2_sdk::config::Config;

#[test]
fn load_config() {
    let config = Config::load_config();
    match config {
        Ok(config) => {
            println!("{:?}", config);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
