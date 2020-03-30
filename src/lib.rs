use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    /// Create a new Threadpool.
    /// 
    /// The size is the number of threads in the pool.
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        ThreadPool {
            threads
        }
    }

    //TODO implement method pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError>

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static 
    {

    }
}