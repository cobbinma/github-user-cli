mod models;

use async_std::task;
use clap::{App, Arg};
use models::Repository;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
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

    task::block_on(async {
        let mut repositories: Vec<Repository> = surf::get(format!(
            "{}{}{}",
            "https://api.github.com/users/", username, "/repos"
        ))
        .recv_json()
        .await
        .unwrap();
        repositories.sort_by(|a, b| b.stars.cmp(&a.stars));

        for r in repositories {
            println!("{}", r)
        }
    });

    Ok(())
}
