use std::collections::HashMap;

use rand::Rng;

use crate::{parser, stack::Stack};

pub fn run(query: &str) {
    let mut vm = VM::new(query);
    while !vm.crash {
        vm.step();
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Set(Register, GeneralRegister),
    Add(GeneralRegister, Register),
    Sub(GeneralRegister, Register),
    Mult(GeneralRegister, Register),
    Div(GeneralRegister, Register),
    Goto(String),
    Gotoz(GeneralRegister, String),
    Print(GeneralRegister),
    Push(GeneralRegister),
    Pop(Register),
    Crash,
    Buy(i32),
    Sell(i32),
    DeclareLabel(String)
}

#[derive(Debug, Clone)]
pub enum GeneralRegister {
    Register(Register),
    Sensor(Sensor),
    Value(i32)
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Register {
    Fund1, Fund2
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Sensor {
    Shares, 
    Stockprice,
    Reputation,
    MarketValue,
    Equity,
    Owned,
    Balance
}

struct VM {
    registers: HashMap<Register, i32>,
    sensors: HashMap<Sensor, i32>,
    labels: HashMap<String, usize>,
    pc: usize,
    program: Vec<Instruction>,
    stack: Stack,
    crash: bool
}

impl VM {
    fn new(query: &str) -> Self {
        let program = parser::read_lines(query);
        VM {
            registers: HashMap::new(),
            sensors: Self::init_sensors(),
            labels: Self::init_labels(&program),
            pc: 0,
            program,
            stack: Stack::new(),
            crash: false
        }
    }

    fn init_labels(lines: &[Instruction]) -> HashMap<String, usize> {
        let mut labels = HashMap::new();

        for (i, inst) in lines.iter().enumerate() {
            if let Instruction::DeclareLabel(s) = inst {
                if labels.contains_key(s) {
                    panic!("[INVM] Duplicate label {s}.")
                }
                labels.insert(s.to_string(), i);
            }
        }
        labels
    }

    fn init_sensors() -> HashMap<Sensor, i32> {
        let mut sensors = HashMap::new();
        sensors.insert(Sensor::Shares, 0);
        sensors.insert(Sensor::Stockprice, 200);
        sensors.insert(Sensor::Reputation, 50);
        sensors.insert(Sensor::MarketValue, 200);
        sensors.insert(Sensor::Equity, 0);
        sensors.insert(Sensor::Owned, 0);
        sensors.insert(Sensor::Balance, 10000);
        sensors
    }

    fn expect_register_value(&mut self, reg: &Register) -> i32 {
        match self.registers.get(reg) {
            None => panic!("[INVM] Use of uninitialized register {reg:?}."),
            Some(n) => *n
        }
    }

    fn expect_sensor_value(&mut self, reg: &Sensor) -> i32 {
        *self.sensors.get(reg).unwrap()
    }

    fn expect_label(&mut self, label: &str) -> usize {
        match self.labels.get(label) {
            None => panic!("[INVM] Use of unknown label {label}."),
            Some(n) => *n
        }
    }

    fn expect_general_reg(&mut self, gr: GeneralRegister) -> i32 {
        match gr {
            GeneralRegister::Register(r) => self.expect_register_value(&r),
            GeneralRegister::Sensor(s) => self.expect_sensor_value(&s),
            GeneralRegister::Value(v) => v,
        }
    }

    fn step(&mut self) {
        let inst = match self.program.get(self.pc) {
            None => {
                self.crash = true;
                return;
            }
            Some(n) => n.clone(),
        };

        match inst {
            Instruction::Set(r, v) => self.set(r, v),
            Instruction::Add(v, r) => self.add(v, r),
            Instruction::Sub(v, r) => self.sub(v, r),
            Instruction::Mult(v, r) => self.mult(v, r),
            Instruction::Div(v, r) => self.div(v, r),
            Instruction::Goto(label) => self.goto(label),
            Instruction::Gotoz(val, label) => self.gotoz(val, label),
            Instruction::Print(val) => self.print(val),
            Instruction::Push(val) => self.push(val),
            Instruction::Pop(reg) => self.pop(reg),
            Instruction::Crash => self.crash(),
            Instruction::Buy(amount) => self.buy(amount),
            Instruction::Sell(amount) => self.sell(amount),
            Instruction::DeclareLabel(_) => (),
        }
        self.pc += 1;
        self.simulate();
    }

    fn simulate(&mut self) {
        let balance = self.expect_sensor_value(&Sensor::Balance);
        let mut stockprice = self.expect_sensor_value(&Sensor::Stockprice);
        let mut reputation = self.expect_sensor_value(&Sensor::Reputation);
        let shares = self.expect_sensor_value(&Sensor::Shares);
        let owned = self.expect_sensor_value(&Sensor::Owned);

        let mut r = rand::rng();
        let rep_shift = r.random_range(-5..=5);
        reputation += rep_shift;

        stockprice += r.random_range(-5..=5);
        if stockprice < 0 {
            stockprice = 0;
        }

        let bias = (reputation - 50) * stockprice / 10;
        let factor = if bias < 0 {
            - (bias * bias)
        } else {
            bias * bias
        };

        let amount = factor * (shares - owned);
        let new = if amount + shares < 0 {
            owned
        } else {
            amount + shares
        };

        self.sensors.insert(Sensor::Shares, new);
        self.sensors.insert(Sensor::Stockprice, stockprice);
        self.sensors.insert(Sensor::Reputation, reputation);
        self.sensors.insert(Sensor::MarketValue, shares * stockprice);
        self.sensors.insert(Sensor::Equity, owned * stockprice);
        self.sensors.insert(Sensor::Balance, balance + 100);

    }

    fn set(&mut self, reg: Register, gr: GeneralRegister) {
        let val = self.expect_general_reg(gr);
        self.registers.insert(reg, val);
    }

    fn add(&mut self, gr: GeneralRegister, reg: Register) {
        let val = self.expect_general_reg(gr);
        let current = self.expect_register_value(&reg);
        self.registers.insert(reg, val + current);
    }

    fn sub(&mut self, gr: GeneralRegister, reg: Register) {
        let val = self.expect_general_reg(gr); 
        let current = self.expect_register_value(&reg);
        self.registers.insert(reg, current - val);
    }

    fn mult(&mut self, gr: GeneralRegister, reg: Register) {
        let val = self.expect_general_reg(gr); 
        let current = self.expect_register_value(&reg);
        self.registers.insert(reg, current * val);
    }

    fn div(&mut self, gr: GeneralRegister, reg: Register) {
        let val = self.expect_general_reg(gr); 
        let current = self.expect_register_value(&reg);
        self.registers.insert(reg, current / val);
    }

    fn goto(&mut self, label: String) {
        self.pc = self.expect_label(&label);
    }

    fn gotoz(&mut self, gr: GeneralRegister, label: String) {
        let val = self.expect_general_reg(gr);

        if val == 0 {
            self.goto(label);
        }
    }

    fn print(&mut self, gr: GeneralRegister) {
        let val = self.expect_general_reg(gr);

        println!("{val}");
    }

    fn push(&mut self, gr: GeneralRegister) {
        let val = self.expect_general_reg(gr);
        self.stack.push(val); 
    }

    fn pop(&mut self, reg: Register) {
        let val = self.stack.pop();
        self.registers.insert(reg, val);
    }

    fn crash(&mut self) {
        self.crash = true;
    }

    fn buy(&mut self, amount: i32) {
        let balance = self.expect_sensor_value(&Sensor::Balance);
        let stockprice = self.expect_sensor_value(&Sensor::Stockprice);
        let total_price = amount * stockprice;
        if balance < total_price {
            panic!("[INVM] Insufficient balance ({balance}) to buy {amount} stocks at price {stockprice} (total price: {total_price}).");
        }


        let owned = self.expect_sensor_value(&Sensor::Owned);
        self.sensors.insert(Sensor::Balance, balance - total_price);
        self.sensors.insert(Sensor::Owned, owned + amount);
    }

    fn sell(&mut self, amount: i32) {
        let owned = self.expect_sensor_value(&Sensor::Owned);
        if amount > owned {
            panic!("[INVM] Insufficient stocks to sell (owned: {owned}, sell: {amount}).");
        }

        let stockprice = self.expect_sensor_value(&Sensor::Stockprice);
        let total_price = amount * stockprice;
        let balance = self.expect_sensor_value(&Sensor::Balance);
        self.sensors.insert(Sensor::Balance, balance + total_price);
        self.sensors.insert(Sensor::Owned, owned - amount);
    }
}
