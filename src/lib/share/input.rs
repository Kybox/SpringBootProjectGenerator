use std::io::{stdin};

pub fn get_user_input() -> String {

    let reader = stdin();
    let mut buffer = String::new();

    // Get user input
    reader.read_line(&mut buffer).ok().expect("Input error");

    // Remove line breaks
    buffer.truncate(buffer.trim_end_matches(&['\r', '\n'][..]).len());

    return buffer;
}
