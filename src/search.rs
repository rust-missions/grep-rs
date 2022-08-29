use crate::error::Error;
use std::fs;

pub fn run(keyword: &str, path: &String) -> String {
    let contents = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => return Error::FailedToReadTextFile(path.to_string()).to_string(),
    };
    let mut search_results = Vec::new();
    for (line_idx, line_content) in grep(keyword, &contents) {
        search_results.push(format!("{} {} - {}", path, line_idx, line_content));
    }

    search_results.join("\n")
}

fn grep<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let mut results = Vec::new();
    for (idx, line) in contents.lines().enumerate() {
        if line.contains(query) {
            results.push((idx, line));
        }
    }
    results
}
