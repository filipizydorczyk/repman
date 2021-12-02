use clap::{App, Arg, ArgMatches};
use colored::*;
use std::process::Stdio;

use crate::commands::CHECK_COMMAND_NAME;
use crate::directorystorage;

const COMMAND_DESCRIPTION: &str = "Checks if there is any updates to pull.";
const SUB_COMMAND_DESRIPTION: &str = "Name of repository to check.";

pub const COMMAND: &str = "check";
pub const SUB_COMMAND_PATH: &str = "name";

pub fn create_command() -> App<'static> {
    let app = App::new(COMMAND).about(COMMAND_DESCRIPTION).arg(
        Arg::new(SUB_COMMAND_PATH)
            .about(SUB_COMMAND_DESRIPTION)
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
        let repo_name = repo;
        let header = format!("{} {}", "Changes for".bold(), repo_name.yellow());

        println!("\n{}", header);
        println!("{}", "=".repeat(header.chars().count() - 17));

        std::process::Command::new("bash")
            .arg("-c")
            .arg(format!(
                "source {}/{}.sh && {}",
                directorystorage::get_storage_path().to_str().unwrap(),
                repo_name,
                CHECK_COMMAND_NAME
            ))
            .stdout(Stdio::inherit())
            .output()
            .expect("Failed to execute updateCheck function");
    });
}
