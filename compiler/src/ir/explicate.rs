/*
    turn the ast into an intermediate language that makes the order of
    execution explicit
*/

#![allow(dead_code)]

use natord;

use crate::frontend::ast::{AstNode, Program};
use crate::types::{IdString};

use std::collections::HashMap;

/*

Atm   ::= (Int int) | (Var var)
Exp   ::= atm | (Prim read ()) |(Prim - (atm)) |(Prim + (atm atm))
Stmt  ::= (Assign (Var var) exp)
Tail  ::= (Return exp) | (Seq stmt tail)
Clang ::= (IRProgram info ((label . tail) ...))

info will be a list of local variables

*/

#[derive(Clone, Debug, PartialEq)]
pub enum Atm {
    Int(i64),
    Var { name: IdString },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Exp {
    Atm(Atm),
    Prim { op: IdString, args: Vec<Atm> },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Assign(Atm, Exp),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Tail {
    Return(Exp),
    Seq(Stmt, Box<Tail>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct IRFunction {
    pub locals: Vec<IdString>, // function variables
    pub labels: HashMap<IdString, Tail>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IRProgram {
    pub functions: HashMap<IdString, IRFunction>,
}

struct Explicator {
    local_vars: Vec<IdString>,
}

#[derive(Debug, Clone)]
enum Arithmetic {
    Add,
    Sub,
}

fn prim_args_to_ir_atm_vec(vec: Vec<AstNode>) -> Vec<Atm> {

    let mut v: Vec<Atm> = vec!();

    for node in vec {
        match node {
            AstNode::Int(n) => {
                v.push(Atm::Int(n));
            },

            AstNode::Var { name } => {
                v.push(Atm::Var { name: name.clone() });
            },

            _ => {
                println!("{}:{}: expected an atom, got: '{:?}'",
                    crate::function!(),
                    line!(),
                    node
                );
                unreachable!();
            }
        }
    }

    v
}

impl Explicator {

    pub fn new() -> Explicator {
        Explicator {
            local_vars: vec!(),
        }
    }

    fn explicate_tail(&mut self, exp: AstNode) -> Tail {
        match exp {

            AstNode::Var { name } => {
                let return_node = 
                    Tail::Return (
                        Exp::Atm (
                            Atm::Var {
                                name: name.clone()
                            }
                        )
                    );

                return_node
            },

            AstNode::Int(n) => {
                let return_node = 
                    Tail::Return (
                        Exp::Atm (
                            Atm::Int(n)
                        )
                    );

                return_node
            },

            AstNode::Let { bindings, body } => {

                let last_tail = self.explicate_tail(*body.clone());

                let assignments = 
                    bindings
                    .iter()
                    .rev()
                    .fold(
                        last_tail,
                        | tail, assign |
                        self.explicate_assign(
                            assign.expr.clone(),
                            assign.identifier.clone(),
                            tail
                        )
                    );

                assignments
            },

            AstNode::Prim {op, args} => {

                match &op[..] {
                    "+" | "-" | "read" => {
                        Tail::Return (
                            Exp::Prim {
                                op: op.clone(),
                                args: prim_args_to_ir_atm_vec(args)
                            }
                        )
                    },

                    _ => {
                        println!("{}:{}: unknown operator: '{}'",
                            crate::function!(),
                            line!(),
                            op
                        );
                        unreachable!();
                    }
                }

            },

            _ => {
                unreachable!();
            }
        }
    }

    fn explicate_assign(&mut self, exp: AstNode, var: IdString, acc: Tail) -> Tail {
        match exp {

            AstNode::Var { name } => {
                
                self.local_vars.push(var.clone());

                Tail::Seq(
                    Stmt::Assign(
                        Atm::Var {name: var},
                        Exp::Atm (
                            Atm::Var {
                                name: name.clone()
                            }
                        )
                    ),
                    Box::new(acc)
                )

            },

            AstNode::Int(n) => {

                self.local_vars.push(var.clone());

                Tail::Seq(
                    Stmt::Assign(
                        Atm::Var{ name: var },
                        Exp::Atm (
                            Atm::Int(n)
                        )
                    ),
                    Box::new(acc)
                )
            },

            AstNode::Let { bindings, body } => {

                let body_assign = self.explicate_assign(*body.clone(), var, acc);

                let assignments = 
                    bindings
                    .iter()
                    .rev()
                    .fold(
                        body_assign,
                        | tail, assign |
                        self.explicate_assign(
                            assign.expr.clone(),
                            assign.identifier.clone(),
                            tail
                        )
                    );

                assignments

            },

            AstNode::Prim { op, args } => {

                match &op[..] {

                    "read" | "+" | "-" => {
                        self.local_vars.push(var.clone());

                        Tail::Seq(
                            Stmt::Assign(
                                Atm::Var { name: var },
                                Exp::Prim {
                                    op: op.clone(),
                                    args: prim_args_to_ir_atm_vec(args)
                                }
                            ),
                            Box::new(acc)
                        )
                    },
                    _ => {
                        println!("{}:{}: unrecognized operator: '{:?}'",
                            crate::function!(),
                            line!(),
                            op
                        );
                        unreachable!();
                    }
                }
            },

            _ => {
                println!("{}:{}: unrecognized expression: '{:?}'",
                    crate::function!(),
                    line!(),
                    exp
                );
                unreachable!();
            }
        }
    }
}

pub fn explicate_control(program: Program) -> IRProgram {
    let mut explicator = Explicator::new();

    IRProgram {
        functions:
            program.functions
            .iter()
            .map(|(key, value)| {
                return (
                    key.clone(), {
                        let explicator = Explicator::new();
                        let instructions = explicator.explicate_tail(value.exp);

                        /*
                            this isn't absolutely required, but means that the locals will look like this:

                            [tmp.0, tmp.1]
                            
                            instead of:

                            [tmp.1, tmp.0]

                            for example
                        */
                        let mut fn_locals = explicator.local_vars;
                        fn_locals.sort_by(
                            |a, b|
                            natord::compare(&*a, &*b)
                        );

                        IRFunction {
                            locals: fn_locals,
                            labels: crate::map!(crate::idstr!(".l1") => instructions),
                        }
                    }
                )
            }).collect()
    }
}