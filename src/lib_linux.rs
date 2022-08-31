use dbus::{
    arg::{OwnedFd, PropMap},
    blocking::Connection,
};
use std::{error::Error, fs::File, os::unix::prelude::AsRawFd, path::PathBuf, time::Duration};

mod org_freedesktop_portal_openuri;
use org_freedesktop_portal_openuri::*;
mod org_freedesktop_filemanager1;
use org_freedesktop_filemanager1::*;

pub fn open(file_path: PathBuf) -> Result<(), Box<dyn Error>> {
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

pub fn open_with(file_path: PathBuf) -> Result<(), Box<dyn Error>> {
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

pub fn show_in_folder(file_path: PathBuf) -> Result<(), Box<dyn Error>> {
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

pub fn show_properties(file_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let dbus_connection = Connection::new_session()?;
    let file_manager_proxy = dbus_connection.with_proxy(
        "org.freedesktop.FileManager1",
        "/org/freedesktop/FileManager1",
        Duration::from_millis(2000),
    );

    let mut file_path: String = file_path
        .into_os_string()
        .into_string()
        .map_err(|_e| "Failed to convert path to string")?;

    file_path.insert_str(0, "file://");

    file_manager_proxy
        .show_item_properties(Vec::from([&*file_path]), "")
        .map_err(|e| Box::new(e) as Box<dyn Error>)
}
