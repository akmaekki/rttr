use std::io::{self, Read};
use std::io::Error;
use std::result::{self};

pub fn read_from_stdin(buffer: &mut String) -> result::Result<(), Error> {
    let mut stdin = io::stdin();
    stdin.read_to_string(buffer)?;

    Ok(())
}


#[cfg(test)]
mod test {
    // todo: später weglassen
    use super::*;

    #[test]
    fn one_is_one() {
        assert_eq!(1, 1);
    }

}
