extern crate dirs;
use clap::App;

const COMMAND_DESCRIPTION: &str = "Lists repository that was added to track with repman.";

pub const COMMAND: &str = "list";

use crate::directorystorage;

pub fn create_command() -> App<'static> {
    let app = App::new(COMMAND).about(COMMAND_DESCRIPTION);
    app
}

pub fn command_handler() {
    let paths = directorystorage::get_stored_repositories().unwrap();

    for path in paths {
        println!("{}", path.unwrap().path().display())
    }
}
