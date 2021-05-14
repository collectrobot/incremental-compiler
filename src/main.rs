mod repl;
mod lexer;
mod token;
mod ast;
mod parser;
mod io;
mod interp;
mod uniquify;
mod decomplify;
mod explicate;

#[macro_use]
mod utility;

use repl::{Repl};

fn main() {

    let mut repl = Repl::new();

    let _ = repl.run();
}