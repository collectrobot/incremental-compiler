use std::rc::Rc;

use crate::frontend::ast::{AstNode, Program, LetBinding};
use crate::frontend::lexer::{Lexer};
use crate::frontend::parser::{Parser};
use super::{uniquify_program};

#[test]
fn uniquify_let() {
    let ast = 
        Parser::new(
            Lexer::new("(let ([x 42]) x)")
            .lex())
        .parse();

    let unique_program = uniquify_program(ast);

    let x_var_unq = crate::idstr!("x.1");

    let expected = Program {
        info: (),
        exp: AstNode::Let {
            bindings: vec!(
                LetBinding {
                    identifier: x_var_unq.clone(),
                    expr: AstNode::Int(42)
                }
            ),

            body: Box::new(AstNode::Var { name: x_var_unq })
        }
    };

    assert_eq!(unique_program, expected);
}

#[test]
fn uniquify_let_addition() {
    let ast = 
        Parser::new(
            Lexer::new("(let ([x 42][y 10]) (+ x y))")
            .lex())
        .parse();

    let unique_program = uniquify_program(ast);

    let x_var_unq = crate::idstr!("x.1");
    let y_var_unq = crate::idstr!("y.1");

    let expected = Program {
        info: (),
        exp: AstNode::Let {
            bindings: vec!(
                LetBinding {
                    identifier: x_var_unq.clone(),
                    expr: AstNode::Int(42)
                },
                LetBinding {
                    identifier: y_var_unq.clone(),
                    expr: AstNode::Int(10)
                }
            ),

            body: Box::new(
                AstNode::Prim {
                    op: crate::idstr!("+"),
                    args: vec!(
                        AstNode::Var { name: x_var_unq },
                        AstNode::Var { name: y_var_unq },
                    )
                }
            )
        }
    };

    assert_eq!(unique_program, expected);
}

#[test]
fn uniquify_nested_let() {
    let ast = 
        Parser::new(
            Lexer::new("(let ([x (let ([y 42]) y)]) x)")
            .lex())
        .parse();

    let unique_program = uniquify_program(ast);

    let x_var_unq = crate::idstr!("x.1");
    let y_var_unq = crate::idstr!("y.2");

    let expected = Program {
        info: (),
        exp: AstNode::Let {
            bindings: vec!(
                LetBinding {
                    identifier: x_var_unq.clone(),
                    expr: AstNode::Let {
                        bindings: vec!(
                            LetBinding {
                                identifier: y_var_unq.clone(),
                                expr: AstNode::Int(42)
                            },
                        ),
                        body: Box::new( AstNode::Var { name: y_var_unq } )
                    }
                }
            ),

            body: Box::new(
                AstNode::Var { name: x_var_unq },
            )
        }
    };

    assert_eq!(unique_program, expected);
}