use std::{fmt, fs};

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
        Ok(Target { keyword, paths: find_all_paths(path)? })
    }
}

fn find_all_paths(path: String) -> Result<Vec<String>, Error> {
    return match fs::metadata(&path) {
        Ok(metadata) => {
            if metadata.is_dir() {
               return match find_files_in_directory(path) {
                   Ok(paths) => Ok(paths),
                   Err(_) => Err(Error::FileSystemError),
               };
            }
            Ok(vec![path])
        }
        Err(_) => Err(Error::InvalidPath(path)),
    };
}

fn find_files_in_directory(dir_path: String) -> Result<Vec<String>, Error> {
    let mut paths: Vec<String> = Vec::new();
    let entries = match fs::read_dir(&dir_path) {
        Ok(read_dir) => read_dir,
        Err(_) => return Err(Error::FileSystemError),
    };

    for entry in entries {
        let path = entry.unwrap().path();
        let path_name = path.file_name().unwrap().to_string_lossy().into_owned();
        if path.is_dir() {
            paths.append(&mut find_files_in_directory(path_name)?);
            continue;
        }
        paths.push(format!("{}/{}", dir_path, path_name));
    }

    Ok(paths)
}

pub enum Error {
    MissingKeyword,
    MissingPath,
    TooManyArgs(Vec<String>),
    InvalidPath(String),
    FileSystemError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::MissingKeyword => write!(f, "Missing arguments: keyword, path"),
            Error::MissingPath => write!(f, "Missing arguments: path"),
            Error::TooManyArgs(redundant_args) => {
                let args = redundant_args.join(", ");
                write!(f, "Unnecessary arguments: {}", args)
            }
            Error::InvalidPath(path) => write!(f, "Invalid path: {}", path),
            Error::FileSystemError => write!(f, "Internal Error"),
        }
    }
}
