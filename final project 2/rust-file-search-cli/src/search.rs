// src/search.rs
// Contains search logic for Rust File Search CLI

pub fn search_lines<'a>(keyword: &str, lines: &'a [String]) -> Vec<(usize, &'a str)> {
    lines
        .iter()
        .enumerate()
        .filter_map(|(i, line)| {
            if line.contains(keyword) {
                Some((i + 1, line.as_str()))
            } else {
                None
            }
        })
        .collect()
}
