extern crate datatypes;

pub mod interp_ast;
pub mod interp_ir;

use datatypes::{RuntimeI64};

use crate::types::{Environment};

pub struct InterpResult {
    pub value: Option<RuntimeI64>,
    pub environment: Environment,
}