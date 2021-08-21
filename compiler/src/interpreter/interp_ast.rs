use crate::frontend::ast::{Program, AstNode};
use crate::io::{get_line};

use std::collections::HashMap;
use std::rc::Rc;

// Rlang -> exp ::= int | (read) | (- exp) | (+ exp exp)
//               | var | (let ([var exp]) exp)
struct Rlang {
    interpretation_error: bool,
    errors: Vec<String>
}

impl Rlang {
    fn new() -> Rlang {
        Rlang {
            interpretation_error: false,
            errors: vec!()
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

    fn add_error(&mut self, string: String) -> Result <i64, String> {
        self.error();

        self.errors.push(string.clone());

        Err(string)
    }

    fn interp_exp(&mut self, env: &mut HashMap<Rc<String>, i64>, e: AstNode) -> Result<i64, String> {
        match e {

            AstNode::Int(n) => Ok(n),

            AstNode::Prim{op, args} => {
                match &op[..] {
                    "+" => {
                        let arg1 = self.interp_exp(env, args[0].clone());
                        let arg2 = self.interp_exp(env, args[1].clone());

                        if arg1.is_err() {
                            return self.add_error(arg1.unwrap_err())
                        }

                        if arg2.is_err() {
                            return self.add_error(arg2.unwrap_err())
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
                                return self.add_error(format!("{}", error))
                            }
                        };
                    },
                    _ => {  
                            self.add_error(format!("Unrecognized operator in interp_exp: {}", op))
                        }
                }
            },

            AstNode::Let{ bindings, body } => {

                for binding in bindings {
                    let the_var = binding.identifier;

                    let already_exists = env.contains_key(&*the_var);

                    if already_exists {
                        return self.add_error(format!("{} is already defined!", the_var))
                    } else {
                        let the_value = binding.expr;
                        let result = self.interp_exp(env, the_value).unwrap();
                        let _ = env.insert(the_var, result);
                    }
                }

                self.interp_exp(env, *body)
            },

            AstNode::Var{ name } => {

                match env.get(&name) {
                    Some(n) => Ok(*n),
                    _ => return self.add_error(format!("{} is not defined!", name))
                }
            },

            AstNode::Error {msg, token} => {
                self.add_error(format!("{}{:?}", msg, token))
            },
        }
    }

    fn interp_r(&mut self, p: Program) -> Result<i64, String> {
        self.interp_exp(&mut HashMap::new(), p.exp)
    }
}

pub struct Interpreter {
    interpreter: Rlang,
    program: Program
}

impl Interpreter {

    pub fn new(p: Program) -> Interpreter {
        Interpreter {
            interpreter: Rlang::new(),
            program: p 
        }
    }

    pub fn had_error(&self) -> bool {
        !self.interpreter.interpret_success()
    }

    pub fn print_errors(&self) {
        self.interpreter.print_errors()
    }

    pub fn interpret(&mut self) -> Result<i64, String> {
        self.interpreter.interp_r(self.program.clone())
    }

}