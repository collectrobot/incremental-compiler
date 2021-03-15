
use crate::ast::{AstNode, Program};

use std::collections::HashMap;

struct Rco {
    num: i64,
}

impl Rco {
    pub fn new() -> Rco {
        Rco {
            num: 0,
        }
    }

    fn tmp(&mut self) -> String {
        let current = self.num;

        let new_tmp_var = "tmp.".to_owned() + &current.to_string();

        self.num += 1;

        new_tmp_var
    }

    fn rco_atom(&mut self, e: AstNode, bindings: &mut Vec<(String, AstNode)>) -> AstNode {

        match &e {
            AstNode::Int(_) => {
                e
            },

            AstNode::Var { .. } => {
                e
            },

            AstNode::Let { bindings, body } => {

                unreachable!()
            },

            AstNode::Prim { op, args } => {

                match &op[..] {
                    "read" | "-" => {
                        // need to bind the read to a variable
                        let new_tmp = self.tmp();

                        let bound_to = AstNode::Prim {
                            op: op.to_owned(),
                            args: args.to_owned()
                        };

                        bindings.push((new_tmp.clone(), bound_to));

                        AstNode::Var {
                            name: new_tmp,
                        }

                    },

                    _ => {

                        let new_tmp = self.tmp();
                        let expr = self.rco_expr(e, bindings);

                        bindings.push((new_tmp.clone(), expr));

                        AstNode::Var {
                            name: new_tmp
                        }
                    }
                }
            },

            _ => {
                unreachable!();
            }
        }
    }

    fn rco_expr(&mut self, e: AstNode, bindings: &mut Vec<(String, AstNode)>) -> AstNode {

        match &e {
            AstNode::Int(_) => {
                e
            }

            AstNode::Var { .. } => {
                e
            },

            AstNode::Prim { op, args } => {

                match &op[..] {
                    "read" => {
                        AstNode::Prim{op:"read".to_owned(),args:vec!()}
                    },

                    "+" => {

                        let mut let_bindings: Vec<(String, AstNode)> = vec!();

                        let lhand = self.rco_atom(args[0].clone(), bindings);
                        let rhand = self.rco_atom(args[1].clone(), bindings);

                        let ast_array: [&AstNode;2] = [&lhand, &rhand];

                        let mut was_atomized = false;

                        for node in &ast_array {
                            match node {
                                AstNode::Int(_) => {},

                                AstNode::Var { name } => {

                                    for binding in  &mut *bindings {
                                        if binding.0 == *name {

                                            let_bindings.push(
                                                (name.clone(), binding.1.clone())
                                            );

                                            was_atomized = true;

                                            break;
                                        }
                                    }

                                },

                                _ => {
                                    println!("{:?}", ast_array);
                                    unreachable!()
                                }
                            }
                        }

                        if was_atomized {
                            AstNode::Let {
                                bindings: let_bindings,
                                body: Box::new(
                                    AstNode::Prim {
                                        op: "+".to_owned(),
                                        args: vec!(lhand, rhand)
                                    }
                                )
                            }
                        } else {
                            e
                        }
                    },

                    _ => {
                        unreachable!();
                    },
                }

            },
            _ => {
                unreachable! {}
            }
        }
    }

    pub fn decomplify(&mut self, p: Program) -> AstNode {
        self.rco_expr(p.exp, &mut Vec::new())
    }
}


pub fn decomplify_program(program: Program) -> Program {

    let mut rco = Rco::new();

    Program {
        info: (),
        exp: rco.decomplify(program),
    }
}