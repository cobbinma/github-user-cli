use crate::models::{CacheClient, Repository, RepositoryClient};
use log::info;
use std::error::Error;

pub struct Service {
    config: Config,
}

pub struct Config {
    pub username: String,
    pub repository_client: Box<dyn RepositoryClient>,
    pub cache_client: Box<dyn CacheClient>,
    pub clear_cache: bool,
}

impl Service {
    pub fn new(config: Config) -> Self {
        Service { config }
    }

    pub async fn get_repositories(&self) -> Result<Vec<Repository>, Box<dyn Error>> {
        let mut repositories: Vec<Repository> =
            if !self.config.clear_cache && self.config.cache_client.exists(&self.config.username) {
                info!("getting repositories from cache");
                self.config.cache_client.get_repositories(&self.config.username)?
            } else {
                info!("getting repositories from repository client");
                let repositories = self
                    .config
                    .repository_client
                    .get_repositories(&self.config.username)
                    .await?;

                info!("setting repositories to cache");
                self.config.cache_client.set_repositories(&self.config.username, &repositories)?;

                repositories
            };

        repositories.sort_by(|a, b| b.stars.cmp(&a.stars));
        repositories.truncate(10);

        Ok(repositories)
    }
}
