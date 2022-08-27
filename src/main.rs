use crate::pool::ThreadPool;
use crate::target::Target;
use std::{env, process};

mod controller;
mod error;
mod pool;
mod search;
mod target;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Err(e) = controller::run(args) {
        eprintln!("{}", e);
        process::exit(1);
    };
}
