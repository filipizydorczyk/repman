extern crate dirs;

use clap::App;
use std::fs;

pub const COMMAND: &str = "list";

use crate::directorystorage;

pub fn create_command() -> App<'static> {
    let app = App::new(COMMAND).about("Lists repository that was added to track with repman.");
    app
}

pub fn command_handler() {
    let paths = fs::read_dir(directorystorage::get_storage_path()).unwrap();

    for path in paths {
        println!("{}", path.unwrap().path().display())
    }
}
