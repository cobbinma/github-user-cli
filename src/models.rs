use serde::export::Formatter;
use std::error::Error;
use async_trait::async_trait;

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
pub trait RepositoryService {
    async fn get_repositories(&self, username: &str) -> Result<Vec<Repository>, Box<dyn Error>>;
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
