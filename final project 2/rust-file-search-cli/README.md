# Rust File Search CLI

High-performance CLI tool in Rust for searching keywords in text files with line numbers.

## Features
- Search for a keyword in a text file
- Print matching lines with line numbers
- Handles errors gracefully
- Uses only Rust standard library

## Usage
```
cargo run <KEYWORD> <FILE_PATH>
```
Example:
```
cargo run ERROR data/sample.txt
```

## Example Output
```
Line 2: ERROR Database connection failed
Line 5: ERROR Network timeout
```

## Project Structure
- src/main.rs
- src/search.rs
- src/utils.rs
- data/sample.txt
