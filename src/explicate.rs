/*
    turn the ast into an intermediate language that makes the order of
    execution explicit
*/


use crate::ast::{AstNode};

use std::collections::HashMap;

/*

Atm   ::= (Int int) | (Var var)
Exp   ::= atm | (Prim read ()) |(Prim - (atm)) |(Prim + (atm atm))
Stmt  ::= (Assign (Var var) exp)
Tail  ::= (Return exp) | (Seqstmt tail)
Clang ::= (CProgram info ((label . tail)...))

*/

type Var = String;

pub enum Atm {
    Int(i64),
    Var(Var),
}

pub enum Exp {
    Atm(Atm),
    Prim { op: String, args: Vec<Atm> },
}

pub enum Stmt {
    Assign(Var, Exp),
}

pub enum Tail {
    Return(Exp),
    Seq(Stmt, Box<Tail>),
}

pub enum Clang {
    CProgram { info: (), labels: HashMap<String, Tail> },
}