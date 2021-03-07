
use crate::ast::{AstNode, Program};

use std::collections::HashMap;

fn uniquify_exp(env: &mut HashMap<String, String>, e: AstNode) -> AstNode {
    match e {
        AstNode::Int(n) => AstNode::Int(n),

        AstNode::Var { name } => {
            let new_name;

            if env.contains_key(&name) {
                new_name = env.get(&name).unwrap();
            } else {
                new_name = &name;
            }

            AstNode::Var {
                name: new_name.to_string()
            }
        },

        AstNode::Prim { op, mut args } => {
            for i in 0..args.len() {
                let new_arg_expr = uniquify_exp(env, args[i].clone());
                args[i] = new_arg_expr;
            }

            AstNode::Prim {
                op: op,
                args: args
            }
        },

        AstNode::Let { var, value, in_exp, } => {

            let mut number = 1;

            let mut new_name = var.clone() + "." + (&number.to_string());

            if env.contains_key(&var) {
                while env.contains_key(&new_name) {
                    number += 1;
                    new_name = var.clone() + "." + (&number.to_string());
                }
            }

            //let unq_var = var.clone() + "." + (&number.to_string());

            env.insert( var, new_name.clone() );

            let unq_value = uniquify_exp(env, *value);

            let unq_body = uniquify_exp(env, *in_exp);

            AstNode::Let {
                var: new_name,
                value: Box::new(unq_value),
                in_exp: Box::new(unq_body)
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
        exp: uniquify_exp(&mut HashMap::new(), p.exp),
    }
}