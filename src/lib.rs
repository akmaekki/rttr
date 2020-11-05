use std::io::{self, Read};
use std::io::Error;
use std::result::{self};

#[derive(Debug)]
pub struct Config {
    search_characters: String,
    replacement_characters: String,
}


pub fn parse_config(args: &[String]) -> Config {
    let search = args[1].clone();
    let replacement = args[2].clone();

    Config {
        search_characters: search,
        replacement_characters: replacement,
    }
}


#[cfg(test)]
mod test_config {
    use crate::parse_config;

    #[test]
    fn one_is_one() {
        assert_eq!(1, 1);
    }

    #[test]
    fn reads_cmd_line_args_to_config() {
        let args: [String; 3] = [
            String::from("rttr"),
            String::from("'abc'"),
            String::from("'def")
        ];

        let config = parse_config(&args);

        assert_eq!(args[1], config.search_characters);
        assert_eq!(args[2], config.replacement_characters);
    }
}


pub fn read_from_stdin(buffer: &mut String) -> result::Result<(), Error> {
    let mut stdin = io::stdin();
    stdin.read_to_string(buffer)?;

    Ok(())
}


#[cfg(test)]
mod test_read_from_stdin {
    // todo: später weglassen
    // use super::*;

    #[test]
    fn one_is_one() {
        assert_eq!(1, 1);
    }
}
