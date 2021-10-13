//! Operation related types

pub(super) mod cqe;
pub(super) mod sqe;
pub(super) mod storage;

use bitflags::bitflags;

use crate::sys::IORING_FSYNC_DATASYNC;

bitflags! {
    /// Synchronized I/O file or data integrity completion.
    pub struct FsyncFlags: u32 {
        /// Flush data and all associated metadata to the underlying hardware.
        const FILESYNC = 0;
        /// Flush data, but will only flush metadata updates that are required
        /// to allow a subsequent read operation to complete successfully. This
        /// can reduce the number of disk operations that are required for
        /// applications that don't need the guarantees of file integrity
        /// completion.
        const DATASYNC = IORING_FSYNC_DATASYNC;
    }
}

bitflags! {
    /// The bit mask specifying the events the application is interested in.
    pub struct PollEvent: u32 {
        /// There is data to read.
        const IN = libc::POLLIN as _;
        /// There is urgent data to read.
        const PRI = libc::POLLPRI as _;
        /// Writing is now possible.
        const OUT = libc::POLLOUT as _;
        /// Error condition.
        const ERR = libc::POLLERR as _;
        /// Hung up.
        const HUP = libc::POLLHUP as _;
        /// Invalid polling request.
        const NVAL = libc::POLLNVAL as _;
    }
}
