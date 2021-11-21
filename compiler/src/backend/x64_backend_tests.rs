use std::collections::{HashSet};

use super::x64_def::{*};

use super::x64_backend::liveness::{build_liveness_set};

#[test]
fn x64_build_liveness_set_one_var() {

    let v_var = Arg::Var(crate::idstr!("v.1"));
    let rax = Arg::Reg(Reg::Rax);

    let instr: Vec<Instr> = vec!(
        Instr::Mov64(v_var.clone(), Arg::Imm(1)), 
        Instr::Mov64(rax.clone(), v_var.clone()),
        Instr::Add64(rax.clone(), Arg::Imm(6)),
        Instr::Ret,
        // {}
    );

    let expected: Vec<HashSet<&Arg>> = vec!(
        crate::set!(),
        crate::set!(&v_var),
        crate::set!(&rax),
        crate::set!()
    );

    let ls = build_liveness_set(&instr);

    assert_eq!(ls, expected);
}

#[test]
fn x64_build_liveness_set_six_vars() {

    let v_var = Arg::Var(crate::idstr!("v.1"));
    let w_var = Arg::Var(crate::idstr!("w.1"));
    let x_var = Arg::Var(crate::idstr!("x.1"));
    let y_var = Arg::Var(crate::idstr!("y.1"));
    let z_var = Arg::Var(crate::idstr!("z.1"));
    let t_var = Arg::Var(crate::idstr!("t.1"));
    let rax   = Arg::Reg(Reg::Rax);

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
        // {t, rax}
        Instr::Add64(rax.clone(), t_var.clone()),
        // {}
        Instr::Ret,
    );

    let expected: Vec<HashSet<&Arg>> = vec!(
        crate::set!(),
        crate::set!(&v_var),
        crate::set!(&v_var, &w_var),
        crate::set!(&w_var, &x_var),
        crate::set!(&w_var, &x_var),
        crate::set!(&w_var, &x_var, &y_var),
        crate::set!(&w_var, &y_var, &z_var),
        crate::set!(&y_var, &z_var),
        crate::set!(&t_var, &z_var),
        crate::set!(&t_var, &z_var),
        crate::set!(&t_var, &rax),
        crate::set!(),
    );

    let ls = build_liveness_set(&instr);

    assert_eq!(ls, expected);
}
