//! # urio
//!
//! urio is a [io_uring](https://kernel.dk/io_uring.pdf) library written in
//! Rust. It provides a **safe** Rust-friendly interface.

mod builder;
pub use builder::Builder;

pub mod op;
pub use op::{cqe::Cqe, sqe::Packer};

mod queue;
pub use queue::{Cq, Reaper, Sq};

mod sys;

use std::{io, os::unix::io::RawFd};

use sys::{IORING_SETUP_IOPOLL, IORING_SETUP_SQPOLL};

/// Create a new io_uring instance with given `entries` entries and default
/// configuration values. On success, [`Sq`] and [`Cq`] will be returned.
///
/// `entries` denote the number of sqes and it must be a power of 2, in the
/// range `1..=4096`
///
/// See the [`Builder`] for more details on configuration options.
pub fn new(entries: u32) -> io::Result<(Sq, Cq)> {
    Builder::new(entries).build()
}

struct Uring {
    fd: RawFd,
    flags: u32,
    features: u32,
}

impl Uring {
    /// Return `true` if IO polling is utilized.
    #[inline]
    fn is_io_polled(&self) -> bool {
        self.flags & IORING_SETUP_IOPOLL != 0
    }

    /// Return `true` if the kernel side SQ polling thread exist.
    #[inline]
    fn has_sqpoll(&self) -> bool {
        self.flags & IORING_SETUP_SQPOLL != 0
    }
}

impl Drop for Uring {
    #[inline]
    fn drop(&mut self) {
        unsafe { libc::close(self.fd) };
    }
}
