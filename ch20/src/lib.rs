use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    send: Sender<Job>,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

impl Worker {
    fn new(id: usize, recv: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let job = recv.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job; executing.", id);

            job.call_box();
        });
        Worker { id, thread }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The `size` is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is 0.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);
        let (send, recv) = mpsc::channel();
        let recv = Arc::new(Mutex::new(recv));

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&recv)));
        }

        ThreadPool { workers, send }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.send.send(job).unwrap();
    }
}
