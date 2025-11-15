use std::{collections::HashMap, u16};

mod simulation;

use crate::{heap::Heap, parser, stack::Stack};

pub fn run(query: &str) {
    let mut vm = VM::new(query);
    while !vm.crash {
        vm.step();
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Set(Reference, Reference),
    Add(Reference, Reference),
    Sub(Reference, Reference),
    Mult(Reference, Reference),
    Div(Reference, Reference),
    Goto(String),
    GoIf(Condition, Reference, String),
    Print(Reference, Type),
    Push(Reference),
    Pop(Reference),
    Crash,
    Buy(i32),
    Sell(i32),
    DeclareLabel(String),
}

#[derive(Debug, Clone)]
pub enum Type {
    Int, Bool, Char
}

#[derive(Debug, Clone)]
pub enum Condition {
    Equals, Different, Greater, Lesser, GreaterOrEqual, LesserOrEqual
}

#[derive(Debug, Clone)]
pub enum Reference {
    Register(Register),
    Sensor(Sensor),
    Value(i32),
    Address(GeneralRegister)
}

#[derive(Debug, Clone)]
pub enum GeneralRegister {
    Register(Register),
    Sensor(Sensor),
    Value(i32),
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
    heap: Heap,
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
            heap: Heap::new(),
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

    fn expect_register_value(&self, reg: &Register) -> i32 {
        match self.registers.get(reg) {
            None => panic!("[INVM] Use of uninitialized register {reg:?}."),
            Some(n) => *n
        }
    }

    fn expect_sensor_value(&self, reg: &Sensor) -> i32 {
        *self.sensors.get(reg).unwrap()
    }

    fn expect_label(&mut self, label: &str) -> usize {
        match self.labels.get(label) {
            None => panic!("[INVM] Use of unknown label {label}."),
            Some(n) => *n
        }
    }

    fn expect_general_reg(&self, gr: &GeneralRegister) -> i32 {
        match gr {
            GeneralRegister::Register(r) => self.expect_register_value(r),
            GeneralRegister::Sensor(s) => self.expect_sensor_value(s),
            GeneralRegister::Value(v) => *v,
        }
    }

    fn expect_reference(&self, r: &Reference) -> i32 {
        match r {
            Reference::Register(r) => self.expect_register_value(r),
            Reference::Sensor(s) => self.expect_sensor_value(s),
            Reference::Value(n) => *n,
            Reference::Address(gr) => self.heap.get(
                self.to_address(self.expect_general_reg(gr))
            ),
        }
    }

    fn to_address(&self, v: i32) -> u16 {
        if !(0..65535).contains(&v) {
            panic!("[INVM] Segmentation fault.");
        }
        v as u16
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
            Instruction::GoIf(cond, val, label) => self.go_if(cond, val, label),
            Instruction::Print(val, t) => self.print(val, t),
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


    fn set(&mut self, reg: Reference, reg2: Reference) {
        let val = self.expect_reference(&reg2); 
        match reg {
            Reference::Register(r) => { self.registers.insert(r, val); },
            Reference::Address(g) => {
                let v = self.expect_general_reg(&g);
                self.heap.set(self.to_address(v), val)
            }
            _ => panic!("[INVM] Unable to modify readonly value {reg:?} at SET instruction.")
        }
    }

    fn add(&mut self, reg: Reference, reg2: Reference) {
        let val = self.expect_reference(&reg);
        let current = self.expect_reference(&reg2);

        let sum = val + current;

        match reg2 {
            Reference::Register(r) => { self.registers.insert(r, sum); },
            Reference::Address(g) => {
                let v = self.expect_general_reg(&g);
                self.heap.set(self.to_address(v), sum)
            },
            _ => panic!("[INVM] Unable to modify readonly value {reg2:?} at ADD instruction.")
        }
    }

    fn sub(&mut self, reg: Reference, reg2: Reference) {
        let val = self.expect_reference(&reg);
        let current = self.expect_reference(&reg2);
        let res = current - val;
        match reg2 {
            Reference::Register(r) => { self.registers.insert(r, res); },
            Reference::Address(g) => {
                let v = self.expect_general_reg(&g);
                self.heap.set(self.to_address(v), res)
            },
            _ => panic!("[INVM] Unable to modify readonly value {reg2:?} at SUB instruction.")
        }
    }

    fn mult(&mut self, reg: Reference, reg2: Reference) {
        let val = self.expect_reference(&reg);
        let current = self.expect_reference(&reg2);
        let res = val * current;
        match reg2 {
            Reference::Register(r) => { self.registers.insert(r, res); },
            Reference::Address(g) => {
                let v = self.expect_general_reg(&g);
                self.heap.set(self.to_address(v), res)
            },
            _ => panic!("[INVM] Unable to modify readonly value {reg2:?} at MULT instruction.")
        }
    }

    fn div(&mut self, reg: Reference, reg2: Reference) {
        let val = self.expect_reference(&reg);
        let current = self.expect_reference(&reg2);
        let res = val / current;
        match reg2 {
            Reference::Register(r) => { self.registers.insert(r, res); },
            Reference::Address(g) => {
                let v = self.expect_general_reg(&g);
                self.heap.set(self.to_address(v), res)
            },
            _ => panic!("[INVM] Unable to modify readonly value {reg2:?} at DIV instruction.")
        }
    }

    fn goto(&mut self, label: String) {
        self.pc = self.expect_label(&label);
    }

    fn go_if(&mut self, cond: Condition, reg: Reference, label: String) {
        let val = self.expect_reference(&reg);

        let c = match cond {
            Condition::Equals => val == 0,
            Condition::Different => val != 0,
            Condition::Lesser => val < 0,
            Condition::Greater => val > 0,
            Condition::GreaterOrEqual => val >= 0,
            Condition::LesserOrEqual => val <= 0,
        };

        if c {
            self.goto(label);
        }
    }

    fn print(&mut self, reg: Reference, t: Type) {
        let val = self.expect_reference(&reg);
        match t {
            Type::Int => println!("{val}"),
            Type::Bool => println!("{}", val != 0),
            Type::Char => println!("{}", (val % 256) as u8 as char),
        }
    }

    fn push(&mut self, reg: Reference) {
        let val = self.expect_reference(&reg);
        self.stack.push(val); 
    }

    fn pop(&mut self, reg: Reference) {
        let val = self.stack.pop();
        match reg {
            Reference::Register(r) => { self.registers.insert(r, val); },
            Reference::Address(g) => {
                let v = self.expect_general_reg(&g);
                self.heap.set(self.to_address(v), val)
            },
            _ => panic!("[INVM] Unable to modify readonly value {reg:?}.")
        }
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
