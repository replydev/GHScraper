use serde::{Deserialize, Serialize};

/// Rapresents a GhRepository from Github API
#[derive(Serialize, Deserialize)]
pub struct GhRepository {
    id: u64,
    clone_url: String,
}
