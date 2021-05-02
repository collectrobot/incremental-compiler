/*
    this pass changes certain operations to only take atoms (literals or variables) as
    their operands

    i.e. (+ 2 (+ 2 2)) will be change to

        let [tmp.0 (+ 2 2)]
            (+ 2 tmp.0)

*/
use crate::ast::{AstNode, Program};

struct Rco {
    num: i64,
    env: Vec<(String, AstNode)>,
}

impl Rco {
    pub fn new() -> Rco {
        Rco {
            num: 0,
            env: vec!()
        }
    }

    fn tmp(&mut self) -> String {
        let current = self.num;

        let new_tmp_var = "tmp.".to_owned() + &current.to_string();

        self.num += 1;

        new_tmp_var
    }

    fn env_get(&self, find: &String) -> Option<AstNode> {
        for binding in &self.env {
            if binding.0 == *find {
                return Some(binding.1.clone())
            }
        }

        return None
    }

    fn env_set(&mut self, name: String, expr: AstNode) {
        self.env.push((name, expr));
    }

    fn rco_atom(&mut self, e: AstNode) -> (bool, AstNode) {

        match &e {
            AstNode::Int(_) => {
                (false, e)
            },

            AstNode::Var { .. } => {
                (false, e)
            },

            AstNode::Let { .. } => {
                unreachable!()
            },

            AstNode::Prim { op, args } => {

                match &op[..] {
                    "read" | "-" => {
                        let new_tmp = self.tmp();

                        let bound_to = AstNode::Prim {
                            op: op.to_owned(),
                            args: args.to_owned()
                        };

                        self.env_set(new_tmp.clone(), bound_to);

                        (true, AstNode::Var {
                            name: new_tmp,
                        })

                    },

                    _ => {

                        let new_tmp = self.tmp();
                        let expr = self.rco_expr(e);

                        self.env_set(new_tmp.clone(), expr);

                        (true, AstNode::Var {
                            name: new_tmp
                        })
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
                let mut changed_bindings: Vec<(String, AstNode)> = Vec::new();

                // check if any of the existing bindings need to be atomized
                for i in 0..original_bindings.len() {

                    let current_binding = original_bindings[i].clone();

                    let maybe_new_binding = self.rco_expr(current_binding.1);

                    match maybe_new_binding {

                        // a tmp binding was needed because of atomization
                        AstNode::Let { mut bindings, body } => {

                            let var_name = current_binding.0;

                            changed_bindings.append(&mut bindings);
                            changed_bindings.push(
                                (var_name, *body)
                            );
                        }

                        // nothing needed to be done, keep the old binding as it was
                        _ => {
                            untouched_bindings.push(original_bindings[i].clone());
                        }
                    }
                }

                let new_body = self.rco_expr(*body.clone());

                changed_bindings.append(&mut untouched_bindings);

                let new_node =
                    AstNode::Let {
                        bindings: changed_bindings,
                        body: Box::new(new_body),
                    };

                new_node
            },

            AstNode::Prim { op, args } => {

                match &op[..] {
                    "read" | "-" => {
                        e
                    },

                    "+" => {

                        let mut let_bindings: Vec<(String, AstNode)> = vec!();

                        let lhand = self.rco_atom(args[0].clone());
                        let rhand = self.rco_atom(args[1].clone());

                        let results = vec!(&lhand, &rhand);

                        let mut was_atomized = false;

                        for node in &results {
                            match node {
                                (_, AstNode::Int(_)) => {},

                                (atm, AstNode::Var { name }) => {

                                    if *atm {
                                        match self.env_get(name) {
                                            Some(expr) => {
                                                let_bindings.push(
                                                    (name.clone(), expr)
                                                );

                                                was_atomized = true;
                                            },

                                            _ => {
                                                println!("{}", name);
                                                panic!("(internal)rco_expr [{}]: tmp var binding not found.", line!());
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
                                        op: "+".to_owned(),
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