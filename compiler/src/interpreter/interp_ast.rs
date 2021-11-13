#![allow(dead_code)]
#![allow(unused_imports)]

use runtime::types::{RuntimeI64};

use crate::frontend::ast::{Program, AstNode};
use crate::io::{get_line};
use crate::types::{Environment};
use crate::interpreter::{Interpretable, InterpretResult, RuntimeValue};

// AstInterpreter -> exp ::= int | (read) | (- exp) | (+ exp exp)
//               | var | (let ([var exp]) exp)
pub struct AstInterpreter {
    program: Program,
    interpretation_error: bool,
    errors: Vec<String>,
}

impl AstInterpreter<> {
    pub fn new(p: Program) -> AstInterpreter {
        AstInterpreter {
            program: p,
            interpretation_error: false,
            errors: vec!(),
        }
    }

    pub fn run(&mut self) -> Option<RuntimeI64> {
        let mut envir = Environment::new();

        let start = self.program.functions.get(&crate::idstr!("start"));

        if let Some(func) = start {
            self.interp_exp(&mut envir, &func.exp)
        } else {
            self.add_error("entry point could not be found, expected 'start'".to_owned())
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

    fn add_error(&mut self, string: String) -> Option<RuntimeI64> {
        self.error();

        self.errors.push(string.clone());

        None
    }

    fn interp_exp(&mut self, env: &mut Environment, e: &AstNode) -> Option<RuntimeI64> {
        match e {

            AstNode::Int(n) => Some(*n),

            AstNode::Prim {op, args} => {
                match &op[..] {
                    "+" => {
                        let arg1 = self.interp_exp(env, &args[0]);
                        let arg2 = self.interp_exp(env, &args[1]);

                        if arg1.is_none() {
                            return None;
                        }

                        if arg2.is_none() {
                            return None;
                        }

                        Some(arg1.unwrap() + arg2.unwrap())
                    },
                    "-" => {
                        let arg1 = self.interp_exp(env, &args[0]);

                        Some(-arg1.unwrap())
                    },
                    "read" => {

                        // either we're using cached runtime calls (unlikely as this is the first interpreter being run)
                        // or we're caching calls to the runtime

                        let fn_name = crate::idstr!("read");

                        let input = get_line();

                        match input.parse::<RuntimeI64>() {
                            Ok(n) => {
                                return Some(n);
                            },

                            Err(error) => {
                                return self.add_error(format!("{}", error));
                            }
                        }
                    },
                    _ => {  
                            return self.add_error(format!("Unrecognized operator in interp_exp: {}", op));
                    }
                }
            },

            AstNode::Let { bindings, body } => {

                for binding in bindings {
                    let the_var = binding.identifier.clone();

                    let already_exists = env.exists(the_var.clone());

                    if already_exists {
                        return self.add_error(format!("{} is already defined!", the_var));

                    } else {
                        let the_value = &binding.expr;
                        let result = self.interp_exp(env, the_value).unwrap();
                        let _ = env.insert(the_var, AstNode::Int(result));
                    }
                }

                self.interp_exp(env, body)
            },

            AstNode::Var { name } => {

                match env.get_value_of(name.clone()) {
                    Some(&AstNode::Int(n)) => Some(n),
                    _ => {
                        self.add_error(format!("{} is not defined!", name));
                        return None;
                    }
                }
            },

            AstNode::Error {msg, token} => {
                self.add_error(format!("{}{:?}", msg, token));

                return None;
            },
        }
    }
}

impl Interpretable for AstInterpreter {
    fn interpret(&mut self) -> InterpretResult {
        let value = self.run();

        InterpretResult {
            value: value,
            had_error: self.interpretation_error,
            errors: self.errors.clone(),
        }
    }
}