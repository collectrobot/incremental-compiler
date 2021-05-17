// the abstract syntax of x64 assembly
// as defined in the book, figure 2.8, page 29

use std::collections::HashMap;

pub enum Reg {
    Rsp, Rbp, Rax, Rbx,
    Rcx, Rdx, Rsi, Rdi,
    R8, R9, R10, R11,
    R12, R13, R14, R15
}

pub enum Arg<'a> {
    Var(&'a str), // for the first pass where variables are still present
    Imm(i64),
    Reg(Reg),
    Deref(Reg, i64),
}

pub enum Instr<'a> {
    Add64(Arg<'a>, Arg<'a>),
    Sub64(Arg<'a>, Arg<'a>),
    Mov64(Arg<'a>, Arg<'a>),
    Neg64(Arg<'a>),
    Call(&'a str, i64),
    Ret,
    Push(Arg<'a>),
    Pop(Arg<'a>),
    Jmp(&'a str),
}

pub struct Block<'a> {
    info: Vec<()>,
    instr: Vec<Instr<'a>>,
}

pub struct X64Program<'a> {
    info: Vec<()>,
    blocks: HashMap<&'a str, Block<'a>>,
}
