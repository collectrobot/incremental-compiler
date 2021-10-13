extern crate datatypes;

pub mod interp_ast;
pub mod interp_ir;

use std::collections::{HashMap, VecDeque};

use datatypes::{RuntimeI64};

use crate::types::{IdString};

pub type CachedFunctionResult = VecDeque<RuntimeI64>; // this is the cache for a specific function
pub type CachedRuntimeCall = HashMap<IdString, CachedFunctionResult>; // this is a collection of all the cached runtime calls

pub struct InterpResult {
    pub value: Option<RuntimeI64>,
    pub cached_runtime_calls: CachedRuntimeCall,
}