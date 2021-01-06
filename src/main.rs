mod models;
mod service;
mod github;

use clap::{App, Arg};
use std::error::Error;
use crate::github::GitHub;
use crate::service::Service;

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

    let repository_service = GitHub::new();

    let service = Service::new(username.to_string(), repository_service);

    let repositories = service.run().await?;

    for r in repositories {
        println!("{}", r)
    }

    Ok(())
}
