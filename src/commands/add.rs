use clap::{App, Arg, ArgMatches};
use std::path::Path;

const COMMAND_DESCRIPTION: &str = "Add repository to track with repman. If repository is already added whole config file will be reset.";
const SUB_COMMAND_DESRIPTION: &str = "Path to repository.";

pub const COMMAND: &str = "add";
pub const SUB_COMMAND_PATH: &str = "path";

use crate::directorystorage;
use crate::utilities;

pub fn create_command() -> App<'static> {
    let app = App::new(COMMAND).about(COMMAND_DESCRIPTION).arg(
        Arg::new(SUB_COMMAND_PATH)
            .about(SUB_COMMAND_DESRIPTION)
            .required(true),
    );
    app
}

pub fn command_handler(matches: &ArgMatches) {
    let path = Path::new(matches.value_of(SUB_COMMAND_PATH).unwrap());

    println!("Adding {}", path.canonicalize().unwrap().display());

    match directorystorage::save_to_file(
        utilities::create_repository_file(path.canonicalize().unwrap().to_str().unwrap()),
        String::from(path.file_name().unwrap().to_str().unwrap()),
    ) {
        _ => (),
    }
}
