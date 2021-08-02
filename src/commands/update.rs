use clap::{App, Arg, ArgMatches};

use crate::directorystorage;

const COMMAND_DESCRIPTION: &str = "Pulls changes from origin repository.";
const SUB_COMMAND_DESRIPTION: &str = "Name of repository to update.";

pub const COMMAND: &str = "update";
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
    let process = std::process::Command::new("bash")
        .arg("-c")
        .arg(format!(
            "source {}/{}.sh && updatePull",
            directorystorage::get_storage_path().to_str().unwrap(),
            matches.value_of(SUB_COMMAND_PATH).unwrap()
        ))
        .output();

    println!("{}", String::from_utf8_lossy(&process.unwrap().stdout))
}
