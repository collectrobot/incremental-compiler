/*
use crate::frontend::ast::{AstNode, Program, LetBinding};
use crate::utility::{test_ast_helper, AstStep};


fn helper(prog: &'static str) -> Program {
    test_ast_helper(
        prog,
        vec!(AstStep::Uniquify, AstStep::PartialEvaluation, AstStep::Decomplify)
    )
}

#[test]
fn decomplify_addition() {
    let decomplified = helper("(+ x (+ x y))");

    let tmp = crate::idstr!("tmp.0");

    let x = crate::idstr!("x");
    let y = crate::idstr!("y");

    let expected = Program {
        info: (),
        exp: AstNode::Let {
            bindings: vec!(
                LetBinding {
                    identifier: tmp.clone(),
                    expr: AstNode::Prim {
                        op: crate::idstr!("+"),
                        args: vec!(
                            AstNode::Var { name: x.clone() },
                            AstNode::Var { name: y.clone() },
                        )
                    }
                }
            ),

            body: Box::new(AstNode::Prim {
                op: crate::idstr!("+"),
                args: vec!(
                    AstNode::Var { name: x.clone() },
                    AstNode::Var { name: tmp }
                )
            })
        }
    };

    assert_eq!(decomplified, expected);
}

#[test]
fn decomplify_let_read() {
    let decomplified = helper("(let ([x 42]) (+ x (read)))");

    let tmp = crate::idstr!("tmp.0");
    let x_var = crate::idstr!("x.1");

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
*/