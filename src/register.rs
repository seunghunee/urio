use std::{
    io::{self, IoSlice},
    ptr,
    sync::Arc,
};

use crate::{
    sys::{self, IORING_REGISTER_BUFFERS, IORING_UNREGISTER_BUFFERS},
    Uring,
};

/// A Registrar, registering long term kernel references to resoucres.
pub struct Registrar {
    uring: Arc<Uring>,
}

impl Registrar {
    pub(crate) fn new(uring: Arc<Uring>) -> Self {
        Self { uring }
    }

    pub fn register_buffers(&self, bufs: &[IoSlice<'_>]) -> io::Result<()> {
        let ret = unsafe {
            sys::io_uring_register(
                self.uring.fd,
                IORING_REGISTER_BUFFERS,
                bufs.as_ptr() as _,
                bufs.len() as _,
            )
        };
        if ret < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    pub fn unregister_buffers(&self) -> io::Result<()> {
        let ret = unsafe {
            sys::io_uring_register(self.uring.fd, IORING_UNREGISTER_BUFFERS, ptr::null(), 0)
        };
        if ret < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }
}
