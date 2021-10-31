#![allow(dead_code)]
#![allow(unused_imports)]

#[cfg(test)]
mod partial_eval_tests;

use runtime::types::{RuntimeI64, RuntimeValue};
use crate::types::{IdString};
use crate::frontend::ast::*;

fn partial_eval_unary(operation: IdString, r: &AstNode) -> AstNode {

    let right = partial_eval_exp(r);

    match &operation[..] {
        "-" => {
            match right {
                AstNode::Int(n) => {
                    AstNode::Int(0 - n)
                },

                _ => {
                    r.clone()
                },
            }
        },

        _ => {
            unreachable!();
        }
    }
}

fn partial_eval_binary(operation: IdString, l: &AstNode, r: &AstNode) -> AstNode {

    let left = partial_eval_exp(l);
    let right = partial_eval_exp(r);

    match &operation[..] {
        "+" => {
            match (&left, &right) {
                (AstNode::Int(n), AstNode::Int(m)) => {
                    AstNode::Int(n + m)
                },

                _ => {
                    AstNode::Prim {
                        op: operation.clone(),
                        args: vec!(left, right)
                    }
                }
            }
        },

        _ => {
            unreachable!();
        }
    }
}

fn partial_eval_prim(exp: &AstNode) -> AstNode {
    match exp {
        AstNode::Prim { op, args } => {
            match &op[..] {

                "+" => {
                    partial_eval_binary(op.clone(), &args[0], &args[1])
                },

                "-" => {
                    partial_eval_unary(op.clone(), &args[0])
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

fn partial_eval_exp(exp: &AstNode) -> AstNode {

    match exp {
        AstNode::Int(_) => {
            exp.clone()
        }

        AstNode::Prim { .. } => {
            partial_eval_prim(&exp)
        },

        AstNode::Let { bindings, body } => {
            
            let new_bindings = 
                bindings
                    .iter()
                    .map(
                        | b |
                        LetBinding {
                            identifier: b.identifier.clone(),
                            expr: partial_eval_exp(&b.expr)
                        }
                    )
                    .collect();

            AstNode::Let {
                bindings: new_bindings,
                body: Box::new(partial_eval_exp(*&body))
            }
        },

        _ => {
            exp.clone()
        }
    }
}

pub fn partial_evaluate(ast: Program) -> Program {

    let result = partial_eval_exp(&ast.exp);

    Program {
        info: (),
        exp: result
    }
}