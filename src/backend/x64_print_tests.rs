use std::rc::Rc;
use std::collections::HashMap;
use std::collections::HashSet;

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
        )
        .use_runtime(false)
        .transform();

    let asm_text = X64Printer::new(x64_asm).print();

    let expect_print =
"global start

section .text

start:
    mov rax, 2
    ret
".to_owned();

    assert_eq!(asm_text, expect_print);

}

#[test]
fn x64_print_addition() {
    let ast = 
    Parser::new(
        Lexer::new("(+ 2 2)")
        .lex())
    .parse(); 

    let x64_asm =
        IRToX64Transformer::new(
            explicate_control(
                decomplify_program(uniquify_program(ast))
            )
        )
        .use_runtime(false)
        .transform();

    let asm_text = X64Printer::new(x64_asm).print();

    let expect_print =
"global start

section .text

start:
    mov rax, 4
    ret
".to_owned();

    assert_eq!(asm_text, expect_print);

}

#[test]
fn x64_print_negate_then_add() {
    let ast = 
    Parser::new(
        Lexer::new("(+ (- 10) 42)")
        .lex())
    .parse(); 

    let x64_asm =
        IRToX64Transformer::new(
            explicate_control(
                decomplify_program(uniquify_program(ast))
            )
        )
        .use_runtime(false)
        .transform();

    let asm_text = X64Printer::new(x64_asm).print();

    let expect_print =
"global start

section .text

start:
    push rbp
    mov rbp, rsp
    sub rsp, 8
    mov qword [rbp-8], 10
    neg qword [rbp-8]
    mov rax, qword [rbp-8]
    add rax, 42
    mov rsp, rbp
    pop rbp
    ret
".to_owned();

    assert_eq!(asm_text, expect_print);

}