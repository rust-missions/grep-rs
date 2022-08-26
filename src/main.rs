use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Missing arguments");
        process::exit(1);
    }
    if args.len() > 3 {
        eprintln!("Too much arguments");
        process::exit(1);
    }
    let keyword = args[1].clone();
    let path = args[2].clone();
}
