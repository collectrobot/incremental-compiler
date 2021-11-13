#![allow(dead_code)]
#![allow(unused_imports)]

pub mod interp_ast;

use runtime::types::{RuntimeI64, RuntimeValue};

use std::collections::{HashMap, VecDeque};

use crate::types::{IdString};

pub trait Interpretable {
    fn interpret(&mut self) -> InterpretResult;
}

pub struct InterpretResult {
    pub value: Option<RuntimeI64>,
    pub had_error: bool,
    pub errors: Vec<String>,
}

pub struct Interpreter<'a> {
    program: &'a mut dyn Interpretable,
}

impl<'a> Interpreter<'a> {

    pub fn run(&mut self) -> InterpretResult {
        self.program.interpret()
    }

    pub fn new(prog: &'a mut dyn Interpretable) -> Self {
        Self {
            program: prog,
        }
    }
}