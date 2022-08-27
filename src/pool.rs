use crate::error::Error;
use crate::{search, Target};

use std::cmp::min;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::{process, thread};

type Job = Box<dyn FnOnce() + Send + 'static>;
type JobResult = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    pub master: Master,
    pub workers: Vec<Worker>,
}

impl ThreadPool {
    pub fn new(workload: &usize) -> Result<ThreadPool, Error> {
        if workload == 0 {
            return Err(Error::ThreadPoolError);
        }
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let master = Master { sender };
        let mut workers = Vec::new();
        for _ in 0..min(10, *workload) {
            workers.push(Worker::new(receiver.clone()));
        }

        Ok(ThreadPool { master, workers })
    }

    pub fn run(&self, target: Target) -> Result<(), Error> {
        self.master.run(target);

        Ok(())
    }
}

pub struct Master {
    sender: Sender<Job>,
}

impl Master {
    pub fn run(&self, target: Target) {
        let (result_sender, result_receiver) = mpsc::channel::<JobResult>();

        for path in target.paths.to_vec() {
            let keyword = target.keyword.clone();
            let result_sender = result_sender.clone();

            execute(&self.sender, move || match search::run(&keyword, &path) {
                Ok(search_result) => result_sender
                    .send(Box::new(move || println!("{}", search_result)))
                    .unwrap(),
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                }
            });
        }
        self.close(target.paths.len(), &result_receiver);
    }

    fn close(&self, total_workload: usize, result_receiver: &Receiver<JobResult>) {
        for (idx, result) in result_receiver.iter().enumerate() {
            result();
            if idx == total_workload - 1 {
                process::exit(0);
            }
        }
    }
}

fn execute<F>(sender: &Sender<Job>, f: F)
where
    F: FnOnce() + Send + 'static,
{
    let job = Box::new(f);
    sender.send(job).unwrap();
}

pub struct Worker {
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
