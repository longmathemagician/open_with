#[cfg(target_os = "linux")]
pub mod lib_linux;

#[cfg(target_os = "linux")]
pub use lib_linux::*;

#[cfg(target_os = "macos")]
pub mod lib_macos;

#[cfg(target_os = "macos")]
pub use lib_macos::*;

#[cfg(target_os = "windows")]
pub mod lib_windows;

#[cfg(target_os = "windows")]
pub use lib_windows::*;

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
