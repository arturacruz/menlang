use crate::{lexer::{Lexer, Token}, vm::{Condition, GeneralRegister, Instruction, Reference}};

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

    fn expect_read_only(&mut self, inst: &str, usage: &str) -> Reference {
        match self.lex.next() {
            Some(Token::Reference) => {
                let r = match self.lex.next() {
                    Some(Token::Reg(r)) => GeneralRegister::Register(r),
                    Some(Token::Value(n)) => GeneralRegister::Value(n),
                    Some(Token::Sens(s)) => GeneralRegister::Sensor(s),
                    _ => panic!("[Parser] Expected a register or number in {inst} instruction: {usage}."),
                };
                Reference::Address(r)
            }
            Some(Token::Reg(r)) => Reference::Register(r),
            Some(Token::Value(n)) => Reference::Value(n),
            Some(Token::Sens(s)) => Reference::Sensor(s),
            _ => panic!("[Parser] Expected a register or number in {inst} instruction: {usage}."),
        }
    }

    fn expect_write(&mut self, inst: &str, usage: &str) -> Reference {
        match self.lex.next() {
            Some(Token::Reference) => {
                let r = match self.lex.next() {
                    Some(Token::Reg(r)) => GeneralRegister::Register(r),
                    Some(Token::Value(n)) => GeneralRegister::Value(n),
                    Some(Token::Sens(s)) => GeneralRegister::Sensor(s),
                    _ => panic!("[Parser] Expected a register in {inst} instruction: {usage}."),
                };
                Reference::Address(r)
            }
            Some(Token::Reg(r)) => Reference::Register(r),
            _ => panic!("[Parser] Expected a register in number {inst} instruction: {usage}."),
        }

    }
    fn match_instruction(&mut self) -> Option<Instruction> {
        let inst = loop {
            break match self.lex.next()? {
                Token::Endline => continue,
                Token::Set => self.set(),
                Token::Add => self.add(),
                Token::Sub => self.sub(),
                Token::Mult => self.mult(),
                Token::Div => self.div(),
                Token::Goto => self.goto(),
                Token::GoIf => self.go_if(),
                Token::Print => self.print(),
                Token::Push => self.push(),
                Token::Pop => self.pop(),
                Token::Crash => self.crash(),
                Token::Buy => self.buy(),
                Token::Sell => self.sell(),
                Token::LabelDeclare(n) => Instruction::DeclareLabel(n),
                Token::Label(n) => panic!("[Parser] Incorrect use of label {n}."),
                n => panic!("[Parser] Unknown instruction {n:?}.")
            };
        };
        self.expect_endline();
        Some(inst)
    }

    fn set(&mut self) -> Instruction {
        let inst = "SET";
        let usage = "SET *R/n *R";
        let reg = self.expect_write(inst, usage);
        let value = self.expect_read_only(inst, usage);

        Instruction::Set(reg, value)
    }

    fn add(&mut self) -> Instruction {
        let inst = "ADD";
        let usage = "ADD *R/n *R";
        let value = self.expect_read_only(inst, usage);
        let reg = self.expect_write(inst, usage);

        Instruction::Add(value, reg)
    }

    fn sub(&mut self) -> Instruction {
        let inst = "SUB";
        let usage = "SUB *R/n *R";
        let value = self.expect_read_only(inst, usage);
        let reg = self.expect_write(inst, usage);

        Instruction::Sub(value, reg)
    }

    fn mult(&mut self) -> Instruction {
        let inst = "MULT";
        let usage = "MULT *R/n *R";
        let value = self.expect_read_only(inst, usage);
        let reg = self.expect_write(inst, usage);

        Instruction::Mult(value, reg)
    }

    fn div(&mut self) -> Instruction {
        let inst = "DIV";
        let usage = "DIV *R/n *R";
        let value = self.expect_read_only(inst, usage);
        let reg = self.expect_write(inst, usage);

        Instruction::Div(value, reg)
    }

    fn goto(&mut self) -> Instruction {
        match self.lex.next() {
            Some(Token::Label(s)) => Instruction::Goto(s),
            _ => panic!("[Parser] Expected a label in GOTO instruction: GOTO label.")
        }
    }

    fn go_if(&mut self) -> Instruction {
        let cond = match self.lex.next() {
            Some(Token::Equals) => Condition::Equals,
            Some(Token::Greater) => Condition::Greater,
            Some(Token::Lesser) => Condition::Lesser,
            Some(Token::GreaterOrEqual) => Condition::GreaterOrEqual,
            Some(Token::LesserOrEqual) => Condition::LesserOrEqual,
            Some(Token::Different) => Condition::Different,
            n => panic!("[Parser] Expected a Condition in GOIF instruction (GOIF COND R label), got {n:?}"),
        };

        let reg = self.expect_read_only("GOIF", "GOIF COND *R/n label");

        let label = match self.lex.next() {
            Some(Token::Label(s)) => s,
            _ => panic!("[Parser] Expected a label in GOIF instruction: GOIF R label.")
        };

        Instruction::GoIf(cond, reg, label)
    }

    fn print(&mut self) -> Instruction {
        let val = self.expect_read_only("PRINT", "PRINT *R/n type");

        let t = match self.lex.next() {
            Some(Token::Type(t)) => t,
            _ => panic!("[Parser] Expected a type after PRINT instruction (PRINT R/n type).")
        };
        Instruction::Print(val, t)
    }

    fn push(&mut self) -> Instruction {
        let val = self.expect_read_only("PUSH", "PUSH *R/n");
        Instruction::Push(val)

    }

    fn pop(&mut self) -> Instruction {
        let reg = self.expect_write("POP", "POP *R");
        Instruction::Pop(reg)
    }

    fn crash(&mut self) -> Instruction {
        Instruction::Crash
    }

    fn buy(&mut self) -> Instruction {
        let val = match self.lex.next() {
            Some(Token::Value(n)) => n,
            _ => panic!("[Parser] Expected a positive value in BUY instruction: BUY n.")
        };
        Instruction::Buy(val)
    }

    fn sell(&mut self) -> Instruction {
        let val = match self.lex.next() {
            Some(Token::Value(n)) => n,
            _ => panic!("[INVM] Expected a positive value in SELL instruction: SELL n.")
        };
        Instruction::Sell(val)
    }
}
