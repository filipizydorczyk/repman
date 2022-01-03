use clap::{App, Arg, ArgMatches};
use colored::*;
use std::process::Stdio;

use crate::commands::{CHECK_COMMAND_NAME, CHECK_COUNT_COMMAND_NAME};
use crate::directorystorage;

const COMMAND_DESCRIPTION: &str = "Checks if there is any updates to pull.";
const SUB_COMMAND_DESRIPTION: &str = "Name of repository to check. You can provide multiple repositories. If no repository is provided all repos will be considered.";
const COUNT_FLAG_DESRIPTION: &str =
    "If this flag is used command will display ammount of commits instead of commit messages.";

pub const COMMAND: &str = "check";
const COUNT_FLAG: &str = "count";
pub const SUB_COMMAND_PATH: &str = "name";

pub fn create_command() -> App<'static> {
    let app = App::new(COMMAND)
        .about(COMMAND_DESCRIPTION)
        .arg(
            Arg::new(COUNT_FLAG)
                .about(COUNT_FLAG_DESRIPTION)
                .long(COUNT_FLAG)
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::new(SUB_COMMAND_PATH)
                .about(SUB_COMMAND_DESRIPTION)
                .required(false)
                .min_values(0),
        );
    app
}

pub fn command_handler(matches: &ArgMatches) {
    let args = matches.values_of(SUB_COMMAND_PATH);
    let is_count_enabled = matches.is_present(COUNT_FLAG);

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

        if is_count_enabled {
            println!("\n{} ", header);
            std::process::Command::new("bash")
                .arg("-c")
                .arg(format!(
                    "source {}/{}.sh && {}",
                    directorystorage::get_storage_path().to_str().unwrap(),
                    repo_name,
                    CHECK_COUNT_COMMAND_NAME
                ))
                .stdout(Stdio::inherit())
                .output()
                .expect("Failed to execute updateCheck function");
        } else {
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
        }
    });
}
