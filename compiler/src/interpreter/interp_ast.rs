extern crate datatypes;

use std::collections::HashMap;

use crate::frontend::ast::{Program, AstNode};
use crate::io::{get_line};
use crate::types::{IdString, Environment};
use crate::interpreter::{InterpResult};

use datatypes::{RuntimeI64};

// Rlang -> exp ::= int | (read) | (- exp) | (+ exp exp)
//               | var | (let ([var exp]) exp)
struct Rlang {
    interpretation_error: bool,
    errors: Vec<String>,
    env: Environment,
}

impl Rlang {
    fn new(env: HashMap<IdString, RuntimeI64>) -> Rlang {
        Rlang {
            interpretation_error: false,
            errors: vec!(),
            env: env
        }
    }

    pub fn interpret_success(&self) -> bool {
        !self.interpretation_error
    }

    pub fn print_errors(&self) {
        for error in &self.errors {
            println!("{}", error);
        }
    }

    fn error(&mut self) {
        self.interpretation_error = true
    }

    fn add_error(&mut self, string: String) {
        self.error();

        self.errors.push(string.clone());
    }

    fn interp_exp(&mut self, env: &mut Environment, e: AstNode) -> Option<RuntimeI64> {
        match e {

            AstNode::Int(n) => Some(n),

            AstNode::Prim {op, args} => {
                match &op[..] {
                    "+" => {
                        let arg1 = self.interp_exp(env, args[0].clone());
                        let arg2 = self.interp_exp(env, args[1].clone());

                        if arg1.is_none() {
                            return None;
                        }

                        if arg2.is_none() {
                            return None;
                        }

                        Some(arg1.unwrap() + arg2.unwrap())
                    },
                    "-" => {
                        let arg1 = self.interp_exp(env, args[0].clone());

                        Some(-arg1.unwrap())
                    },
                    "read" => {
                        let input = get_line();

                        match input.parse::<RuntimeI64>() {
                            Ok(n) => {
                                return Some(n);
                            },
                            Err(error) => {
                                self.add_error(format!("{}", error));

                                return None;
                            }
                        };
                    },
                    _ => {  
                            self.add_error(format!("Unrecognized operator in interp_exp: {}", op));

                            return None;
                    }
                }
            },

            AstNode::Let { bindings, body } => {

                for binding in bindings {
                    let the_var = binding.identifier;

                    let already_exists = env.contains_key(&*the_var);

                    if already_exists {
                        self.add_error(format!("{} is already defined!", the_var));

                        return None;
                    } else {
                        let the_value = binding.expr;
                        let result = self.interp_exp(env, the_value).unwrap();
                        let _ = env.insert(the_var, result);
                    }
                }

                self.interp_exp(env, *body)
            },

            AstNode::Var { name } => {

                match env.get(&name) {
                    Some(n) => Some(*n),
                    _ => return self.add_error(format!("{} is not defined!", name))
                }
            },

            AstNode::Error {msg, token} => {
                self.add_error(format!("{}{:?}", msg, token))
            },
        }
    }

    fn interp_r(&mut self, p: Program) -> Result<InterpResult, String> {
        let mut envir = self.env.clone();
        let value = self.interp_exp(&mut envir, p.exp);

        InterpResult {
            value: value,
            environment: envir,
        }
    }
}

pub struct Interpreter {
    interpreter: Rlang,
    program: Program,
}

impl Interpreter {

    pub fn new(p: Program, env: Environment) -> Interpreter {
        Interpreter {
            interpreter: Rlang::new(env),
            program: p,
        }
    }

    pub fn had_error(&self) -> bool {
        !self.interpreter.interpret_success()
    }

    pub fn print_errors(&self) {
        self.interpreter.print_errors()
    }

    pub fn interpret(&mut self) -> Result<InterpResult, String> {
        let result = self.interpreter.interp_r(self.program.clone());

        result
    }

}