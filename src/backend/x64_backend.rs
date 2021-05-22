// the first part of the journey from clang to x64

use std::collections::HashMap;
use std::rc::Rc;

use super::x64_def;

use crate::ir::explicate;

pub struct IRToX64Transformer {
    cprog: explicate::CProgram,
    blocks: HashMap<Rc<String>, x64_def::Block>,
    vars: Vec<x64_def::Home>,
}

#[derive(Default, Clone, Debug)]
pub struct TransformData {
    vars: Vec<x64_def::Home>,
    instr: Vec<x64_def::Instr>,
}

pub mod select_instruction {

    use super::x64_def::*;
    use super::TransformData;
    use super::IRToX64Transformer;
    use super::explicate::{Atm, Stmt, Tail, Exp};

    pub trait SelectInstruction {
        fn handle_atom(&self, atm: &Atm, td: &mut TransformData) -> Arg;
        fn handle_stmt(&self, stmt: &Stmt, td: &mut TransformData);
        fn handle_tail(&self, tail: &Tail, td: &mut TransformData);
    }

    impl SelectInstruction for IRToX64Transformer {

        fn handle_atom(&self, atm: &Atm, td: &mut TransformData) -> Arg {

            match atm {
                Atm::Int(n) => {
                    Arg::Imm(*n)
                },

                Atm::Var { name } => {
                    td.vars.push(
                        Home {
                            name: name.clone(),
                            loc: VarLoc::Undefined
                        }
                    );

                    Arg::Var(name.clone())
                }
            }
        }

        fn handle_stmt(&self, stmt: &Stmt, td: &mut TransformData) {
            match stmt {
                Stmt::Assign(atm, expr) => {
                    let assignee = self.handle_atom(atm, td);

                    match expr {
                        Exp::Atm(atm) => {
                            let assigned = self.handle_atom(atm, td);
                            td.instr.push(Instr::Mov64(assignee, assigned));
                        }

                        Exp::Prim { op, args } => {

                            match &op[..] {
                                "-" => {
                                    let assigned = self.handle_atom(&args[0], td);

                                    td.instr.push(Instr::Mov64(assignee.clone(), assigned));
                                    td.instr.push(Instr::Neg64(assignee));
                                },

                                "+" => {
                                    let latm = self.handle_atom(&args[0], td);
                                    let ratm = self.handle_atom(&args[1], td);

                                    td.instr.push(Instr::Mov64(assignee.clone(), latm));
                                    td.instr.push(Instr::Add64(assignee, ratm));
                                },

                                _ => {
                                    unreachable!();
                                }
                            }
                        },
                    };
                },
            }
        }

        fn handle_tail(&self, tail: &Tail, td: &mut TransformData) {

            match tail {
                Tail::Seq(stmt, tail) => {
                    self.handle_stmt(stmt, td);
                    self.handle_tail(tail, td);
                },

                Tail::Return(exp) => {

                    match exp {
                        Exp::Atm(atm) => {
                            let the_atom = self.handle_atom(atm, td);
                            td.instr.push(Instr::Mov64(Arg::Reg(Reg::Rax), the_atom));
                            td.instr.push(Instr::Ret);
                        },

                        Exp::Prim { op, args } => {
                            match &op[..] {
                                "read" => {
                                    td.instr.push(Instr::Call(op.clone(), 0));
                                    td.instr.push(Instr::Ret);
                                },

                                "-" => {
                                    let the_atm = self.handle_atom(&args[0], td);
                                    td.instr.push(Instr::Mov64(Arg::Reg(Reg::Rax), the_atm.clone()));
                                    td.instr.push(Instr::Neg64(Arg::Reg(Reg::Rax)));
                                    td.instr.push(Instr::Ret);
                                },
                                "+" => {
                                    let latm = self.handle_atom(&args[0], td);
                                    let ratm = self.handle_atom(&args[1], td);

                                    td.instr.push(Instr::Mov64(Arg::Reg(Reg::Rax), latm));
                                    td.instr.push(Instr::Add64(Arg::Reg(Reg::Rax), ratm));
                                    td.instr.push(Instr::Ret);
                                },

                                _ => {
                                    unimplemented!();
                                }
                            }
                        },
                    }
                }
            }
        }
    }
}

mod assign_homes {

}

mod patch_instructions {

}

impl IRToX64Transformer {
    pub fn new(cprog: explicate::CProgram) -> Self {
        IRToX64Transformer {
            cprog: cprog,
            blocks: HashMap::new(),
            vars: vec!()
        }
    }

    pub fn transform(&mut self) -> x64_def::X64Program {

        use select_instruction::SelectInstruction;

        for (label, tail) in &self.cprog.labels {

            let mut td = TransformData::default();

            self.handle_tail(
                tail,
                &mut td
            );

            self.blocks.insert(
                label.clone(),
                x64_def::Block {
                    info: (),
                    instr: td.instr
                }
            );

            self.vars.extend(td.vars);
        }

        x64_def::X64Program {
            vars: self.vars.to_owned(),
            blocks: self.blocks.to_owned()
        }
    }
}
