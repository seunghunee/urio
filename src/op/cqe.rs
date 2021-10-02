use std::io;

use crate::{resultify, sys::io_uring_cqe};

/// CQE(Completion Queue Event), which represents a completed IO event.
///
/// This is added by kernel to CQ(Completion Queue) for each SQE that is
/// submitted. It contains the result of the operation submitted as part of the
/// SQE.
pub struct Cqe(io_uring_cqe);

impl Cqe {
    #[inline]
    pub(crate) fn new(cqe: &io_uring_cqe) -> Self {
        Self(*cqe)
    }

    #[inline]
    pub fn user_data(&self) -> u64 {
        self.0.user_data
    }

    #[inline]
    pub fn result(&self) -> io::Result<u32> {
        resultify(self.0.res)
    }
}
