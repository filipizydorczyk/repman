use clap::{App, Arg};

pub const COMMAND: &str = "list";

pub fn create_command() -> App<'static> {
    let app = App::new(COMMAND).arg(Arg::new("path").required(true));
    app
}
