# A simple library for open-with dialog invocation

This library provides a simple multiplatform interface for opening a file with the system default application, invoking the platform's open-with dialog to open a file with one of several programs, revealing the file in the default file manager, or showing the file's properties through the default handler.

Supports Windows through shell-execute, Linux/systemd through xdg-desktop-portal, and macOS through finder automation.

The macOS implementation requires escalated privileges on recent OS versions and is occasionally unreliable for reasons I've been unable to ascertain. If anyone has a better solution for this platform a PR would be greatly appreciated.

## Example
```rust
let file_path = PathBuf::from("src/lib.rs");
let _result = open_with(file_path);
```