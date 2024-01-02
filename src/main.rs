use std::sync::Arc;

use clap::Arg;
use repositories::pokemon::{InMemoryRepository, Repository, SqliteRepository};

mod api;
mod cli;
mod domain;
mod repositories;

#[macro_use]
extern crate rouille;

#[macro_use]
extern crate clap;

fn main() {
    let matches = command!()
        .version(crate_version!())
        .name(crate_name!())
        .author(crate_authors!())
        .arg(
            Arg::new("cli")
                .long("cli")
                .action(clap::ArgAction::SetFalse)
                .help("Runs in CLI mode"),
        )
        .arg(Arg::new("sqlite").long("sqlite").value_name("PATH"))
        .get_matches();

    let repo = build_repo(matches.get_one::<String>("sqlite"));

    match matches.contains_id("cli") {
        true => cli::run(repo.clone()),
        false => api::serve("localhost:8000", repo),
    }
}

fn build_repo(sqlite_value: Option<&String>) -> Arc<dyn Repository> {
    if let Some(path) = sqlite_value {
        match SqliteRepository::try_new(path) {
            Ok(repo) => return Arc::new(repo),
            _ => panic!("Error while creating sqlite repo"),
        }
    }

    Arc::new(InMemoryRepository::new())
}
