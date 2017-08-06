extern crate crossbeam;

use self::crossbeam::*;
use std::marker::Send;

/// Pool represents a pool of crossbeam scoped threads!
pub struct Pool {
    workers: Vec<CrossbeamWorker>
}

/// CrossbeamPoolErrors represents various errors
/// occurring while creating and managing the
/// crossbeam pool of scoped threads!
#[derive(Debug)]
pub enum CrossbeamPoolError {
    CreationError(&'static str)
}

/// CrossbeamWorker contains a handle to the scoped thread
struct CrossbeamWorker {
    id: usize,
    thread: ScopedJoinHandle<()>,
}

impl CrossbeamWorker {
    /// Returns a new CrossbeamWorker with the given `id`
    ///
    /// # Arguments
    ///
    /// * `id` - A usize number to uniquely identify a CrossbeamWorker
    ///
    fn new(id: usize) -> Self {
        let scoped_thread = crossbeam::scope(|scope| {
            scope.spawn(|| {})
        });
        CrossbeamWorker { id, thread: scoped_thread }
    }
}

pub type CrossbeamPoolResult<'a> = Result<Pool, CrossbeamPoolError>;

impl Pool {
    /// Returns a new crossbeam pool of scoped threads
    /// with the given size
    ///
    /// # Arguments
    ///
    /// * `size` - A usize number that represents the size of the pool. Must be > 0.
    ///
    /// # Example
    ///
    /// use crossbeam_pool::pool::{Pool};
    /// let pool = Pool::new(9).unwrap();
    ///
    pub fn new(size: usize) -> CrossbeamPoolResult<'static> {
        if size <= 0 {
            Err(CrossbeamPoolError::CreationError("Pool size must be greater than zero!"))
        } else {
            let mut workers = Vec::with_capacity(size);
            for id in 0..size {
                workers.push(CrossbeamWorker::new(id));
            }
            Ok(Pool { workers })
        }
    }

    pub fn execute<F, T>(&self, f: F) where F: FnOnce() -> T + Send + 'static {}
}
