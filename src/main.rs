use std::sync::Arc;

use clap::Arg;
use repositories::pokemon::InMemoryRepository;

mod api;
mod cli;
mod domain;
mod repositories;

#[macro_use]
extern crate rouille;

#[macro_use]
extern crate clap;

fn main() {
    let repo = Arc::new(InMemoryRepository::new());

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
        .get_matches();

    match matches.contains_id("cli") {
        true => cli::run(repo.clone()),
        false => api::serve("localhost:8000", repo),
    }
}
