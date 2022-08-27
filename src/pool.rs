use crate::error::Error;
use std::sync::{mpsc, Arc, Mutex};
use std::{process, thread};
use std::sync::mpsc::Receiver;
use crate::{search, Target};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool;

impl ThreadPool {
    pub fn run(target: Target) -> Result<(), Error> {
        let total_workload = &target.paths.len();
        let (job_sender, job_receiver) = mpsc::channel();
        let (result_sender, result_receiver) = mpsc::channel();

        let job_receiver = Arc::new(Mutex::new(job_receiver));
        let mut workers = Vec::new();
        for _ in 0..5 {
            workers.push(Worker::new(job_receiver.clone()));
        }

        for path in target.paths.to_vec() {
            let keyword = target.keyword.clone();
            let result_sender = result_sender.clone();
            execute(&job_sender, move || match search::run(&keyword, &path) {
                Ok(search_result) => result_sender
                    .send(Box::new(move || println!("{}", search_result)))
                    .unwrap(),
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                }
            });
        }

        for (idx, result) in result_receiver.iter().enumerate() {
            result();
            if idx == total_workload - 1 {
                return Ok(());
            }
        }
        Ok(())
    }
}

fn execute<F>(sender: &mpsc::Sender<Job>, f: F)
    where
        F: FnOnce() + Send + 'static,
{
    let job = Box::new(f);
    sender.send(job).unwrap();
}

struct Worker {
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            job();
        });
        Worker { thread }
    }
}
