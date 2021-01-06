use std::error::Error;
use crate::models::{Repository, RepositoryService};

pub struct Service {
    username: String,
    repository_service: Box<dyn RepositoryService>
}

impl Service {
    pub fn new(username: String, repository_service: Box<dyn RepositoryService>) -> Self {
        Service { username, repository_service }
    }

    pub async fn run(&self) -> Result<Vec<Repository>, Box<dyn Error>> {
        let mut repositories = self.repository_service.get_repositories(&self.username).await?;

        repositories.sort_by(|a, b| b.stars.cmp(&a.stars));
        repositories.truncate(10);

        Ok(repositories)
    }
}