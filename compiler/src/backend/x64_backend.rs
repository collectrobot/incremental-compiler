// the first part of the journey from c-lang to x64

#![allow(unused)]

use natord;

use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;

use crate::types::{IdString};
use crate::ir::explicate;

use super::x64_def;

// the IRToX64Transformer works on a single function at a time
#[derive(Debug)]
pub struct IRToX64Transformer<'a> {
    externals: RefCell<HashSet<IdString>>,
    cfunc: &'a explicate::IRFunction,
    blocks: HashMap<IdString, x64_def::Block>,
    liveness_set: Vec<IdString>,
    vars: RefCell<HashSet::<x64_def::Home>>,
    rbp_offset: i64,
    prologue_necessary: bool, // do we need a frame pointer ?
    memory_patch: x64_def::Reg, // we might need a register for the case when we end up with an operation taking to memory operands
    mp_used: bool, // flag for above value
}

// map the ir code to x64 instructions 
mod select_instruction {

    use std::collections::{HashSet};

    use crate::types::{IdString};

    use super::x64_def::*;
    use super::IRToX64Transformer;
    use super::explicate::{Atm, Stmt, Tail, Exp};

    impl<'a> IRToX64Transformer<'a> {

        fn handle_atom(&self, atm: &Atm, blk_data: &mut Block) -> Arg {

            match atm {
                Atm::Int(n) => {
                    Arg::Imm(*n)
                },

                Atm::Var { name } => {
                    self.vars.borrow_mut().insert(
                        Home {
                            name: name.clone(),
                            loc: VarLoc::Undefined,
                        }
                    );

                    Arg::Var(name.clone())
                }
            }
        }

