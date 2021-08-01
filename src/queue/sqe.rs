use crate::sys::io_uring_sqe;

/// Pack data into a SQE(Submission Queue Entry).
pub struct Packer<'a>(&'a mut io_uring_sqe);

impl<'a> Packer<'a> {
    pub fn new(sqe: &'a mut io_uring_sqe) -> Self {
        Self(sqe)
    }
}
