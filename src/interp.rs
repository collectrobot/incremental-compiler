use crate::ast::{Program, AstNode};
use crate::io::{get_line};

use std::collections::HashMap;

trait Interpret {
    fn interp_exp(&mut self, env: &mut HashMap<String, i64>, e: AstNode) -> Result<i64, String>;
    fn interp_r(&mut self, p: Program) -> Result<i64, String>;
}

// Rlang -> exp ::= int | (read) | (- exp) | (+ exp exp)
//               | var | (let ([var exp]) exp)
struct Rlang {
    error: bool,
}

impl Rlang {
    fn new() -> Rlang {
        Rlang {
            error: false,
        }
    }
}

impl Interpret for Rlang {
    fn interp_exp(&mut self, env: &mut HashMap<String, i64>, e: AstNode) -> Result<i64, String> {
        if self.error {
            return Err("Couldn't continue execution because of an error.".to_owned())
        }

        match e {

            AstNode::Int(n) => Ok(n),

            AstNode::Prim{op, args} => {
                match &op[..] {
                    "+" => {
                        let arg1 = self.interp_exp(env, args[0].clone());
                        let arg2 = self.interp_exp(env, args[1].clone());

                        if arg1.is_err() {
                            let thing = arg1.unwrap_err();
                            return Err(thing)
                        }

                        if arg2.is_err() {
                            return Err(arg2.unwrap_err())
                        }

                        Ok(arg1.unwrap() + arg2.unwrap())
                    },
                    "-" => {
                        let arg1 = self.interp_exp(env, args[0].clone());

                        Ok(-arg1.unwrap())
                    },
                    "read" => {
                        let input = get_line();

                        match input.parse::<i64>() {
                            Ok(n) => return Ok(n),
                            Err(error) => {
                                return Err(format!("{}", error));
                            }
                        };
                    },
                    _ => {  self.error = true;
                            Err(format!("Unrecognized operator in interp_exp: {}", op))
                        }
                }
            },

            AstNode::Let{ bindings, in_exp } => {

                for binding in bindings {
                    let the_var = binding.0;

                    let already_exists = env.contains_key(&the_var);

                    if already_exists {
                        return Err(format!("{} is already defined!", the_var))
                    } else {
                        let the_value = binding.1;
                        let result = self.interp_exp(env, the_value).unwrap();
                        let _ = env.insert(the_var, result);
                    }
                }

                self.interp_exp(env, *in_exp)
            },

            AstNode::Var{ name } => {

                match env.get(&name) {
                    Some(n) => Ok(*n),
                    _ => return Err(format!("{} is not defined!", name))
                }
            },

            AstNode::Error {msg, token} => {
                Err(format!("{}{:?}", msg, token))
            },

            _ => {
                Err(format!("Unknown ast node: {:?}", e))
            },
        }
        
    }

    fn interp_r(&mut self, p: Program) -> Result<i64, String> {
        self.interp_exp(&mut HashMap::new(), p.exp)
    }
}

pub struct Interpreter {
    interp: Box<dyn Interpret>,
}

impl Interpreter {

    pub fn new() -> Interpreter {
        Interpreter {
            interp: Box::new(Rlang::new())
        }
    }

    pub fn interpret(&mut self, p: Program) -> Result<i64, String> {
        self.interp.interp_r(p)
    }

}