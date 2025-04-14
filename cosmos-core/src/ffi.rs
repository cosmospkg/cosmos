use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Mutex;
use std::ptr;

// A thread-safe, mutable global to hold the last error message
static LAST_ERROR: Mutex<Option<CString>> = Mutex::new(None);

fn set_last_error(msg: &str) {
    let mut last = LAST_ERROR.lock().unwrap();
    *last = Some(CString::new(msg).unwrap_or_else(|_| CString::new("Unknown error").unwrap()));
}

#[no_mangle]
pub extern "C" fn cosmos_install(star_name: *const c_char) -> i32 {
    if star_name.is_null() {
        set_last_error("star_name was null");
        return -1;
    }

    let c_str = unsafe { CStr::from_ptr(star_name) };
    match c_str.to_str() {
        Ok(name) => {
            // TODO: Real install logic
            println!("Installing star: {}", name);
            0
        }
        Err(_) => {
            set_last_error("Invalid UTF-8 in star name");
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn cosmos_uninstall(star_name: *const c_char) -> i32 {
    if star_name.is_null() {
        set_last_error("star_name was null");
        return -1;
    }

    let c_str = unsafe { CStr::from_ptr(star_name) };
    match c_str.to_str() {
        Ok(name) => {
            println!("Uninstalling star: {}", name);
            0
        }
        Err(_) => {
            set_last_error("Invalid UTF-8 in star name");
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn cosmos_update(star_name: *const c_char) -> i32 {
    if star_name.is_null() {
        set_last_error("star_name was null");
        return -1;
    }

    let c_str = unsafe { CStr::from_ptr(star_name) };
    match c_str.to_str() {
        Ok(name) => {
            println!("Updating star: {}", name);
            0
        }
        Err(_) => {
            set_last_error("Invalid UTF-8 in star name");
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn cosmos_last_error() -> *const c_char {
    let last = LAST_ERROR.lock().unwrap();
    match &*last {
        Some(cstr) => cstr.as_ptr(),
        None => ptr::null(),
    }
}
