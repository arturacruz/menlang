use crate::{parser, vm::{GeneralRegister, Register}};

#[derive(Debug)]
pub enum Instruction {
    Set(Register, GeneralRegister),
    Add(GeneralRegister, Register),
    Sub(GeneralRegister, Register),
    Goto(String),
    Gotoz(Register, String),
    Print(GeneralRegister),
    Push(GeneralRegister),
    Pop(Register),
    Crash,
    Buy(u32),
    Sell(u32)
}

pub struct Program {
    lines: Vec<Instruction>
}

impl Program {
    pub fn new(query: &str) -> Self {
        Program { lines: parser::read_lines(query) }
    }

    pub fn step(&mut self) -> Option<Instruction> {
        todo!()
    }
}


