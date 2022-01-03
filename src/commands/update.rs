use clap::{App, Arg, ArgMatches};
use colored::*;
use std::process::Stdio;

use crate::commands::PULL_COMMAND_NAME;
use crate::directorystorage;

const COMMAND_DESCRIPTION: &str = "Pulls changes from origin repository.";
const SUB_COMMAND_DESRIPTION: &str =
    "Names of repositories to update. All repos will be updated if there is no name provided.";

pub const COMMAND: &str = "update";
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

    repos.iter().for_each(|repo| {
        let header = format!("{} {}", "Pulling changes for".bold(), repo.yellow());

        println!("\n{}", header);
        println!("{}", "=".repeat(header.chars().count() - 17));
        std::process::Command::new("bash")
            .arg("-c")
            .arg(format!(
                "source {}/{}.sh && {}",
                directorystorage::get_storage_path().to_str().unwrap(),
                repo,
                PULL_COMMAND_NAME
            ))
            .stdout(Stdio::inherit())
            .output()
            .expect("Update failed");
    });
}
