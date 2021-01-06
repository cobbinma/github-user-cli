use crate::models::{Repository, RepositoryService};
use std::error::Error;

pub struct Service {
    config: Config,
}

pub struct Config {
    pub username: String,
    pub repository_service: Box<dyn RepositoryService>,
    pub clear_cache: bool,
}

impl Service {
    pub fn new(config: Config) -> Self {
        Service { config }
    }

    pub async fn get_repositories(&self) -> Result<Vec<Repository>, Box<dyn Error>> {
        println!("{}", self.config.clear_cache);
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
