use std::{io, io::Write, process::exit};

use crate::default::*;

pub fn input() -> String {
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("to read input");
    return str.trim().to_owned();
}

pub fn print(msg: &str) {
    print!("{}", msg);
    io::stdout().flush().unwrap();
}

pub fn wait_and_exit(code: i32) {
    let mut hold = String::new();
    print("\nPress enter to continue...");
    io::stdin().read_line(&mut hold).unwrap();
    exit(code)
}

pub fn value_or_default(mut str: String, default_value: &str) -> String {
    if str.trim() == "" {
        str.replace_range(.., default_value);
    }
    str
}

pub fn receive_directory_path() -> String {
    let wd = std::env::current_dir().expect("access to current directory");

    print(&format!("Directory Path ( {} ): ", wd.to_str().unwrap()));

    let str = input();
    value_or_default(str, DEFAULT_DIR)
}

pub fn receive_text_to_replace() -> String {
    print(&format!(
        "Enter text-to-replace ( {DEFAULT_TEXT_TO_REPLACE} ): "
    ));

    let str = input();
    value_or_default(str, DEFAULT_TEXT_TO_REPLACE)
}

pub fn receive_replace_text(original: &str) -> String {
    print(&format!("Replace \"{}\" with ( \"\" [EMPTY] ): ", original));

    let str = input();
    str.trim().to_owned()
}
