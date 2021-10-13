#![allow(dead_code)]

extern crate datatypes;

use datatypes::{RuntimeI64};

use crate::frontend::ast::{Program, AstNode};
use crate::io::{get_line};
use crate::types::{IdString, Environment};
use crate::interpreter::{InterpResult, CachedRuntimeCall, CachedFunctionResult};

// AstInterpreter -> exp ::= int | (read) | (- exp) | (+ exp exp)
//               | var | (let ([var exp]) exp)
pub struct AstInterpreter {
    program: Program,
    interpretation_error: bool,
    errors: Vec<String>,
    crcs: CachedRuntimeCall,
    using_cached_runtime_calls: bool,
    storing_cached_runtime_calls: bool
}

impl AstInterpreter {

    pub fn new(p: Program) -> AstInterpreter {
        AstInterpreter {
            program: p,
            interpretation_error: false,
            errors: vec!(),
            crcs: CachedRuntimeCall::new(),
            using_cached_runtime_calls: false,
            storing_cached_runtime_calls: true,
        }
    }

    pub fn interpret(&mut self) -> InterpResult {
        let mut envir = Environment::new();

        let value = self.interp_exp(&mut envir, &self.program.exp.clone());

        InterpResult {
            value: value,
            cached_runtime_calls: self.crcs.clone(),
        }
    }

    pub fn set_cached_runtime_calls(mut self, crcs: CachedRuntimeCall) -> Self {
        self.using_cached_runtime_calls = true;
        self.storing_cached_runtime_calls = false;
        self.crcs = crcs;

        self
    }

    fn get_cached_runtime_fn(&mut self, fn_name: IdString) -> Option<&mut CachedFunctionResult> {
        return self.crcs.get_mut(&fn_name);
    }

    fn get_cached_result_of(&mut self, fn_name: IdString) -> RuntimeI64 {
        let maybe_read_calls = self.get_cached_runtime_fn(fn_name);

        match maybe_read_calls {
            Some(read_calls) => {
                return read_calls.pop_front().unwrap();
            },

            _ => {
                println!(
                    "{}: was told to look for a cached call for the 'read' function; could not find it.",
                    "ast-interpreter",
                );
                unreachable!();
            }
        }
    }

    fn set_cached_result_of(&mut self, fn_name: IdString, val: RuntimeI64) {
        let maybe_cached_runtime_fn = self.get_cached_runtime_fn(fn_name.clone());

        match maybe_cached_runtime_fn {
            Some(cached_runtime_fn) => {
                cached_runtime_fn.push_back(val);
            },

            _ => {
                let mut new_fn_cache = CachedFunctionResult::new();
                new_fn_cache.push_back(val);
                self.crcs.insert(fn_name, new_fn_cache);
            }
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

                        if self.using_cached_runtime_calls {

                            return Some(self.get_cached_result_of(fn_name));

                        } else {
                            let input = get_line();

                            match input.parse::<RuntimeI64>() {
                                Ok(n) => {

                                    self.set_cached_result_of(fn_name, n);

                                    return Some(n);
                                },

                                Err(error) => {
                                    return self.add_error(format!("{}", error));
                                }
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

                    let already_exists = env.contains_key(&*the_var);

                    if already_exists {
                        return self.add_error(format!("{} is already defined!", the_var));

                    } else {
                        let the_value = &binding.expr;
                        let result = self.interp_exp(env, the_value).unwrap();
                        let _ = env.insert(the_var, result);
                    }
                }

                self.interp_exp(env, body)
            },

            AstNode::Var { name } => {

                match env.get(name) {
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
}