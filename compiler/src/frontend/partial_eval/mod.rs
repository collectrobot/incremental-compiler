#![allow(dead_code)]
#![allow(unused_imports)]

#[cfg(test)]
mod partial_eval_tests;

use runtime::types::{RuntimeI64, RuntimeValue};
use crate::frontend::ast::*;

fn partial_eval_unary(exp: AstNode) -> AstNode {
    exp
}

fn partial_eval_binary(exp: AstNode) -> AstNode {
    exp
}

fn partial_eval_prim(exp: AstNode) -> AstNode {
    match exp {
        AstNode::Prim { op, .. } => {
            match &op[..] {
                "read" => {
                    AstNode::Prim {
                        op: op.clone(),
                        args: vec!()
                    }
                },

                "+" => {
                    partial_eval_binary(exp)
                },

                "-" => {
                    partial_eval_unary(exp)
                },

                _ => {
                    exp
                },
            }
        },

        _ => {
            unreachable!();
        }
    }
}

fn partial_eval_exp(exp: AstNode) -> AstNode {

    match exp {
        AstNode::Int(_) => {
            exp
        },

        AstNode::Var { .. } => {
            exp
        },

        AstNode::Prim { .. } => {
            partial_eval_prim(exp)
        }


        AstNode::Let { bindings, body } => {

            //let mut partially_evaluated_bindings: Vec<LetBinding> = vec!();

            /*
            for binding in bindings {
                partially_evaluated_bindings.push(
                    LetBinding {
                        identifier: binding.identifier.clone(),
                        expr: partial_eval_exp(binding.expr)
                    }
                )
            }
            */

            AstNode::Let {
                bindings: bindings.iter().map(|b| LetBinding { identifier: b.identifier.clone(), expr: partial_eval_exp(b.expr)} ).collect(),
                body: Box::new(partial_eval_exp(*body))
            }
        },

        _ => {
            exp
        }
    }
}

fn partial_evaluate(ast: Program) -> Program {

    Program {
        info: (),
        exp: partial_eval_exp(ast.exp)
    }
}