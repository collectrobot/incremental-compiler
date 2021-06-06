// the first part of the journey from clang to x64

#![allow(dead_code)]

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
    prologue_necessary: bool, // do we need a frame pointer ?
    memory_patch: x64_def::Reg, // we might need a register for the case when we end up with an operation taking to memory operands
    mp_used: bool, // flag for above value
}

#[derive(Default, Clone, Debug)]
pub struct FunctionData {
    vars: HashSet<x64_def::Home>,
    instr: Vec<x64_def::Instr>,
}

mod select_instruction {

    use super::x64_def::*;
    use super::FunctionData;
    use super::IRToX64Transformer;
    use super::explicate::{Atm, Stmt, Tail, Exp};

    impl IRToX64Transformer {
        pub fn handle_atom(&self, atm: &Atm, fn_data: &mut FunctionData) -> Arg {

            match atm {
                Atm::Int(n) => {
                    Arg::Imm(*n)
                },

                Atm::Var { name } => {
                    fn_data.vars.insert(
                        Home {
                            name: name.clone(),
                            loc: VarLoc::Undefined
                        }
                    );

                    Arg::Var(name.clone())
                }
            }
        }

        pub fn handle_stmt(&self, stmt: &Stmt, fn_data: &mut FunctionData) {
            match stmt {
                Stmt::Assign(atm, expr) => {
                    let assignee = self.handle_atom(atm, fn_data);

                    match expr {
                        Exp::Atm(atm) => {
                            let assigned = self.handle_atom(atm, fn_data);
                            fn_data.instr.push(Instr::Mov64(assignee, assigned));
                        }

                        Exp::Prim { op, args } => {

                            match &op[..] {
                                "read" => {
                                    fn_data.instr.push(Instr::Call(op.clone(), 0));
                                    fn_data.instr.push(Instr::Mov64(assignee, Arg::Reg(Reg::Rax)));
                                }

                                "-" => {
                                    let assigned = self.handle_atom(&args[0], fn_data);

                                    fn_data.instr.push(Instr::Mov64(assignee.clone(), assigned));
                                    fn_data.instr.push(Instr::Neg64(assignee));
                                },

                                "+" => {
                                    let latm = self.handle_atom(&args[0], fn_data);
                                    let ratm = self.handle_atom(&args[1], fn_data);

                                    fn_data.instr.push(Instr::Mov64(assignee.clone(), latm));
                                    fn_data.instr.push(Instr::Add64(assignee, ratm));
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

        pub fn handle_tail(&self, tail: &Tail, fn_data: &mut FunctionData) {

            match tail {
                Tail::Seq(stmt, tail) => {
                    self.handle_stmt(stmt, fn_data);
                    self.handle_tail(tail, fn_data);
                },

                Tail::Return(exp) => {

                    match exp {
                        Exp::Atm(atm) => {
                            let the_atom = self.handle_atom(atm, fn_data);
                            fn_data.instr.push(Instr::Mov64(Arg::Reg(Reg::Rax), the_atom));
                        },

                        Exp::Prim { op, args } => {
                            match &op[..] {
                                "read" => {
                                    fn_data.instr.push(Instr::Call(op.clone(), 0));
                                },

                                "-" => {
                                    let the_atm = self.handle_atom(&args[0], fn_data);
                                    fn_data.instr.push(Instr::Mov64(Arg::Reg(Reg::Rax), the_atm.clone()));
                                    fn_data.instr.push(Instr::Neg64(Arg::Reg(Reg::Rax)));
                                },
                                "+" => {
                                    let latm = self.handle_atom(&args[0], fn_data);
                                    let ratm = self.handle_atom(&args[1], fn_data);

                                    fn_data.instr.push(Instr::Mov64(Arg::Reg(Reg::Rax), latm));
                                    fn_data.instr.push(Instr::Add64(Arg::Reg(Reg::Rax), ratm));
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
            prologue_necessary: false,
            memory_patch: x64_def::Reg::R15,
            mp_used: false
        }
    }

    pub fn transform(&mut self) -> x64_def::X64Program {

        use x64_def::*;

        for (label, tail) in &self.cprog.labels {

            let mut fn_data = FunctionData::default();

            self.handle_tail(
                tail,
                &mut fn_data
            );

            let mut fn_start: Vec<Instr> = vec!();
            let mut fn_end: Vec<Instr> = vec!();

            if self.prologue_necessary {
                fn_start.push(Instr::Push(Arg::Reg(Reg::Rbp)));
                fn_start.push(Instr::Mov64(Arg::Reg(Reg::Rbp), Arg::Reg(Reg::Rsp)));

                fn_end.push(Instr::Mov64(Arg::Reg(Reg::Rsp), Arg::Reg(Reg::Rbp)));
                fn_end.push(Instr::Pop(Arg::Reg(Reg::Rbp)));

                if self.mp_used {
                    fn_start.insert(0, Instr::Push(Arg::Reg(self.memory_patch)));
                    fn_end.push(Instr::Pop(Arg::Reg(self.memory_patch)));
                }
            }

            fn_end.push(Instr::Ret);

            fn_start.extend(fn_data.instr);
            fn_start.extend(fn_end);

            self.blocks.insert(
                label.clone(),
                Block {
                    info: (),
                    instr: fn_start
                }
            );

            self.vars.extend(fn_data.vars);
        }

        X64Program {
            vars: self.vars.to_owned(),
            blocks: self.blocks.to_owned()
        }
    }
}
