extern crate rttr;

use std::env;
use std::io::{self};

pub fn main() -> io::Result<()> {
    // Read command line arguments
    //
    // Also possible to pass as parse_config(args) if args
    // are not used anymore in main program.
    let args: Vec<String> = env::args().collect();
    let config = rttr::parse_config(&args);

    // Allocate buffer owned by main
    let mut buffer = String::new();

    // Read from stdin into buffer
    rttr::read_from_stdin(&mut buffer)?;

    // Pass string to make replacements in, characters to replace and
    // replacement characters to function and return processed input.
    rttr::replace()

    // Output read data from stdin
    println!("{}", buffer);

    Ok(())
}

