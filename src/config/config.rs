use std::path::PathBuf;

use dirs::home_dir;
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncReadExt};

const GH_SCRAPER_CONFIG_PATH_ENV_VARIABLE: &'static str = "GH_SCRAPER_CONFIG_PATH";
const GH_SCRAPER_CONFIG_FILENAME: &'static str = "config.json";

/// Struct rapresenting a config readen from the filesystem
#[derive(Serialize, Deserialize)]
pub struct Config {
    /// Config version
    version: u16,

    /// Github token used to query and clone available Github repositories
    pub gh_token: String,
}

impl Config {
    /// Read config from the path defined in the GH_SCRAPER_CONFIG_PATH env variable or the default folder.
    pub async fn read_from_file() -> Self {
        let mut file = File::open(config_path())
            .await
            .expect("Cannot open config file");

        let mut file_content = String::new();
        file.read_to_string(&mut file_content)
            .await
            .expect("Cannot read config file");

        parse_config_content(&file_content)
    }
}

fn config_path() -> PathBuf {
    std::env::var(GH_SCRAPER_CONFIG_PATH_ENV_VARIABLE).map_or_else(
        |_| {
            home_dir()
                .unwrap()
                .join(".config")
                .join("ghscraper")
                .join(GH_SCRAPER_CONFIG_FILENAME)
        },
        |v| PathBuf::from(v),
    )
}

fn parse_config_content(content: &str) -> Config {
    serde_json::from_str(content).expect("Cannot deserialize config file. It's probably corrupted.")
}
