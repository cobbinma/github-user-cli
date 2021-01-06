use async_trait::async_trait;
use colored::*;
use serde::export::Formatter;
use std::error::Error;
use std::fmt::Display;

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
    fn exists(&self, username: &str) -> bool;
    fn get_repositories(&self, username: &str) -> Result<Vec<Repository>, Box<dyn Error>>;
    fn set_repositories(
        &self,
        username: &str,
        repositories: &[Repository],
    ) -> Result<(), Box<dyn Error>>;
}

impl Display for Repository {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn write_field(f: &mut Formatter<'_>, name: &str, value: &str) -> std::fmt::Result {
            write!(f, "{} : {}, ", name.green(), value.blue())
        }
        write_field(f, "name", &self.name)?;
        if let Some(d) = &self.description {
            write_field(f, "description", d)?;
        };
        write_field(f, "url", &self.url)?;
        write_field(f, "stars", &self.stars.to_string())
    }
}
