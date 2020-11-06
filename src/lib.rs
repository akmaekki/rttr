use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::io::Error;
use std::io::{self, Read};
use std::result::{self};

mod helpers;

#[derive(Debug)]
pub struct Config {
    pub search_characters: String,
    pub replac_characters: String,
    pub is_delete_mode: bool,
}

pub fn read_from_stdin(buffer: &mut String) -> result::Result<(), Error> {
    let mut stdin = io::stdin();
    stdin.read_to_string(buffer)?;

    Ok(())
}

pub fn parse_config(args: &[String]) -> Config {
    let search;
    let mut replace = "".to_string();
    let is_delete_mode;

    if args[1].contains("-d") {
        is_delete_mode = true;
        search = helpers::strip_from_start_and_end(&args[2], &['"', '\'']);
    } else {
        is_delete_mode = false;
        search = helpers::strip_from_start_and_end(&args[1], &['"', '\'']);
        replace = helpers::strip_from_start_and_end(&args[2], &['"', '\'']);
    }

    Config {
        search_characters: search,
        replac_characters: replace,
        is_delete_mode: is_delete_mode,
    }
}

// Start with single character to search and replace for
pub fn replace_single_single(
    input: &str,
    search_character: char,
    replace_character: char,
) -> String {
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
pub fn replace_multiple_multiple(input: &str, search_chars: &str, replace_chars: &str) -> String {
    let (search_chars_normalised, replace_chars_normalised) =
        normalize_char_arrays(search_chars, replace_chars);
    let chars_mapping_table =
        create_character_mapping_table(&search_chars_normalised, &replace_chars_normalised);

    input
        .chars()
        .map(|c| match c {
            _ if chars_mapping_table.contains_key(&c) => chars_mapping_table[&c],
            _ => c,
        })
        .collect()
}

fn normalize_char_arrays(search_characters: &str, replace_characters: &str) -> (String, String) {
    let len_s = search_characters.len();
    let len_r = replace_characters.len();

    if len_s < len_r {
        let new_replace = String::from(&replace_characters[0..len_s]);
        (String::from(search_characters), String::from(new_replace))
    } else {
        let last_char = replace_characters.chars().nth(len_r - 1).unwrap();
        let append = (0..(len_s - len_r)).map(|_| last_char).collect::<String>();
        let new_replace = String::from([&replace_characters[..], &append[..]].concat());
        (String::from(search_characters), String::from(new_replace))
    }
}

fn create_character_mapping_table(
    search_chars: &str,
    replace_chars: &str,
) -> HashMap<char, char, RandomState> {
    // if search_characters.len() != replace_characters.len() => throw
    // if search_characters.len() == 0 throw

    let mut table = HashMap::new();

    // search: 'aaa' and 'xyz' would lead to
    // a -> x, a -> y, a -> z and totally to
    // a -> z and finally this map: {'a': 'z'}
    // Therefore, traversing search-array from
    // to left is easier.

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
    #[test]
    fn reads_cmd_line_args_to_config_and_strips_surrounding_quotes() {
        let args = [
            String::from("rttr"),
            String::from("abc"),
            String::from("def"),
        ];

        let config = crate::parse_config(&args);

        assert_eq!(
            "abc", config.search_characters,
            "Read `search string` from command line"
        );
        assert_eq!(
            "def", config.replac_characters,
            "Read `replace string` from command line"
        );
    }

    #[test]
    fn replaces_starting_and_ending_quotes_from_search_and_replace() {
        let args = [
            String::from("rttr"),
            String::from("'abc'"),
            String::from("\"def\""),
        ];

        let config = crate::parse_config(&args);

        assert_eq!(
            "abc", config.search_characters,
            "Replace surrounding quotes (single/double) from `search string`"
        );
        assert_eq!(
            "def", config.replac_characters,
            "Replace surrounding quotes (single/double) from `replace string`"
        );
    }

    #[test]
    fn replaces_not_single_starting_or_ending_quotes_from_search_and_replace() {
        let args = [
            String::from("rttr"),
            String::from("'abc"),
            String::from("def\""),
        ];

        let config = crate::parse_config(&args);

        assert_eq!(
            "'abc", config.search_characters,
            "Do not replace quotes (single/double) from `search string` that are not pairwise"
        );
        assert_eq!(
            "def\"", config.replac_characters,
            "Do not replace quotes (single/double) from `replace string` that are not pairwise"
        );
    }

    #[test]
    fn recognizes_delete_cmd_line_flag() {
        let args = [
            String::from("rttr"),
            String::from("-d"),
            String::from("abc"),
            String::from("def"),
        ];

        let config = crate::parse_config(&args);

        assert_eq!(true, config.is_delete_mode, "Read `delete flag` if present");
    }
}

#[cfg(test)]
mod test_single {
    #[test]
    fn replace_single_search_character_with_single_replacement_character() {
        let input = "abcdefghijklmnopqrstuvwxyz";
        let search_character = 'k';
        let replace_character = '_';

        let result = crate::replace_single_single(input, search_character, replace_character);

        assert_eq!(
            "abcdefghij_lmnopqrstuvwxyz", result,
            "Replace single character if marked for deletion"
        );
    }

    #[test]
    fn replace_nothing_empty_string() {
        let input = "";
        let search_character = 'k';
        let replace_character = '_';

        let result = crate::replace_single_single(input, search_character, replace_character);

        assert_eq!(input, result, "Do not replace in empty string");
    }

    #[test]
    fn replace_multiple_single() {
        let input = "abc";
        let search_character = "ab";
        let replace_character = 'x';

        let result = crate::replace_multiple_single(input, search_character, replace_character);

        assert_eq!(
            "xxc", result,
            "Replace multiple different characters with only character in `replace string`"
        );
    }
}

#[cfg(test)]
mod test_normalize_character_arrays {
    #[test]
    fn normalize_character_arrays_if_search_longer_than_replace() {
        let search_character = "abcd";
        let replace_character = "xy";

        //let result = crate::get_mapping_table(&mut search_character, &mut replace_character);
        let (a, b) = crate::normalize_char_arrays(&search_character, &replace_character);

        assert_eq!("abcd", a, "Keep `search string` always unchanged");
        assert_eq!("xyyy", b, "Pad right part of `replace string` with last character to have equal size to `search string`");
    }

    #[test]
    fn normalize_character_arrays_if_search_shorter_than_replace() {
        let search_character = "ab";
        let replace_character = "xyz";

        //let result = crate::get_mapping_table(&mut search_character, &mut replace_character);
        let (a, b) = crate::normalize_char_arrays(&search_character, &replace_character);

        assert_eq!(
            "ab", a,
            "Keep `search string` unchanged if smaller than `replace string`"
        );
        assert_eq!(
            "xy", b,
            "Remove right part of `replace string` to have equal size to `search string`"
        );
    }
}

#[cfg(test)]
mod test_character_generate_mapping_table {
    #[test]
    fn generate_character_mapping_table_for_shorter_search_string() {
        let search_character = "a";
        let replace_character = "xyz";

        let (search_character, replace_character) =
            crate::normalize_char_arrays(&search_character, &replace_character);
        let table = crate::create_character_mapping_table(&search_character, &replace_character);

        assert_eq!(
            table.get(&'a'),
            Some(&'x'),
            "Map first character in `search string` to first character of `replace string`"
        );
        assert_eq!(
            1,
            table.len(),
            "Table length should be length of unique entries in `search string`"
        );
    }

    #[test]
    fn generate_character_mapping_table_for_shorter_replacement_string() {
        let search_character = "abcd";
        let replace_character = "xy";

        let (search_character, replace_character) =
            crate::normalize_char_arrays(&search_character, &replace_character);
        let table = crate::create_character_mapping_table(&search_character, &replace_character);

        assert_eq!(
            table.get(&'a'),
            Some(&'x'),
            "Map first character in `search string` to first character of `replace string`"
        );
        assert_eq!(
            table.get(&'b'),
            Some(&'y'),
            "Map second character in `search string` to second character of `replace string`"
        );
        assert_eq!(
            table.get(&'c'),
            Some(&'y'),
            "Map third character in `search string` to second character of `replace string` if length of `replace string` is two"
        );
        assert_eq!(
            table.get(&'d'),
            Some(&'y'),
            "Map fourth character in `search string` to second character of `replace string` if length of `replace string` is two"
        );
        assert_eq!(
            4,
            table.len(),
            "Table length should be length of unique entries in `search string`"
        );
    }

    #[test]
    fn generate_character_mapping_table_for_duplicate_characters_in_search_string() {
        let search_character = "abac";
        let replace_character = "xyz";

        let (search_character, replace_character) =
            crate::normalize_char_arrays(&search_character, &replace_character);
        let table = crate::create_character_mapping_table(&search_character, &replace_character);

        assert_eq!(
            table.get(&'a'),
            Some(&'z'),
            "Map first character in `search string` not to first character of `replace string` if it appears more often in `search string`"
        );
        assert_eq!(
            table.get(&'b'),
            Some(&'y'),
            "Map second character in `search string` to second character of `replace string` if both appear only once"
        );
        assert_eq!(
            table.get(&'c'),
            Some(&'z'),
            "Map fourth character in `search string` to third character of `replace string` if length of `replace string` is three"
        );
        assert_eq!(
            3,
            table.len(),
            "Table length should be length of unique entries in `search string`"
        );
    }
}

#[cfg(test)]
mod test_multi {
    #[test]
    fn replace_multiple_search_character_with_multiple_replacement_character() {
        let input = "abcdefghijklmnopqrstuvwxyz";
        let search_character = "akz";
        let replace_character = "-_/";

        let result = crate::replace_multiple_multiple(input, search_character, replace_character);

        assert_eq!(
            "-bcdefghij_lmnopqrstuvwxy/", result,
            "Replace multiple charactes"
        );
    }

    #[test]
    fn replacemultiple_nothing_if_empty_input_string() {
        let input = "";
        let search_character = "xyz";
        let replace_character = "abc";

        let result = crate::replace_multiple_multiple(input, search_character, replace_character);

        assert_eq!(input, result, "Do not replace in empty string");
    }

    #[test]
    fn replacemultiple_search_string_longer_than_replacement_string() {
        let input = "abcdefghijklmnopqrstuvwxyz";
        let search_character = "abcz";
        let replace_character = "xxy";

        let result = crate::replace_multiple_multiple(input, search_character, replace_character);

        assert_eq!(
            "xxydefghijklmnopqrstuvwxyy", result,
            "Replace multiple different characters with only character in `replace string`"
        );
    }

    #[test]
    fn replacemultiple_search_string_shorter_than_replacement_string() {
        let input = "abcdefghijklmnopqrstuvwxyz";
        let search_character = "ab";
        let replace_character = "123";

        let result = crate::replace_multiple_multiple(input, search_character, replace_character);

        assert_eq!(
            "12cdefghijklmnopqrstuvwxyz", result,
            "Replace multiple different characters with only character in `replace string`"
        );
    }
}
