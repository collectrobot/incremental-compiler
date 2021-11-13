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

    fn new(prog: Program) -> Self {
        PartialEvaluator {
            prog: prog,
            env: Environment::new(), 
        }
    }

    fn evaluate(&mut self) -> Program {

        Program {
            info: (),
            functions: 
                self.prog.functions
                .iter()
                .map(| (key, value) | {
                    return (key.clone(), Function { exp: self.partial_eval_exp(&value.exp)} )
                }).collect()
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
                return AstNode::Int(n + m);
            },

            (AstNode::Var { name: left_name }, AstNode::Var { name: right_name }) => {
                let lvv = self.env.get_value_of(left_name.clone());
                let rvv = self.env.get_value_of(right_name.clone());

                match (lvv, rvv) {
                    (Some(&AstNode::Int(n)), Some(&AstNode::Int(m))) => {
                        return AstNode::Int(n + m);
                    },

                    _ => {}
                }

            }

            (AstNode::Var { ref name }, AstNode::Int(n)) | 
            (AstNode::Int(n), AstNode::Var { ref name })
            => {
                let vv = self.env.get_value_of(name.clone());

                match vv {
                    Some(&AstNode::Int(m)) => {
                        return AstNode::Int(n + m);
                    },

                    _ => {}
                }
            },

            _ => {
            }
        }

        AstNode::Prim {
            op: crate::idstr!("+"),
            args: vec!(left, right)
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

pub fn partially_evaluate(prog: Program) -> Program {
    let mut pe = PartialEvaluator::new(prog);
    pe.evaluate()
}