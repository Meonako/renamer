mod args;
mod default;
mod utils;

use std::fs;

use clap::Parser;

use args::*;
use default::*;
use utils::*;

fn main() {
    let arguments = Arguments::parse();

    let dir_to_read = if arguments.dir_to_read.is_some() {
        arguments.dir_to_read.unwrap()
    } else if arguments.dir_to_read_pos.is_some() {
        arguments.dir_to_read_pos.unwrap()
    } else {
        receive_directory_path()
    };

    let text_to_replace = if arguments.text_to_replace.is_some() {
        arguments.text_to_replace.unwrap()
    } else if arguments.text_to_replace_pos.is_some() {
        arguments.text_to_replace_pos.unwrap()
    } else {
        receive_text_to_replace()
    };

    let replace_with = if arguments.replace_with.is_some() {
        arguments.replace_with.unwrap()
    } else if arguments.replace_with_pos.is_some() {
        arguments.replace_with_pos.unwrap()
    } else {
        receive_replace_text(&text_to_replace)
    };

    println!("--------------------------------------------------");

    let dir = std::path::Path::new(&dir_to_read);

    if !dir.exists() {
        print(&format!("Directory \'{dir_to_read}\' not found!"));
        wait_and_exit();
    }

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

    let paths = fs::read_dir(dir).expect("read directory");

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

    wait_and_exit();
}
