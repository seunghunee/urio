pub mod builder;

mod queue;
mod sys;

use std::io;

use builder::Builder;
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
}

impl Drop for Uring {
    fn drop(&mut self) {
        unsafe { libc::close(self.fd) };
    }
}
