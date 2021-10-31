#![allow(dead_code)]
#![allow(unused_imports)]

pub mod interp_ast;
pub mod interp_ir;

use runtime::types::{RuntimeI64, RuntimeValue};

use std::collections::{HashMap, VecDeque};

use crate::types::{IdString};

pub type CachedFunctionResult = VecDeque<RuntimeValue>;
pub type CRC = HashMap<IdString, CachedFunctionResult>;

pub trait Interpretable {
    fn interpret(&mut self) -> InterpretResult;
}

pub struct InterpretResult {
    pub value: Option<RuntimeI64>,
    pub had_error: bool,
    pub errors: Vec<String>,
}

pub struct CachedRuntimeCall {
    result: CRC,
    write: bool,
}

impl CachedRuntimeCall {

    pub fn new() -> Self {
        Self {
            result: CRC::new(),
            write: true,
        }
    }

    pub fn do_write(&mut self, should_i: bool) {
        self.write = should_i;
    }

    pub fn set_crc(mut self, crc: CRC) -> Self {
        self.result = crc;
        self.write = false;

        self
    }

    pub fn get_cached_runtime_fn(&mut self, fn_name: IdString) -> Option<&mut CachedFunctionResult> {
        return self.result.get_mut(&fn_name);
    }

    pub fn get_cached_result_of(&mut self, fn_name: IdString) -> RuntimeValue {
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

    pub fn set_cached_result_of(&mut self, fn_name: IdString, val: RuntimeValue) {
        let maybe_cached_runtime_fn = self.get_cached_runtime_fn(fn_name.clone());

        match maybe_cached_runtime_fn {
            Some(cached_runtime_fn) => {
                cached_runtime_fn.push_back(val);
            },

            _ => {
                let mut new_fn_cache = CachedFunctionResult::new();
                new_fn_cache.push_back(val);
                self.result.insert(fn_name, new_fn_cache);
            }
        }
    }
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