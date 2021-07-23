use clap::{App, Arg, ArgMatches};
use std::path::Path;

pub const COMMAND: &str = "add";
pub const SUB_COMMAND_PATH: &str = "path";

pub fn create_command() -> App<'static> {
    let app = App::new(COMMAND).arg(Arg::new(SUB_COMMAND_PATH).required(true));
    app
}

pub fn command_handler(matches: &ArgMatches) {
    let path = Path::new(matches.value_of(SUB_COMMAND_PATH).unwrap());
    println!("Adding {}", path.canonicalize().unwrap().display());
}
