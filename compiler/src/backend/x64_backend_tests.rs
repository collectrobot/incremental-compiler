/*

use crate::frontend::lexer::{Lexer};
use crate::frontend::parser::{Parser};
use crate::frontend::uniquify::{uniquify_program};
use crate::frontend::decomplify::{decomplify_program};
use crate::ir::explicate::{explicate_control};

use super::x64_def::*;
use crate::utility::{test_x64_helper};

fn helper(prog: &'static str) -> X64Program {
    test_x64_helper(prog)
}

#[test]
fn x64_ret_constant() {
    let x64_asm = helper("(2)");
    let block = 
        Block {
            info: (),
            instr: vec!(
                Instr::Mov64(Arg::Reg(Reg::Rax), Arg::Imm(2)),
                Instr::Ret
            )
        };

    let expected = X64Program {
        external: crate::set!(),
        //vars: vec!(),
        blocks: crate::map!(start_label => block)
    };

    assert_eq!(x64_asm, expected);
}

#[test]
fn x64_add_negate() {
    let ast = 
    Parser::new(
        Lexer::new("(+ 2 (-1))")
        .lex())
    .parse(); 

    let x64_asm =
        IRToX64Transformer::new(
            explicate_control(
                decomplify_program(uniquify_program(ast))
            )
        )
        .transform();

    let temp_var = crate::idstr!("tmp.0");

    let start_label = crate::idstr!("start");

    let block = 
        Block {
            info: (),
            instr: vec!(
                Instr::Push(Arg::Reg(Reg::Rbp)),
                Instr::Mov64(Arg::Reg(Reg::Rbp), Arg::Reg(Reg::Rsp)),
                Instr::Sub64(Arg::Reg(Reg::Rsp), Arg::Imm(8)),
                Instr::Mov64(Arg::Var(temp_var.clone()), Arg::Imm(1)),
                Instr::Neg64(Arg::Var(temp_var.clone())),
                Instr::Mov64(Arg::Reg(Reg::Rax), Arg::Imm(2)),
                Instr::Add64(Arg::Reg(Reg::Rax), Arg::Var(temp_var.clone())),
                Instr::Mov64(Arg::Reg(Reg::Rsp), Arg::Reg(Reg::Rbp)),
                Instr::Pop(Arg::Reg(Reg::Rbp)),
                Instr::Ret
            )
        };

    //let vars = x64_asm.vars.clone();

    let expected = X64Program {
        external: crate::set!(),
        //vars: vars,
        blocks: crate::map!(start_label => block)
    };

    assert_eq!(x64_asm, expected);
}

#[test]
fn x64_patch_instruction() {

    // here the patch instruction phase comes into play
    // setting y to x will result in two memory operands
    // and so we need to use the patch register to first move one
    // value into the patch register, and then use the patch register as one of the operands

    let ast = 
    Parser::new(
        Lexer::new("(let ([x 42]) (let ([y x]) y))")
        .lex())
    .parse(); 

    let x64_asm =
        IRToX64Transformer::new(
            explicate_control(
                decomplify_program(uniquify_program(ast))
            )
        )
        .transform();

    let x_var = crate::idstr!("x.1");
    let y_var = crate::idstr!("y.2");

    let start_label = crate::idstr!("start");

    let block = 
        Block {
            info: (),
            instr: vec!(
                Instr::Push(Arg::Reg(Reg::R15)),
                Instr::Push(Arg::Reg(Reg::Rbp)),
                Instr::Mov64(Arg::Reg(Reg::Rbp), Arg::Reg(Reg::Rsp)),
                Instr::Sub64(Arg::Reg(Reg::Rsp), Arg::Imm(16)),
                Instr::Mov64(Arg::Var(x_var.clone()), Arg::Imm(42)),
                Instr::Mov64(Arg::Reg(Reg::R15), Arg::Var(x_var.clone())),
                Instr::Mov64(Arg::Var(y_var.clone()), Arg::Reg(Reg::R15)),
                Instr::Mov64(Arg::Reg(Reg::Rax), Arg::Var(y_var.clone())),
                Instr::Mov64(Arg::Reg(Reg::Rsp), Arg::Reg(Reg::Rbp)),
                Instr::Pop(Arg::Reg(Reg::Rbp)),
                Instr::Pop(Arg::Reg(Reg::R15)),
                Instr::Ret
            )
        };

    //let vars = x64_asm.vars.clone();

    let expected = X64Program {
        external: crate::set!(),
        //vars: vars,
        blocks: crate::map!(start_label => block)
    };

    for block in &x64_asm.blocks {
        let x64_block = block.1;
        let exp_block = expected.blocks.get(block.0).unwrap();

        for i in 0..x64_block.instr.len() {
            assert_eq!(x64_block.instr[i], exp_block.instr[i]);
        }
    }
}
*/