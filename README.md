# A simple library for open-with dialog invocation

Given a file, this library provides a simple way to either open with system defaults, invoke the open-with dialog, or show the properties dialog.

Supports Windows through shell-execute, Linux/systemd through xdg-desktop-portal, and macOS through finder automation.

The macOS implementation requires escalated privileges on recent OS versions as is occasionally unreliable for reasons I've been unable to ascertain. If anyone has a better solution for this platform a PR would be greatly appreciated.

## Example
```rust
let file_path = PathBuf::from("src/lib.rs");
let _result = open_with(file_path);
```