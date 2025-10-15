use std::io;

trait Addressable<T> {
    fn read(&self, addr: u16) -> T;

    fn write(&mut self, addr: u16, data: T);
}

struct Heap {
    data: [u8; 65536]
}

impl Heap {
    pub fn new() -> Self {
        Heap { data: [0; 65536] }
    }

    fn get(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    fn set(&mut self, addr: u16, data: u8) {
        self.data[addr as usize] = data;
    }
}

impl Addressable<u8> for Heap {
    fn read(&self, addr: u16) -> u8 {
        self.get(addr)
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.set(addr, data)
    }
}

impl Addressable<i32> for Heap {
    fn read(&self, addr: u16) -> i32 {
        let mut res = 0;
        for i in 0..4 {
            res |= (self.get(addr + i) as i32) << (i * 8);
        }
        res
    }

    fn write(&mut self, addr: u16, value: i32) {
        let p0 = (value & 0xff) as u8;
        let p1 = ((value & 0xff00) >> 8) as u8;
        let p2 = ((value & 0xff0000) >> 16) as u8;
        let p3 = ((value & 0xff000000u32 as i32) >> 24) as u8;
        self.set(addr, p0);
        self.set(addr+1, p1);
        self.set(addr+2, p2);
        self.set(addr+3, p3);
    }
}
