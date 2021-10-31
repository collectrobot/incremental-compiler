/*
    this pass changes certain operations to only take atoms (literals or variables) as
    their operands

    i.e. (+ 2 (+ 2 2)) will be change to

        let [tmp.0 (+ 2 2)]
            (+ 2 tmp.0)

*/

#[cfg(test)]
mod decomplify_tests;

use crate::types::{IdString};

use super::ast::{AstNode, LetBinding, Program};

struct Rco {
    num: i64,
    env: Vec<(IdString, AstNode)>,
}

impl Rco {
    pub fn new() -> Rco {
        Rco {
            num: 0,
            env: vec!()
        }
    }

    fn tmp(&mut self) -> IdString {
        let current = self.num;

        let new_tmp_var = "tmp.".to_owned() + &current.to_string();

        self.num += 1;

        crate::idstr!(new_tmp_var)
    }

    fn env_get(&self, find: &String) -> Option<AstNode> {
        for binding in &self.env {
            if **binding.0 == *find {
                return Some(binding.1.clone())
            }
        }

        return None
    }

    fn env_set(&mut self, name: IdString, expr: AstNode) {
        self.env.push((name, expr));
    }

    // returns (true, AstNode) if whatever was passed in had to be atomized
    fn rco_atom(&mut self, e: AstNode) -> (bool, AstNode) {

        match &e {
            // already an atom
            AstNode::Int(_) => {
                (false, e)
            },

            // already an atom
            AstNode::Var { .. } => {
                (false, e)
            },

            // we need a tmp variable to bind the let expression to
            AstNode::Let { .. } => {
                let new_tmp = self.tmp();
                let expr = self.rco_expr(e);

                self.env_set(new_tmp.clone(), expr);

                (true, AstNode::Var {
                    name: new_tmp
                })
            },

            AstNode::Prim { op, .. } => {

                match &op[..] {
                    "+" => {
                        let new_tmp = self.tmp();
                        let expr = self.rco_expr(e);

                        self.env_set(new_tmp.clone(), expr);

                        (true, AstNode::Var {
                            name: new_tmp
                        })
                    },

                    "read" | "-" => {
                        let new_tmp = self.tmp();

                        self.env_set(new_tmp.clone(), e);

                        (true, AstNode::Var {
                            name: new_tmp,
                        })

                    },

                    _ => {
                        unreachable!();
                    }
                }
            },

            _ => {
                unreachable!();
            }
        }
    }

    fn rco_expr(&mut self, e: AstNode) -> AstNode {

        match &e {
            AstNode::Int(_) => {
                e
            }

            AstNode::Var { .. } => {
                e
            },

            AstNode::Let { bindings, body } => {

                let original_bindings = bindings.clone();

                let mut untouched_bindings = vec!();

                /*
                    this will contain new bindings that are created when
                    the exp in the let binding was atomized

                    the original expression will be turned into a new let expression

                    i.e. let [x (+ 2 (-10))] will be turned into

                    let [tmp.0 (-10)] [x (+ 2 tmp.0)]

                */
                let mut changed_bindings: Vec<LetBinding> = Vec::new();

                // check if any of the existing bindings need to be atomized
                for i in 0..original_bindings.len() {

                    let current_binding = original_bindings[i].clone();

                    let maybe_new_binding = self.rco_expr(current_binding.expr);

                    match maybe_new_binding {

                        // a tmp binding was needed because of atomization
                        AstNode::Let { mut bindings, body } => {

                            let var_name = current_binding.identifier;

                            changed_bindings.append(&mut bindings);
                            changed_bindings.push(
                                LetBinding {
                                    identifier: var_name,
                                    expr: *body
                                }
                            );
                        }

                        // nothing needed to be done, keep the old binding as it was
                        _ => {
                            untouched_bindings.push(original_bindings[i].clone());
                        }
                    }
                }

                let new_body = self.rco_expr(*body.clone());

                untouched_bindings.append(&mut changed_bindings);

                let new_node =
                    AstNode::Let {
                        bindings: untouched_bindings,
                        body: Box::new(new_body),
                    };

                new_node
            },

            AstNode::Prim { op, args } => {

                match &op[..] {
                    "read" => {
                        e
                    },

                    // potentially need to atomize args[0]
                    "-" => {
                        let arg = self.rco_atom(args[0].clone());

                        if arg.0 == true {

                            let var_name = 
                                match &arg.1 {
                                    AstNode::Var { name } => {
                                        name.clone()
                                    }
                                    
                                    _ => {
                                        unreachable!();
                                    }
                                };

                            let let_binding: Vec<LetBinding> = vec!(
                                LetBinding {
                                    identifier: var_name.clone(),
                                    expr: self.env_get(&*var_name).unwrap()
                                }
                            );

                            AstNode::Let {
                                bindings: let_binding,
                                body: Box::new(
                                    AstNode::Prim {
                                        op: op.clone(),
                                        args: vec!(arg.1)
                                    }
                                )
                            }
                        } else {
                            e
                        }
                    },

                    "+" => {

                        let mut let_bindings: Vec<LetBinding> = vec!();

                        let lhand = self.rco_atom(args[0].clone());
                        let rhand = self.rco_atom(args[1].clone());

                        let results = vec!(&lhand, &rhand);

                        let mut was_atomized = false;

                        for node in &results {
                            match node {
                                (_, AstNode::Int(_)) => {},

                                (atm, AstNode::Var { name }) => {

                                    if *atm {
                                        match self.env_get(&(**name)) {
                                            Some(expr) => {
                                                let_bindings.push(
                                                    LetBinding {
                                                        identifier: name.clone(),
                                                        expr: expr
                                                    }
                                                );

                                                was_atomized = true;
                                            },

                                            _ => {
                                                panic!("rco_expr:{}: tmp var '{}' binding not found.", line!(), name);
                                            }
                                        }
                                    }
                                },

                                _ => {
                                    unreachable!()
                                },
                            }
                        }

                        if was_atomized {
                            AstNode::Let {
                                bindings: let_bindings,
                                body: Box::new(
                                    AstNode::Prim {
                                        op: op.clone(),
                                        args: vec!(lhand.1, rhand.1)
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
        self.rco_expr(p.exp)
    }
}


pub fn decomplify_program(program: Program) -> Program {

    let mut rco = Rco::new();

    Program {
        info: (),
        exp: rco.decomplify(program),
    }
}