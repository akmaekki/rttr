extern crate rttr;

use std::env;

pub fn main() {
    // Read command line arguments
    let args: Vec<String> = env::args().collect();
    let config = rttr::parse_config(&args);

    // Allocate buffer owned by main
    let mut buffer = String::new();

    // Read from stdin into buffer
    rttr::read_from_stdin(&mut buffer).expect("Reading from stdin should work");

    // Pass string to make replacements in, characters to replace and
    // replacement characters to function and return processed input.
    // rttr::replace()

    // Logik eventuell auslagern in `run`-Methode.
    // `main` sollte kurz bleiben.
    if config.is_delete_mode {
        let result = rttr::delete(&buffer, &config.search_characters);
        print!("{}", result);
    } else {
        let result = rttr::replace_multiple_multiple(
            &buffer,
            &config.search_characters,
            &config.replac_characters,
        );
        print!("{}", result);
    }
}
