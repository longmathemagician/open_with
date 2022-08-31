use std::{error::Error, path::PathBuf, process::Command};
mod payloads;

pub fn open(file_path: PathBuf) -> Result<(), Box<dyn Error>> {
    Command::new("open")
        .arg(file_path)
        .output()
        .map(|_| ())
        .map_err(|e| Box::new(e) as Box<dyn Error>)
}

pub fn open_with(file_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let path = file_path.to_str().expect("Failed to parse string");

    Command::new("osascript")
        .arg("-e")
        .arg(format!("set file_path to POSIX file \"{path}\""))
        .arg("-e")
        .arg(payloads::OPEN_WITH)
        .output()
        .map(|_| ())
        .map_err(|e| Box::new(e) as Box<dyn Error>)
}

pub fn show_in_folder(file_path: PathBuf) -> Result<(), Box<dyn Error>> {
    Command::new("open")
        .arg("-R")
        .arg(file_path)
        .output()
        .map(|_| ())
        .map_err(|e| Box::new(e) as Box<dyn Error>)
}

pub fn show_properties(file_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let path = file_path.to_str().expect("Failed to parse string");

    Command::new("osascript")
        .arg("-e")
        .arg(format!("set file_path to POSIX file \"{path}\""))
        .arg("-e")
        .arg(payloads::GET_INFO)
        .output()
        .map(|_| ())
        .map_err(|e| Box::new(e) as Box<dyn Error>)
}
