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
                    identifier: Rc::new("x".to_owned()),
                    expr: AstNode::Int(10)
                }
            ),
            body: Box::new(AstNode::Var {
                name: Rc::new("x".to_owned())
            })
        }
    };

    assert_eq!(ast, expected);
}