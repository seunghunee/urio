mod builder;
mod queue;
mod sys;

use std::io;

pub use builder::Builder;
pub use queue::sqe::Packer;
use queue::{cq::Cq, sq::Sq};

/// io_uring interface.
pub struct Uring {
    fd: i32,
    sq: Sq,
    cq: Cq,
    flags: u32,
    features: u32,
}

impl Uring {
    /// Create a new [`Uring`] instance with given `entries` entries
    /// and default configuration values.
    ///
    /// `entries` denote the number of sqes and it must be a power of 2,
    /// in the range `1..=4096`
    ///
    /// See the [`Builder`] for more details on configuration options.
    pub fn new(entries: u32) -> io::Result<Uring> {
        Builder::new(entries).build()
    }

    /// Allocate and push a vacant SQE(Submission Queue Entry) to the end
    /// of the SQ(Submission Queue) and return a new sqe data [`Packer`].
    ///
    /// # Errors
    ///
    /// If the SQ is full, then an error is returned.
    pub fn alloc_sqe(&mut self) -> Result<Packer, &'static str> {
        self.sq.alloc_sqe()
    }
}

impl Drop for Uring {
    fn drop(&mut self) {
        unsafe { libc::close(self.fd) };
    }
}
