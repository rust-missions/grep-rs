use std::fmt;

pub struct Args {
    pub keyword: String,
    pub path: String,
}

impl Args {
    pub fn build(args: &[String]) -> Result<Args, Error> {
        let mut args = args.iter().skip(1);
        let keyword = match args.next() {
            Some(keyword) => keyword.clone(),
            None => return Err(Error::MissingKeyword),
        };
        let path = match args.next() {
            Some(path) => path.clone(),
            None => return Err(Error::MissingPath),
        };
        if args.len() > 0 {
            let args = args.map(|x| x.to_string()).collect();
            return Err(Error::TooManyArgs(args));
        }

        Ok(Args { keyword, path })
    }
}

pub enum Error {
    MissingKeyword,
    MissingPath,
    TooManyArgs(Vec<String>),
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
        }
    }
}
