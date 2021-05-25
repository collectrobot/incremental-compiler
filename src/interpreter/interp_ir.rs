use std::collections::HashMap;
use std::rc::Rc;

use crate::ir::explicate::{CProgram, Tail, Stmt, Exp, Atm};

struct Clang {
    interpretation_error: bool,
    errors: Vec<String>,
    cprog: CProgram,
    vars: HashMap<Rc<String>, Atm>,
}

impl Clang {

    fn add_error(&mut self, err: String) -> Result<i64, String> {
        self.interpretation_error = true;
        self.errors.push(err.clone());

        Err(err)
    }

    fn handle_exp(&mut self, exp: &Exp) -> Result<i64, String> {
        Ok(0)
    }

    fn handle_stmt(&mut self, stmt: &Stmt) -> Result<i64, String> {
        match stmt {
            Stmt::Assign (Atm, Exp) => {

            }
        }
    }

    fn handle_tail(&mut self, tail: &Tail) -> Result<i64, String> {
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

    pub fn new(cprog: CProgram) -> Self {
        Clang {
            cprog: cprog,
            interpretation_error: false,
            errors: vec!(),
            vars: HashMap::new(),
        }
    }

    pub fn interpret(&mut self) -> Result<i64, String> {

        let start_label =
            if let Some(tail) = self.cprog.labels.get(&Rc::new("start".to_owned())) {
                Some(tail.clone())
            } else {
                None
            };
        
        if let Some(tail) = start_label {
            self.handle_tail(&tail)
        } else {
            self.add_error("entry point 'start' not found!".to_owned())
        }
    }
}