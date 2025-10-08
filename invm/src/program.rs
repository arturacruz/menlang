use crate::vm::Register;

pub enum Instruction {
    Set(Register, i32),
    Add(i32, Register),
    Sub(i32, Register),
    Goto(String),
    Gotoz(Register, String),
    Print(i32),
    Push(i32),
    Pop(Register),
    Crash,
    Buy(u32),
    Sell(u32)
}

struct Program {
    
}


