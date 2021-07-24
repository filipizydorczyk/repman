pub const NAME: &str = "repman";
pub const AUTHOR: &str = "Filip Izydorczyk <filip.izydorczyk@protonmail.com>";
pub const VER: &str = "1.0";
pub const DESCRIPTION: &str = "Software to manage updates of softwares that I build from source.";

#[cfg(not(debug_assertions))]
pub const STORAGE_DIR: &str = ".repman";
