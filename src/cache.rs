use crate::models::{CacheClient, Repository};
use std::error::Error;
use std::fs::File;
use std::path::Path;

pub struct Cache {
    file_path: String,
}

impl Cache {
    pub fn new(username: &str) -> Box<Self> {
        let file_path = format!("{}.json", username);
        Box::new(Self { file_path })
    }
}

impl CacheClient for Cache {
    fn exists(&self) -> bool {
        Path::new(&self.file_path).exists()
    }

    fn get_repositories(&self) -> Result<Vec<Repository>, Box<dyn Error>> {
        serde_json::from_reader(File::open(Path::new(&self.file_path))?).map_err(From::from)
    }

    fn set_repositories(&self, repositories: &[Repository]) -> Result<(), Box<dyn Error>> {
        serde_json::to_writer(File::create(Path::new(&self.file_path))?, &repositories)
            .map_err(From::from)
    }
}
