use std::io;

pub struct Heap {
    data: [i32; 65536]
}

impl Heap {
    pub fn new() -> Self {
        Heap { data: [0; 65536] }
    }

    pub fn get(&self, addr: u16) -> i32 {
        self.data[addr as usize]
    }

    pub fn set(&mut self, addr: u16, data: i32) {
        self.data[addr as usize] = data;
    }
}
