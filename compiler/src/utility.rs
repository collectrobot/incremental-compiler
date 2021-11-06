#![allow(dead_code)]
#![allow(unused_imports)]

use crate::frontend::ast::{Program};
use crate::frontend::lexer::{Lexer};
use crate::frontend::parser::{Parser};

use crate::frontend::uniquify::{uniquify_program};
use crate::frontend::partial_eval::{partially_evaluate};
use crate::frontend::decomplify::{decomplify_program};
use crate::ir::explicate::{explicate_control, IRProgram};
use crate::backend::x64_backend::{IRToX64Transformer};
use crate::backend::x64_def::{X64Program};

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum AstStep {
    Uniquify,
    PartialEvaluation,
    Decomplify,
}

pub fn test_ast_helper(prog: &'static str, transform: Vec<AstStep>) -> Program {
    let mut p = Parser::new(Lexer::new(prog).lex()).parse();

    for step in transform {
        match step {
            AstStep::Uniquify => {
                p = uniquify_program(p);
            },

            AstStep::PartialEvaluation => {
                p = partially_evaluate(p);
            },

            AstStep::Decomplify => {
                p = decomplify_program(p);
            }
        }
    }

    p
}

pub fn test_ir_helper(prog: &'static str) -> IRProgram {
    let p = test_ast_helper(prog, vec!(AstStep::Uniquify, AstStep::PartialEvaluation, AstStep::Decomplify));

    explicate_control(p)
}

pub fn test_x64_helper(prog: &'static str) -> X64Program {
    let ir = test_ir_helper(prog);

    IRToX64Transformer::new(ir).transform()
}


// credits to https://stackoverflow.com/a/63904992 for this macro
#[macro_export]
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);

        // Find and cut the rest of the path
        match &name[..name.len() - 3].rfind(':') {
            Some(pos) => &name[pos + 1..name.len() - 3],
            None => &name[..name.len() - 3],
        }
    }};
}

#[macro_export]
macro_rules! idstr {
    ($s:expr) => {
        crate::types::IdString::new($s.to_owned())
    }
}

#[macro_export]
macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };

    {} => {
        std::collections::HashMap::new()
    }
);

#[macro_export]
macro_rules! set(
    { $($key:expr),+ } => {
        {
            let mut m = std::collections::HashSet::new();
            $(
                m.insert($key);
            )+
            m
        }
     };

    {} => {
        std::collections::HashSet::new()
    }
);