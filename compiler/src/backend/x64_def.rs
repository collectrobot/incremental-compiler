// the abstract syntax of x64 assembly
// as defined in the book, figure 2.8, page 29

#![allow(dead_code)]

use std::collections::HashMap;
use std::collections::HashSet;

use crate::types::{IdString};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Reg {
    Rsp, Rbp, Rax, Rbx,
    Rcx, Rdx, Rsi, Rdi,
    R8, R9, R10, R11,
    R12, R13, R14, R15
}

#[derive(Clone, Debug, PartialEq)]
pub enum Arg {
    Var(IdString), // for the first pass where variables are still present
    Imm(i64),
    Reg(Reg),
    Deref(Reg, i64),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum VarLoc {
    // a variable can live in either
    Reg(Reg), // a register or
    Rbp(i64), // an offset from rbp
    Undefined, // initial value
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Home {
    pub name: IdString,
    pub loc: VarLoc,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Instr {
    Add64(Arg, Arg),
    Sub64(Arg, Arg),
    Mov64(Arg, Arg),
    Neg64(Arg),
    Call(IdString, i64),
    Ret,
    Push(Arg),
    Pop(Arg),
    Jmp(IdString),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    pub info: (),
    pub instr: Vec<Instr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub blocks: HashMap<IdString, Block>,
    pub vars: Vec<Home>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct X64Program {
    pub external: HashSet<IdString>,
    //pub vars: Vec<Home>, // vars that have a defined home (stack or register)
    pub functions: HashMap<IdString, Function>,
}
