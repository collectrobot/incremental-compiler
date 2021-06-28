use std::rc::Rc;
use std::collections::HashMap;

use crate::frontend::lexer::{Lexer};
use crate::frontend::parser::{Parser};
use crate::frontend::uniquify::{uniquify_program};
use crate::frontend::decomplify::{decomplify_program};
use super::explicate::*;

#[test]
fn explicate_constant() {
    let ast = 
    Parser::new(
        Lexer::new("(123)")
        .lex())
    .parse(); 

    let unq_decomplified = decomplify_program(uniquify_program(ast));

    let ir = explicate_control(unq_decomplified);

    let mut labels = HashMap::new();

    labels.insert(
        crate::idstr!("start"),
        Tail::Return(
            Exp::Atm(
                Atm::Int(123)
            )
        )
    );

    let expected = CProgram {
        locals: vec!(),
        labels: labels
    };

    assert_eq!(ir, expected);
}

#[test]
fn explicate_add_constants() {
    let ast = 
    Parser::new(
        Lexer::new("(+ 2 2)")
        .lex())
    .parse(); 

    let unq_decomplified = decomplify_program(uniquify_program(ast));

    let ir = explicate_control(unq_decomplified);

    let mut labels = HashMap::new();

    labels.insert(
        crate::idstr!("start"),
        Tail::Return(
            Exp::Atm(
                Atm::Int(4)
            )
        )
    );

    let expected = CProgram {
        locals: vec!(),
        labels: labels
    };

    assert_eq!(ir, expected);
}

#[test]
fn explicate_let() {
    let ast = 
    Parser::new(
        Lexer::new("(let ([x 10]) x)")
        .lex())
    .parse(); 

    let unq_decomplified = decomplify_program(uniquify_program(ast));

    let ir = explicate_control(unq_decomplified);

    let mut labels = HashMap::new();

    let x_var = crate::idstr!("x.1");

    labels.insert(
        crate::idstr!("start"),
        Tail::Seq(
            Stmt::Assign(
                Atm::Var { name: x_var.clone() },
                Exp::Atm(Atm::Int(10))
            ),
            Box::new(
                Tail::Return(
                    Exp::Atm(
                        Atm::Var { name: x_var.clone() }
                    )
                )
            )
        )
    );

    let expected = CProgram {
        locals: vec!(x_var.clone()),
        labels: labels
    };

    assert_eq!(ir, expected);
}

#[test]
fn explicate_let_nested() {
    let ast = 
    Parser::new(
        Lexer::new("(let ([x (let ([y 42]) y)]) x)")
        .lex())
    .parse(); 

    let unq_decomplified = decomplify_program(uniquify_program(ast));

    let ir = explicate_control(unq_decomplified);

    let mut labels = HashMap::new();

    let x_var = crate::idstr!("x.1");
    let y_var = crate::idstr!("y.2");

    labels.insert(
        crate::idstr!("start"),
        Tail::Seq(
            Stmt::Assign(
                Atm::Var { name: y_var.clone() },
                Exp::Atm(Atm::Int(42))
            ),
            Box::new(
                Tail::Seq(
                    Stmt::Assign(
                        Atm::Var { name: x_var.clone() },
                        Exp::Atm(Atm::Var { name: y_var.clone() })
                    ),
                    Box::new(
                        Tail::Return(
                            Exp::Atm(
                                Atm::Var { name: x_var.clone() }
                            )
                        )
                    )
                ),
            )
        )
    );

    let expected = CProgram {
        locals: vec!(x_var, y_var),
        labels: labels
    };

    assert_eq!(ir, expected);
}