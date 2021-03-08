
use crate::ast::{AstNode, Program};

use std::collections::HashMap;

fn uniquify_exp(environments: &mut Vec<HashMap<String, String>>, e: AstNode) -> AstNode {
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

        AstNode::Let { var, value, in_exp, } => {

            environments.push(HashMap::new());

            let last = environments.len()-1;
            let current_env = environments.get_mut(last).unwrap();

            let new_name = var.clone() + "." + &(last+1).to_string();

            current_env.insert( var.clone(), new_name.clone() );

            let unq_value = uniquify_exp(environments, *value);

            let unq_body = uniquify_exp(environments, *in_exp);

            environments.pop();

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
        exp: uniquify_exp(&mut Vec::<HashMap<String, String>>::new(), p.exp),
    }
}