use std::rc::Rc;

use crate::frontend::ast::{AstNode, Program, LetBinding};
use crate::frontend::parser::{Parser};
use crate::frontend::lexer::{Lexer};

#[test]
fn parse_constant() {
    let mut parser = Parser::new(
        Lexer::new("(2)").lex()
    );

    let ast = parser.parse();

    let expected = Program {
        info: (),
        exp: AstNode::Int(2)
    };

    assert_eq!(ast, expected);
}

#[test]
fn parse_add_with_negate() {
    let mut parser = Parser::new(
        Lexer::new("(+ 2 (-1))").lex()
    );

    let ast = parser.parse();

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
    let mut parser = Parser::new(
        Lexer::new("(let ([x 10]) x)").lex()
    );

    let ast = parser.parse();

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
    let mut parser = Parser::new(
        Lexer::new("(let ([x (let ([y 42]) y)]) x)").lex()
    );

    let ast = parser.parse();

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
    let mut parser = Parser::new(
        Lexer::new("2").lex()
    );

    let ast = parser.parse();

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
    let mut parser = Parser::new(
        Lexer::new("(let ([x 10) x)").lex()
    );

    let ast = parser.parse();

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