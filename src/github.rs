use crate::models::{Repository, RepositoryClient};
use async_trait::async_trait;
use std::error::Error;

pub struct GitHub {
    token: Option<String>,
}

impl GitHub {
    pub fn new(token: Option<String>) -> Box<GitHub> {
        Box::new(GitHub { token })
    }
}

#[async_trait]
impl RepositoryClient for GitHub {
    async fn get_repositories(&self, username: &str) -> Result<Vec<Repository>, Box<dyn Error>> {
        surf::get(format!("https://api.github.com/users/{}/repos", username))
            .recv_json()
            .await
            .map_err(From::from)
    }
}
