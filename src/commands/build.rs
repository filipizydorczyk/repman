use clap::{App, Arg, ArgMatches};
use pbr::ProgressBar;
use std::process::Stdio;

use crate::directorystorage;

const COMMAND_DESCRIPTION: &str = "Builds project from source files in repository. First u need to provide instructions by using repman edit <repository> command.";
const SUB_COMMAND_DESRIPTION: &str = "Name of repository to build.";

pub const COMMAND: &str = "build";
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
    let mut pb = ProgressBar::new(2);
    pb.inc();
    std::process::Command::new("bash")
        .arg("-c")
        .arg(format!(
            "source {}/{}.sh && updateBuild",
            directorystorage::get_storage_path().to_str().unwrap(),
            matches.value_of(SUB_COMMAND_PATH).unwrap()
        ))
        .stdout(Stdio::inherit())
        .output()
        .expect("Build failed");

    pb.inc();
}
