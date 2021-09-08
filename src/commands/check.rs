use clap::{App, Arg, ArgMatches};
use std::process::Stdio;

use crate::directorystorage;

const COMMAND_DESCRIPTION: &str = "Checks if there is any updates to pull.";
const SUB_COMMAND_DESRIPTION: &str = "Name of repository to check.";

pub const COMMAND: &str = "check";
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
    let output = std::process::Command::new("bash")
        .arg("-c")
        .arg(format!(
            "source {}/{}.sh && updateCheck",
            directorystorage::get_storage_path().to_str().unwrap(),
            matches.value_of(SUB_COMMAND_PATH).unwrap()
        ))
        .stdout(Stdio::inherit())
        .output()
        .expect("Failed to execute updateCheck function");

    let sout = String::from_utf8(output.stdout).expect("No utf8");

    print!("{}", sout);
}
