use std::{path::PathBuf, sync::Arc};

use git2::{build::RepoBuilder, Cred, Error, FetchOptions, RemoteCallbacks, Repository};
use tokio::task::JoinSet;

use crate::{api_fetcher::gh_repository::GhRepository, config::config::Config};

pub async fn clone_repos(repositories: &[GhRepository], config: &Config) {
    let mut join_set = JoinSet::new();
    let repos: Vec<UrlWithPath> = map_repos(repositories, default_path(config));
    for url_with_path in repos {
        let username = config.gh_username.to_owned();
        let password = config.gh_password().to_string();

        join_set.spawn(async move {
            let mut remote_callbacks = RemoteCallbacks::new();
            remote_callbacks.credentials(|_url, username_from_url, _allowed_types| {
                Cred::userpass_plaintext(&username, &password)
            });

            let mut fetch_options = FetchOptions::new();
            fetch_options.remote_callbacks(remote_callbacks);

            let mut repo_builder = RepoBuilder::new();
            repo_builder.fetch_options(fetch_options);
            if url_with_path.path.is_dir() {
                let path = url_with_path.path.clone();
                std::fs::remove_dir_all(path.as_path());
            }
            RepoBuilder::clone(
                &mut repo_builder,
                url_with_path.url.as_str(),
                &url_with_path.path,
            )
        });
    }

    handle_results(join_set).await
}

async fn handle_results(mut join_set: JoinSet<Result<Repository, Error>>) {
    while let Some(res) = join_set.join_next().await {
        if let Ok(clone_res) = res {
            if let Ok(repo) = clone_res {
                println!(
                    "{} is ok",
                    repo.path()
                        .to_str()
                        .unwrap_or("<CANNOT CONVERT PATH TO STRING>")
                )
            } else {
                let error = if let Err(e) = clone_res {
                    e
                } else {
                    unreachable!()
                };
                eprintln!("An error occurred: {:?}", error)
            }
        } else {
            let error = if let Err(e) = res { e } else { unreachable!() };
            eprintln!("A concurrency error occurred: {:?}", error);
        }
    }
}

struct UrlWithPath {
    url: String,
    path: PathBuf,
}

fn map_repos(repositories: &[GhRepository], default_path: PathBuf) -> Vec<UrlWithPath> {
    repositories
        .into_iter()
        .map(|repo| UrlWithPath {
            url: repo.clone_url.to_owned(),
            path: default_path.join(repo.name.as_str()),
        })
        .collect()
}

fn default_path(config: &Config) -> PathBuf {
    config
        .download_path
        .as_ref()
        .map(|dp| PathBuf::from(dp))
        .unwrap_or_else(|| {
            dirs::home_dir()
                .map(|home| home.join("gh_scraped_repos"))
                .expect("Cannot retrieve home dir")
        })
}
