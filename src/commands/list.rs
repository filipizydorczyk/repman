extern crate dirs;

use clap::App;
use std::fs;

pub const COMMAND: &str = "list";

#[cfg(not(debug_assertions))]
use crate::config::STORAGE_DIR;

pub fn create_command() -> App<'static> {
    let app = App::new(COMMAND).about("Lists repository that was added to track with repman.");
    app
}

pub fn command_handler() {
    #[cfg(debug_assertions)]
    let paths = fs::read_dir(".").unwrap();

    #[cfg(not(debug_assertions))]
    let paths = fs::read_dir(dirs::home_dir().unwrap().join(STORAGE_DIR)).unwrap();

    for path in paths {
        println!("{}", path.unwrap().path().display())
    }
}
