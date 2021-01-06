use crate::models::{CacheClient, Repository};
use std::error::Error;
use std::fs::File;
use std::path::Path;

pub struct Cache {}

impl Cache {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl CacheClient for Cache {
    fn exists(&self, username: &str) -> bool {
        let file_path = format!("{}.json", username);
        Path::new(&file_path).exists()
    }

    fn get_repositories(&self, username: &str) -> Result<Vec<Repository>, Box<dyn Error>> {
        let file_path = format!("{}.json", username);
        serde_json::from_reader(File::open(Path::new(&file_path))?).map_err(From::from)
    }

    fn set_repositories(
        &self,
        username: &str,
        repositories: &[Repository],
    ) -> Result<(), Box<dyn Error>> {
        let file_path = format!("{}.json", username);
        serde_json::to_writer(File::create(Path::new(&file_path))?, &repositories)
            .map_err(From::from)
    }
}
