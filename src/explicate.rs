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
Clang ::= (CProgram info ((label . tail) ...))

info will be a list of local variables

*/

#[derive(Clone, Debug)]
pub enum Atm {
    Int(i64),
    Var { name: String },
}

#[derive(Clone, Debug)]
pub enum Exp {
    Atm(Atm),
    Prim { op: String, args: Vec<Atm> },
}

#[derive(Clone, Debug)]
pub enum Stmt {
    Assign(Atm, Exp),
}

#[derive(Clone, Debug)]
pub enum Tail {
    Return(Exp),
    Seq(Stmt, Box<Tail>),
}

#[derive(Debug)]
pub struct CProgram {
    info: Vec<String>, // local variables
    labels: Vec<(String, Tail)>,
}

struct Explicator {
    //out_acumulator: Vec<Tail>, // this needs to be reversed when done, to get the right order
    local_vars: Vec<String>, // a vector of Atm::Var
}

#[derive(Debug, Clone)]
enum Arithmetic {
    Add,
    Sub,
}

impl Explicator {

    pub fn new() -> Explicator {
        Explicator {
            //out_acumulator: vec!(),
            local_vars: vec!(),
        }
    }

    // perform arithmetic on two constants
    fn arithm_atoms(&self, kind: Arithmetic, a1: Atm, a2: Atm) -> Atm {
        let v1 = match a1 {
            Atm::Int(n) => n,
            _ => {
                println!("{}:{}: expected a constant atom, got: '{:?}'",
                    crate::function!(),
                    line!(),
                    a1
                );
                unreachable!();
            }
        };

        let v2 = match a2 {
            Atm::Int(n) => n,
            _ => {
                println!("{}:{}: expected a constant atom, got: '{:?}'",
                    crate::function!(),
                    line!(),
                    a2
                );
                unreachable!();
            }
        };

        match kind {
            Arithmetic::Add => {
                Atm::Int(v1 + v2)
            },

            Arithmetic::Sub => {
                Atm::Int(v1 - v2)
            },

            _ => {
                println!("{}:{}: arithmetic kind unknown: '{:?}'",
                    crate::function!(),
                    line!(),
                    kind
                );
                unreachable!();
            }
        }
    }

    // returns an atom, and false if it's a var, true if it's a const
    fn extract_atom(&mut self, exp: AstNode) -> (Atm, bool) {
        match &exp {

            AstNode::Prim { op, args } => {
                match &op[..] {
                    "-" => {

                        let (the_atm, is_const) = self.extract_atom(args[0].clone());

                        if is_const {
                            (self.arithm_atoms(
                                Arithmetic::Sub,
                                Atm::Int(0),
                                the_atm
                            ), is_const)

                        } else {
                            (the_atm, is_const)
                        }
                    },

                    _ => {
                        println!("{}:{}: expected negation, got: '{:?}'",
                            crate::function!(),
                            line!(),
                            exp
                        );
                        unreachable!();
                    }
                }
            },

            AstNode::Int(n) => {
                (Atm::Int(*n), true)
            },
            AstNode::Var{name} => {
                (Atm::Var{name: name.clone()}, false)
            },

            _ => {
                println!("{}:{}: expected atom, got: '{:?}'",
                    crate::function!(),
                    line!(),
                    exp
                );
                unreachable!();
            }
        }
    }

    fn explicate_tail(&mut self, exp: AstNode) -> Tail {
        match exp {

            AstNode::Var { name } => {
                let return_node = 
                    Tail::Return (
                        Exp::Atm (
                            Atm::Var {
                                name: name
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

                let last_tail = self.explicate_tail(*body);

                let assignments = 
                    bindings
                    .iter()
                    .rev()
                    .fold(
                        last_tail,
                        | tail, assign |
                        self.explicate_assign(
                            assign.1.clone(),
                            assign.0.clone(),
                            tail
                        )
                    );

                assignments
            },

            AstNode::Prim {op, args} => {

                match &op[..] {
                    "+" => {

                        let (left_atom, left_is_const) = self.extract_atom(args[0].clone());
                        let (right_atom, right_is_const) = self.extract_atom(args[1].clone());

                        if left_is_const && right_is_const {
                            Tail::Return (
                                Exp::Atm(
                                    self.arithm_atoms(Arithmetic::Add, left_atom, right_atom)
                                )
                            )
                        } else {
                            Tail::Return (
                                Exp::Prim {
                                    op: "+".to_owned(),
                                    args: vec!(left_atom, right_atom)
                                }
                            )
                        }
                    },

                    "-" => {
                        unreachable!();
                    }

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

    fn explicate_assign(&mut self, exp: AstNode, var: String, acc: Tail) -> Tail {
        match &exp {
            AstNode::Var { name } => {
                unimplemented!();
            },

            AstNode::Int(n) => {

                self.local_vars.push(var.clone());

                Tail::Seq(
                    Stmt::Assign(
                        Atm::Var{ name: var },
                        Exp::Atm (
                            Atm::Int(*n)
                        )
                    ),
                    Box::new(acc)
                )
            },

            AstNode::Let { bindings, body } => {
                unimplemented!();
            },

            AstNode::Prim { op, args } => {

                match &op[..] {
                    "-" => {
                        self.local_vars.push(var.clone());

                        Tail::Seq(
                            Stmt::Assign(
                                Atm::Var{name: var},
                                Exp::Prim {
                                    op: "-".to_owned(),
                                    args: vec!(
                                        self.extract_atom(args[0].clone()).0
                                    )
                                }
                            ),
                            Box::new(acc)
                        )
                    },
                    _ => {
                        println!("{}:{}: unrecognized operator: '{:?}'",
                            crate::function!(),
                            line!(),
                            exp
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

pub fn explicate_control(program: Program) -> CProgram {
    let mut explicator = Explicator::new();

    let instructions = explicator.explicate_tail(program.exp);
    //explicator.out_acumulator.reverse();

    // start is the entry point in clang
    CProgram {
        info: explicator.local_vars,
        labels: vec!(("start".to_owned(), instructions)),
    }
}