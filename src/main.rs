mod cache;
mod github;
mod models;
mod service;

use crate::cache::Cache;
use crate::github::GitHub;
use crate::models::Repository;
use crate::service::{Config, Service};
use clap::{App, Arg};
use log::{debug, info};
use std::error::Error;
use std::fs::File;
use std::path::Path;

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
        .arg(
            Arg::with_name("save_json")
                .short("j")
                .long("save_json")
                .value_name("PATH")
                .help("save json output file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("save_yaml")
                .short("y")
                .long("save_yaml")
                .value_name("PATH")
                .help("save yaml output file")
                .takes_value(true),
        )
        .get_matches();

    let username = matches.value_of("username").unwrap();
    debug!("username: {}", username);
    let clear_cache = matches.is_present("clear_cache");
    debug!("clear cache: {}", clear_cache);

    let service = Service::new(Config {
        cache_client: Cache::new(),
        username: username.to_string(),
        repository_client: GitHub::new(),
        clear_cache,
    });

    info!("getting repositories from service");
    let repositories = service.get_repositories().await?;

    let json_path = matches.value_of("save_json");
    let yaml_path = matches.value_of("save_yaml");

    match (json_path, yaml_path) {
        (Some(path), None) => save_json_output(path, &repositories),
        (None, Some(path)) => save_yaml_output(path, &repositories),
        (Some(json_path), Some(yaml_path)) => {
            save_json_output(json_path, &repositories)?;
            save_yaml_output(yaml_path, &repositories)
        }
        (None, None) => {
            for r in repositories {
                println!("{}", r)
            }
            Ok(())
        }
    }
}

fn save_json_output(path: &str, repositories: &[Repository]) -> Result<(), Box<dyn Error>> {
    info!("saving json output {}", path);
    serde_json::to_writer_pretty(File::create(Path::new(path))?, &repositories).map_err(From::from)
}

fn save_yaml_output(path: &str, repositories: &[Repository]) -> Result<(), Box<dyn Error>> {
    info!("saving yaml output {}", path);
    serde_yaml::to_writer(File::create(Path::new(path))?, &repositories).map_err(From::from)
}
