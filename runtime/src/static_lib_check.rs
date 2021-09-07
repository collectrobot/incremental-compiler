// this file is only used to get the static libraries we need to link against
// -> rustc --crate-type=staticlib --print=native-static-libs .\static_lib_check.rs
// created to avoid rustc complaining about crates not found (i.e. no dependencies)

use std::io;

#[no_mangle]
pub extern "C" fn print(string: String) -> () {
    println!("{}", string);
}

#[no_mangle]
pub extern "C" fn read_int() -> i64 {
    let mut input = String::new();

    let the_int: i64;

    'retry_int: loop {
        let result = io::stdin().read_line(&mut input);
        match result {
            Ok(_) => {
                let maybe_int = input.trim().parse::<i64>();
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