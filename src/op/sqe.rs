use std::{
    io::{IoSlice, IoSliceMut},
    os::unix::io::RawFd,
};

use crate::sys::{
    io_uring_sqe, IORING_OP_FSYNC, IORING_OP_NOP, IORING_OP_POLL_ADD, IORING_OP_READV,
    IORING_OP_WRITEV,
};

use super::{FsyncFlags, PollEvent};

/// Pack data into a SQE(Submission Queue Entry).
pub struct Packer<'a>(&'a mut io_uring_sqe);

impl<'a> Packer<'a> {
    /// Create a new [`Packer`] with the given mutable reference to SQE.
    pub(crate) fn new(sqe: &'a mut io_uring_sqe) -> Self {
        sqe.flags = 0;
        sqe.ioprio = 0;
        sqe.__bindgen_anon_3.rw_flags = 0;
        sqe.user_data = 0;
        sqe.__bindgen_anon_4.buf_index = 0;
        sqe.personality = 0;
        sqe.__bindgen_anon_5.file_index = 0;
        sqe.__pad2 = [0, 0];
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

    /// Pack up data for the operation that reads from the file descriptor `fd`
    /// into the slice of buffers `bufs`
    ///
    /// It's similar to preadv2(2). If the file is not seekable, off must be set
    /// to zero.
    #[inline]
    pub fn packup_read_vectored(&mut self, fd: RawFd, bufs: &mut [IoSliceMut<'_>], offset: u64) {
        self.pack(
            IORING_OP_READV,
            fd,
            bufs.as_mut_ptr() as u64,
            bufs.len() as _,
            offset,
        );
    }

    /// Pack up data for the oepration that writes the slice of buffers `bufs`
    /// to the file descriptor `fd`.
    ///
    /// It's similar to pwritev2(2). If the file is not seekable, off must be
    /// set to zero.
    #[inline]
    pub fn packup_write_vectored(&mut self, fd: RawFd, bufs: &[IoSlice<'_>], offset: u64) {
        self.pack(
            IORING_OP_WRITEV,
            fd,
            bufs.as_ptr() as u64,
            bufs.len() as _,
            offset,
        );
    }

    /// Pack up data for the operation that synchronize in-core state of the
    /// file referred to by the file descriptor `fd` with storage device.
    ///
    /// See also fsync(2). Note that, while I/O is initiated in the order in
    /// which it appears in the submission queue, completions are unordered. For
    /// example, an application which places a write I/O followed by an fsync in
    /// the submission queue cannot expect the fsync to apply to the write. The
    /// two operations execute in parallel, so the fsync may complete before the
    /// write is issued to the storage. The same is also true for previously
    /// issued writes that have not completed prior to the fsync.
    #[inline]
    pub fn packup_fsync(&mut self, fd: RawFd, flags: FsyncFlags) {
        self.pack(IORING_OP_FSYNC, fd, 0, 0, 0);
        self.0.__bindgen_anon_3.fsync_flags = flags.bits();
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
