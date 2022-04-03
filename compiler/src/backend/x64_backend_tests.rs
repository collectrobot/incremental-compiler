use std::collections::{HashSet};

use super::x64_def::{*};

use super::x64_backend::{AllBlockLiveSet};
use super::x64_backend::liveness::{build_liveness_set_for_block, build_liveness_set};

#[test]
fn x64_build_liveness_set_for_block_one_var() {

    let v_var = Arg::Var(crate::idstr!("v.1"));
    let rax = Arg::Reg(Reg::Rax);

    let instr: Vec<Instr> = vec!(
        Instr::Mov64(v_var.clone(), Arg::Imm(1)), 
        Instr::Mov64(rax.clone(), v_var.clone()),
        Instr::Add64(rax.clone(), Arg::Imm(6)),
        Instr::Ret,
        // {}
    );

    let expected: Vec<HashSet<Arg>> = vec!(
        crate::set!(),
        crate::set!(v_var),
        crate::set!(rax),
        crate::set!()
    );

    // no jump instructions in this block
    let (ls, _) = build_liveness_set_for_block(&instr);

    assert_eq!(ls, expected);
}

#[test]
fn x64_build_liveness_set_read_test() {
    let v_var = Arg::Var(crate::idstr!("v.1"));
    let rax = Arg::Reg(Reg::Rax);

    let instr: Vec<Instr> = vec!(
        Instr::Mov64(v_var.clone(), Arg::Imm(1)), 
        Instr::Add64(v_var.clone(), Arg::Imm(23)), 
        Instr::Add64(v_var.clone(), Arg::Imm(6)), 
        Instr::Mov64(rax.clone(), v_var.clone()),
        Instr::Jmp(crate::idstr!(".conclusion")),
        // {}
    );

    let expected: Vec<HashSet<Arg>> = vec!(
        crate::set!(),
        crate::set!(v_var.clone()),
        crate::set!(v_var.clone()),
        crate::set!(v_var.clone()),
        crate::set!(),
    );

    // no jump instructions in this block
    let (ls, _) = build_liveness_set_for_block(&instr);

    assert_eq!(ls, expected);
}

#[test]
fn x64_build_liveness_set_for_block_six_vars() {

    let v_var = Arg::Var(crate::idstr!("v.1"));
    let w_var = Arg::Var(crate::idstr!("w.1"));
    let x_var = Arg::Var(crate::idstr!("x.1"));
    let y_var = Arg::Var(crate::idstr!("y.1"));
    let z_var = Arg::Var(crate::idstr!("z.1"));
    let t_var = Arg::Var(crate::idstr!("t.1"));
    let rax   = Arg::Reg(Reg::Rax);
    let conclusion = crate::idstr!(".conclusion");

    let instr: Vec<Instr> = vec!(
        Instr::Mov64(v_var.clone(), Arg::Imm(1)), 
        // {v}
        Instr::Mov64(w_var.clone(), Arg::Imm(42)),
        // {v, w}
        Instr::Mov64(x_var.clone(), v_var.clone()),
        // {w, x}
        Instr::Add64(x_var.clone(), Arg::Imm(7)),
        // {w, x}
        Instr::Mov64(y_var.clone(), x_var.clone()),
        // {w, x, y}
        Instr::Mov64(z_var.clone(), x_var.clone()),
        // {w, y, z}
        Instr::Add64(z_var.clone(), w_var.clone()),
        // {y, z}
        Instr::Mov64(t_var.clone(), y_var.clone()),
        // {t, z}
        Instr::Neg64(t_var.clone()),
        // {t, z}
        Instr::Mov64(rax.clone(), z_var.clone()),
        // {t}
        Instr::Add64(rax.clone(), t_var.clone()),
        // {}
        Instr::Jmp(conclusion),
    );

    let expected: Vec<HashSet<Arg>> = vec!(
        crate::set!(),
        crate::set!(v_var.clone()),
        crate::set!(v_var.clone(), w_var.clone()),
        crate::set!(w_var.clone(), x_var.clone()),
        crate::set!(w_var.clone(), x_var.clone()),
        crate::set!(w_var.clone(), x_var.clone(), y_var.clone()),
        crate::set!(w_var.clone(), y_var.clone(), z_var.clone()),
        crate::set!(y_var.clone(), z_var.clone()),
        crate::set!(t_var.clone(), z_var.clone()),
        crate::set!(t_var.clone(), z_var.clone()),
        crate::set!(t_var.clone(), rax),
        crate::set!(),
    );

    let (ls, _) = build_liveness_set_for_block(&instr);

    assert_eq!(ls, expected);
}


