use serde::export::Formatter;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    #[serde(rename = "stargazers_count")]
    pub stars: i64,
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
