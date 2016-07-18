//! An auto-generated wrapper around the [Duktape][1] library.
//!
//! The API of this wrapper is not stable, and currently exposes
//! transient library APIs.
//!
//! [1]: http://duktape.org/
#![allow(non_camel_case_types, non_snake_case)]

extern crate libc;
#[cfg(any(feature = "debug", feature = "trace", feature = "spam"))]
#[macro_use]
extern crate log;

mod ffi;

pub use ffi::*;

#[cfg(any(feature = "debug", feature = "trace", feature = "spam"))]
#[no_mangle]
pub unsafe extern "C" fn __duktape_sys_debug_write(
    level: libc::c_long,
    file: *const libc::c_char,
    line: libc::c_long,
    func: *const libc::c_char,
    msg: *const libc::c_char) {

    let log_level = if level == DUK_LEVEL_DEBUG {
        log::LogLevel::Debug
    } else {
        log::LogLevel::Trace
    };

    if log_enabled!(log_level) {
        let file_str = ::std::ffi::CStr::from_ptr(file).to_string_lossy();

        let target = format!("{}:{}", module_path!(), file_str);
        if log_enabled!(target: &target, log_level) {
            let func_str = ::std::ffi::CStr::from_ptr(func).to_string_lossy();
            let msg_str = ::std::ffi::CStr::from_ptr(msg).to_string_lossy();
            log!(target: &target, log_level, "{}.{}:{}: {}", file_str, func_str, line, msg_str);
        }
    }
}
