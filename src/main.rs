use clap::App;
use std::path::Path;

mod commands;
mod config;

fn main() {
    let matches = App::new(config::NAME)
        .version(config::VER)
        .author(config::AUTHOR)
        .about(config::DESCRIPTION)
        .subcommand(commands::list::create_command())
        .get_matches();

    match matches.subcommand() {
        Some((commands::list::COMMAND, clone_matches)) => {
            // Now we have a reference to clone's matches
            let path = Path::new(clone_matches.value_of("path").unwrap());
            println!("Cloning {}", path.canonicalize().unwrap().display());
        }
        None => println!("No subcommand was used"),
        _ => unreachable!(),
    }

    // Same as previous example...
}
