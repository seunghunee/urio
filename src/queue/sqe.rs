use crate::sys::io_uring_sqe;

/// Pack data into a SQE(Submission Queue Entry).
pub struct Packer<'a>(&'a mut io_uring_sqe);

impl<'a> Packer<'a> {
    /// Create a new [`Packer`] with the given mutable reference to SQE.
    pub fn new(sqe: &'a mut io_uring_sqe) -> Self {
        sqe.flags = 0;
        sqe.ioprio = 0;
        sqe.__bindgen_anon_3.rw_flags = 0;
        sqe.user_data = 0;
        sqe.__bindgen_anon_4.__pad2 = [0, 0, 0];
        Self(sqe)
    }
}
