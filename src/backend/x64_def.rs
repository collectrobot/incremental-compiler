// the abstract syntax of x64 assembly
// as defined in the book, figure 2.8, page 29

use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Copy)]
pub enum Reg {
    Rsp, Rbp, Rax, Rbx,
    Rcx, Rdx, Rsi, Rdi,
    R8, R9, R10, R11,
    R12, R13, R14, R15
}

#[derive(Clone)]
pub enum Arg {
    Var(Rc<String>), // for the first pass where variables are still present
    Imm(i64),
    Reg(Reg),
    Deref(Reg, i64),
}

#[derive(Clone)]
pub enum VarLoc {
    // a variable can live in either
    Reg(Reg), // a register or
    Imm(i64), // an offset from rbp
}

#[derive(Clone)]
pub enum Instr {
    Add64(Arg, Arg),
    Sub64(Arg, Arg),
    Mov64(Arg, Arg),
    Neg64(Arg),
    Call(Rc<String>, i64),
    Ret,
    Push(Arg),
    Pop(Arg),
    Jmp(Rc<String>),
}

pub struct Home {
    name: Rc<String>,
    loc: VarLoc,
}

pub struct Block {
    info: Vec<()>,
    instr: Vec<Instr>,
}

pub struct X64Program {
    homes: Vec<Home>,
    blocks: HashMap<Rc<String>, Block>,
}
