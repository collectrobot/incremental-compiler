// the abstract syntax of x64 assembly
// as defined in the book, figure 2.8, page 29

use crate::ir::explicate::{CProgram};

use std::collections::HashMap;
use std::rc::Rc;

pub enum Reg {
    Rsp, Rbp, Rax, Rbx,
    Rcx, Rdx, Rsi, Rdi,
    R8, R9, R10, R11,
    R12, R13, R14, R15
}

pub enum Arg {
    Var(Rc<String>), // for the first pass where variables are still present
    Imm(i64),
    Reg(Reg),
    Deref(Reg, i64),
}

pub enum VarLoc {
    // a variable can live in either
    Reg(Reg), // a register or
    Imm(i64), // an offset from rbp
}

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

pub struct X64Transform {
    ir: CProgram,
}

impl X64Transform {
    pub fn new(cprog: CProgram) -> Self {
        X64Transform {
            ir: cprog
        }
    }
}