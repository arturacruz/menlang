use std::collections::HashMap;

use crate::{program::Instruction, stack::Stack};

pub enum Register {
    Fund1, Fund2
}

pub enum Sensor {
    Shares, 
    Stockprice,
    Reputation,
    MarketValue,
    Equity,
    Owned,
    Balance
}

struct InvestmentVM {
    registers: HashMap<Register, i32>,
    sensors: HashMap<Sensor, i32>,
    program: Vec<Instruction>,
    labels: HashMap<String, usize>,
    pc: usize,
    stack: Stack
}
