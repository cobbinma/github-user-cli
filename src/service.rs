use crate::models::{Repository, RepositoryService};
use std::error::Error;

pub struct Service {
    config: Config,
}

pub struct Config {
    pub(crate) username: String,
    pub(crate) repository_service: Box<dyn RepositoryService>,
}

impl Service {
    pub fn new(config: Config) -> Self {
        Service { config }
    }

    pub async fn get_repositories(&self) -> Result<Vec<Repository>, Box<dyn Error>> {
        let mut repositories = self
            .config
            .repository_service
            .get_repositories(&self.config.username)
            .await?;

        repositories.sort_by(|a, b| b.stars.cmp(&a.stars));
        repositories.truncate(10);

        Ok(repositories)
    }
}
