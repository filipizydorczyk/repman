use clap::{App, Arg, ArgMatches};

use crate::directorystorage;

const COMMAND_DESCRIPTION: &str = "Opens config file for given repository in default editor. U need to have variable EDITOR set so that this command worked. U can just edit file in ~/.repman instead.";
const SUB_COMMAND_DESRIPTION: &str = "Name of repository to open config file.";

pub const COMMAND: &str = "edit";
pub const SUB_COMMAND_PATH: &str = "name";

pub fn create_command() -> App<'static> {
    let app = App::new(COMMAND).about(COMMAND_DESCRIPTION).arg(
        Arg::new(SUB_COMMAND_PATH)
            .about(SUB_COMMAND_DESRIPTION)
            .required(true),
    );
    app
}

pub fn command_handler(matches: &ArgMatches) {
    let editor = std::env::var("EDITOR").unwrap_or(String::from("nano"));

    let process = std::process::Command::new(editor)
        .arg(format!(
            "{}/{}.sh",
            directorystorage::get_storage_path().to_str().unwrap(),
            matches.value_of(SUB_COMMAND_PATH).unwrap()
        ))
        .spawn();

    match process.unwrap().wait_with_output() {
        _ => (),
    }
}
