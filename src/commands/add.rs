use clap::{App, Arg, ArgMatches};
use std::path::Path;

pub const COMMAND: &str = "add";
pub const SUB_COMMAND_PATH: &str = "path";

pub fn create_command() -> App<'static> {
    let app = App::new(COMMAND)
        .about("Add repository to track with repman.")
        .arg(
            Arg::new(SUB_COMMAND_PATH)
                .about("Path to repository.")
                .required(true),
        );
    app
}

pub fn command_handler(matches: &ArgMatches) {
    let path = Path::new(matches.value_of(SUB_COMMAND_PATH).unwrap());
    println!("Adding {}", path.canonicalize().unwrap().display());
}
