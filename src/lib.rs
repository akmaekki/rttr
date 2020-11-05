use std::io::Error;
use std::io::{self, Read};
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

// Start with single character to search and replace for
pub fn replace_single_single(
    input: &str,
    search_character: char,
    replace_character: char,
) -> String {
    let result = String::from(input);

    input
        .chars()
        .map(|c| match c {
            _ if c == search_character => replace_character,
            _ => c,
        })
        .collect()
}

/*
 Start with multiple character to search and replace for.
 Example: echo "abc" | tr 'abc' 'x' => 'xxx' because
 'abc' 'x' will be interpreted as 'abc' 'xxx' to fill
 up to 3 characters. Then 'a' will be mapped to 'x',
 'b' will be mapped to 'x' and 'c' will be mapped to 'x'.
 Example: echo "abc" | tr 'aba' 'x' => 'zyc' because
*/
pub fn replace_multiple_single(input: &str, search_characters: &str, replace_character: char) -> String {
    let result = String::from(input);

    input
        .chars()
        .map(|c| match c {
            _ if search_characters.contains(c) => replace_character,
            _ => c
        })
        .collect()
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
            String::from("'def"),
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
mod test {
    use super::*;

    #[test]
    fn one_is_one() {
        assert_eq!(1, 1);
    }

    #[test]
    fn replace_single_search_character_with_single_replacement_character() {
        let input = "abcdefghijklmnopqrstuvwxyz";
        let search_character = 'k';
        let replace_character = '_';

        let result = replace_single_single(input, search_character, replace_character);

        assert_eq!("abcdefghij_lmnopqrstuvwxyz", result);
    }

    #[test]
    fn replace_nothing_empty_string() {
        let input = "";
        let search_character = 'k';
        let replace_character = '_';

        let result = crate::replace_single_single(input, search_character, replace_character);

        assert_eq!(input, result);
    }

    #[test]
    fn replace_multiple_single() {
        let input = "abc";
        let search_character = "ab";
        let replace_character = 'x';

        let result = crate::replace_multiple_single(input, search_character, replace_character);

        println!("result: {}", result);
        assert_eq!("xxc", result);
    }
}
