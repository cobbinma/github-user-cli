mod models;

use clap::{App, Arg};
use models::Repository;
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("GitHub User CLI")
        .version("0.1.0")
        .author("Matthew Cobbing <cobbinma@gmail.com>")
        .about("find information about GitHub users")
        .arg(
            Arg::with_name("username")
                .short("u")
                .long("username")
                .value_name("USERNAME")
                .help("github username")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let username = matches.value_of("username").unwrap();

    let mut repositories = get_repositories(username).await?;

    repositories.sort_by(|a, b| b.stars.cmp(&a.stars));
    repositories.truncate(10);

    for r in repositories {
        println!("{}", r)
    }

    Ok(())
}

async fn get_repositories(username: &str) -> Result<Vec<Repository>, Box<dyn Error>> {
    surf::get(format!("https://api.github.com/users/{}/repos", username))
        .recv_json()
        .await
        .map_err(From::from)
}
