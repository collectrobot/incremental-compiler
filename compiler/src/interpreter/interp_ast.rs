extern crate datatypes;

use std::collections::{HashMap, VecDeque};

use crate::frontend::ast::{Program, AstNode};
use crate::io::{get_line};
use crate::types::{IdString, Environment};
use crate::interpreter::{InterpResult, CachedRuntimeCalls};

use datatypes::{RuntimeI64};

// Rlang -> exp ::= int | (read) | (- exp) | (+ exp exp)
//               | var | (let ([var exp]) exp)
struct Rlang {
    interpretation_error: bool,
    errors: Vec<String>,
    crcs: CachedRuntimeCalls,
}

impl Rlang {
    fn new(crcs: CachedRuntimeCalls) -> Rlang {
        Rlang {
            interpretation_error: false,
            errors: vec!(),
            crcs: crcs,
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

                        // first check to see if there are cached calls to read
                        // the ast interpreter should normally be called first,
                        // so this most likely will never be the case, but check for this
                        // in case that changes

                        let id = crate::idstr!("read");

                        let mut cached_reads = self.crcs.get_mut(&id);

                        let mut n = None;

                        if let Some(cached) = cached_reads {
                            n = cached.pop_front();

                            if cached.len() == 0 {
                                self.crcs.remove_entry(&id);
                            }
                        }

                        if n.is_none() {
                            let input = get_line();

                            match input.parse::<RuntimeI64>() {
                                Ok(num) => {
                                    n = Some(num);
                                    self.crcs.inser
                                },

                                Err(error) => {
                                    return self.add_error(format!("{}", error));
                                }
                            }
                        }

                        return n;
                    },
                    _ => {  
                            return self.add_error(format!("Unrecognized operator in interp_exp: {}", op));
                    }
                }
            },

            AstNode::Let { bindings, body } => {

                for binding in bindings {
                    let the_var = binding.identifier;

                    let already_exists = env.contains_key(&*the_var);

                    if already_exists {
                        return self.add_error(format!("{} is already defined!", the_var));

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

    fn interp_r(&mut self, p: Program) -> InterpResult {
        let mut envir = Environment::new();
        let value = self.interp_exp(&mut envir, p.exp);

        InterpResult {
            value: value,
            cached_runtime_calls: self.crcs.clone(),
        }
    }
}

pub struct Interpreter {
    interpreter: Rlang,
    program: Program,
}

impl Interpreter {

    pub fn new(p: Program, crcs: CachedRuntimeCalls) -> Interpreter {
        Interpreter {
            interpreter: Rlang::new(crcs),
            program: p,
        }
    }

    pub fn had_error(&self) -> bool {
        !self.interpreter.interpret_success()
    }

    pub fn print_errors(&self) {
        self.interpreter.print_errors()
    }

    pub fn interpret(&mut self) -> InterpResult {
        let result = self.interpreter.interp_r(self.program.clone());

        result
    }

}