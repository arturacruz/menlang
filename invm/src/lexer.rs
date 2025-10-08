use std::{iter::Peekable, panic, str::Chars};

use crate::vm::{Register, Sensor};

#[derive(Debug)]
pub enum Token {
    Value(i32),
    Reg(Register),
    Sens(Sensor),
    Set, Add, Sub, Goto, Gotoz, Print, Push, Pop, Buy, Sell, Crash, Mult, Div,
    LabelDeclare(String),
    Label(String),
    Endline
}

pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(query: &'a str) -> Self {
        Lexer { source: query.chars().peekable() } 
    }

    /// Groups the numbers in a query together and returns the Token::Value containing that value.
    fn parse_number(&mut self, c: char) -> Token {
        let mut number = String::from(c);

        loop {
            // Peeks here so that the character is not consumed.
            let next = self.source.peek();
            if next.is_none() || !next.unwrap().is_numeric() {
                let n = number.parse::<i32>().unwrap();
                return Token::Value(n);
            } 
            // Consumes here, in case it is a number.
            number.push(self.source.next().unwrap());
        }

    }

    fn get_keyword(s: String) -> Token {
        match s.as_str() {
            "SET" => Token::Set,
            "ADD" => Token::Add,
            "SUB" => Token::Sub,
            "MULT" => Token::Mult,
            "DIV" => Token::Div,
            "GOTO" => Token::Goto,
            "GOTOZ" => Token::Gotoz,
            "PRINT" => Token::Print,
            "PUSH" => Token::Push,
            "POP" => Token::Pop,
            "CRASH" => Token::Crash,
            "BUY" => Token::Buy,
            "SELL" => Token::Sell,
            "FUND1" => Token::Reg(Register::Fund1),
            "FUND2" => Token::Reg(Register::Fund2),
            "SHARES" => Token::Sens(Sensor::Shares),
            "STOCKPRICE" => Token::Sens(Sensor::Stockprice),
            "REPUTATION" => Token::Sens(Sensor::Reputation),
            "MARKETVAL" => Token::Sens(Sensor::MarketValue),
            "EQUITY" => Token::Sens(Sensor::Equity),
            "OWNED" => Token::Sens(Sensor::Owned),
            "BALANCE" => Token::Sens(Sensor::Balance),
            _ => panic!("[Lexer] Unknown instruction or register: {s}.")
        }
    }

    fn parse_keyword(&mut self, c: char) -> Token {
        let mut iden = String::from(c);
        loop {
            let peek = self.source.peek();
            let next = match peek {
                Some(s) => s,
                None => return Lexer::get_keyword(iden)
            };
            match next { 
                ':' => {
                    self.source.next();
                    return Token::LabelDeclare(iden);
                }
                'A'..='Z' | 'a'..='z' | '_' | '0'..='9' => iden.push(*next),
                _ => return Lexer::get_keyword(iden),
            }
            self.source.next();
        }
    }

    fn parse_label(&mut self) -> Token {
        let mut iden = String::new();
        loop {
            let peek = self.source.peek();
            let next = match peek {
                Some(s) => s,
                None => return Token::Label(iden),
            };
            match next { 
                'A'..='Z' | 'a'..='z' | '_' | '0'..='9' => iden.push(*next),
                _ => return Token::Label(iden),
            }
            self.source.next();
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut char = self.source.next()?;

        loop {
            match char {
                ' ' | '\t' => char = self.source.next()?,
                '\n' => return Some(Token::Endline),
                '$' => return Some(self.parse_label()),
                ':' => panic!("[Lexer] Empty label identifier."),
                '0'..='9' => return Some(self.parse_number(char)),
                'a'..='z' | 'A'..='Z' => return Some(self.parse_keyword(char)),
                _ => panic!("[Lexer] Invalid symbol: {char}.")
            }
        }
    }
}
