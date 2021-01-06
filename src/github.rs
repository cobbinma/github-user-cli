use crate::models::{Repository, RepositoryService};
use async_trait::async_trait;
use std::error::Error;

pub struct GitHub {}

impl GitHub {
    pub fn new() -> Box<GitHub> {
        Box::new(GitHub {})
    }
}

#[async_trait]
impl RepositoryService for GitHub {
    async fn get_repositories(&self, username: &str) -> Result<Vec<Repository>, Box<dyn Error>> {
        surf::get(format!("https://api.github.com/users/{}/repos", username))
            .recv_json()
            .await
            .map_err(From::from)
    }
}
