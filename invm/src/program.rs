use std::{fs::File, io::{self, BufRead, BufReader, Lines}};

use crate::{lexer::Lexer, prepro, vm::Register};

pub enum Instruction {
    Set(Register, i32),
    Add(i32, Register),
    Sub(i32, Register),
    Goto(String),
    Gotoz(Register, String),
    Print(i32),
    Push(i32),
    Pop(Register),
    Crash,
    Buy(u32),
    Sell(u32)
}

pub struct Program<'a> {
    lex: Lexer<'a>,
}

impl<'a> Program<'a> {
    pub fn new(query: &'a str) -> Self {
        Program { lex: Lexer::new(query)  }
    }

    pub fn step(&mut self) -> Option<Instruction> {
        let line = self.lex.next()?;
        
        Some(Instruction::Crash)
    }


}


