pub const CHECK_COMMAND_NAME: &str = "update_check";
pub const PULL_COMMAND_NAME: &str = "update_pull";
pub const BUILD_COMMAND_NAME: &str = "update_build";
pub const INSTALL_COMMAND_NAME: &str = "build_install";
pub const CHECK_COUNT_COMMAND_NAME: &str = "update_pull_count";

pub mod add;
pub mod build;
pub mod check;
pub mod edit;
pub mod install;
pub mod list;
pub mod remove;
pub mod update;
