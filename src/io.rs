
use std::io::{self};

pub fn get_line() -> String {
    let mut the_input = String::new();
    match io::stdin().read_line(&mut the_input) {
        Ok(_n) => {
            if the_input.ends_with("\n") {
                the_input.pop();
            }
            if the_input.ends_with("\r") {
                the_input.pop();
            }
        },
        Err(_error) => {
            the_input = "".to_owned();
        }
    }

    the_input
}