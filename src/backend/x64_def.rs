// the abstract syntax of x64 assembly
// as defined in the book, figure 2.8, page 29

use std::collections::HashMap;

pub enum Reg {
    Rsp, Rbp, Rax, Rbx,
    Rcx, Rdx, Rsi, Rdi,
    R8, R9, R10, R11,
    R12, R13, R14, R15
}

pub enum Arg {
    Imm(i64),
    Reg(Reg),
    Deref(Reg, i64),
}

pub enum Instr<'a> {
    Add64(Arg, Arg),
    Sub64(Arg, Arg),
    Mov64(Arg, Arg),
    Neg64(Arg),
    Call(&'a str, i64),
    Ret,
    Push(Arg),
    Pop(Arg),
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
