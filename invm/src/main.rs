use std::fs;

use crate::{args::ArgError, program::Program};

mod vm; 
mod stack;
mod program;
mod lexer;
mod prepro;
mod args;

fn main() {
    let filepath = match args::parse_args() {
        Err(err) => match err {
            ArgError::IncorrectSize => panic!("[Run] Incorrect number of arguments."),
            ArgError::InvalidExtension => panic!("[Run] Expected a .go file."),
        },
        Ok(f) => f
    };

    let query = fs::read_to_string(filepath)
        .expect("[Run] File not found");

    let filtered = prepro::filter(query);

    let program = Program::new(&filtered);
}
