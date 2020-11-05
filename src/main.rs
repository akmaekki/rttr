extern crate rttr;

use std::io::{self};

pub fn main() -> io::Result<()> {
    // Allocate buffer owned by main
    let mut buffer = String::new();

    // Read from stdin into buffer
    rttr::read_from_stdin(&mut buffer)?;

    // Output read data from stdin
    println!("{}", buffer);

    Ok(())
}

