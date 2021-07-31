pub mod builder;

mod queue;
mod sys;

use queue::{cq::Cq, sq::Sq};

/// io_uring interface.
pub struct Uring {
    fd: i32,
    sq: Sq,
    cq: Cq,
    flags: u32,
    features: u32,
}

impl Drop for Uring {
    fn drop(&mut self) {
        unsafe { libc::close(self.fd) };
    }
}
