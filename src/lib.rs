use std::thread;
use std::sync::{Arc, mpsc, Mutex};

pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Creates a new ThreadPool instance
    /// 
    /// The size is the number of threads in the pool.
    /// 
    /// # Panics
    /// 
    /// Will panic if size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert_ne!(size, 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut threads = Vec::with_capacity(size);

        for i in 0..size {
            threads.push(Worker::new(i, Arc::clone(&receiver)));
        }

        ThreadPool {
            threads,
            sender,
        }
    }

    pub fn execute<F>(&self, task: F)
        where F : FnOnce() + Send + 'static
    {
        self.sender.send(Message::NewJob(Box::new(task))).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Dropping the thread pool!");
        
        for _ in 0..self.threads.len() {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.threads {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let msg = receiver.lock().unwrap().recv().unwrap();

                match msg {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job.", id);
                        job();
                    },
                    Message::Terminate => {
                        break;
                    }
                }
            }
            
            println!("Worker {} is dying.", id);
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}