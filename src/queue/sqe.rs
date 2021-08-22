use bitflags::bitflags;
use std::os::unix::io::RawFd;

use crate::sys::{io_uring_sqe, IORING_OP_NOP, IORING_OP_POLL_ADD};

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

/// Pack data into a SQE(Submission Queue Entry).
pub struct Packer<'a>(&'a mut io_uring_sqe);

impl<'a> Packer<'a> {
    /// Create a new [`Packer`] with the given mutable reference to SQE.
    pub(crate) fn new(sqe: &'a mut io_uring_sqe) -> Self {
        sqe.flags = 0;
        sqe.ioprio = 0;
        sqe.__bindgen_anon_3.rw_flags = 0;
        sqe.user_data = 0;
        sqe.__bindgen_anon_4.__pad2 = [0, 0, 0];
        Self(sqe)
    }

    /// Pack `user_data` which to be passed back at completion time.
    #[inline]
    pub fn user_data(&mut self, user_data: u64) -> &mut Self {
        self.0.user_data = user_data;
        self
    }

    /// Pack IOSQE_* `flags`.
    #[inline]
    pub fn flags(&mut self, flags: u8) -> &mut Self {
        self.0.flags = flags;
        self
    }

    /// Pack up for the operation that does not perform any I/O.
    ///
    /// This is useful for testing the performance of the io_uring
    /// implementation it‐self.
    #[inline]
    pub fn packup_nop(&mut self) {
        self.pack(IORING_OP_NOP, -1, 0, 0, 0);
    }

    /// Pack up data for the operation that poll the specified `fd` for the
    /// `events`.
    ///
    /// Unlike poll or epoll without `EPOLLONESHOT`, this interface always
    /// works in one shot mode. That is, once the poll operation is
    /// completed, it will have to be resubmitted. This command works like
    /// an async `poll(2)` and the completion event result is the returned
    /// mask of events.
    #[inline]
    pub fn packup_poll_add(&mut self, fd: RawFd, events: PollEvent) {
        self.pack(IORING_OP_POLL_ADD, fd, 0, 0, 0);
        #[cfg(target_endian = "big")]
        {
            self.0.__bindgen_anon_3.poll32_events = events.bits() << 16 | events.bits() >> 16;
        }
        #[cfg(target_endian = "little")]
        {
            self.0.__bindgen_anon_3.poll32_events = events.bits();
        }
    }

    #[inline]
    fn pack(&mut self, opcode: u8, fd: i32, addr: u64, len: u32, offset: u64) {
        self.0.opcode = opcode;
        self.0.fd = fd;
        self.0.__bindgen_anon_1.off = offset;
        self.0.__bindgen_anon_2.addr = addr;
        self.0.len = len;
    }
}
