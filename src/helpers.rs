pub fn test() {
    println!("test")
}

pub fn starts_and_ends_with_one_of(input: &str, chars: &[char]) -> bool {
    if input.len() < 2 {
        return false;
    }

    let first = input.chars().nth(0).unwrap();
    let last = input.chars().nth(input.len() - 1).unwrap();
    if first != last {
        return false;
    }

    let starts_with_char = chars.contains(&first);
    let ends_with_char = chars.contains(&last);

    starts_with_char && ends_with_char
}
