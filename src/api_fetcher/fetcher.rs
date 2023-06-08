use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};

use crate::config::config::Config;

use super::gh_repository::GhRepository;

const USER_AGENT: &'static str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36";
const GITHUB_API_VERSION: &'static str = "2022-11-28";
const GITHUB_RESPONSE_ENCODING: &'static str = "application/vnd.github+json";

pub async fn fetch_repos(config: &Config) -> Vec<GhRepository> {
    let mut page_number = 1;

    let org_name: &str = config.organization.as_ref();

    let mut repositories: Vec<GhRepository> = vec![];

    let http_client = Client::builder()
        .default_headers(headers(config))
        .build()
        .expect("Cannot build http client");

    loop {
        let response = http_client
            .get(format!(
                "https://api.github.com/orgs/{org_name}/repos?per_page=100&page={page_number}"
            ))
            .send()
            .await
            .expect("Failure on GET request to Github");

        if response.status().as_u16() == 401 {
            panic!("Unauthorized request");
        }

        let body = response.text().await.expect("Cannot retrieve body");

        let mut fetched =
            serde_json::from_str::<Vec<GhRepository>>(body.as_str()).expect("Failed to parse json");

        repositories.append(&mut fetched);

        if fetched.len() < 100 || fetched.is_empty() {
            // No more repositories
            break;
        }

        page_number += 1;
        if page_number > config.max_requests.unwrap_or(4) {
            break;
        }
    }
    repositories
}

fn headers(config: &Config) -> HeaderMap {
    let authorization = format!("Bearer {}", &config.gh_token);
    let mut headers = HeaderMap::with_capacity(4);
    headers.insert("Accept", HeaderValue::from_static(GITHUB_RESPONSE_ENCODING));
    headers.insert(
        "X-GitHub-Api-Version",
        HeaderValue::from_static(GITHUB_API_VERSION),
    );
    headers.insert(
        "Authorization",
        HeaderValue::from_str(authorization.as_str()).unwrap(),
    );
    headers.insert("User-Agent", HeaderValue::from_static(USER_AGENT));
    headers
}
