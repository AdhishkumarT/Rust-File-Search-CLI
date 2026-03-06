// src/utils.rs
// Handles file reading utilities for Rust File Search CLI

use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn read_lines(path: &str) -> Result<Vec<String>, String> {
    let file = File::open(path).map_err(|_| format!("File not found: {}", path))?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(l) => lines.push(l),
            Err(_) => return Err("Failed to read line from file.".to_string()),
        }
    }
    Ok(lines)
}
