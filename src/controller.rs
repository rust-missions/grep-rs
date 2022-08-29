use crate::error::Error;
use crate::{search, Target, ThreadPool};

pub fn run(args: Vec<String>) -> Result<(), Error> {
    let target: Target = Target::build(&args)?;
    let workload = &target.paths.len();
    if workload > &1 {
        ThreadPool::new(workload)?.run(target)?;
        return Ok(());
    }
    match target.paths.get(0) {
        Some(path) => println!("{}", search::run(&target.keyword, path)),
        None => return Err(Error::EmptyDirectory(target.raw_path)),
    };
    Ok(())
}
