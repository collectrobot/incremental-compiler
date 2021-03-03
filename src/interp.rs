use crate::ast::{Program, AstNode};
use crate::io::{get_line};

use std::collections::HashMap;

trait Interpret {
    fn interp_exp(&mut self, env: &mut HashMap<String, i64>, e: AstNode) -> Result<i64, String>;
    fn interp_r(&mut self, p: Program) -> Result<i64, String>;
}

// R1 -> exp ::= int | (read) | (- exp) | (+ exp exp)
struct R1 {
    error: bool,
}

impl Interpret for R1 {
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
            AstNode::Error {msg, token} => {
                Err(format!("{}{:?}", msg, token))
            },
            _ => {
                Err(format!("Unknown ast node: {:?}", e))
            } 
        }
    }

    fn interp_r(&mut self, p: Program) -> Result<i64, String> {
        self.interp_exp(&mut HashMap::new(), p.exp)
    }
}

impl R1 {
    fn new() -> R1 {
        R1 {
            error: false
        }
    }
}

// Rvar -> exp ::= int | (read) | (- exp) | (+ exp exp)
//               | var | (let ([var exp]) exp)
struct Rvar {
    error: bool,
    parent: R1
}

impl Interpret for Rvar {
    fn interp_exp(&mut self, env: &mut HashMap<String, i64>, e: AstNode) -> Result<i64, String> {
        if self.error {
            return Err("Couldn't continue execution because of an error.".to_owned())
        }

        match e {
            AstNode::Let{ var, value, in_exp } => {
                let already_exists = env.contains_key(&var);

                if already_exists {
                    return Err(format!("{} is already defined!", var))
                } else {
                    let result = self.interp_exp(env, *value).unwrap();
                    let _ = env.insert(var, result);
                }

                self.interp_exp(env, *in_exp)
            },

            _ => self.parent.interp_exp(env, e),
        }
        
    }

    fn interp_r(&mut self, p: Program) -> Result<i64, String> {
        self.interp_exp(&mut HashMap::new(), p.exp)
    }
}

pub struct Interpreter {
    versions: Vec<Box<dyn Interpret>>,
}

impl Interpreter {

    pub fn new() -> Interpreter {
        let mut interp = Interpreter{versions: vec!()};

        interp.versions.push(Box::new(R1::new()));

        interp
    }

    pub fn interpret(&mut self, p: Program) -> Result<i64, String> {
        self.versions.pop().unwrap().interp_r(p)
    }

}