use std::collections::HashMap;
use std::io::{self, Read};
use std::io::Error;
use std::result::{self};
use std::collections::hash_map::RandomState;

#[derive(Debug)]
pub struct Config {
    search_characters: String,
    replacement_characters: String,
    delete_mode_active: bool,
}

pub fn read_from_stdin(buffer: &mut String) -> result::Result<(), Error> {
    let mut stdin = io::stdin();
    stdin.read_to_string(buffer)?;

    Ok(())
}

pub fn parse_config(args: &[String]) -> Config {
    let mut search = args[1].clone();
    let mut replace = args[2].clone();
    let mut is_delete_mode = false;

    if helpers::starts_and_ends_with_one_of(&search, &['"', '\'']) {
        search.remove(0);
        search.remove(search.len() - 1);
    }

    if helpers::starts_and_ends_with_one_of(&replace, &['"', '\'']) {
        replace.remove(0);
        replace.remove(replace.len() - 1);
    }

    if args.len() > 3 && args[3].contains("-d") {
        is_delete_mode = true;
    }

    Config {
        search_characters: search,
        replacement_characters: replace,
        delete_mode_active: is_delete_mode,
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
pub fn replace_multiple_single(
    input: &str,
    search_characters: &str,
    replace_character: char,
) -> String {
    let result = String::from(input);

    input
        .chars()
        .map(|c| match c {
            _ if search_characters.contains(c) => replace_character,
            _ => c,
        })
        .collect()
}

/*
 Example: echo "abc" | tr 'aba' 'xyz' => 'zyc' because
 first characters 'a' of "source" will be mapped to first
 character 'z' of "target", second character 'b' of "source"
 will be mapped to second character 'y' of "target" and third
 character 'a' of "source" will be mapped to third character
 'z' of "target".
 Example: echo "abc" | tr 'abbb' 'xyzklm' => 'xkc'
*/
pub fn replace_multiple_multiple(input: &str, search_characters: &str, replace_character: &str)
/*-> String*/
{
    /*
    let result = String::from(input);

    input
        .chars()
        .map(|c| match c {
            _ if search_characters.contains(c) => replace_character,
            _ => c
        })
        .collect()
     */
}

fn normalize_char_arrays(search_characters: &str, replace_characters: &str) -> (String, String) {
    let len_s = search_characters.len();
    let len_r = replace_characters.len();

    if len_s < len_r {
        let new_replace = String::from(&replace_characters[0..len_s]);
        (String::from(search_characters), String::from(new_replace))
    } else {
        let last_char = replace_characters.chars().nth(len_r - 1).unwrap();
        let append = (0..(len_s - len_r)).map(|c| last_char).collect::<String>();
        let new_replace = String::from([&replace_characters[..], &append[..]].concat());
        (String::from(search_characters), String::from(new_replace))
    }
}

fn create_mapping_table(search_chars: & str, replace_chars: &str) -> HashMap<char, char, RandomState> {
    // if search_characters.len() != replace_characters.len() => throw
    // if search_characters.len() == 0 throw

    let mut table = HashMap::new();

    // search: 'aaa' and 'xyz' would lead to
    // a -> x, a -> y, a -> z and totally to
    // a -> z and finally this map: {'a': 'z'}
    // Therefore, traversing search-array from
    // to left is easier.

    println!("search: {:?}", search_chars.chars().rev());
    println!("replace: {:?}", replace_chars.chars().rev());

    for (i, c) in search_chars.chars().rev().enumerate() {
        if !table.contains_key(&c) {
            table.insert(c, replace_chars.chars().rev().nth(i).unwrap());
        }
    }

    // Alternative idea:
    // Use zip to create array of pairs and
    // from this array create a map/set. But
    // could contain duplicate keys.

    table
}

pub fn delete(input: &str, chars_for_deletion: &str) -> String {
    input
        .chars()
        .filter(|c| !chars_for_deletion.contains(*c))
        .collect()
}

#[cfg(test)]
mod test_delete {
    use crate::delete;

    #[test]
    fn delete_chars_if_present_in_input() {
        let input = "abc_def_ghi";
        let chars_for_deletion = "dEf";

        let result = crate::delete(input, chars_for_deletion);

        assert_eq!("abc_e_ghi", result, "Delete characters marked for deletion");
    }

    #[test]
    fn delete_no_chars_if_not_included_in_input() {
        let input = "abc_def_ghi";
        let chars_for_deletion = "xyz";

        let result = crate::delete(input, chars_for_deletion);

        assert_eq!(
            "abc_def_ghi", result,
            "Do not delete characters not marked for deletion"
        );
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
            String::from("'def"),
        ];

        let config = parse_config(&args);

    #[test]
    fn recognizes_delete_cmd_line_flag() {
        let args: [String; 4] = [
            String::from("rttr"),
            String::from("abc"),
            String::from("def"),
            String::from("-d"),
        ];

        let config = crate::parse_config(&args);

        assert_eq!(
            true, config.delete_mode_active,
            "Read `delete flag` if present"
        );
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

    #[test]
    fn normalize_character_arrays() {
        let search_character = "abcd";
        let replace_character = "xy";

        //let result = crate::get_mapping_table(&mut search_character, &mut replace_character);
        let (a, b) = crate::normalize_char_arrays(&search_character, &replace_character);

        assert_eq!("abcd", a);
        assert_eq!("xyyy", b);
    }

    #[test]
    fn normalize_character_arrays_2() {
        let search_character = "ab";
        let replace_character = "xyz";

        //let result = crate::get_mapping_table(&mut search_character, &mut replace_character);
        let (a, b) = crate::normalize_char_arrays(&search_character, &replace_character);

        assert_eq!("ab", a);
        assert_eq!("xy", b);
    }

    #[test]
    fn normalize_character_arrays_3() {
        let search_character = "a";
        let replace_character = "xyz";

        let (a, b) = crate::normalize_char_arrays(&search_character, &replace_character);

        assert_eq!("a", a);
        assert_eq!("x", b);
    }

    #[test]
    fn generate_mapping_table_for_shorter_search_string() {
        let search_character = "a";
        let replace_character = "xyz";

        let (search_character, replace_character) = crate::normalize_char_arrays(&search_character, &replace_character);
        let table = crate::create_mapping_table(&search_character, &replace_character);

        assert_eq!(table.get(&'a'), Some(&'x'));
        assert_eq!(1, table.len());
    }

    #[test]
    fn generate_mapping_table_for_shorter_replacement_string() {
        let search_character = "abcd";
        let replace_character = "xy";

        let (search_character, replace_character) = crate::normalize_char_arrays(&search_character, &replace_character);
        let table = crate::create_mapping_table(&search_character, &replace_character);

        println!("table: {:?}", table);
        assert_eq!(table.get(&'a'), Some(&'x'));
        assert_eq!(table.get(&'b'), Some(&'y'));
        assert_eq!(table.get(&'c'), Some(&'y'));
        assert_eq!(table.get(&'d'), Some(&'y'));
        assert_eq!(4, table.len());
    }

    #[test]
    fn generate_mapping_table_for_duplicate_characters_in_search_string() {
        let search_character = "abac";
        let replace_character = "xyz";

        let (search_character, replace_character) = crate::normalize_char_arrays(&search_character, &replace_character);
        let table = crate::create_mapping_table(&search_character, &replace_character);

        assert_eq!(table.get(&'a'), Some(&'z'));
        assert_eq!(table.get(&'b'), Some(&'y'));
        assert_eq!(table.get(&'c'), Some(&'z'));
        assert_eq!(3, table.len());
    }
}
