# Rust File Search CLI (Mini Grep)

A lightweight command-line tool written in Rust that searches for keywords inside text files and returns matching lines along with their line numbers.

This project demonstrates file handling, command-line argument parsing, and efficient text searching using Rust.

---

## 🚀 Overview

Searching through large text files is a common task for developers, system administrators, and data engineers.

Rust File Search CLI implements a simplified version of the popular Unix `grep` utility.  
It scans a file for a given keyword and prints all matching lines with their corresponding line numbers.

The project focuses on building a fast and efficient command-line tool using Rust’s standard library.

---

## ✨ Features

- Fast keyword search inside text files
- Displays matching lines with line numbers
- Lightweight command-line interface
- Efficient file reading using Rust
- Minimal dependencies
- Clean modular architecture

---

## 🛠 Tech Stack

- Rust
- Rust Standard Library

---

## 📂 Project Structure

rust-file-search-cli
│
├── src
│   ├── main.rs
│   ├── search.rs
│   └── utils.rs
│
├── data
│   └── sample.txt
│
├── Cargo.toml
│
└── README.md

---

## 📄 Sample Data File

Example `data/sample.txt`:

INFO User login successful  
ERROR Database connection failed  
WARNING Disk usage high  
INFO Payment processed  
ERROR Network timeout  
INFO Service started  

---

## ▶️ How to Run

### 1. Clone the repository

git clone https://github.com/yourusername/rust-file-search-cli.git

cd rust-file-search-cli

---

### 2. Build the project

cargo build

---

### 3. Run the program

cargo run ERROR data/sample.txt

---

## 📊 Example Output

Line 2: ERROR Database connection failed  
Line 5: ERROR Network timeout  

---

## 🧠 How It Works

The application works in three steps:

1. The main module reads command-line arguments.
2. The search module scans the file line by line.
3. Matching lines containing the keyword are displayed along with their line numbers.

---

## 🎯 Learning Objectives

This project demonstrates key Rust programming concepts:

- File handling in Rust
- Command-line argument parsing
- String pattern matching
- Modular project structure
- CLI application development

---

## 📌 Future Improvements

Possible enhancements include:

- Case-insensitive search
- Regex pattern matching
- Directory-wide search
- Colored terminal output
- Performance optimizations for large files

