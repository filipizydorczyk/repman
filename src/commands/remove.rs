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
            .required(true)
            .min_values(1),
    );
    app
}

pub fn command_handler(matches: &ArgMatches) {
    let args = matches.values_of(SUB_COMMAND_PATH);

    let repos: Vec<String> = match args {
        None => Vec::new(),
        _ => args
            .unwrap()
            .map(|element| -> String { String::from(element) })
            .collect(),
    };

    repos.iter().for_each(|repo| {
        match directorystorage::remove_file(repo.to_owned()) {
            Ok(_) => println!("{} removed", repo),
            Err(_) => println!("Adding {}", "XD"),
        };
    });
}
