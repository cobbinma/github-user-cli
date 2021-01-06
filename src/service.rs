use crate::models::{Repository, RepositoryService};
use std::error::Error;
use std::fs::File;
use std::path::Path;
use log::info;

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
        let file_name = format!("{}.json", self.config.username);
        let path = Path::new(&file_name);

        info!("{}", path.exists());
        info!("{}", self.config.clear_cache);

        let mut repositories: Vec<Repository> = if !self.config.clear_cache && path.exists() {
            info!("reading repositories from cache");

            serde_json::from_reader(File::open(path)?)?
        } else {
            info!("getting repositories from repository service");
            let repositories = self.config
                .repository_service
                .get_repositories(&self.config.username)
                .await?;

            info!("saving cache");
            serde_json::to_writer(File::create(path)?, &repositories)?;

            repositories
        };

        repositories.sort_by(|a, b| b.stars.cmp(&a.stars));
        repositories.truncate(10);

        Ok(repositories)
    }
}
