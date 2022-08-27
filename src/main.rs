use crate::pool::ThreadPool;
use crate::target::Target;
use std::{env, process};

mod error;
mod pool;
mod search;
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
    let thread_pool = match ThreadPool::new(&target.paths.len()) {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    if let Err(e) = thread_pool.run(target) {
        eprintln!("{}", e);
        process::exit(1);
    };
}
