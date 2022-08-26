use std::process;

pub struct Args {
    pub keyword: String,
    pub path: String,
}

impl Args {
    pub fn build(args: &[String]) -> Result<Args, &'static str> {
        if args.len() > 3 {
            return Err("Too many arguments");
        }
        let keyword = match args.get(1) {
            Some(keyword) => keyword.clone(),
            None => return Err("missing arguments: keyword, path"),
        };
        let path = match args.get(2) {
            Some(path) => path.clone(),
            None => return Err("missing arguments: path"),
        };

        Ok(Args { keyword, path })
    }
}
