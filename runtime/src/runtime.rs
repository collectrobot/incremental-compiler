extern crate libc;
extern crate static_vcruntime;
extern crate datatypes;

use libc::{c_char};
use std::ffi::CStr;
use std::io;

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
pub extern "C" fn read_int() -> datatypes::RuntimeI64 {
    let mut input = String::new();

    let the_int: datatypes::RuntimeI64;

    'retry_int: loop {
        let result = io::stdin().read_line(&mut input);
        match result {
            Ok(_) => {
                let maybe_int = input.trim().parse::<datatypes::RuntimeI64>();
                match maybe_int {
                    Ok(x) => {
                        the_int = x;
                        break 'retry_int;
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }

    the_int
}

#[no_mangle]
pub extern "C" fn print_int(int: datatypes::RuntimeI64) {
    let printee = int.to_string();
    println!("{}", printee);
}

#[no_mangle]
pub extern "C" fn println(string: *const c_char) {
    let c_buf = string;

    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };

    let str_slice: &str = c_str.to_str().unwrap();

    let str_buf: String = str_slice.to_owned();

    println!("{}", str_buf);
}
