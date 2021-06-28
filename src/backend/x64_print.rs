use std::collections::HashMap;

use crate::types::{IdString};

use super::x64_def::*;

pub struct X64Printer {
    asm: X64Program,
}

impl X64Printer {

    fn get_var(&self, name: &IdString) -> Option<&Home> {
        for home in &self.asm.vars {
            if home.name == *name {
                return Some(home)
            }
        }

        None
    }

    fn reg_to_string(&self, reg: &Reg) -> String {
        match reg {
            Reg::Rsp => {
                "rsp".to_owned()
            },
            
            Reg::Rbp => {
                "rbp".to_owned()
            },
            
            Reg::Rax => {
                "rax".to_owned()
            },
            
            Reg::Rbx => {
                "rbx".to_owned()
            },

            Reg::Rcx => {
                "rcx".to_owned()
            },
            
            Reg::Rdx => {
                "rdx".to_owned()
            },
            
            Reg::Rsi => {
                "rsi".to_owned()
            },
            
            Reg::Rdi => {
                "rdi".to_owned()
            },

            Reg::R8 => {
                "r8".to_owned()
            },

            Reg::R9 => {
                "r9".to_owned()
            },

            Reg::R10 => {
                "r10".to_owned()
            },

            Reg::R11 => {
                "r11".to_owned()
            },
            
            Reg::R12 => {
                "r12".to_owned()
            },

            Reg::R13 => {
                "r13".to_owned()
            },

            Reg::R14 => {
                "r14".to_owned()
            },

            Reg::R15 => {
                "r15".to_owned()
            }
        }
    }

    fn arg_to_string(&self, arg: &Arg) -> String {
        match arg {
            Arg::Var(name) => {
                let the_var = self.get_var(name).unwrap();

                match &the_var.loc {
                    VarLoc::Reg(reg) => {
                        self.reg_to_string(reg)
                    },

                    VarLoc::Rbp(offset) => {
                        format!("qword [rbp-{}]", offset)
                    },

                    VarLoc::Undefined => {
                        panic!(
                            format!("Undefined variable location for '{}'", name)
                        );
                    }
                }
            },

            Arg::Imm(int64) => {
                int64.to_string()
            },

            Arg::Reg(reg) => {
                self.reg_to_string(reg)
            },

            _ => {
                panic!("Unknown/unused argument {:?}", arg)
            }
        }
    }

    fn instr_to_text(&self, instr: &Instr) -> String {
        match instr {
            Instr::Add64(arg1, arg2) => {
                format!(
                    "add {}, {}\n",
                    self.arg_to_string(arg1),
                    self.arg_to_string(arg2)
                )
            },

            Instr::Sub64(arg1, arg2) => {
                format!(
                    "sub {}, {}\n",
                    self.arg_to_string(arg1),
                    self.arg_to_string(arg2)
                )
            },

            Instr::Mov64(arg1, arg2) => {
                format!(
                    "mov {}, {}\n",
                    self.arg_to_string(arg1),
                    self.arg_to_string(arg2)
                )
            },

            Instr::Neg64(arg) => {
                format!(
                    "neg {}\n",
                    self.arg_to_string(arg),
                )
            },

            Instr::Call(func, _) => {
                format!(
                    "call {}\n",
                    func
                )
            },

            Instr::Ret => {
                "ret\n".to_owned()
            },

            Instr::Push(arg) => {
                format!(
                    "push {}\n",
                    self.arg_to_string(arg)
                )
            },

            Instr::Pop(arg) => {
                format!(
                    "pop {}\n",
                    self.arg_to_string(arg)
                )
            },

            Instr::Jmp(label) => {
                format!(
                    "jmp {}\n",
                    label
                )
            }

        }
    }

    pub fn new(asm: X64Program) -> Self {
        Self {
            asm: asm
        }
    }

    pub fn print(&self) -> String {
        let mut program = String::new();

        let mut external_functions: Vec<String> = vec!();

        let ext_prepend = "extern ".to_owned();

        for ext in &self.asm.external {
            let mut new_ext = ext_prepend.clone();
            new_ext.push_str(&*ext.clone());
            external_functions.push(new_ext);
        }

        let globals = vec!("global start".to_owned());

        if external_functions.len() > 0 {
            program += &external_functions.join("\n");
            program += "\n\n";
        }

        program += &globals.join("\n");
        program += "\n\n";

        program += "section .text";
        program += "\n\n";

        for (label, block) in &self.asm.blocks {
            program += label;
            program += ":\n";

            for instr in &block.instr {
                program += "    ";
                program += &self.instr_to_text(instr);
            }
        }

        program
    }
}