
use crate::ast::{AstNode};

pub enum Atm {
    Int(i64),
    Var(String),
}

pub enum Exp {
    Atm(Atm),
    Prim { op: String, args: Vec<AstNode> },
}

pub enum Stmt {
    Assign( Atm::Var, Exp },
}

pub enum Tail {
    Return(Exp),
    Seq(Stmt, Tail),

}

pub enum Clang {
    CProgram { info: (), label: String, tail: Tail },
}