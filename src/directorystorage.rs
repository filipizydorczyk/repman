extern crate dirs;

use std::fs::File;
use std::fs::ReadDir;
use std::io::Write;
use std::path::PathBuf;

pub const STORAGE_DIR: &str = ".repman";

pub fn init_storage() {
    #[cfg(debug_assertions)]
    match std::fs::create_dir_all(format!("./target/{}", STORAGE_DIR)) {
        _ => (),
    }

    #[cfg(not(debug_assertions))]
    match std::fs::create_dir_all(dirs::home_dir().unwrap().join(STORAGE_DIR)) {
        _ => (),
    }
}

pub fn get_storage_path() -> PathBuf {
    #[cfg(debug_assertions)]
    let result = PathBuf::from(format!("./target/{}", STORAGE_DIR));

    #[cfg(not(debug_assertions))]
    let result = dirs::home_dir().unwrap().join(STORAGE_DIR);

    return result;
}

pub fn get_stored_repositories() -> std::io::Result<ReadDir> {
    let paths = std::fs::read_dir(get_storage_path());

    return paths;
}

pub fn save_to_file(content: String, name: String) -> std::io::Result<()> {
    let dir = get_storage_path();
    let path = format!("{}/{}.sh", dir.to_str().unwrap(), name);

    let mut output = File::create(path)?;
    write!(output, "{}", content)?;

    Ok(())
}
