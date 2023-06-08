use api_fetcher::fetcher::fetch_repos;

use crate::config::config::Config;

mod api_fetcher;
mod config;

#[tokio::main]
async fn main() {
    let config = Config::read_from_file().await;
    let repos = fetch_repos(&config).await;
    println!("{:?}", serde_json::to_string_pretty(&repos).unwrap());
}
