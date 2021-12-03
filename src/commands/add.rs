use clap::{App, Arg, ArgMatches};
use std::path::Path;

const COMMAND_DESCRIPTION: &str = "Add repository to track with repman. If repository is already added whole config file will be reset.";
const SUB_COMMAND_DESRIPTION: &str = "Path to repository.";

pub const COMMAND: &str = "add";
pub const SUB_COMMAND_PATH: &str = "path";

use crate::directorystorage;
use crate::utilities;

struct NewRepoEntry {
    path: String,
    name: String,
}

pub fn create_command() -> App<'static> {
    let app = App::new(COMMAND).about(COMMAND_DESCRIPTION).arg(
        Arg::new(SUB_COMMAND_PATH)
            .about(SUB_COMMAND_DESRIPTION)
            .required(true)
            .min_values(1),
    );
    app
}

pub fn command_handler(matches: &ArgMatches) {
    let args = matches.values_of(SUB_COMMAND_PATH);

    let repos: Vec<NewRepoEntry> = match args {
        None => Vec::new(),
        _ => args
            .unwrap()
            .map(|element| -> NewRepoEntry {
                let path = Path::new(element);
                return NewRepoEntry {
                    path: String::from(path.canonicalize().unwrap().to_str().unwrap()),
                    name: String::from(path.file_name().unwrap().to_str().unwrap()),
                };
            })
            .collect(),
    };

    repos.iter().for_each(|repo| {
        println!("Adding {}", repo.name);

        match directorystorage::save_to_file(
            utilities::create_repository_file(repo.path.clone()),
            repo.name.clone(),
        ) {
            _ => println!("{} added", repo.name),
        }
    });
}
