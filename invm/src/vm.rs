use std::collections::HashMap;

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
            labels: HashMap::new(),
            pc: 0,
            program,
            stack: Stack::new(),
            crash: false
        }
    }

    fn init_sensors() -> HashMap<Sensor, i32> {
        let mut sensors = HashMap::new();
        sensors.insert(Sensor::Shares, 0);
        sensors.insert(Sensor::Stockprice, 0);
        sensors.insert(Sensor::Reputation, 50);
        sensors.insert(Sensor::MarketValue, 0);
        sensors.insert(Sensor::Equity, 0);
        sensors.insert(Sensor::Owned, 0);
        sensors.insert(Sensor::Balance, 10000);
        sensors
    }

    fn init_label(&mut self, label: String) {
        if self.labels.contains_key(&label) {
            panic!("[INVM] Duplicate label {label}.")
        }

        self.labels.insert(label, self.pc);
    }

    fn expect_register_value(&mut self, reg: &Register) -> i32 {
        *self.registers.get(reg).expect("[INVM] Use of uninitialized register {reg:?}.")
    }

    fn expect_sensor_value(&mut self, reg: &Sensor) -> i32 {
        *self.sensors.get(reg).unwrap()
    }

    fn expect_label(&mut self, label: &str) -> usize {
        *self.labels.get(label).expect("[INVM] Use of unknown label.")
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
            Instruction::Goto(label) => self.goto(label),
            Instruction::Gotoz(val, label) => self.gotoz(val, label),
            Instruction::Print(val) => self.print(val),
            Instruction::Push(val) => self.push(val),
            Instruction::Pop(reg) => self.pop(reg),
            Instruction::Crash => self.crash(),
            Instruction::Buy(amount) => self.buy(amount),
            Instruction::Sell(amount) => self.sell(amount),
            Instruction::DeclareLabel(label) => self.init_label(label),
        }
        self.pc += 1;
        self.simulate();
    }

    fn simulate(&mut self) {

    }

    fn set(&mut self, reg: Register, gr: GeneralRegister) {
        let val = match gr {
            GeneralRegister::Register(r) => self.expect_register_value(&r),
            GeneralRegister::Sensor(s) => self.expect_sensor_value(&s),
            GeneralRegister::Value(v) => v,
        };
        self.registers.insert(reg, val);
    }

    fn add(&mut self, gr: GeneralRegister, reg: Register) {
        let val = match gr {
            GeneralRegister::Register(r) => self.expect_register_value(&r),
            GeneralRegister::Sensor(s) => self.expect_sensor_value(&s),
            GeneralRegister::Value(v) => v,
        };
        let current = self.expect_register_value(&reg);
        self.registers.insert(reg, val + current);
    }

    fn sub(&mut self, gr: GeneralRegister, reg: Register) {
        let val = match gr {
            GeneralRegister::Register(r) => self.expect_register_value(&r),
            GeneralRegister::Sensor(s) => self.expect_sensor_value(&s),
            GeneralRegister::Value(v) => v,
        };
        let current = self.expect_register_value(&reg);
        self.registers.insert(reg, current - val);
    }

    fn goto(&mut self, label: String) {
        self.pc = self.expect_label(&label);
    }

    fn gotoz(&mut self, gr: GeneralRegister, label: String) {
        let val = match gr {
            GeneralRegister::Register(r) => self.expect_register_value(&r),
            GeneralRegister::Sensor(s) => self.expect_sensor_value(&s),
            GeneralRegister::Value(v) => v,
        };

        if val == 0 {
            self.goto(label);
        }
    }

    fn print(&mut self, gr: GeneralRegister) {
        let val = match gr {
            GeneralRegister::Register(r) => self.expect_register_value(&r),
            GeneralRegister::Sensor(s) => self.expect_sensor_value(&s),
            GeneralRegister::Value(v) => v,
        };

        println!("{val}");
    }

    fn push(&mut self, gr: GeneralRegister) {
        let val = match gr {
            GeneralRegister::Register(r) => self.expect_register_value(&r),
            GeneralRegister::Sensor(s) => self.expect_sensor_value(&s),
            GeneralRegister::Value(v) => v,
        };
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
