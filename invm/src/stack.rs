use std::panic;

pub struct Stack {
    mem: [i32; 2048],
    sp: usize
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            mem: [0; 2048],
            sp: 0
        }
    }

    pub fn push(&mut self, val: i32) {
        self.mem[self.sp] = val;
        self.sp += 1;
        if self.sp >= 2048 {
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

}
