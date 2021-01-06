use crate::models::{RepositoryService, Repository};
use std::error::Error;
use async_trait::async_trait;

pub struct GitHub {}

impl GitHub {
    pub fn new() -> Box<GitHub> {
        Box::new(GitHub{})
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