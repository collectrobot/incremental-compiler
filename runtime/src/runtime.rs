extern crate libc;
extern crate static_vcruntime;

use libc::{c_char, c_int};
use std::ffi::CStr;

extern "C" {
    fn _CRT_INIT() -> ();
    fn ExitProcess(exitcode: u32) -> ();
    fn start() -> u64;
}

#[no_mangle]
pub extern "C" fn __runtime_startup() {

    unsafe {
        _CRT_INIT();
        let exitcode = start();

        ExitProcess(exitcode as u32);
    }

}

#[no_mangle]
pub extern "C" fn println(string: *const c_char) {
    let c_buf = string;

    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };

    let str_slice: &str = c_str.to_str().unwrap();

    let str_buf: String = str_slice.to_owned();

    println!("{}", str_buf);
}