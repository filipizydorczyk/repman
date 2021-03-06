use clap::App;

mod commands;
mod config;
mod directorystorage;
mod utilities;

fn main() {
    directorystorage::init_storage();

    let matches = App::new(config::NAME)
        .version(config::VER)
        .author(config::AUTHOR)
        .about(config::DESCRIPTION)
        .subcommand(commands::list::create_command())
        .subcommand(commands::add::create_command())
        .subcommand(commands::remove::create_command())
        .subcommand(commands::edit::create_command())
        .subcommand(commands::check::create_command())
        .subcommand(commands::update::create_command())
        .subcommand(commands::build::create_command())
        .subcommand(commands::install::create_command())
        .get_matches();

    match matches.subcommand() {
        Some((commands::list::COMMAND, _)) => commands::list::command_handler(),
        Some((commands::add::COMMAND, matches)) => commands::add::command_handler(matches),
        Some((commands::remove::COMMAND, matches)) => commands::remove::command_handler(matches),
        Some((commands::edit::COMMAND, matches)) => commands::edit::command_handler(matches),
        Some((commands::check::COMMAND, matches)) => commands::check::command_handler(matches),
        Some((commands::update::COMMAND, matches)) => commands::update::command_handler(matches),
        Some((commands::build::COMMAND, matches)) => commands::build::command_handler(matches),
        Some((commands::install::COMMAND, matches)) => commands::install::command_handler(matches),
        None => println!("No subcommand was used"),
        _ => unreachable!(),
    }
}
