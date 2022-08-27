use crate::target::Target;
use std::{env, process};

mod error;
mod target;
mod print;

fn main() {
    let args: Vec<String> = env::args().collect();
    let target: Target = match Target::build(&args) {
        Ok(target) => target,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    if let Err(e) = print::run(target) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
