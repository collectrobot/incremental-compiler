#![allow(dead_code)]
#![allow(unused_imports)]

#[cfg(test)]
mod partial_eval_tests;

use runtime::types::{RuntimeI64, RuntimeValue};

use crate::types::{IdString, Environment};
use crate::frontend::ast::*;

struct PartialEvaluator {
    prog: Program,
    env: Environment,
}

impl PartialEvaluator {

    pub fn new(prog: Program) -> Self {
        PartialEvaluator {
            prog: prog,
            env: Environment::new(), 
        }
    }

    pub fn evaluate(&mut self) -> Program {

        let p = &self.prog.exp.clone();
        let result = self.partial_eval_exp(p);

        Program {
            info: (),
            exp: result
        }
    }

    fn partial_eval_negate(&mut self, r: &AstNode) -> AstNode {

        let right = self.partial_eval_exp(r);

        match &right {
            AstNode::Int(n) => {
                return AstNode::Int(0 - n);
            },

            AstNode::Var { name } => {
                let value = self.env.get_value_of(name.clone());

                if let Some(&AstNode::Int(n)) = value {
                    return AstNode::Int(0 - n);
                }
            },

            _ => {
            },
        }

        AstNode::Prim {
            op: crate::idstr!("-"),
            args: vec!(right.clone())
        }
    }

    fn partial_eval_add(&mut self, l: &AstNode, r: &AstNode) -> AstNode {

        let left = self.partial_eval_exp(l);
        let right = self.partial_eval_exp(r);

        match (&left, &right) {
            (AstNode::Int(n), AstNode::Int(m)) => {
                AstNode::Int(n + m)
            },

            (AstNode::Var { name: left_name }, AstNode::Var { name: right_name }) => {
                let lvv = self.env.get_value_of(left_name.clone()).unwrap();
                let rvv = self.env.get_value_of(right_name.clone()).unwrap();

                let left_value = if let AstNode::Int(n) = lvv {
                    n
                } else {
                    unreachable!();
                };

                let right_value = if let AstNode::Int(m) = rvv {
                    m
                } else {
                    unreachable!();
                };

                AstNode::Int(left_value + right_value)
            }

            (AstNode::Var { ref name }, AstNode::Int(n)) | 
            (AstNode::Int(n), AstNode::Var { ref name })
            => {
                let vv = self.env.get_value_of(name.clone()).unwrap();
                let value = if let AstNode::Int(m) = vv {
                    m
                } else {
                    unreachable!();
                };

                AstNode::Int(
                    value + n
                )
            },

            _ => {
                AstNode::Prim {
                    op: crate::idstr!("+"),
                    args: vec!(left, right)
                }
            }
        }
    }

    fn partial_eval_prim(&mut self, exp: &AstNode) -> AstNode {
        match exp {
            AstNode::Prim { op, args } => {
                match &op[..] {

                    "+" => {
                        self.partial_eval_add(&args[0], &args[1])
                    },

                    "-" => {
                        self.partial_eval_negate(&args[0])
                    },

                    _ => {
                        exp.clone()
                    },
                }
            },

            _ => {
                unreachable!();
            }
        }
    }

    fn partial_eval_exp(&mut self, exp: &AstNode) -> AstNode {

        match exp {
            AstNode::Var { .. } => {
                exp.clone()
            },

            AstNode::Int(_) => {
                exp.clone()
            }

            AstNode::Prim { .. } => {
                self.partial_eval_prim(&exp)
            },

            AstNode::Let { bindings, body } => {
                
                let new_bindings = 
                    bindings
                        .iter()
                        .map(
                            | b |
                            {
                                let new_expr = self.partial_eval_exp(&b.expr);
                                self.env.insert(b.identifier.clone(), new_expr.clone());
                                LetBinding {
                                    identifier: b.identifier.clone(),
                                    expr: new_expr 
                                }
                            }
                        )
                        .collect();

                let new_body = self.partial_eval_exp(*&body);
                match new_body {
                    // we were able to evaluate everything to a single integer
                    AstNode::Int(_) => {
                        new_body
                    },

                    AstNode::Var { name } => {
                        self.env.get_value_of(name.clone()).unwrap().clone()
                    },

                    _ => {
                        AstNode::Let {
                            bindings: new_bindings,
                            body: Box::new(self.partial_eval_exp(*&body))
                        }
                    }
                }

            },

            _ => {
                exp.clone()
            }
        }
    }
}
