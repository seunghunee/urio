pub(super) mod cqe;
pub(super) mod sqe;

use bitflags::bitflags;

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
