use std::rc::Rc;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::frontend::lexer::{Lexer};
use crate::frontend::parser::{Parser};
use crate::frontend::uniquify::{uniquify_program};
use crate::frontend::decomplify::{decomplify_program};
use crate::ir::explicate::{explicate_control};

use super::x64_def::*;
use super::x64_backend::{IRToX64Transformer};

#[test]
fn x64_constant() {
    let ast = 
    Parser::new(
        Lexer::new("(2)")
        .lex())
    .parse(); 

    let x64_asm =
        IRToX64Transformer::new(
            explicate_control(
                decomplify_program(uniquify_program(ast))
            )
        ).transform();

    //let x_var = Rc::new("x".to_owned());

    let start_label = Rc::new("start".to_owned());

    let block = 
        Block {
            info: (),
            instr: vec!(
                Instr::Mov64(Arg::Reg(Reg::Rax), Arg::Imm(2)),
                Instr::Ret,
            )
        };
    
    let mut blocks = HashMap::new();
    blocks.insert(start_label, block);

    let expected = X64Program {
        vars: HashSet::new(),
        blocks: blocks
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
        ).transform();

    let temp_var = Rc::new("tmp.0".to_owned());

    let start_label = Rc::new("start".to_owned());

    let block = 
        Block {
            info: (),
            instr: vec!(
                Instr::Push(Arg::Reg(Reg::Rbp)),
                Instr::Mov64(Arg::Reg(Reg::Rbp), Arg::Reg(Reg::Rsp)),
                Instr::Mov64(Arg::Var(temp_var.clone()), Arg::Imm(1)),
                Instr::Neg64(Arg::Var(temp_var.clone())),
                Instr::Mov64(Arg::Reg(Reg::Rax), Arg::Imm(2)),
                Instr::Add64(Arg::Reg(Reg::Rax), Arg::Var(temp_var.clone())),
                Instr::Mov64(Arg::Reg(Reg::Rsp), Arg::Reg(Reg::Rbp)),
                Instr::Pop(Arg::Reg(Reg::Rbp)),
                Instr::Ret,
            )
        };

    let mut vars = HashSet::new();
    vars.insert(
        Home {
            name: temp_var,
            loc: VarLoc::Rbp(-8),
        });
    
    let mut blocks = HashMap::new();
    blocks.insert(start_label, block);

    let expected = X64Program {
        vars: vars,
        blocks: blocks
    };

    assert_eq!(x64_asm, expected);
}

#[test]
fn x64_let_nested() {

    // here the patch instruction phase comes into play
    // setting y to x will result in two memory operands

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
        ).transform();

    let x_var = Rc::new("x.1".to_owned());
    let y_var = Rc::new("y.2".to_owned());

    let start_label = Rc::new("start".to_owned());

    let block = 
        Block {
            info: (),
            instr: vec!(
                Instr::Push(Arg::Reg(Reg::Rbp)),
                Instr::Mov64(Arg::Reg(Reg::Rbp), Arg::Reg(Reg::Rsp)),
                Instr::Mov64(Arg::Var(x_var.clone()), Arg::Imm(42)),
                Instr::Mov64(Arg::Reg(Reg::R15), Arg::Var(x_var.clone())),
                Instr::Mov64(Arg::Var(y_var.clone()), Arg::Reg(Reg::R15)),
                Instr::Mov64(Arg::Reg(Reg::Rax), Arg::Var(y_var.clone())),
                Instr::Mov64(Arg::Reg(Reg::Rsp), Arg::Reg(Reg::Rbp)),
                Instr::Pop(Arg::Reg(Reg::Rbp)),
                Instr::Ret,
            )
        };

    let vars = x64_asm.vars.clone();
    
    let mut blocks = HashMap::new();
    blocks.insert(start_label, block);

    let expected = X64Program {
        vars: vars,
        blocks: blocks
    };

    for block in &x64_asm.blocks {
        let x64_block = block.1;
        let exp_block = expected.blocks.get(block.0).unwrap();

        for i in 0..x64_block.instr.len() {
            assert_eq!(x64_block.instr[i], exp_block.instr[i]);
        }
    }
}