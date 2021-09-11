use crate::frontend::ast::{AstNode, Program, LetBinding};
use crate::frontend::lexer::{Lexer};
use crate::frontend::parser::{Parser};
use super::{decomplify_program};

#[test]
fn decomplify_addition() {
    let ast = 
    Parser::new(
        Lexer::new("(+ 2 (+ 2 2))")
        .lex())
    .parse(); 

    let decomplified = decomplify_program(ast);

    let tmp = crate::idstr!("tmp.0");

    let expected = Program {
        info: (),
        exp: AstNode::Let {
            bindings: vec!(
                LetBinding {
                    identifier: tmp.clone(),
                    expr: AstNode::Prim {
                        op: crate::idstr!("+"),
                        args: vec!(
                            AstNode::Int(2),
                            AstNode::Int(2)
                        )
                    }
                }
            ),

            body: Box::new(AstNode::Prim {
                op: crate::idstr!("+"),
                args: vec!(
                    AstNode::Int(2),
                    AstNode::Var { name: tmp }
                )
            })
        }
    };

    assert_eq!(decomplified, expected);
}

#[test]
fn decomplify_let_read() {
    let ast = 
    Parser::new(
        Lexer::new("(let ([x 42]) (+ x (read)))")
        .lex())
    .parse(); 

    let decomplified = decomplify_program(ast);

    let tmp = crate::idstr!("tmp.0");
    let x_var = crate::idstr!("x");

    let expected = Program {
        info: (),
        exp: AstNode::Let {
            bindings: vec!(
                LetBinding {
                    identifier: x_var.clone(),
                    expr: AstNode::Int(42)
                }
            ),

            body: Box::new(
                AstNode::Let {
                    bindings: vec!(
                        LetBinding {
                            identifier: tmp.clone(),
                            expr: AstNode::Prim {
                                op: crate::idstr!("read"),
                                args: vec!()
                            }
                        },
                    ),

                    body: 
                        Box::new(AstNode::Prim {
                            op: crate::idstr!("+"),
                            args: vec!(
                                AstNode::Var { name: x_var},
                                AstNode::Var { name: tmp },
                            )
                        })
                    })
                }
    };

    assert_eq!(decomplified, expected);
}