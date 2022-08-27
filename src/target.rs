use crate::error::Error;

use std::fs;
use std::path::Path;

pub struct Target {
    pub keyword: String,
    pub paths: Vec<String>,
}

impl Target {
    pub fn build(args: &[String]) -> Result<Target, Error> {
        let mut args = args.iter().skip(1);
        let keyword = match args.next() {
            Some(keyword) => keyword.clone(),
            None => return Err(Error::MissingKeyword),
        };
        let path = match args.next() {
            Some(path) => path.to_string(),
            None => return Err(Error::MissingPath),
        };
        if args.len() > 0 {
            let args = args.map(|x| x.to_string()).collect();
            return Err(Error::TooManyArgs(args));
        }
        Ok(Target {
            keyword,
            paths: find_all_paths(path)?,
        })
    }
}

fn find_all_paths(path: String) -> Result<Vec<String>, Error> {
    match fs::metadata(&path) {
        Ok(metadata) => {
            if metadata.is_file() {
                return Ok(vec![path]);
            }
            match find_files_in_directory(path) {
                Ok(paths) => Ok(paths),
                Err(_) => Err(Error::FileSystem),
            }
        }
        Err(_) => Err(Error::InvalidPath(path)),
    }
}

fn find_files_in_directory(dir_path: String) -> Result<Vec<String>, Error> {
    let mut paths: Vec<String> = Vec::new();
    for entry in read_directory(&dir_path)? {
        let path = match entry {
            Ok(entry) => entry.path(),
            Err(_) => return Err(Error::FileSystem),
        };
        let full_path = format!("{}/{}", dir_path, to_path_name(&path)?);
        if path.is_dir() {
            paths.append(&mut find_files_in_directory(full_path)?);
            continue;
        }
        paths.push(full_path);
    }

    Ok(paths)
}

fn read_directory(dir_path: &String) -> Result<fs::ReadDir, Error> {
    match fs::read_dir(dir_path) {
        Ok(read_dir) => Ok(read_dir),
        Err(_) => Err(Error::FileSystem),
    }
}

fn to_path_name(path: &Path) -> Result<String, Error> {
    match path.file_name() {
        Some(file_name) => Ok(file_name.to_string_lossy().into_owned()),
        None => Err(Error::FileSystem),
    }
}
