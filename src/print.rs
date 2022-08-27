use crate::error::Error;
use crate::target::Target;
use std::fs;

pub fn run(target: Target) -> Result<(), Error> {
    for path in target.paths.iter() {
        let contents = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(_) => return Err(Error::FileSystemError),
        };
        for (line_idx, line_content) in search(&target.keyword, &contents) {
            println!("{} {} - {}", path, line_idx, line_content);
        }
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