        fn handle_stmt(&self, stmt: &Stmt, blk_data: &mut Block) {
            match stmt {
                Stmt::Assign(atm, expr) => {
                    let assignee = self.handle_atom(atm, blk_data);

                    match expr {
                        Exp::Atm(atm) => {
                            let assigned = self.handle_atom(atm, blk_data);
                            blk_data.instr.push(Instr::Mov64(assignee, assigned));
                        }

                        Exp::Prim { op, args } => {

                            match &op[..] {
                                "read" => {
                                    // this function is named "read_int" in the runtime library
                                    let runtime_name = crate::idstr!("read_int");

                                    self.externals.borrow_mut().insert(runtime_name.clone());

                                    blk_data.instr.push(Instr::Call(runtime_name, 0));
                                    blk_data.instr.push(Instr::Mov64(assignee, Arg::Reg(Reg::Rax)));
                                }

                                "-" => {
                                    let assigned = self.handle_atom(&args[0], blk_data);

                                    blk_data.instr.push(Instr::Mov64(assignee.clone(), assigned));
                                    blk_data.instr.push(Instr::Neg64(assignee));
                                },

                                "+" => {
                                    let latm = self.handle_atom(&args[0], blk_data);
                                    let ratm = self.handle_atom(&args[1], blk_data);

                                    if latm == ratm {
                                        blk_data.instr.push(Instr::Add64(assignee, ratm));
                                    } else {
                                        blk_data.instr.push(Instr::Mov64(assignee.clone(), latm));
                                        blk_data.instr.push(Instr::Add64(assignee, ratm));
                                    }
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

        pub fn get_externals(self) -> HashSet<IdString> {
            self.externals.take()
        }

        pub fn select_instruction(&self, tail: &Tail, blk_data: &mut Block) {

            match tail {
                Tail::Seq(stmt, tail) => {
                    self.handle_stmt(stmt, blk_data);
                    self.select_instruction(tail, blk_data);
                },

                Tail::Return(exp) => {

                    match exp {
                        Exp::Atm(atm) => {
                            let the_atom = self.handle_atom(atm, blk_data);
                            blk_data.instr.push(Instr::Mov64(Arg::Reg(Reg::Rax), the_atom));
                        },

                        Exp::Prim { op, args } => {
                            match &op[..] {
                                "read" => {
                                    // this function is named "read_int" in the runtime library
                                    let runtime_name = crate::idstr!("read_int");

                                    self.externals.borrow_mut().insert(runtime_name.clone());

                                    blk_data.instr.push(Instr::Call(runtime_name, 0));
                                },

                                "-" => {
                                    let the_atm = self.handle_atom(&args[0], blk_data);
                                    blk_data.instr.push(Instr::Mov64(Arg::Reg(Reg::Rax), the_atm.clone()));
                                    blk_data.instr.push(Instr::Neg64(Arg::Reg(Reg::Rax)));
                                },
                                "+" => {
                                    let latm = self.handle_atom(&args[0], blk_data);
                                    let ratm = self.handle_atom(&args[1], blk_data);

                                    if latm == ratm {
                                        blk_data.instr.push(Instr::Add64(Arg::Reg(Reg::Rax), ratm));
                                    } else {
                                        blk_data.instr.push(Instr::Mov64(Arg::Reg(Reg::Rax), latm));
                                        blk_data.instr.push(Instr::Add64(Arg::Reg(Reg::Rax), ratm));
                                    }
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

// assign homes to variables
// currently this is just an offset from rbp (i.e. variables live on the stack)
mod assign_homes {

    use std::collections::HashSet;
    use std::cell::{RefCell};

    use super::x64_def::{*};
    use super::IRToX64Transformer;

    impl<'a> IRToX64Transformer<'a> {
        pub fn assign_homes(&mut self) {

            //let liveness_set = self.build_liveness_set(&)

            // sort variables in natural order so we end up with a deterministic
            // variable ordering
            let mut the_vars: Vec<Home> = self.vars.borrow().clone().into_iter().collect();
            the_vars.sort_by(
                |a, b|
                natord::compare(&*a.name, &*b.name)
            );

            let mut found_homes: Vec<Home> = vec!();

            for var in the_vars {
                let mut assigned = var.clone();

                let next_rbp_offset = self.next_rbp_offset();

                assigned.loc = VarLoc::Rbp(next_rbp_offset);

                found_homes.push(assigned);
            }

            if found_homes.len() > 0 {
                self.prologue_necessary = true;
                self.vars = RefCell::new(found_homes.into_iter().collect());
            }
        }
    }
}

// sometimes we need to patch instructions
// e.g. (let ([a 42]) (let ([b a]) b))
// one instruction will be the following
//                 Mov64(Var("b.2"), Var("a.1"))
// x64 does not allow us to issue a mov where both operands are
// memory locations, and so we need to use a register to patch this operation
// we'll use R15 for the time being
// R15 is a callee saved register in both the Windows and System V abi, and so if patching with R15 is done
// we need to save it to the stack beforehand, and restore it after.
mod patch_instructions {

    use super::x64_def::*;
    use super::IRToX64Transformer;

    fn patch(instr: Vec<Instr>) -> (bool, Vec<Instr>) {

        let mut patched_instructions = vec!();
        let mut patched = false;

        for instruction in &instr {
            match instruction {
                Instr::Add64(src, dest) => {
                    patched_instructions.push(instruction.clone());
                },

                Instr::Mov64(src, dest) => {

                    match (src, dest) {

                        (Arg::Var(x), Arg::Var(y)) => {
                            patched_instructions.push(Instr::Mov64(Arg::Reg(Reg::R15), Arg::Var(y.clone())));

                            patched_instructions.push(
                                Instr::Mov64(Arg::Var(x.clone()), Arg::Reg(Reg::R15)),
                            );

                            patched = true;
                        },

                        _ => {
                            patched_instructions.push(instruction.clone());
                        }
                    }
                },

                Instr::Sub64(src, dest) => {
                    patched_instructions.push(instruction.clone());
                }

                _ => {
                    patched_instructions.push(instruction.clone());
                }
            }
        }

        (patched, patched_instructions)
    }

    impl<'a> IRToX64Transformer<'a> {
        pub fn patch_instructions(&mut self) {
            for block in &mut self.blocks {
                let instructions = block.1.instr.clone();

                let (patched, instructions) = patch(instructions);

                if patched {
                    self.mp_used = true;
                    block.1.instr = instructions;
                }
            }
        }
    }
}

impl<'a> IRToX64Transformer<'a> {

    fn next_rbp_offset(&mut self) -> i64 {
        // rbp_offset starts at 0, so need to decrement
        // the offset first, so that rbp isn't overwritten
        self.rbp_offset += 8;

        self.rbp_offset
    }

    pub fn new(cfunc: &'a explicate::IRFunction) -> Self {
        IRToX64Transformer {
            externals: RefCell::new(crate::set!()),
            cfunc: cfunc,
            blocks: HashMap::new(),
            liveness_set: Vec::new(),
            vars: RefCell::new(HashSet::new()),
            rbp_offset: 0,
            prologue_necessary: false,
            memory_patch: x64_def::Reg::R15,
            mp_used: false,
        }
    }

    pub fn transform(&mut self) -> x64_def::Function {

        use super::x64_def::{*};

        for (label, tail) in &self.cfunc.labels {
            let mut block = Block { info: (), instr: vec!() };

            self.select_instruction(&tail, &mut block);

            self.blocks.insert(label.clone(), block);
        }

        // this will let us know if we need to patch the entry point
        self.assign_homes();

        // this might set mp_used
        self.patch_instructions();

        let start = self.blocks.get_mut(&crate::idstr!(".l1")).unwrap();

        let mut fn_start = start.instr.clone();

        let mut fn_end: Vec<Instr> = vec!();

        if self.prologue_necessary {
            // patch the entry function if we need to

            fn_start.insert(0, Instr::Push(Arg::Reg(Reg::Rbp)));
            fn_start.insert(1, Instr::Mov64(Arg::Reg(Reg::Rbp), Arg::Reg(Reg::Rsp)));

            // need to also allocate space for variables, i.e. decrement RSP
            let mut rsp_decrement = 0;
            for home in self.vars.borrow().iter() {
                match home.loc {
                    VarLoc::Rbp(_) => {
                        rsp_decrement += 8;
                    },

                    _ => {}
                }
            }

            if rsp_decrement > 0 {
                fn_start.insert(2, Instr::Sub64(Arg::Reg(Reg::Rsp), Arg::Imm(rsp_decrement)));
            }

            fn_end.push(Instr::Mov64(Arg::Reg(Reg::Rsp), Arg::Reg(Reg::Rbp)));
            fn_end.push(Instr::Pop(Arg::Reg(Reg::Rbp)));

            if self.mp_used {
                fn_start.insert(0, Instr::Push(Arg::Reg(self.memory_patch)));
                fn_end.push(Instr::Pop(Arg::Reg(self.memory_patch)));
            }

        }

        fn_end.push(Instr::Ret);

        fn_start.extend(fn_end);

        start.instr = fn_start;

        let mut vars: Vec<Home> = self.vars.borrow().clone().into_iter().collect();

        Function {
            blocks: self.blocks.to_owned(),
            vars: vars,
        }
    }
}

pub fn ir_to_x64(cprog: explicate::IRProgram) -> x64_def::X64Program {

    let mut externals: HashSet<IdString> = HashSet::new();

    let x64_fns = 
        cprog.functions
        .iter()
        .map(|(name, func)| {

            let mut transformer = IRToX64Transformer::new(func);
            let func = transformer.transform();

            externals.extend(transformer.get_externals());

            return (
                name.clone(),
                func
            )
        }).collect();

    x64_def::X64Program {
        external: externals,
        functions: x64_fns
    }
}
