use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Allocate buffer owned by main
    let mut buffer = String::new();

    // Read from stdin into buffer
    read_from_stdin(&mut buffer)?;

    // Output read data from stdin
    println!("{}", buffer);

    Ok(())
}

fn read_from_stdin(buffer: &mut String) -> io::Result<()>{
    let mut stdin = io::stdin();
    stdin.read_to_string(buffer)?;

    Ok(())
}
