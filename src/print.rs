use crate::error::Error;
use std::fs;

pub fn run(keyword: &String, path: &String) -> Result<(), Error> {
    let contents = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => return Err(Error::FileSystemError),
    };
    for (line_idx, line_content) in search(keyword, &contents) {
        println!("{} {} - {}", path, line_idx, line_content);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let mut results = Vec::new();
    for (idx, line) in contents.lines().enumerate() {
        if line.contains(query) {
            results.push((idx, line));
        }
    }
    results
}
