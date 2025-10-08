use std::env;

#[derive(Debug)]
pub enum ArgError {
    IncorrectSize,
    InvalidExtension
}

/// Checks the passed arguments.
/// Expected: cargo run -- "{query}"
/// If the argument number is different than 2, results in an error.
pub fn parse_args() -> Result<String, ArgError> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(ArgError::IncorrectSize);
    }
    let filename = match args.get(1) {
        Some(arg) => arg.to_string(),
        None => return Err(ArgError::IncorrectSize),
    };
    let last = {
        let dot = filename.len() - 5;
        &filename[dot..]
    };
    if last != ".invm" {
        return Err(ArgError::InvalidExtension);
    }
    Ok(filename)

}
