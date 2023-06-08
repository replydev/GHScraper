use crate::config::config::Config;

mod config;

#[tokio::main]
async fn main() {
    let config = Config::read_from_file().await;
}
