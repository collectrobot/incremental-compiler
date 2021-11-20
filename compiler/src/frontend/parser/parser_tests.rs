/*

#![allow(dead_code)]
#![allow(unused_imports)]

use crate::frontend::ast::{AstNode, Program, LetBinding};

use crate::utility::{test_ast_helper};

pub fn helper(prog: &'static str) -> Program {
    test_ast_helper(prog, vec!())
}

#[test]
fn parse_constant() {
    let ast = helper("(2)");

    let expected = Program {
        info: (),
        exp: AstNode::Int(2)
    };

    assert_eq!(ast, expected);
}

#[test]
fn parse_add_with_negate() {
    let ast = helper("(+ 2 (-1))");

    let expected = Program {
        info: (),
        exp: AstNode::Prim {
            op: crate::idstr!("+"),
            args: vec!(
                AstNode::Int(2),
                AstNode::Prim {
                    op: crate::idstr!("-"),
                    args: vec!(AstNode::Int(1))
                }
            )
        }
    };

    assert_eq!(ast, expected);
}

#[test]
fn parse_let() {
    let ast = helper("(let ([x 10]) x)");

    let expected = Program {
        info: (),
        exp: AstNode::Let {
            bindings: vec!(
                LetBinding {
                    identifier: crate::idstr!("x"),
                    expr: AstNode::Int(10)
                }
            ),
            body: Box::new(AstNode::Var {
                name: crate::idstr!("x")
            })
        }
    };

    assert_eq!(ast, expected);
}

#[test]
fn parse_nested_let() {
    let ast = helper("(let ([x (let ([y 42]) y)]) x)");

    let var_x = crate::idstr!("x");
    let var_y = crate::idstr!("y");

    let expected = Program {
        info: (),
        exp: AstNode::Let {
            bindings: vec!(
                LetBinding {
                    identifier: var_x.clone(),
                    expr: AstNode::Let {
                        bindings: vec! (
                            LetBinding {
                                identifier: var_y.clone(),
                                expr: AstNode::Int(42)
                            }
                        ),
                        body: Box::new(AstNode::Var { name: var_y.clone() })
                    }
                }
            ),
            body: Box::new(AstNode::Var {
                name: crate::idstr!("x")
            })
        }
    };

    assert_eq!(ast, expected);
}

#[test]
fn parse_fail_expect_leftparen () {
    let ast = helper("2");

    match ast.exp {
        AstNode::Error { msg, .. } => {
            assert_eq!(
                "Expected '(', found '2'".to_owned(),
                *msg
            )
        },

        _ => panic!()
    }
}

#[test]
fn parse_fail_expect_rightbracket () {
    let ast = helper("(let ([x 10) x)");

    match ast.exp {
        AstNode::Error { msg, .. } => {
            assert_eq!(
                "Expected ']', found ')'".to_owned(),
                *msg
            )
        },

        _ => panic!()
    }
}
*/