
use crate::ast::{AstNode};

use std::collections::HashMap;

/*

Atm   ::= (Int int) | (Var var)
Exp   ::= atm | (Prim read ()) |(Prim - (atm)) |(Prim + (atm atm))
Stmt  ::= (Assign (Var var) exp)
Tail  ::= (Return exp) | (Seqstmt tail)
Clang ::= (CProgram info ((label . tail)...))

*/

pub enum Atm {
    Int(i64),
    Var(String),
}

pub enum Exp {
    Atm(Atm),
    Prim { op: String, args: Vec<Atm> },
}

pub enum Stmt {
    Assign( Atm::Var, Exp },
}

pub enum Tail {
    Return(Exp),
    Seq(Stmt, Tail),
}

pub enum Clang {
    CProgram { info: (), HashMap<String, Tail> },
}