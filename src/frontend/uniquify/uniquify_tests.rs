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

    let x_var_unq = Rc::new("x.1".to_owned());

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

    let x_var_unq = Rc::new("x.1".to_owned());
    let y_var_unq = Rc::new("y.1".to_owned());

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
                    op: Rc::new("+".to_owned()),
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

    let x_var_unq = Rc::new("x.1".to_owned());
    let y_var_unq = Rc::new("y.2".to_owned());

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