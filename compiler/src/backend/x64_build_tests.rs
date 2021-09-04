#![allow(unused)]

use crate::frontend::lexer::{Lexer};
use crate::frontend::parser::{Parser};
use crate::frontend::uniquify::{uniquify_program};
use crate::frontend::decomplify::{decomplify_program};
use crate::ir::explicate::{explicate_control};

use super::x64_def::*;
use super::x64_backend::{IRToX64Transformer};
use super::x64_print::{X64Printer};
use super::x64_build::{X64Builder};

#[test]
fn x64_build_constant() {
    let ast = 
    Parser::new(
        Lexer::new("(2)")
        .lex())
    .parse(); 

    let x64_asm =
        IRToX64Transformer::new(
            explicate_control(
                decomplify_program(uniquify_program(ast))
            )
        )
        .use_runtime(true)
        .transform();

    let asm_text = X64Printer::new(x64_asm).print();
    
    let builder = X64Builder::new("test".to_owned(), asm_text);
    builder.build();
}

#[test]
fn x64_build_add_two_read() {
    let ast = 
    Parser::new(
        Lexer::new("(+ (read) (read)")
        .lex())
    .parse(); 

    let x64_asm =
        IRToX64Transformer::new(
            explicate_control(
                decomplify_program(uniquify_program(ast))
            )
        )
        .use_runtime(true)
        .transform();

    let asm_text = X64Printer::new(x64_asm).print();
    
    let builder = X64Builder::new("test".to_owned(), asm_text);
    builder.build();
}