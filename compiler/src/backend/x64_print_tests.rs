#![allow(unused)]

use std::rc::Rc;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::utility::{test_x64_helper};

use super::x64_def::*;
use super::x64_backend::{IRToX64Transformer};
use super::x64_print::{X64Printer};

fn helper(prog: &'static str) -> String {
    X64Printer::new(test_x64_helper(prog)).print()
}

#[test]
fn x64_print_constant() {
    let asm_text = helper("(2)"); 

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
    let asm_text = helper("(+ 2 2)");

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
    let asm_text = helper("(+ (- 10) 42)");

    let expect_print =
"global start

section .text

start:
    mov rax, 32
    ret
".to_owned();

    assert_eq!(asm_text, expect_print);

}

#[test]
fn x64_print_add_two_read() {
    let asm_text = helper("(+ (read) (read))");

    let expect_print =
"extern read_int

global start

section .text

start:
    push rbp
    mov rbp, rsp
    sub rsp, 16
    call read_int
    mov qword [rbp-8], rax
    call read_int
    mov qword [rbp-16], rax
    mov rax, qword [rbp-8]
    add rax, qword [rbp-16]
    mov rsp, rbp
    pop rbp
    ret
".to_owned();

    assert_eq!(asm_text, expect_print);

}

#[test]
fn x64_print_let_add() {
    let asm_text = helper("(let ([x (read)]) (+ x x)");

    let expect_print =
"extern read_int

global start

section .text

start:
    push rbp
    mov rbp, rsp
    sub rsp, 8
    call read_int
    mov qword [rbp-8], rax
    mov rax, qword [rbp-8]
    add rax, qword [rbp-8]
    mov rsp, rbp
    pop rbp
    ret
".to_owned();

    assert_eq!(asm_text, expect_print);
}