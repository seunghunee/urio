//! # urio
//!
//! urio is a [io_uring](https://kernel.dk/io_uring.pdf) library written in
//! Rust. It provides a **safe** Rust-friendly interface.

mod builder;
pub mod op;
mod queue;
mod sys;

use std::{
    io,
    os::unix::io::{AsRawFd, RawFd},
    ptr,
};

pub use builder::Builder;
pub use op::{cqe::Cqe, sqe::Packer};
use queue::{
    cq::{Cq, Reaper},
    sq::Sq,
};
use sys::{
    IORING_ENTER_GETEVENTS, IORING_ENTER_SQ_WAKEUP, IORING_SETUP_IOPOLL, IORING_SETUP_SQPOLL,
};

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

    /// Allocate a vacant SQE(Submission Queue Entry) and push it to the
    /// end of the SQ(Submission Queue).
    /// Returns a new sqe data [`Packer`].
    ///
    /// # Errors
    ///
    /// If the SQ is full, then an error is returned.
    pub fn alloc_sqe(&mut self) -> Result<Packer, &'static str> {
        self.sq.alloc_sqe()
    }

    /// Submit pending sqes in the SQ ring to the kernel.
    /// Returns number of sqes submitted.
    pub fn submit(&mut self) -> io::Result<usize> {
        self.submit_and_wait(0)
    }

    /// Like [`submit`], but allows waiting for events as well.
    /// Returns number of sqes submitted.
    ///
    /// [`submit`]: method@Self::submit
    pub fn submit_and_wait(&mut self, min_complete: u32) -> io::Result<usize> {
        let mut flags = 0;
        let to_submit = self.sq.flush();

        if self.has_sqpoll() {
            if self.sq.needs_wakeup() {
                flags |= IORING_ENTER_SQ_WAKEUP;
            } else if min_complete == 0 {
                return Ok(to_submit as _);
            }
        }

        if min_complete > 0 || self.is_io_polled() {
            flags |= IORING_ENTER_GETEVENTS;
        }

        let ret = unsafe { sys::enter(self.fd, to_submit, min_complete, flags, ptr::null()) };
        if ret < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(ret as _)
    }

    /// Reap a CQE(Completion Queue Event). Returns a new [`Cqe`].
    ///
    /// # Errors
    ///
    /// If the CQ(Completion Queue) is empty, then an error is returned.
    #[inline]
    pub fn reap_cqe(&mut self) -> Result<Cqe, &'static str> {
        Ok(self.reap_exact_cqes(1)?.next().unwrap())
    }

    /// Like [`reap_cqe`], but it reaps the exact `n` CQEs. Returns a
    /// [`Reaper`].
    ///
    /// # Errors
    ///
    /// If CQEs in the CQ(Completion Queue) is less than `n`, then an error is
    /// returned.
    ///
    /// [`reap_cqe`]: method@Self::reap_cqe
    #[inline]
    pub fn reap_exact_cqes(&mut self, n: usize) -> Result<Reaper, &'static str> {
        self.cq.reap(n)
    }

    /// Returns the number of entries the SQ can hold.
    #[inline]
    pub fn sq_capacity(&self) -> usize {
        self.sq.capacity()
    }

    /// Returns the number of events the CQ can hold.
    #[inline]
    pub fn cq_capacity(&self) -> usize {
        self.cq.capacity()
    }

    /// Returns the number of events in the CQ.
    #[inline]
    pub fn cq_len(&self) -> usize {
        self.cq.len()
    }

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

impl AsRawFd for Uring {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}
