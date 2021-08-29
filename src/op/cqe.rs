use std::io;

use crate::sys::io_uring_cqe;

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
        if self.0.res < 0 {
            Err(io::Error::from_raw_os_error(-self.0.res))
        } else {
            Ok(self.0.res as _)
        }
    }
}