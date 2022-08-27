use crate::target::Target;
use std::{env, process};
use crate::pool::ThreadPool;

mod error;
mod print;
mod target;
mod pool;

fn main() {
    let args: Vec<String> = env::args().collect();
    let target: Target = match Target::build(&args) {
        Ok(target) => target,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = ThreadPool::run(target) {
        eprintln!("{}", e);
        process::exit(1);
    };
}
