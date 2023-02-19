mod default;
mod utils;

use std::{fs, io};

use default::*;
use utils::*;

fn main() {
    let dir_to_read = receive_directory_path();
    let text_to_replace = receive_text_to_replace();
    let replace_with = receive_replace_text(&text_to_replace);

    println!("--------------------------------------------------");

    loop {
        println!("Are you sure you want to do this? (y/n)");
        println!(
            "Replace \"{}\" with \"{}\" {}",
            text_to_replace,
            replace_with,
            if replace_with == "" {
                "[EMPTY] aka REMOVED FROM FILE NAME"
            } else {
                ""
            }
        );
        println!(
            "In Directory: {}",
            if dir_to_read == DEFAULT_DIR {
                "Current Directory"
            } else {
                &dir_to_read
            }
        );
        print("Your choice: ");
        let answer = input();

        match answer.to_lowercase().trim() {
            "y" | "yes" => break,
            "n" | "no" | "q" | "quit" => return,
            _ => continue,
        }
    }

    let paths = fs::read_dir(dir_to_read).expect("read directory");

    println!("--------------------------------------------------");

    let mut renamed = 0;
    for path in paths {
        let file = path.expect("path value");

        let mut full_path = file.path();
        let old_path = full_path.clone();

        let filename = file.file_name();
        let filename = filename.to_str().expect("filename to string");

        if filename.contains(&text_to_replace) {
            let new_filename = filename.replace(&text_to_replace, &replace_with);

            println!("Renaming: {} to {}", filename, new_filename);

            full_path.set_file_name(new_filename);
            if let Err(e) = fs::rename(old_path, full_path) {
                println!("{}", e)
            }

            renamed += 1;
        }
    }

    println!("Renamed: {} file(s)", renamed);

    let mut hold = String::new();
    print("\nPress enter to continue...");
    io::stdin().read_line(&mut hold).unwrap();
}
