use std::{error::Error, mem::size_of, path::PathBuf, process::Command, ptr, thread};

use winapi::ctypes::{c_int, c_void};
use winapi::um::{combaseapi::CoInitializeEx, combaseapi::CoUninitialize, shellapi::*};

pub fn open(file_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let operation = String::new(); // default operation is open
    call_shell_execute_ex_a(file_path, operation)
}

pub fn open_with(file_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let operation = String::from("openas");
    call_shell_execute_ex_a(file_path, operation)
}

pub fn show_in_folder(file_path: PathBuf) -> Result<(), Box<dyn Error>> {
    Command::new("explorer")
        .arg("/select,")
        .arg(file_path.canonicalize()?)
        .output()
        .map(|_| ())
        .map_err(|e| Box::new(e) as Box<dyn Error>)
}

pub fn show_properties(file_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let operation = String::from("properties");
    call_shell_execute_ex_a(file_path, operation)
}

fn vec_decomp(vec: Vec<u8>) -> Vec<i8> {
    let mut vec = std::mem::ManuallyDrop::new(vec);
    let ptr = vec.as_mut_ptr();
    let len = vec.len();
    let cap = vec.capacity();
    unsafe { Vec::from_raw_parts(ptr as *mut i8, len, cap) }
}

fn call_shell_execute_ex_a(file_path: PathBuf, operation: String) -> Result<(), Box<dyn Error>> {
    let handler = thread::spawn(move || {
        let mut path: Vec<u8> = file_path
            .canonicalize()
            .unwrap()
            .to_str()
            .unwrap()
            .as_bytes()
            .to_vec();
        path.push(0);
        let new_path = vec_decomp(path);

        let mut operation_vec: Vec<i8> = vec_decomp(operation.as_bytes().to_vec());
        operation_vec.push(0);

        let _hresult = unsafe { CoInitializeEx(ptr::null_mut(), 0x2 | 0x4) };
        let _hresult = unsafe {
            let mut info: SHELLEXECUTEINFOA = SHELLEXECUTEINFOA {
                cbSize: size_of::<SHELLEXECUTEINFOA>() as u32,
                fMask: SEE_MASK_INVOKEIDLIST | SEE_MASK_NOCLOSEPROCESS | SEE_MASK_NOASYNC,
                hwnd: ptr::null_mut(),
                lpVerb: operation_vec.as_ptr(),
                lpFile: new_path.as_ptr(),
                lpParameters: ptr::null(),
                lpDirectory: ptr::null(),
                nShow: 1 as c_int,
                hInstApp: ptr::null_mut(),
                lpIDList: std::ptr::null_mut::<c_void>(),
                lpClass: ptr::null(),
                hkeyClass: ptr::null_mut(),
                dwHotKey: 0,
                hMonitor: ptr::null_mut(),
                hProcess: ptr::null_mut(),
            };
            ShellExecuteExA(&mut info as *mut SHELLEXECUTEINFOA)
        };
        std::thread::sleep(std::time::Duration::from_millis(10));
        unsafe { CoUninitialize() };
    });

    handler.join().unwrap();

    // Everything happens in another thread, so there's no returning any errors
    Ok(())
}
