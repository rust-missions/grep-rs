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

    pub fn run(self, target: Target) -> Result<(), Error> {
        match self.master.run(target) {
            Ok(_) => {
                for worker in self.workers {
                    if let Err(_) = worker.thread.join() {
                        return Err(Error::ThreadPoolError);
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

        for path in target.paths.to_vec() {
            let keyword = target.keyword.clone();
            let result_sender = result_sender.clone();

            if let Err(e) = self.execute(move || match search::run(&keyword, &path) {
                Ok(search_result) => {
                    if let Err(e) =
                    result_sender.send(Box::new(move || println!("{}", search_result)))
                    {
                        eprintln!("{}", e);
                        process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                }
            }) {
                return Err(e);
            };
        }
        self.close(target.paths.len(), &result_receiver);

        Ok(())
    }

    fn execute<F>(&self, f: F) -> Result<(), Error>
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        match self.sender.send(job) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::ThreadPoolError),
        }
    }

    fn close(self, total_workload: usize, result_receiver: &Receiver<JobResult>) -> () {
        for (idx, result) in result_receiver.iter().enumerate() {
            result();
            if idx == total_workload - 1 {
                drop(self.sender);
                return ();
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
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(1);
                }
            }
        });
        Worker { thread }
    }
}
