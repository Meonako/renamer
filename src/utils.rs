use std::{io, io::Write};

use crate::default::*;

pub fn input() -> String {
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("to read input");
    return str.trim().to_owned()
}

pub fn print(msg: &str) {
    print!("{}", msg);
    io::stdout().flush().unwrap();
}

pub fn value_or_default(mut str: String, default_value: &str) -> String {
    if str.trim() == "" {
        str.replace_range(.., default_value);
    }
    str
}

pub fn receive_directory_path() -> String {
    print("Directory Path: ");

    let str = input();
    value_or_default(str, DEFAULT_DIR)
}

pub fn receive_text_to_replace() -> String {
    print("Enter text-to-replace: ");

    let str = input();
    value_or_default(str, DEFAULT_TEXT_TO_REPLACE)
}

pub fn receive_replace_text() -> String {
    print("Replace that text with: ");

    let str = input();
    value_or_default(str, "new-name")
}