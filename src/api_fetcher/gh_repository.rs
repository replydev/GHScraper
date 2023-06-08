use serde::{Deserialize, Serialize};

/// Rapresents a GhRepository from Github API
#[derive(Serialize, Deserialize)]
pub struct GhRepository {
    id: u64,
    pub name: String,
    pub clone_url: String,
}
