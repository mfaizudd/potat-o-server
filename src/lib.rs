use std::{thread::{JoinHandle, self}, sync::{mpsc, Arc, Mutex}};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

impl ThreadPool {

    /// Create a new thread pool
    /// 
    /// The count is the number of threads in the pool
    /// 
    /// # Panics
    /// 
    /// The function will panic if the count is zero
    pub fn new(count: usize) -> ThreadPool {
        assert!(count > 0);
        let mut workers = Vec::with_capacity(count);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..count {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: JoinHandle<()>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        Worker { id, thread: thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            print!("Worker {} got a job. Executing.", id);
            job();
        })}
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;