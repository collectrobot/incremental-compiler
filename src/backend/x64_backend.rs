// the first part of the journey from clang to x64

use std::collections::HashMap;
use std::rc::Rc;

use super::x64_def;

use crate::ir::explicate;

pub struct IRToX64Transformer {
    cprog: explicate::CProgram,
    instructions: HashMap<Rc<String>, Vec<x64_def::Instr>>,
}

mod select_instruction {

    use super::x64_def::*;
    use super::explicate;
    use super::IRToX64Transformer;

    pub fn handle_atom(atm: explicate::Atm) -> Arg {

        ()
    }

    pub fn handle_stmt(trans: &mut IRToX64Transformer, stmt: explicate::Stmt, acc: &mut Vec<Instr>) {

        ()
    }

    pub fn handle_tail(trans: &mut IRToX64Transformer, tail: explicate::Tail, acc: &mut Vec<Instr>) {

        ()
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
            instructions: HashMap::new(),
        }
    }

    pub fn transform(&mut self) {

        for (label, tail) in self.cprog.labels {

            let the_instructions: Vec<x64_def::Instr> = vec!();

            select_instruction::handle_tail(
                self,
                tail,
                &mut the_instructions
            );

            self.instructions.insert(label, the_instructions);
        }
    }
}
