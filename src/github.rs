use crate::models::{Repository, RepositoryClient};
use async_trait::async_trait;
use base64::encode;
use std::error::Error;

const GITHUB_BASE_URL: &str = "https://api.github.com";

pub struct GitHub {
    url: String,
    token: Option<String>,
}

impl GitHub {
    pub fn new(url: Option<String>, token: Option<String>) -> Box<GitHub> {
        Box::new(GitHub {
            token,
            url: url.unwrap_or_else(|| GITHUB_BASE_URL.to_string()),
        })
    }
}

#[async_trait]
impl RepositoryClient for GitHub {
    async fn get_repositories(&self, username: &str) -> Result<Vec<Repository>, Box<dyn Error>> {
        let mut req = surf::get(format!("{}/users/{}/repos", &self.url, username));

        if let Some(token) = &self.token {
            req = req.header(
                "Authorization",
                format!("Basic {}", encode(format!("{}:{}", username, token))),
            );
        }

        req.recv_json().await.map_err(From::from)
    }
}

#[cfg(test)]
mod tests {
    use crate::models::RepositoryClient;
    use crate::GitHub;
    use httpmock::MockServer;
    use serde_json::json;

    #[async_std::test]
    async fn get_repositories_success_with_token_test() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method("GET")
                .path("/users/cobbinma/repos")
                .header("Authorization", "Basic Y29iYmlubWE6VE9LRU4=");
            then.status(200)
                .json_body(json!([
  {
    "name": "r-cache",
    "html_url": "https://github.com/cobbinma/r-cache",
    "description": "r-cache is an in memory key value store. It is thread safe and values can have expiry times",
    "stargazers_count": 6,
  },
  {
    "name": "example-go-api",
    "html_url": "https://github.com/cobbinma/example-go-api",
    "description": "Example Go REST API",
    "stargazers_count": 63,
  }
]));
        });
        let client = GitHub::new(Some(server.base_url()), Some("TOKEN".to_string()));

        let result = client.get_repositories("cobbinma").await;

        mock.assert();
        assert_eq!(result.is_ok(), true);
        let repositories = result.unwrap();

        insta::assert_json_snapshot!(&repositories);
    }

    #[async_std::test]
    async fn get_repositories_success_without_token_test() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method("GET")
                .path("/users/cobbinma/repos");
            then.status(200)
                .json_body(json!([
  {
    "name": "example-go-api",
    "html_url": "https://github.com/cobbinma/example-go-api",
    "description": "Example Go REST API",
    "stargazers_count": 63,
  },
  {
    "name": "r-cache",
    "html_url": "https://github.com/cobbinma/r-cache",
    "description": "r-cache is an in memory key value store. It is thread safe and values can have expiry times",
    "stargazers_count": 6,
  }
]));
        });
        let client = GitHub::new(Some(server.base_url()), None);

        let result = client.get_repositories("cobbinma").await;

        mock.assert();
        assert_eq!(result.is_ok(), true);
        let repositories = result.unwrap();

        insta::assert_json_snapshot!(&repositories);
    }
}
