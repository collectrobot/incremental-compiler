use std::collections::HashMap;

use crate::utility::{test_ir_helper};

use super::explicate::*;

fn helper(prog: &'static str) -> IRProgram {
    test_ir_helper(prog)
}

#[test]
fn explicate_constant() {
    let ir = helper("(123)");

    let mut labels = HashMap::new();

    labels.insert(
        crate::idstr!("start"),
        Tail::Return(
            Exp::Atm(
                Atm::Int(123)
            )
        )
    );

    let expected = IRProgram {
        locals: vec!(),
        labels: labels
    };

    assert_eq!(ir, expected);
}

#[test]
fn explicate_add_constants() {
    let ir = helper("(+ 2 2)");

    let mut labels = HashMap::new();

    labels.insert(
        crate::idstr!("start"),
        Tail::Return(
            Exp::Atm(
                Atm::Int(4)
            )
        )
    );

    let expected = IRProgram {
        locals: vec!(),
        labels: labels
    };

    assert_eq!(ir, expected);
}

#[test]
fn explicate_let_constant() {
    let ir = helper("(let ([x 10]) x)");

    let mut labels = HashMap::new();

    labels.insert(
        crate::idstr!("start"),
        Tail::Return (
            Exp::Atm(
                Atm::Int(10)
            )
        )
    );

    let expected = IRProgram {
        locals: vec!(),
        labels: labels
    };

    assert_eq!(ir, expected);
}

#[test]
fn explicate_let_nested_const() {
    let ir = helper("(let ([x (let ([y 42]) y)]) x)");

    let mut labels = HashMap::new();

    labels.insert(
        crate::idstr!("start"),
        Tail::Return(
            Exp::Atm(
                Atm::Int(42)
            )
        )
    );

    let expected = IRProgram {
        locals: vec!(),
        labels: labels
    };

    assert_eq!(ir, expected);
}

#[test]
fn explicate_add_read() {
    let ir = helper("(+ (read) (read))");

    let mut labels = HashMap::new();

    let tmp = crate::idstr!("tmp.0");
    let tmp1 = crate::idstr!("tmp.1");

    labels.insert(
        crate::idstr!("start"),
        Tail::Seq(
            Stmt::Assign(
                Atm::Var { name: tmp.clone() },
                Exp::Prim { op: crate::idstr!("read"), args: vec!() },
            ),
            Box::new(
                Tail::Seq(
                    Stmt::Assign(
                        Atm::Var { name: tmp1.clone() },
                        Exp::Prim { op: crate::idstr!("read"), args: vec!() },
                    ),
                    Box::new(
                        Tail::Return(
                            Exp::Prim {
                                op: crate::idstr!("+"),
                                args: vec!(Atm::Var {name: tmp.clone()}, Atm::Var {name: tmp1.clone()}),
                            }
                        )
                    )
                ),
            )
        )
    );

    let expected = IRProgram {
        locals: vec!(tmp.clone(), tmp1.clone()),
        labels: labels
    };

    assert_eq!(ir, expected);
}