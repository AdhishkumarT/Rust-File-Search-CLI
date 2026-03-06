// Build a Rust CLI application called "Rust File Search CLI".
//
// Requirements:
//
// 1. The program should search for a keyword inside a text file.
// 2. The user should provide two command line arguments:
//    - keyword
//    - file path
//
// 3. The program should read the file and return:
//    - matching lines
//    - line numbers
//
// 4. Project structure:
//
// src/main.rs
// src/search.rs
// src/utils.rs
//
// 5. main.rs should handle command line arguments
// 6. search.rs should contain the search logic
// 7. utils.rs should handle file reading utilities
//
// 8. Use only Rust standard library.
// 9. Output should be formatted in terminal.
// 10. Handle errors like file not found.

mod search;
mod utils;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: cargo run <KEYWORD> <FILE_PATH>");
        std::process::exit(1);
    }

    let keyword = &args[1];
    let file_path = &args[2];

    match utils::read_lines(file_path) {
        Ok(lines) => {
            let results = search::search_lines(keyword, &lines);
            if results.is_empty() {
                println!("No matches found.");
            } else {
                for (num, line) in results {
                    println!("Line {}: {}", num, line);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
