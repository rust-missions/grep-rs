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
        if *workload == 0 {
            return Err(Error::Thread);
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

    pub fn run(self, target: Target) -> Result<(), Error> {
        match self.master.run(target) {
            Ok(_) => {
                for worker in self.workers {
                    if worker.thread.join().is_err() {
                        return Err(Error::Thread);
                    }
                }
            }
            Err(e) => return Err(e),
        };

        Ok(())
    }
}

pub struct Master {
    sender: Sender<Job>,
}

impl Master {
    pub fn run(self, target: Target) -> Result<(), Error> {
        let (result_sender, result_receiver) = mpsc::channel::<JobResult>();

        for path in target.paths.clone() {
            let keyword = target.keyword.clone();
            let result_sender = result_sender.clone();

            self.send_job(move || {
                Self::print_search_result(result_sender, search::run(&keyword, &path))
            })?
        }
        self.close(target.paths.len(), &result_receiver);

        Ok(())
    }

    fn send_job<F>(&self, job: F) -> Result<(), Error>
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(job);
        if self.sender.send(job).is_err() {
            return Err(Error::Thread);
        }
        Ok(())
    }

    fn print_search_result(result_sender: Sender<JobResult>, result: String) {
        let send_result = result_sender.send(Box::new(move || println!("{}", result)));
        if send_result.is_err() {
            println!("{}", Error::Thread);
            process::exit(1);
        }
    }

    fn close(self, total_workload: usize, result_receiver: &Receiver<JobResult>) {
        for (idx, result) in result_receiver.iter().enumerate() {
            result();
            if idx == total_workload - 1 {
                drop(self.sender);
                return;
            }
        }
    }
}

pub struct Worker {
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            match receiver.lock() {
                Ok(mutex) => match mutex.recv() {
                    Ok(job) => job(),
                    Err(_) => break,
                },
                Err(_) => {
                    eprintln!("{}", Error::Thread);
                    process::exit(1);
                }
            }
        });
        Worker { thread }
    }
}
