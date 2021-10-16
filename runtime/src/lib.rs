
pub mod types;

use std::io;

use crate::types::{RuntimeI64};

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
pub extern "C" fn read_int() -> RuntimeI64 {
    let mut input = String::new();

    let the_int: RuntimeI64;

    'retry_int: loop {
        let result = io::stdin().read_line(&mut input);
        match result {
            Ok(_) => {
                let maybe_int = input.trim().parse::<RuntimeI64>();
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
pub extern "C" fn print_int(int: RuntimeI64) {
    let printee = int.to_string();
    println!("{}", printee);
}