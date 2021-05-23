// the first part of the journey from clang to x64

use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

use super::x64_def;

use crate::ir::explicate;

pub struct IRToX64Transformer {
    cprog: explicate::CProgram,
    blocks: HashMap<Rc<String>, x64_def::Block>,
    vars: HashSet<x64_def::Home>,
    rbp_offset: i64,
    prologue_tag: Rc::<String>,
    epilogue_tag: Rc::<String>,
}

#[derive(Default, Clone, Debug)]
pub struct TransformData {
    vars: HashSet<x64_def::Home>,
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
                    td.vars.insert(
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
                                "read" => {
                                    td.instr.push(Instr::Call(op.clone(), 0));
                                    td.instr.push(Instr::Mov64(assignee, Arg::Reg(Reg::Rax)));
                                }

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

            //println!("{:?}", tail);

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
                            //td.instr.push(Instr::Call(self.epilogue_tag.clone(), 0));
                            //td.instr.push(Instr::Ret);
                        },

                        Exp::Prim { op, args } => {
                            match &op[..] {
                                "read" => {
                                    td.instr.push(Instr::Call(op.clone(), 0));
                                    //td.instr.push(Instr::Call(self.epilogue_tag.clone(), 0));
                                    //td.instr.push(Instr::Ret);
                                },

                                "-" => {
                                    let the_atm = self.handle_atom(&args[0], td);
                                    td.instr.push(Instr::Mov64(Arg::Reg(Reg::Rax), the_atm.clone()));
                                    td.instr.push(Instr::Neg64(Arg::Reg(Reg::Rax)));
                                    //td.instr.push(Instr::Call(self.epilogue_tag.clone(), 0));
                                    //td.instr.push(Instr::Ret);
                                },
                                "+" => {
                                    let latm = self.handle_atom(&args[0], td);
                                    let ratm = self.handle_atom(&args[1], td);

                                    td.instr.push(Instr::Mov64(Arg::Reg(Reg::Rax), latm));
                                    td.instr.push(Instr::Add64(Arg::Reg(Reg::Rax), ratm));
                                    //td.instr.push(Instr::Call(self.epilogue_tag.clone(), 0));
                                    //td.instr.push(Instr::Ret);
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
    fn next_rbp_offset(&mut self) -> i64 {
        let old_value = self.rbp_offset;

        self.rbp_offset += -8;

        old_value
    }

    pub fn new(cprog: explicate::CProgram) -> Self {
        IRToX64Transformer {
            cprog: cprog,
            blocks: HashMap::new(),
            vars: HashSet::new(),
            rbp_offset: 0,
            prologue_tag: Rc::new("prologue".to_owned()),
            epilogue_tag: Rc::new("epilogue".to_owned()),
        }
    }

    pub fn transform(&mut self) -> x64_def::X64Program {

        use x64_def::*;
        use select_instruction::SelectInstruction;

        // setup for start of function
        // push old rbp
        // move new rsp into rbp
        let mut fn_start: Vec<Instr> = vec!(
            Instr::Push(Arg::Reg(Reg::Rbp)),
            Instr::Mov64(Arg::Reg(Reg::Rbp), Arg::Reg(Reg::Rsp)),
        );

        for (label, tail) in &self.cprog.labels {

            let mut td = TransformData::default();

            self.handle_tail(
                tail,
                &mut td
            );

            self.blocks.insert(
                label.clone(),
                Block {
                    info: (),
                    instr: td.instr
                }
            );

            self.vars.extend(td.vars);
        }

        // we're done, set rsp to rbp, pop it, then return
        let fn_end: Vec<Instr> = vec!(
            Instr::Mov64(Arg::Reg(Reg::Rsp), Arg::Reg(Reg::Rbp)),
            Instr::Pop(Arg::Reg(Reg::Rbp)),
            Instr::Ret
        );

        let start_label = Rc::new("start".to_owned());

        let start_fn = self.blocks.get_mut(&start_label).unwrap();

        let old_info = start_fn.info;

        fn_start.extend(start_fn.instr.clone());
        fn_start.extend(fn_end);

        self.blocks.insert(
            start_label,
            Block {
                info: old_info,
                instr: fn_start
            }
        );

        X64Program {
            vars: self.vars.to_owned(),
            blocks: self.blocks.to_owned()
        }
    }
}
