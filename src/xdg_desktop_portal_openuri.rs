// This code was autogenerated with `dbus-codegen-rust `, see https://github.com/diwic/dbus-rs
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopPortalOpenURI {
    fn open_uri(
        &self,
        parent_window: &str,
        uri: &str,
        options: arg::PropMap,
    ) -> Result<dbus::Path<'static>, dbus::Error>;
    fn open_file(
        &self,
        parent_window: &str,
        fd: arg::OwnedFd,
        options: arg::PropMap,
    ) -> Result<dbus::Path<'static>, dbus::Error>;
    fn open_directory(
        &self,
        parent_window: &str,
        fd: arg::OwnedFd,
        options: arg::PropMap,
    ) -> Result<dbus::Path<'static>, dbus::Error>;
    fn version(&self) -> Result<u32, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>> OrgFreedesktopPortalOpenURI
    for blocking::Proxy<'a, C>
{
    fn open_uri(
        &self,
        parent_window: &str,
        uri: &str,
        options: arg::PropMap,
    ) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call(
            "org.freedesktop.portal.OpenURI",
            "OpenURI",
            (parent_window, uri, options),
        )
        .map(|r: (dbus::Path<'static>,)| r.0)
    }

    fn open_file(
        &self,
        parent_window: &str,
        fd: arg::OwnedFd,
        options: arg::PropMap,
    ) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call(
            "org.freedesktop.portal.OpenURI",
            "OpenFile",
            (parent_window, fd, options),
        )
        .map(|r: (dbus::Path<'static>,)| r.0)
    }

    fn open_directory(
        &self,
        parent_window: &str,
        fd: arg::OwnedFd,
        options: arg::PropMap,
    ) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call(
            "org.freedesktop.portal.OpenURI",
            "OpenDirectory",
            (parent_window, fd, options),
        )
        .map(|r: (dbus::Path<'static>,)| r.0)
    }

    fn version(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            self,
            "org.freedesktop.portal.OpenURI",
            "version",
        )
    }
}
