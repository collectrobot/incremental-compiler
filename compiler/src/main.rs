extern crate runtime;

mod repl;
mod io;
mod backend;
mod frontend;
mod interpreter;
mod ir;
mod types;

#[macro_use]
mod utility;

use repl::{Repl};

fn main() {

    let mut repl = Repl::new();

    let _ = repl.run();
}