#[test]
fn x64_build_liveness_set_one_label() {
    
    let l1 = crate::idstr!(".l1");

    let v_var = Arg::Var(crate::idstr!("v.1"));
    let rax = Arg::Reg(Reg::Rax);

    let lbp = vec!(
        LabelBlockPair {
        label: l1.clone(),
        block: 
            Block {
                info: (),
                instr: vec!(
                    Instr::Mov64(v_var.clone(), Arg::Imm(1)), 
                    Instr::Mov64(rax.clone(), v_var.clone()),
                    Instr::Add64(rax.clone(), Arg::Imm(6)),
                    Instr::Ret,
                    // {}
                )
            }
    });

    let expected: AllBlockLiveSet = crate::map!(
        l1.clone() => vec!(
            crate::set!(),
            crate::set!(v_var),
            crate::set!(rax),
            crate::set!()
        )
    );

    let result = build_liveness_set(&lbp);

    assert_eq!(result, expected);
}

#[test]
fn x64_build_liveness_set_two_labels() {
    
    let l1 = crate::idstr!(".l1");
    let l2 = crate::idstr!(".l2");

    let v_var = Arg::Var(crate::idstr!("v.1"));
    let w_var = Arg::Var(crate::idstr!("w.1"));
    let t_var = Arg::Var(crate::idstr!("t.1"));

    let rax = Arg::Reg(Reg::Rax);

    let lbps = vec!(
        LabelBlockPair {
            label: l1.clone(),
            block: 
                Block {
                    info: (),
                    instr: vec!(
                        Instr::Mov64(t_var.clone(), Arg::Imm(230)),
                        Instr::Mov64(v_var.clone(), Arg::Imm(1)), 
                        Instr::Jmp(l2.clone()),
                        Instr::Neg64(v_var.clone()),
                        Instr::Add64(v_var.clone(), Arg::Imm(6)),
                        Instr::Add64(t_var.clone(), v_var.clone()),
                    )
                }
        },

        LabelBlockPair {
            label: l2.clone(),
            block:
                Block {
                    info: (),
                    instr: vec!(
                        Instr::Mov64(w_var.clone(), Arg::Imm(200)),
                        Instr::Add64(w_var.clone(), t_var.clone()),
                        Instr::Neg64(w_var.clone()),
                        Instr::Mov64(rax.clone(), w_var.clone()),
                        Instr::Add64(rax.clone(), t_var.clone()),
                        Instr::Ret,
                    )
                }
        },
    );

    let expected: AllBlockLiveSet = crate::map!(
        l1.clone() => vec!(
            crate::set!(),
            crate::set!(t_var.clone()),

            // below is the jump instruction
            // its liveness set should be that of the first
            // instruction in the target block/label
            crate::set!(t_var.clone()), 

            crate::set!(v_var.clone(), t_var.clone()),
            crate::set!(v_var.clone(), t_var.clone()),
            crate::set!(v_var.clone(), t_var.clone()),
        ),

        l2.clone() => vec!(
            crate::set!(t_var.clone()),
            crate::set!(w_var.clone(), t_var.clone()),
            crate::set!(w_var.clone(), t_var.clone()),
            crate::set!(w_var.clone(), t_var.clone()),
            crate::set!(rax, t_var),
            crate::set!()
        )
    );

    let result = build_liveness_set(&lbps);

    assert_eq!(result, expected);
}

#[test]
fn x64_build_liveness_const_complete() {

    let rax = Arg::Reg(Reg::Rax);
    let rbp = Arg::Reg(Reg::Rbp);
    let rsp = Arg::Reg(Reg::Rsp);

    let prelude = crate::idstr!(".prelude");
    let l1 = crate::idstr!(".l1");
    let conclusion = crate::idstr!(".conclusion");

    let lbps = vec!(
        LabelBlockPair {
            label: prelude.clone(),
            block: 
                Block {
                    info: (),
                    instr: vec!(
                        Instr::Push(rbp.clone()),
                        Instr::Mov64(rbp.clone(), rsp.clone()),
                        Instr::Jmp(l1.clone()),
                    )
                }
        },

        LabelBlockPair {
            label: l1.clone(),
            block:
                Block {
                    info: (),
                    instr: vec!(
                        Instr::Mov64(rax.clone(), Arg::Imm(2)),
                        Instr::Jmp(conclusion.clone()),
                    )
                }
        },

        LabelBlockPair {
            label: conclusion.clone(),
            block:
                Block {
                    info: (),
                    instr: vec!(
                        Instr::Mov64(rsp.clone(), rbp.clone()),
                        Instr::Pop(rbp.clone()),
                        Instr::Ret,
                    )
                }
        },
    );

    let expected: AllBlockLiveSet = crate::map!(
        prelude.clone() => vec!(
            crate::set!(rsp.clone()),
            crate::set!(rsp.clone()),
            crate::set!(rbp.clone())
        ),

        l1.clone() => vec!(
            crate::set!(rbp.clone()),
            crate::set!(rbp.clone(), rax.clone())
        ),

        conclusion.clone() => vec!(
            crate::set!(rbp, rax),
            crate::set!(),
            crate::set!(),
        )
    );

    let result = build_liveness_set(&lbps);

    assert_eq!(result, expected);
}