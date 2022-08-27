use crate::error::Error;
use std::fs;

pub fn run(keyword: &String, path: &String) -> Result<String, Error> {
    let contents = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => return Err(Error::FileSystemError),
    };
    let mut search_results = Vec::new();
    for (line_idx, line_content) in grep(keyword, &contents) {
        search_results.push(format!("{} {} - {}", path, line_idx, line_content));
    }
    Ok(search_results.join("\n"))
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
