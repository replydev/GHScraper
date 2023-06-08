use api_fetcher::fetcher::fetch_repos;

use crate::config::config::Config;
use crate::repo::cloner::clone_repos;

mod api_fetcher;
mod config;
mod repo;

#[tokio::main]
async fn main() {
    println!("Loading config...");
    let config = Config::read_from_file().await;
    println!("Fetching repos...");
    let repos = fetch_repos(&config).await;
    println!("Cloning repos...");
    clone_repos(repos.as_ref(), &config).await;
}
