use crate::target::Target;
use std::{env, process};

mod target;

fn main() {
    let args: Vec<String> = env::args().collect();
    let target: Target = match Target::build(&args) {
        Ok(target) => target,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    let _keyword = target.keyword;
    let _path = target.path;
}
