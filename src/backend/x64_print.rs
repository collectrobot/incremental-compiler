use super::x64_def::*;

pub struct X64Printer {
    asm: X64Program
}

impl X64Printer {
    pub fn new(asm: X64Program) -> Self {
        Self {
            asm: asm
        }
    }

    pub fn print(&self) -> String {
        let mut test = String::new();

    }
}