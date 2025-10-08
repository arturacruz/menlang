use std::fs;

use crate::{args::ArgError};

mod vm; 
mod stack;
mod lexer;
mod prepro;
mod args;
mod parser;

fn main() {
    let filepath = match args::parse_args() {
        Err(err) => match err {
            ArgError::IncorrectSize => panic!("[Run] Incorrect number of arguments."),
            ArgError::InvalidExtension => panic!("[Run] Expected a .invm file."),
        },
        Ok(f) => f
    };

    let query = fs::read_to_string(filepath)
        .expect("[Run] File not found");

    let filtered = prepro::filter(query);

    vm::run(&filtered);
}
