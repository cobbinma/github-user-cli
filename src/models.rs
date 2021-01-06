use async_trait::async_trait;
use serde::export::Formatter;
use std::error::Error;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "html_url")]
    pub url: String,
    #[serde(rename = "stargazers_count")]
    pub stars: i64,
}

#[async_trait]
pub trait RepositoryClient {
    async fn get_repositories(&self, username: &str) -> Result<Vec<Repository>, Box<dyn Error>>;
}

pub trait CacheClient {
    fn exists(&self) -> bool;
    fn get_repositories(&self) -> Result<Vec<Repository>, Box<dyn Error>>;
    fn set_repositories(&self, repositories: &[Repository]) -> Result<(), Box<dyn Error>>;
}

impl std::fmt::Display for Repository {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: '{}', ", self.name)?;
        if let Some(d) = &self.description {
            write!(f, "description: '{}', ", d)?;
        };
        write!(f, "url: '{}', ", self.url)?;
        write!(f, "stars: '{}'", self.stars)
    }
}
