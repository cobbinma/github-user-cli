mod cache;
mod github;
mod models;
mod service;

use crate::cache::Cache;
use crate::github::GitHub;
use crate::service::{Config, Service};
use clap::{App, Arg};
use log::{debug, info};
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    info!("starting up");

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
                .value_name("CLEAR_CACHE")
                .help("clear cache"),
        )
        .get_matches();

    let username = matches.value_of("username").unwrap();
    debug!("username: {}", username);
    let clear_cache = matches.is_present("clear_cache");
    debug!("clear cache: {}", clear_cache);

    let service = Service::new(Config {
        cache_client: Cache::new(&username),
        username: username.to_string(),
        repository_client: GitHub::new(),
        clear_cache,
    });

    info!("getting repositories from service");
    let repositories = service.get_repositories().await?;

    for r in repositories {
        println!("{}", r)
    }

    Ok(())
}
