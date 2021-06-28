#![allow(dead_code)]

use std::collections::HashMap;
use std::rc::Rc;

use crate::io::{get_line};
use crate::ir::explicate::{CProgram, Tail, Stmt, Exp, Atm};

pub struct Clang {
    interpretation_error: bool,
    errors: Vec<String>,
    cprog: CProgram,
    vars: HashMap<Rc<String>, Atm>,
}

#[derive(Debug)]
enum ArithmeticKind {
    Add,
    Negate
}

#[derive(Debug)]
enum Arithmetic {
    Binary(ArithmeticKind, Atm, Atm),
    Unary(ArithmeticKind, Atm)
}

impl Clang {

    fn last_error(&self) -> String {
        if let Some(err) = self.errors.last() {
            err.to_owned()
        } else {
            "".to_owned()
        }
    }

    fn add_error(&mut self, err: String) {
        self.interpretation_error = true;
        self.errors.push(err.clone());
    }

    fn get_var_value(&mut self, var: &Atm) -> Option<Atm> {

        match var {
            Atm::Var { name } => {
                let the_value = self.vars.get(name);

                match the_value {
                    Some(value) => {

                        let v = value.clone();

                        // a variable might be set to another variable
                        // e.g. (let ([x (let ([y 42]) y)]) (+ x 1))
                        // where x = y, and so we need this to return an actual value
                        if let Atm::Var { .. } = v {
                            self.get_var_value(&v)
                        } else {
                            Some(value.clone())
                        }

                    },

                    _ => {
                        self.add_error(
                            format!("Attempted to use undeclared variable: '{}'", name)
                        );

                        None
                    }
                }
            },

            _ => {
                self.add_error(
                    format!("Called get_var_value with a non-var: {:?}", var)
                );

                None
            }
        }
    }

    fn atm_aritmetic(&mut self, arithm: Arithmetic) -> Option<Atm> {
        match arithm {
            Arithmetic::Binary(kind, larg, rarg) => {

                let larg_value = match larg {
                    Atm::Var { .. } => {
                        let maybe_val = self.get_var_value(&larg);

                        self.extract_i64(&maybe_val.unwrap()).unwrap()
                    },

                    Atm::Int(n) => {
                        n
                    }
                };

                let rarg_value = match rarg {
                    Atm::Var { .. } => {
                        let maybe_val = self.get_var_value(&rarg);

                        self.extract_i64(&maybe_val.unwrap()).unwrap()
                    },

                    Atm::Int(n) => {
                        n
                    }
                };

                match kind {
                    ArithmeticKind::Add => {
                        Some(Atm::Int(larg_value + rarg_value))
                    },

                    _ => {
                        self.add_error(
                            format!("Unexpected arithmetic kind: {:?}", kind)
                        );

                        None
                    }
                }
            },

            Arithmetic::Unary(kind, arg) => {
                let arg_value = match arg {
                    Atm::Var { .. } => {
                        let maybe_val = self.get_var_value(&arg);

                        self.extract_i64(&maybe_val.unwrap()).unwrap()
                    },

                    Atm::Int(n) => {
                        n
                    }
                };

                match kind {
                    ArithmeticKind::Negate => {
                        Some(Atm::Int(0 - arg_value))
                    },

                    _ => {
                        self.add_error(
                            format!("Unexpected arithmetic kind: {:?}", kind)
                        );

                        None
                    }
                }
            }
        }
    }

    fn op_to_arithm(&self, kind: ArithmeticKind, args: Vec<Atm>) -> Arithmetic {
        match kind {
            ArithmeticKind::Add => {
                Arithmetic::Binary(
                    kind,
                    args[0].clone(),
                    args[1].clone()
                )
            },

            ArithmeticKind::Negate => {
                Arithmetic::Unary(
                    kind,
                    args[0].clone(),
                )
            }
        }
    }

    fn extract_i64(&self, atm: &Atm) -> Option<i64> {
        match atm {
            Atm::Int(n) => {
                Some(*n)
            },

            _ => {
                None
            }
        }
    }

    fn extract_var(&self, atm: &Atm) -> Option<Rc<String>> {
        match atm {
            Atm::Var { name } => {
                Some(name.clone())
            },

            _ => {
                None
            }
        }
    }

    fn handle_exp(&mut self, exp: &Exp) -> Option<Atm> {
        match exp {
            Exp::Atm(atm) => {

                match atm {
                    Atm::Int(n) => {
                        Some(Atm::Int(*n))
                    },
                    Atm::Var { .. } => {
                        self.get_var_value(atm)
                    }
                }
            },

            Exp::Prim { op, args } => {
                match &op[..] {
                    "+" => {
                        let operation = self.op_to_arithm(ArithmeticKind::Add, args.clone());
                        self.atm_aritmetic(operation)
                    },

                    "-" => {
                        let operation = self.op_to_arithm(ArithmeticKind::Negate, args.clone());
                        self.atm_aritmetic(operation)
                    },

                    "read" => {
                        let input = get_line();

                        match input.parse::<i64>() {
                            Ok(n) => Some(Atm::Int(n)),
                            Err(error) => {
                                self.add_error(format!("{}", error));
                                None
                            }
                        }
                    },

                    _ => {
                        self.add_error(
                            format!("handle_exp: unknown primitive: {:?}", op)
                        );

                        None
                    }
                }
            },
        }
    }

    fn handle_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Assign (atm, exp) => {
                let maybe_var = self.extract_var(atm);
                let maybe_expr = self.handle_exp(exp);

                let expr = {
                    if let Some(expr) = maybe_expr {
                        expr
                    } else {
                        return;
                    }
                };

                let var = {
                    if let Some(variable) = maybe_var {
                        variable
                    } else {
                        return;
                    }
                };

                self.vars.insert(
                    var.clone(),
                    expr
                );
            }
        }
    }

    fn handle_tail(&mut self, tail: &Tail) -> Option<Atm> {
        match tail {
            Tail::Seq (stmt, tail) => {
                self.handle_stmt(stmt);
                self.handle_tail(tail)
            },

            Tail::Return (exp) => {
                self.handle_exp(exp)
            }
        }
    }

    pub fn has_error(&self) -> bool {
        self.interpretation_error
    }

    pub fn print_errors(&self) {
        for error in &self.errors {
            println!("{}", error);
        }
    }

    pub fn new(cprog: CProgram) -> Self {
        Clang {
            cprog: cprog,
            interpretation_error: false,
            errors: vec!(),
            vars: HashMap::new(),
        }
    }

    pub fn interpret(&mut self) -> Option<i64> {

        let start_label =
            if let Some(tail) = self.cprog.labels.get(&crate::idstr!("start")) {
                Some(tail.clone())
            } else {
                None
            };
        
        if let Some(tail) = start_label {
            let maybe_atm = self.handle_tail(&tail);

            match maybe_atm {
                Some(Atm::Int(n)) => {
                    Some(n)
                },

                _ => {
                    self.add_error(
                        format!("{}:{}:Expected the result of executing the IR to be an i64",
                            crate::function!(),
                            line!()
                        )
                    );

                    None
                }
            }


        } else {
            let err = "entry point 'start' not found!".to_owned();
            self.add_error(err.clone());

            None
        }
    }
}