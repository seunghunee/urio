use std::{
    io::{self, IoSlice},
    ptr,
    sync::Arc,
};

use crate::{
    sys::{self, IORING_REGISTER_BUFFERS, IORING_UNREGISTER_BUFFERS},
    Uring,
};

/// A Registrar, registering long term kernel references to resoucres (e.g. user
/// buffers, files, eventfd, personality, restrictions).
///
/// Registering files or user buffers allows the kernel to take long term
/// references to internal data structures or create long term mappings of
/// application memory, greatly reducing per-I/O overhead.
pub struct Registrar {
    uring: Arc<Uring>,
}

impl Registrar {
    pub(crate) fn new(uring: Arc<Uring>) -> Self {
        Self { uring }
    }

    /// Register a slice of buffers.
    ///
    /// The buffers associated with the iovecs will be locked in memory and
    /// charged against the user's `RLIMIT_MEMLOCK` resource limit. See
    /// `getrlimit`(2) for more information. Additionally, there is a size limit
    /// of 1GiB per buffer. Currently, the buffers must be anonymous,
    /// non-file-backed memory, such as that returned by `malloc`(3) or
    /// `mmap`(2) with the `MAP_ANONYMOUS` flag set. It is expected that this
    /// limitation will be lifted in the future. Huge pages are supported as
    /// well. Note that the entire huge page will be pinned in the kernel, even
    /// if only a portion of it is used.
    ///
    /// After a successful call, the supplied buffers are mapped into the kernel
    /// and eligible for I/O. To make use of them, the application must use the
    /// [`packup_read_fixed`] or [`packup_write_fixed`].
    ///
    /// It is perfectly valid to setup a large buffer and then only use part of
    /// it for an I/O, as long as the range is within the originally mapped
    /// region.
    ///
    /// An application can increase or decrease the size or number of registered
    /// buffers by first unregistering the existing buffers, and then issuing a
    /// new call to [`register_buffers`] with the new buffers.
    ///
    /// Note that before kernel 5.13 registering buffers would wait for the ring to
    /// idle. If the application currently has requests in-flight, the
    /// registration will wait for those to finish before proceeding.
    ///
    /// An application need not unregister buffers explicitly before shutting
    /// down the io_uring instance.
    ///
    /// **Available since kernel 5.1.**
    ///
    /// [`packup_read_fixed`]:method@crate::Packer::packup_read_fixed
    /// [`packup_write_fixed`]:method@crate::Packer::packup_write_fixed
    /// [`register_buffers`]:method@Self::register_buffers
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

    /// Unregister all previously registered buffers.
    ///
    /// **Available since kernel 5.1.**
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
