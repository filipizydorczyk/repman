use clap::{App, Arg, ArgMatches};

const COMMAND_DESCRIPTION: &str = "Unregister repository from repman. If there is no provided repository registered, no actions will be executed.";
const SUB_COMMAND_DESRIPTION: &str = "Name of repository to unregister from repman (same as name of direcotry where repositoryu is being kept in).";

pub const COMMAND: &str = "remove";
pub const SUB_COMMAND_PATH: &str = "name";

use crate::directorystorage;

pub fn create_command() -> App<'static> {
    let app = App::new(COMMAND).about(COMMAND_DESCRIPTION).arg(
        Arg::new(SUB_COMMAND_PATH)
            .about(SUB_COMMAND_DESRIPTION)
            .required(true),
    );
    app
}

pub fn command_handler(matches: &ArgMatches) {
    let name = String::from(matches.value_of(SUB_COMMAND_PATH).unwrap());
    match directorystorage::remove_file(name) {
        _ => (),
    };
}
