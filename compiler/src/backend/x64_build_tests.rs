#![allow(unused)]

use crate::frontend::lexer::{Lexer};
use crate::frontend::parser::{Parser};
use crate::frontend::uniquify::{uniquify_program};
use crate::frontend::decomplify::{decomplify_program};
use crate::ir::explicate::{explicate_control};

use super::x64_def::{X64Program};
use super::x64_print::{X64Printer};
use super::x64_build::{X64Builder};

use crate::utility::{test_x64_helper};

fn helper(prog: &'static str) -> X64Program {
    test_x64_helper(prog)
}

#[test]
fn x64_build_constant() {
    let x64_asm = helper("(2)");

    let asm_text = X64Printer::new(x64_asm).print();
    
    let builder = X64Builder::new(crate::function!().to_owned(), asm_text);
    builder.build();
}

#[test]
fn x64_build_add_two_read() {
    let x64_asm = helper("(+ (read) (read)");

    let asm_text = X64Printer::new(x64_asm).print();
    
    let builder = X64Builder::new(crate::function!().to_string(), asm_text);
    builder.build();
}

#[test]
fn x64_build_two_let_read() {
    let x64_asm = helper(
        "(let ([x (read)]) \
            (let ([y (read)]) \
                (+ (+ x y) 42)))"
            );
    
    let asm_text = X64Printer::new(x64_asm).print();

    let builder = X64Builder::new(crate::function!().to_string(), asm_text);
    builder.build();
}