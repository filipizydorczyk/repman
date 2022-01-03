use clap::{App, Arg, ArgMatches};
use pbr::ProgressBar;
use std::process::Stdio;

use crate::commands::BUILD_COMMAND_NAME;
use crate::directorystorage;

const COMMAND_DESCRIPTION: &str = "Builds project from source files in repository. First u need to provide instructions by using repman edit <repository> command.";
const SUB_COMMAND_DESRIPTION: &str = "Name of repository to build. You can provide multiple names. If no name is provided all repos will be considered.";

pub const COMMAND: &str = "build";
pub const SUB_COMMAND_PATH: &str = "name";

pub fn create_command() -> App<'static> {
    let app = App::new(COMMAND).about(COMMAND_DESCRIPTION).arg(
        Arg::new(SUB_COMMAND_PATH)
            .help(SUB_COMMAND_DESRIPTION)
            .required(false)
            .min_values(0),
    );
    app
}

pub fn command_handler(matches: &ArgMatches) {
    let args = matches.values_of(SUB_COMMAND_PATH);
    let repos: Vec<String> = match args {
        None => directorystorage::get_stored_repositories_names(),
        _ => args
            .unwrap()
            .map(|element| -> String { String::from(element) })
            .collect(),
    };

    let mut pb = ProgressBar::new(repos.len() as u64);
    repos.iter().for_each(|repo| {
        pb.set(0);
        std::process::Command::new("bash")
            .arg("-c")
            .arg(format!(
                "source {}/{}.sh && {}",
                directorystorage::get_storage_path().to_str().unwrap(),
                repo,
                BUILD_COMMAND_NAME
            ))
            .stdout(Stdio::inherit())
            .output()
            .expect("Build failed");

        pb.inc();
    });
}
