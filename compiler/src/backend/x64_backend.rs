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

// the blocks kth instruction needs the label's first instruction's liveness set
pub type BlockJumpPatch   = HashMap<(IdString, usize), IdString>; 

pub type BlockLiveSet = Vec<HashSet<x64_def::Arg>>;

pub type AllBlockLiveSet = HashMap<IdString, BlockLiveSet>;

// the IRToX64Transformer works on a single function at a time
#[derive(Debug)]
pub struct IRToX64Transformer<'a> {
    externals: RefCell<HashSet<IdString>>,
    name: IdString,
    cfunc: &'a explicate::IRFunction,
    lbps: Vec<x64_def::LabelBlockPair>,
    liveness_set: AllBlockLiveSet,
    vars: RefCell<HashSet::<x64_def::Home>>,
    rbp_offset: i64,
    prologue_necessary: bool, // do we need a frame pointer ?
    memory_patch: x64_def::Reg, // we might need a register for the case when we end up with an operation taking to memory operands
    mp_used: bool, // flag for above value
}

pub mod liveness {
    use std::collections::{HashSet, HashMap};
    use crate::types::{IdString};
    use super::{BlockJumpPatch, BlockLiveSet, AllBlockLiveSet};
    use super::x64_def::{*};

    fn to_set<'a>(args: Vec<&'a Arg>) -> HashSet<&'a Arg> {
        let s: HashSet<&Arg> = args.iter().map(| a | *a).filter( | a | {
                return match a {
                    Arg::Reg(_) => true,
                    Arg::Var(_) => true,
                    _ => false,
                }
            }
        ).collect();

        s
    }

    fn written_to<'a>(k: usize, i: &'a Vec<Instr>) -> HashSet<&'a Arg> {
        let set = {
            match &i[k] {
                Instr::Mov64(arg, ..) => {
                    to_set(vec!(arg))
                },

                Instr::Add64(arg, ..) => {
                    to_set(vec!(arg))
                },

                Instr::Neg64(arg) => {
                    to_set(vec!(arg))
                }

                _ => {
                    HashSet::new()
                }
            }
        };

        set
    }

    fn read_from<'a>(k: usize, i: &'a Vec<Instr>) -> HashSet<&'a Arg> {
        let set = {

            match &i[k] {
                Instr::Mov64(.., is_read_from) => {
                    to_set(vec!(is_read_from))
                },

                Instr::Add64(is_modified_in_place, is_read_from) => {
                    to_set(vec!(is_modified_in_place, is_read_from))
                },

                Instr::Neg64(is_modified_in_place) => {
                    to_set(vec!(is_modified_in_place))
                },

                _ => {
                    HashSet::new()
                }
            }
        };

        set
    }

    // Lafter (k) = Lbefore (k + 1)
    fn after<'a>(k: usize, i: &'a Vec<Instr>) -> HashSet<&'a Arg> {
        before(k + 1, i)
    }

    // Lbefore(k) = (Lafter(k) – W(k)) ∪R(k)
    fn before<'a>(k: usize, i: &'a Vec<Instr>) -> HashSet<&'a Arg> {

        if k >= i.len() {
            HashSet::new()
        } else {
            let mut before_k = after(k, i);
            let w = written_to(k, i);
            let r = read_from(k, i);

            let diff: HashSet<_> = before_k.difference(&w).collect();
            before_k = diff.iter().map(|val| (*val).clone()).collect();

            let union: HashSet<_> = before_k.union(&r).collect();
            before_k = union.iter().map(|val| (*val).clone()).collect();

            before_k
        }
    }

    pub fn build_liveness_set_for_block<'a>(i: &'a Vec<Instr>) -> (BlockLiveSet, Vec<(usize, IdString)>) {
        let mut jmp_patches: Vec<(usize, IdString)> = vec!();

        let mut ls = BlockLiveSet::new();
        ls.resize(i.len(), crate::set!());

        let n = i.len();

        // ls[k] = the variables that need to be live before this line is executed
        // i.e. if 't.1' is used on line k, it needs to be live (i.e. its home needs to stay the same)
        // until this line (at the very least)
        for k in (0..n).rev() {

            if let Instr::Jmp(label) = &i[k] {
                jmp_patches.push((k, label.clone()));
            }

            let mut b = before(k, i);

            let to_owned: HashSet<_> = b.into_iter().map(|x| x.clone()).collect();

            ls[k] = to_owned;
        }

        (ls, jmp_patches)
    }

    fn patch_jumps(bls: &mut AllBlockLiveSet, patches: &BlockJumpPatch) -> () {
        for ((update_block, position), from_block) in patches {
            let updated_liveness_set = bls.get(from_block).unwrap()[0].clone();

            let block_to_update = bls.get_mut(update_block).unwrap();

            block_to_update[*position] = updated_liveness_set;
        }
    }

    pub fn build_liveness_set(blocks: &Vec<LabelBlockPair>) -> AllBlockLiveSet {
        let mut blocks_live_set = AllBlockLiveSet::new();
        let mut jump_patches = BlockJumpPatch::new();
         
        for lbp in blocks {

            let (block_live_set, jmp_patch) = build_liveness_set_for_block(&lbp.block.instr);
            for (pos, lbl) in &jmp_patch {
                jump_patches.insert((lbp.label.clone(), *pos), lbl.clone());
            }

            blocks_live_set.insert(lbp.label.clone(), block_live_set);
        }

        patch_jumps(&mut blocks_live_set, &jump_patches);

        blocks_live_set
    }
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

    use crate::types::{IdString};

    use super::x64_def::{*};
    use super::IRToX64Transformer;

    impl<'a> IRToX64Transformer<'a> {

        pub fn assign_homes(&mut self) {

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
            for lbp in &mut self.lbps {
                let instructions = lbp.block.instr.clone();

                let (patched, instructions) = patch(instructions);

                if patched {
                    self.mp_used = true;
                    lbp.block.instr = instructions;
                }
            }
        }

        pub fn add_prelude_and_conclusion(&mut self) {
            let pre = crate::idstr!("prelude");
            let con = crate::idstr!("conclusion");

            let prelude = LabelBlockPair {
                label: pre.clone(),
                block: Block {
                    info: (),
                    instr:
                        vec!(
                            Instr::Push(Arg::Reg(Reg::Rbp)),
                            Instr::Mov64(Arg::Reg(Reg::Rbp), Arg::Reg(Reg::Rsp)),
                            Instr::Jmp(crate::idstr!(".l1")),
                        )
                }
            };

            let conclusion = LabelBlockPair {
                label: con.clone(),
                block: Block {
                    info: (),
                    instr:
                        vec!(
                            Instr::Mov64(Arg::Reg(Reg::Rsp), Arg::Reg(Reg::Rbp)),
                            Instr::Pop(Arg::Reg(Reg::Rbp)),
                        )
                }
            };

            let last_label_before_conclusion = self.lbps.len() - 1;
            self.lbps[last_label_before_conclusion].block.instr.push(
                Instr::Jmp(con),
            );

            self.lbps.insert(0, prelude);
            self.lbps.push(conclusion);

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

    pub fn new(name: IdString, cfunc: &'a explicate::IRFunction) -> Self {
        IRToX64Transformer {
            externals: RefCell::new(crate::set!()),
            name: name,
            cfunc: cfunc,
            lbps: Vec::new(),
            liveness_set: AllBlockLiveSet::new(),
            vars: RefCell::new(HashSet::new()),
            rbp_offset: 0,
            prologue_necessary: false,
            memory_patch: x64_def::Reg::R15,
            mp_used: false,
        }
    }

    pub fn transform(&mut self) -> x64_def::Function {

        use super::x64_def::{*};
        use super::x64_backend::liveness;

        for (label, tail) in &self.cfunc.labels {
            let mut block = Block { info: (), instr: vec!() };

            self.select_instruction(&tail, &mut block);

            self.lbps.push(LabelBlockPair::new(label.clone(), block));
        }

        // this means .l1 will appear before .l2 and so on
        // and so self.lbps[self.lbps.len()-1] will be the conclusion
        // (which we might need to change)
        self.lbps.sort_by(
            |a, b|
            natord::compare(&*a.label, &*b.label)
        );

        self.add_prelude_and_conclusion();

        // this might set mp_used
        self.patch_instructions();

        self.liveness_set = liveness::build_liveness_set(&self.lbps);

        // this will let us know if we need to patch the entry point
        self.assign_homes();

        let prologue_index = 0;
        let conclusion_index = self.lbps.len()-1;

        if self.prologue_necessary {
            // patch the entry function if we need to

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
                let rsp_sub_index = 2;
                self.lbps[prologue_index].block.instr.insert(rsp_sub_index, Instr::Sub64(Arg::Reg(Reg::Rsp), Arg::Imm(rsp_decrement)));
                self.lbps[conclusion_index].block.instr.insert(0, Instr::Sub64(Arg::Reg(Reg::Rsp), Arg::Imm(rsp_decrement)));
            }

            if self.mp_used {
                self.lbps[prologue_index].block.instr.push(Instr::Push(Arg::Reg(self.memory_patch)));
                self.lbps[conclusion_index].block.instr.push(Instr::Pop(Arg::Reg(self.memory_patch)));
            }
        }

        self.lbps[conclusion_index].block.instr.push(Instr::Ret);

        let mut vars: Vec<Home> = self.vars.borrow().clone().into_iter().collect();

        Function {
            blocks: self.lbps.to_owned(),
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

            let mut transformer = IRToX64Transformer::new(name.clone(), func);
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
