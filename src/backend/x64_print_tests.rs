use std::rc::Rc;
use std::collections::HashMap;
use std::collections::HashSet;

use std::env::{temp_dir};
use std::fs::{File};

use crate::frontend::lexer::{Lexer};
use crate::frontend::parser::{Parser};
use crate::frontend::uniquify::{uniquify_program};
use crate::frontend::decomplify::{decomplify_program};
use crate::ir::explicate::{explicate_control};

use super::x64_def::*;
use super::x64_backend::{IRToX64Transformer};
use super::x64_print::{X64Printer};

#[test]
fn x64_print_constant() {
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
        ).transform();

    
    let text_asm = 
        X64Printer::new(x64_asm).print();

    let expect_print =
"extern ExitProcess

global start

section .text

start:
    mov rax, 2
    call ExitProcess".to_owned();

    assert_eq!(text_asm, expect_print);
    
    let mut temp = temp_dir();

    temp.set_file_name("rust_test.asm");

    let result = File::create(temp);

    match result {
        Ok(_) => {},
        Err(error) => {
            panic!(error)
        }
    }
}