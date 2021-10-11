extern crate datatypes;

pub mod interp_ast;
pub mod interp_ir;

use std::collections::{HashMap, VecDeque};

use datatypes::{RuntimeI64};

use crate::types::{IdString};

pub type CachedRuntimeCalls = HashMap<IdString, VecDeque<RuntimeI64>>;

pub struct InterpResult {
    pub value: Option<RuntimeI64>,
    pub cached_runtime_calls: CachedRuntimeCalls,
}