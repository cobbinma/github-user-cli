mod github;
mod models;
mod service;

use crate::github::GitHub;
use crate::service::{Config, Service};
use clap::{App, Arg};
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
        .arg(
            Arg::with_name("clear_cache")
                .short("cc")
                .long("clear_cache")
                .default_value("false")
                .value_name("CLEAR_CACHE")
                .help("clear cache"),
        )
        .get_matches();

    let username = matches.value_of("username").unwrap();
    let clear_cache = matches.is_present("clear_cache");

    let service = Service::new(Config {
        username: username.to_string(),
        repository_service: GitHub::new(),
        clear_cache,
    });

    let repositories = service.get_repositories().await?;

    for r in repositories {
        println!("{}", r)
    }

    Ok(())
}
