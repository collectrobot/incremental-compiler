/*
    make it so that variables are numbered depending on the scope they're defined in
    this allows variable shadowing
*/

#[cfg(test)]
mod uniquify_tests;

use std::collections::HashMap;
use crate::types::{IdString};

use super::ast::{AstNode, LetBinding, Program, Function};

fn uniquify_exp(environments: &mut Vec<HashMap<IdString, IdString>>, e: AstNode) -> AstNode {
    match e {
        AstNode::Int(n) => AstNode::Int(n),

        AstNode::Var { name } => {

            let mut new_name = name.clone();

            let last = environments.len();

            for i in (0..last).rev() {
                if environments[i].contains_key(&name) {
                    new_name = environments[i][&name].clone();
                    break;
                }
            }

            AstNode::Var {
                name: new_name
            }
        },

        AstNode::Prim { op, mut args } => {
            for i in 0..args.len() {
                let new_arg_expr = uniquify_exp(environments, args[i].clone());
                args[i] = new_arg_expr;
            }

            AstNode::Prim {
                op: op,
                args: args
            }
        },

        AstNode::Let { bindings, body, } => {

            environments.push(HashMap::new());

            let last = environments.len()-1;

            let mut unique_bindings: Vec<LetBinding> = Vec::new();

            for binding in bindings {

                let the_var = binding.identifier;
                let the_expression = binding.expr;

                let new_name = crate::idstr!((*the_var).clone() + "." + &(last+1).to_string());

                let unq_value = uniquify_exp(environments, the_expression);

                let current_env = environments.get_mut(last).unwrap();

                current_env.insert(the_var, new_name.clone());

                unique_bindings.push(
                    LetBinding {
                        identifier: new_name,
                        expr: unq_value
                    }
                );
            }

            let unq_body = uniquify_exp(environments, *body);

            environments.pop();

            AstNode::Let {
                bindings: unique_bindings,
                body: Box::new(unq_body)
            }
        },

        AstNode::Error { msg, token } => {
            AstNode::Error { msg: msg, token: token }
        },
    }
}

pub fn uniquify_program(p: Program) -> Program {
    Program {
        info: p.info,
        functions: 
            p.functions
            .iter()
            .map(|(key, value)| {
                return (
                    key.clone(),
                    Function {
                        exp: uniquify_exp(
                            &mut Vec::<HashMap<IdString, IdString>>::new(),
                            value.exp.clone()) 
                    }
                )
            }).collect(),
    }
}