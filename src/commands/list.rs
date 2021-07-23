use clap::App;
use std::fs;

pub const COMMAND: &str = "list";

pub fn create_command() -> App<'static> {
    let app = App::new(COMMAND).about("Lists repository that was added to track with repman.");
    app
}

pub fn command_handler() {
    let paths = fs::read_dir(".").unwrap();

    for path in paths {
        println!("{}", path.unwrap().path().display())
    }
}
