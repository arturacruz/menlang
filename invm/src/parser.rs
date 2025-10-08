use std::slice::Iter;

use crate::{lexer::{Lexer, Token}, vm::{GeneralRegister, Instruction}};

struct Parser<'a> {
    lex: Lexer<'a>
}

pub fn read_lines(query: &str) -> Vec<Instruction> {
    let mut parser = Parser::new(query);
    parser.read_lines()
}

impl<'a> Parser<'a> {
    fn new(query: &'a str) -> Self {
        let lex = Lexer::new(query);
        Parser { lex }
    }

    fn read_lines(&mut self) -> Vec<Instruction> {
        let mut vec = vec![];
        while let Some(instruction) = self.match_instruction() {
            vec.push(instruction);
        }
        vec
    }

    fn expect_endline(&mut self) {
        match self.lex.next() {
            Some(Token::Endline) | None => (),
            n => panic!("[INVM] Expected endline after instruction, got {n:?}.")
        }
    }

    fn match_instruction(&mut self) -> Option<Instruction> {
        let instruction = match self.lex.next()? {
            Token::Set => self.set(),
            Token::Add => self.add(),
            Token::Sub => self.sub(),
            Token::Goto => self.goto(),
            Token::Gotoz => self.gotoz(),
            Token::Print => self.print(),
            Token::Push => self.push(),
            Token::Pop => self.pop(),
            Token::Crash => self.crash(),
            Token::Buy => self.buy(),
            Token::Sell => self.sell(),
            Token::Label(n) => Instruction::DeclareLabel(n),
            n => panic!("[INVM] Unknown instruction {n:?}.")
        };
        self.expect_endline();
        Some(instruction)
    }

    fn set(&mut self) -> Instruction {
        let reg = match self.lex.next() {
            Some(Token::Reg(r)) => r,
            _ => panic!("[INVM] Expected a register in SET instruction: SET R n."),
        };

        let value = match self.lex.next() {
            Some(Token::Reg(r)) => GeneralRegister::Register(r),
            Some(Token::Value(n)) => GeneralRegister::Value(n),
            Some(Token::Sens(s)) => GeneralRegister::Sensor(s),
            _ => panic!("[INVM] Expected a register or number in SET instruction: SET R n."),
        };

        Instruction::Set(reg, value)

    }

    fn add(&mut self) -> Instruction {
        let value = match self.lex.next() {
            Some(Token::Reg(r)) => GeneralRegister::Register(r),
            Some(Token::Value(n)) => GeneralRegister::Value(n),
            Some(Token::Sens(s)) => GeneralRegister::Sensor(s),
            _ => panic!("[INVM] Expected a register or number in ADD instruction: ADD R1/n R2."),
        };

        let reg = match self.lex.next() {
            Some(Token::Reg(r)) => r,
            _ => panic!("[INVM] Expected a register in ADD instruction: ADD R1/n R2."),
        };

        Instruction::Add(value, reg)
    }

    fn sub(&mut self) -> Instruction {
        let value = match self.lex.next() {
            Some(Token::Reg(r)) => GeneralRegister::Register(r),
            Some(Token::Value(n)) => GeneralRegister::Value(n),
            Some(Token::Sens(s)) => GeneralRegister::Sensor(s),
            _ => panic!("[INVM] Expected a register or number in SUB instruction: SUB R1/n R2."),
        };

        let reg = match self.lex.next() {
            Some(Token::Reg(r)) => r,
            _ => panic!("[INVM] Expected a register in SUB instruction: SUB R1/n R2."),
        };

        Instruction::Sub(value, reg)
    }

    fn goto(&mut self) -> Instruction {
        match self.lex.next() {
            Some(Token::Label(s)) => Instruction::Goto(s),
            _ => panic!("[INVM] Expected a label in GOTO instruction: GOTO label.")
        }
    }

    fn gotoz(&mut self) -> Instruction {
        let reg = match self.lex.next() {
            Some(Token::Reg(r)) => GeneralRegister::Register(r),
            Some(Token::Value(n)) => GeneralRegister::Value(n),
            Some(Token::Sens(s)) => GeneralRegister::Sensor(s),
            _ => panic!("[INVM] Expected a register or value in GOTOZ instruction: GOTOZ R label."),
        };

        let label = match self.lex.next() {
            Some(Token::Label(s)) => s,
            _ => panic!("[INVM] Expected a label in GOTOZ instruction: GOTOZ R label.")
        };

        Instruction::Gotoz(reg, label)
    }

    fn print(&mut self) -> Instruction {
        let val = match self.lex.next() {
            Some(Token::Reg(r)) => GeneralRegister::Register(r),
            Some(Token::Value(n)) => GeneralRegister::Value(n),
            Some(Token::Sens(s)) => GeneralRegister::Sensor(s),
            _ => panic!("[INVM] Expected a register or value in PRINT instruction: PRINT R/n.")
        };
        Instruction::Print(val)
    }

    fn push(&mut self) -> Instruction {
        let val = match self.lex.next() {
            Some(Token::Reg(r)) => GeneralRegister::Register(r),
            Some(Token::Value(n)) => GeneralRegister::Value(n),
            Some(Token::Sens(s)) => GeneralRegister::Sensor(s),
            _ => panic!("[INVM] Expected a register or value in PUSH instruction: PUSH R/n.")
        };
        Instruction::Push(val)

    }

    fn pop(&mut self) -> Instruction {
        let reg = match self.lex.next() {
            Some(Token::Reg(r)) => r,
            _ => panic!("[INVM] Expected a register in POP instruction: POP R.")
        };
        Instruction::Pop(reg)
    }

    fn crash(&mut self) -> Instruction {
        Instruction::Crash
    }

    fn buy(&mut self) -> Instruction {
        let val = match self.lex.next() {
            Some(Token::Value(n)) => n,
            _ => panic!("[INVM] Expected a positive value in BUY instruction: BUY n.")
        };
        Instruction::Buy(val.try_into().expect("[INVM] Required a positive value in BUY instruction."))
    }

    fn sell(&mut self) -> Instruction {
        let val = match self.lex.next() {
            Some(Token::Value(n)) => n,
            _ => panic!("[INVM] Expected a positive value in SELL instruction: SELL n.")
        };
        Instruction::Sell(val.try_into().expect("[INVM] Required a positive value in SELL instruction."))
    }
}
