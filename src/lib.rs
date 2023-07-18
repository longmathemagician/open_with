use std::error::Error;
use std::path::PathBuf;

#[cfg(target_os = "linux")]
mod lib_linux;
#[cfg(target_os = "linux")]
use lib_linux as backend;

#[cfg(target_os = "macos")]
mod lib_macos;
#[cfg(target_os = "macos")]
use lib_macos as backend;

#[cfg(target_os = "windows")]
mod lib_windows;
#[cfg(target_os = "windows")]
use lib_windows as backend;

/// Opens the file at the given path using the default application.
///
/// # Arguments
///
/// * `file_path` - A `PathBuf` that holds the path to the file to be opened.
///
/// # Errors
///
/// Returns an error if the file cannot be opened.
pub fn open(file_path: PathBuf) -> Result<(), Box<dyn Error>> {
    backend::open(file_path)
}

/// Invokes the platform's "Open With" dialog for the file at the provided path.
///
/// # Arguments
///
/// * `file_path` - A `PathBuf` that holds the path to the file to be opened.
///
/// # Errors
///
/// Returns an error if the file cannot be opened.
pub fn open_with(file_path: PathBuf) -> Result<(), Box<dyn Error>> {
    backend::open_with(file_path)
}

/// Shows the folder containing the file at the given path. Where supported, the file is selected.
///
/// # Arguments
///
/// * `file_path` - A `PathBuf` that holds the path to the file.
///
/// # Errors
///
/// Returns an error if the folder cannot be shown.
pub fn show_in_folder(file_path: PathBuf) -> Result<(), Box<dyn Error>> {
    backend::show_in_folder(file_path)
}

/// Uses the default file manager to show properties of the file at the given path.
///
/// # Arguments
///
/// * `file_path` - A `PathBuf` that holds the path to the file.
///
/// # Errors
///
/// Returns an error if the properties cannot be shown.
pub fn show_properties(file_path: PathBuf) -> Result<(), Box<dyn Error>> {
    backend::show_properties(file_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_open() {
        let file_path = PathBuf::from("src/lib.rs");
        let _result = open(file_path).unwrap();
    }

    #[test]
    fn test_open_with() {
        let file_path = PathBuf::from("src/lib.rs");
        let _result = open_with(file_path).unwrap();
    }

    #[test]
    fn test_open_folder() {
        let file_path = PathBuf::from("src/lib.rs");
        let _result = show_in_folder(file_path).unwrap();
    }

    #[test]
    fn test_show_properties() {
        let file_path = PathBuf::from("src/lib.rs");
        let _result = show_properties(file_path).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10000));
    }
}
