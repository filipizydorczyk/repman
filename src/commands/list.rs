use clap::App;
use std::fs;

pub const COMMAND: &str = "list";

pub fn create_command() -> App<'static> {
    let app = App::new(COMMAND);
    app
}

pub fn command_handler() {
    let paths = fs::read_dir(".").unwrap();

    for path in paths {
        println!("{}", path.unwrap().path().display())
    }
}
