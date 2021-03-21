/*
    turn the ast into an intermediate language that makes the order of
    execution explicit
*/


use crate::ast::{AstNode, Program};

use std::collections::HashMap;

/*

Atm   ::= (Int int) | (Var var)
Exp   ::= atm | (Prim read ()) |(Prim - (atm)) |(Prim + (atm atm))
Stmt  ::= (Assign (Var var) exp)
Tail  ::= (Return exp) | (Seq stmt tail)
Clang ::= (CProgram info (list of 

info will be a list of local variables

*/

pub enum Atm {
    Int(i64),
    Var { name: String },
}

pub enum Exp {
    Atm(Atm),
    Prim { op: String, args: Vec<Atm> },
}

pub enum Stmt {
    Assign(Atm, Exp),
}

pub enum Tail {
    Return(Exp),
    Seq(Stmt, Box<Tail>),
}

pub struct CProgram {
    info: Vec<Atm>, // local variables
    labels: Vec<(String, Tail)>,
}

struct Explicator {
    out_acumulator: Vec<Tail>, // this needs to be reversed when done, to get the right order
    local_vars: Vec<Atm>, // a vector of Atm::Var
}

impl Explicator {

    pub fn new() -> Explicator {
        Explicator {
            out_acumulator: vec!(),
            local_vars: vec!(),
        }
    }

    fn explicate_assign(&self, exp: AstNode) -> Tail {

        ()
    }

    fn explicate_tail(&self, exp: AstNode) -> Tail {

        ()
    }

    pub fn explicate_control(program: Program) -> CProgram {
        let mut explicator = Explicator::new();

        explicator.explicate_tail(program.exp);
        explicator.out_acumulator.reverse();

        CProgram {
            info: explicator.local_vars,
            labels: vec!(("start".to_owned(), explicator.out_acumulator[0])),
        }
    }
}