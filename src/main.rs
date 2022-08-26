use grep_rs::Args;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let args: Args = match Args::build(&args) {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    let _keyword = args.keyword;
    let _path = args.path;
}
