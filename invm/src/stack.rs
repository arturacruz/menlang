use std::panic;

const MAX_MEM: usize = 65536;

pub struct Stack {
    mem: [i32; MAX_MEM],
    pub sp: usize,
    
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            mem: [0; MAX_MEM],
            sp: 0
        }
    }

    pub fn push(&mut self, val: i32) {
        self.mem[self.sp] = val;
        self.sp += 1;
        if self.sp >= MAX_MEM {
            panic!("[INVM] Stack overflow.")
        }
    }

    pub fn pop(&mut self) -> i32 {
        if self.sp == 0 {
            panic!("[INVM] Pop used on an empty stack.")
        }
        self.sp -= 1;
        self.mem[self.sp]
    }

    pub fn get(&self, addr: u16) -> i32 {
        self.mem[addr as usize]
    }

    pub fn set(&mut self, addr: u16, data: i32) {
        self.mem[addr as usize + self.sp] = data;
    }

    pub fn alloc_str(&mut self, data: String) -> u16 {
        let addr = self.sp;
        let size = data.len() as i32;
        self.push(size);
        for c in data.chars() {
            self.push(c as i32);
        }
        addr as u16
    }

    pub fn get_str(&self, addr: u16) -> String {
        let size = self.mem[addr as usize];
        let addr = (addr + 1) as usize;

        let mut str = String::new();

        for i in 0..size {
            let c = self.mem[addr + i as usize];
            let c = char::from_u32(c as u32).expect("[INVM] Invalid conversion of character.");
            str.push(c);
        }

        str
    }
}

