#![allow(dead_code)]
#![allow(unused_imports)]

use crate::frontend::ast::{AstNode, Program};
use crate::frontend::lexer::{Lexer};
use crate::frontend::parser::{Parser};

use crate::frontend::uniquify::{uniquify_program};
use crate::frontend::decomplify::{decomplify_program};

use super::{partial_evaluate};

fn helper(prog: &'static str) -> Program {
    partial_evaluate(
        uniquify_program(
            Parser::new(
                Lexer::new(prog)
                .lex())
            .parse()
        )   
    )
}

fn contains_only(ast: &AstNode, should_contain: &AstNode) -> bool {
    if ast == should_contain {
        true
    } else {
        false
    }
}

#[test]
fn partial_evalute_add_constants() {
    let program = helper("(+ 2 2)");

    let expected = 
        Program {
            info: (),
            exp: AstNode::Int(4)
        };

    assert_eq!(
        program,
        expected
    );
}

#[test]
fn partial_eval_add_constant_add_constants() {
    let program = helper("(+ 2 (+ 2 2))");

    let expected = 
        Program {
            info: (),
            exp: AstNode::Int(6)
        };

    assert_eq!(
        program,
        expected
    )
}

#[test]
fn partial_eval_negate_add_constant_negate_constant() {
    let program = helper("(- (+ 3 (- 5))))))");

    let expected = 
        Program {
            info: (),
            exp: AstNode::Int(2)
        };

    assert_eq!(
        program,
        expected
    )
}

#[test]
fn partial_eval_add_constant_read() {
    let program = helper("(+ 2 (read))");

    let expected = 
        Program {
            info: (),
            exp: AstNode::Prim {
                op: crate::idstr!("+"),
                args: vec!(
                    AstNode::Int(2),
                    AstNode::Prim {
                        op: crate::idstr!("read"),
                        args: vec!(),
                    }
                )
            }
        };

    assert_eq!(
        program,
        expected
    )
}

#[test]
fn partial_eval_add_read_negate_add_two_constants() {
    let program = helper("(+ (read) (- (+ 5 3)))");

    let expected = 
        Program {
            info: (),
            exp: AstNode::Prim {
                op: crate::idstr!("+"),
                args: vec!(
                    AstNode::Prim {
                        op: crate::idstr!("read"),
                        args: vec!(),
                    },
                    AstNode::Int(-8)
                )
            }
        };

    assert_eq!(
        program,
        expected
    )
}