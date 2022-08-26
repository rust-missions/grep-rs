use std::fmt;

pub struct Args {
    pub keyword: String,
    pub path: String,
}

impl Args {
    pub fn build(args: &[String]) -> Result<Args, &'static str> {
        let mut args = args.iter().skip(1);
        let keyword = match args.next() {
            Some(keyword) => keyword.clone(),
            None => return Err("missing arguments: keyword, path"),
        };
        let path = match args.next() {
            Some(path) => path.clone(),
            None => return Err("missing arguments: path"),
        };
        if args.len() > 0 {
            return Err("Too many arguments");
        }

        Ok(Args { keyword, path })
    }
}
