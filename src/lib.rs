use std::{error::Error, fs::File, os::unix::prelude::AsRawFd, path::PathBuf, time::Duration};

mod xdg_desktop_portal_openuri;
use dbus::{
    arg::{OwnedFd, PropMap},
    blocking::Connection,
};
use xdg_desktop_portal_openuri::*;

pub fn open(file_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let dbus_connection = Connection::new_session()?;
    let xdg_desktop_portal_proxy = dbus_connection.with_proxy(
        "org.freedesktop.portal.Desktop",
        "/org/freedesktop/portal/desktop",
        Duration::from_millis(2000),
    );

    unsafe {
        let file = File::open(file_path)?;
        let file_path = OwnedFd::new(file.as_raw_fd());

        let mut properties = PropMap::default();
        properties.insert(String::from("ask"), dbus::arg::Variant(Box::new(&false)));

        xdg_desktop_portal_proxy
            .open_file("", file_path, properties)
            .map(|_| ())
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}

pub fn open_with(file_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let dbus_connection = Connection::new_session()?;
    let xdg_desktop_portal_proxy = dbus_connection.with_proxy(
        "org.freedesktop.portal.Desktop",
        "/org/freedesktop/portal/desktop",
        Duration::from_millis(2000),
    );

    unsafe {
        let file = File::open(file_path)?;
        let file_path = OwnedFd::new(file.as_raw_fd());

        let mut properties = PropMap::default();
        properties.insert(String::from("ask"), dbus::arg::Variant(Box::new(&true)));

        xdg_desktop_portal_proxy
            .open_file("", file_path, properties)
            .map(|_| ())
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}

pub fn open_folder(file_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let dbus_connection = Connection::new_session()?;
    let xdg_desktop_portal_proxy = dbus_connection.with_proxy(
        "org.freedesktop.portal.Desktop",
        "/org/freedesktop/portal/desktop",
        Duration::from_millis(2000),
    );

    unsafe {
        let file = File::open(file_path)?;
        let file_path = OwnedFd::new(file.as_raw_fd());

        let properties = PropMap::default();

        xdg_desktop_portal_proxy
            .open_directory("", file_path, properties)
            .map(|_| ())
            .map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open() {
        let file_path = PathBuf::from("src/lib.rs");
        let _result = open(&file_path).unwrap();
    }
    #[test]
    fn test_open_with() {
        let file_path = PathBuf::from("src/lib.rs");
        let _result = open_with(&file_path).unwrap();
    }

    #[test]
    fn test_open_folder() {
        let file_path = PathBuf::from("src/lib.rs");
        let _result = open_folder(&file_path).unwrap();
    }
}
