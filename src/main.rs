use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();

    stdin.read_to_string(&mut buffer)?;

    println!("{}", buffer);

    Ok(())
}
