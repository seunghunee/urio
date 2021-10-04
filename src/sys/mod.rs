#![allow(deref_nullptr)]
#![allow(nonstandard_style)]
#![allow(unaligned_references)]

pub mod syscall;
pub use syscall::*;

use libc::*;

/// Passed in for io_uring_setup(2). Copied back with updated info on success
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct io_uring_params {
    pub sq_entries: __u32,
    pub cq_entries: __u32,
    pub flags: __u32,
    pub sq_thread_cpu: __u32,
    pub sq_thread_idle: __u32,
    pub features: __u32,
    pub wq_fd: __u32,
    pub resv: [__u32; 3usize],
    pub sq_off: io_sqring_offsets,
    pub cq_off: io_cqring_offsets,
}

// io_uring_params.flags
pub const IORING_SETUP_IOPOLL: __u32 = 1 << 0; // io_context is polled
pub const IORING_SETUP_SQPOLL: __u32 = 1 << 1; // SQ poll thread
pub const IORING_SETUP_SQ_AFF: __u32 = 1 << 2; // sq_thread_cpu is valid
pub const IORING_SETUP_CQSIZE: __u32 = 1 << 3; // app defines CQ size
pub const IORING_SETUP_CLAMP: __u32 = 1 << 4; // clamp SQ/CQ ring sizes
pub const IORING_SETUP_ATTACH_WQ: __u32 = 1 << 5; // attach to existing wq
pub const IORING_SETUP_R_DISABLED: __u32 = 1 << 6; // start with ring disabled

// io_uring_params.features
pub const IORING_FEAT_SINGLE_MMAP: __u32 = 1 << 0;
pub const IORING_FEAT_NODROP: __u32 = 1 << 1;
pub const IORING_FEAT_SUBMIT_STABLE: __u32 = 1 << 2;
pub const IORING_FEAT_RW_CUR_POS: __u32 = 1 << 3;
pub const IORING_FEAT_CUR_PERSONALITY: __u32 = 1 << 4;
pub const IORING_FEAT_FAST_POLL: __u32 = 1 << 5;
pub const IORING_FEAT_POLL_32BITS: __u32 = 1 << 6;
pub const IORING_FEAT_SQPOLL_NONFIXED: __u32 = 1 << 7;
pub const IORING_FEAT_EXT_ARG: __u32 = 1 << 8;
pub const IORING_FEAT_NATIVE_WORKERS: __u32 = 1 << 9;
pub const IORING_FEAT_RSRC_TAGS: __u32 = 1 << 10;

// Magic offsets for the application to mmap the data it needs
pub const IORING_OFF_SQ_RING: off_t = 0;
pub const IORING_OFF_CQ_RING: off_t = 0x0800_0000;
pub const IORING_OFF_SQES: off_t = 0x1000_0000;

/// Filled with the offset for mmap(2)
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct io_sqring_offsets {
    pub head: __u32,
    pub tail: __u32,
    pub ring_mask: __u32,
    pub ring_entries: __u32,
    pub flags: __u32,
    pub dropped: __u32,
    pub array: __u32,
    pub resv1: __u32,
    pub resv2: __u64,
}

// sq_ring.flags
pub const IORING_SQ_NEED_WAKEUP: c_uint = 1 << 0; // needs io_uring_enter wakeup
pub const IORING_SQ_CQ_OVERFLOW: c_uint = 1 << 1; // CQ ring is overflown

// cq_ring.flags
pub const IORING_CQ_EVENTFD_DISABLED: c_uint = 1 << 0; // disable eventfd notifications

// io_uring_enter(2) flags
pub const IORING_ENTER_GETEVENTS: c_uint = 1 << 0;
pub const IORING_ENTER_SQ_WAKEUP: c_uint = 1 << 1;
pub const IORING_ENTER_SQ_WAIT: c_uint = 1 << 2;
pub const IORING_ENTER_EXT_ARG: c_uint = 1 << 3;

/// Filled with the offset for mmap(2)
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct io_cqring_offsets {
    pub head: __u32,
    pub tail: __u32,
    pub ring_mask: __u32,
    pub ring_entries: __u32,
    pub overflow: __u32,
    pub cqes: __u32,
    pub flags: __u32,
    pub resv1: __u32,
    pub resv2: __u64,
}

// IO submission data structure (Submission Queue Entry)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_sqe {
    pub opcode: __u8,  // type of operation for this sqe
    pub flags: __u8,   // IOSQE_ flags
    pub ioprio: __u16, // ioprio for the request
    pub fd: __s32,     // file descriptor to do IO on
    pub __bindgen_anon_1: io_uring_sqe__bindgen_ty_1,
    pub __bindgen_anon_2: io_uring_sqe__bindgen_ty_2,
    pub len: __u32, // buffer size or number of iovecs
    pub __bindgen_anon_3: io_uring_sqe__bindgen_ty_3,
    pub user_data: __u64, // data to be passed back at completion time
    pub __bindgen_anon_4: io_uring_sqe__bindgen_ty_4,
    pub personality: __u16, // personality to use, if used
    pub __bindgen_anon_5: io_uring_sqe__bindgen_ty_5,
    pub __pad2: [__u64; 2usize],
}
impl Default for io_uring_sqe {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union io_uring_sqe__bindgen_ty_1 {
    pub off: __u64, //offset into file
    pub addr2: __u64,
}
impl Default for io_uring_sqe__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union io_uring_sqe__bindgen_ty_2 {
    pub addr: __u64, // pointer to buffer or iovecs
    pub splice_off_in: __u64,
}
impl Default for io_uring_sqe__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type __kernel_rwf_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union io_uring_sqe__bindgen_ty_3 {
    pub rw_flags: __kernel_rwf_t,
    pub fsync_flags: __u32,
    pub poll_events: __u16,   // compatibility
    pub poll32_events: __u32, // word-reversed for BE
    pub sync_range_flags: __u32,
    pub msg_flags: __u32,
    pub timeout_flags: __u32,
    pub accept_flags: __u32,
    pub cancel_flags: __u32,
    pub open_flags: __u32,
    pub statx_flags: __u32,
    pub fadvise_advice: __u32,
    pub splice_flags: __u32,
    pub rename_flags: __u32,
    pub unlink_flags: __u32,
    pub hardlink_flags: __u32,
}
impl Default for io_uring_sqe__bindgen_ty_3 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
// pack this to avoid bogus arm OABI complaints

pub union io_uring_sqe__bindgen_ty_4 {
    pub buf_index: __u16, // index into fixed buffers, if used
    pub buf_group: __u16, // for grouped buffer selection
}
impl Default for io_uring_sqe__bindgen_ty_4 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union io_uring_sqe__bindgen_ty_5 {
    pub splice_fd_in: __s32,
    pub file_index: __u32,
}
impl Default for io_uring_sqe__bindgen_ty_5 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}

// sqe.flags
pub const IOSQE_FIXED_FILE: __u8 = 1 << 0; // use fixed fileset
pub const IOSQE_IO_DRAIN: __u8 = 1 << 1; // issue after inflight IO
pub const IOSQE_IO_LINK: __u8 = 1 << 2; // links next sqe
pub const IOSQE_IO_HARDLINK: __u8 = 1 << 3; // like LINK, but stronger
pub const IOSQE_ASYNC: __u8 = 1 << 4; // always go async
pub const IOSQE_BUFFER_SELECT: __u8 = 1 << 5; // select buffer from sqe->buf_group

// opcode
pub const IORING_OP_NOP: __u8 = 0;
pub const IORING_OP_READV: __u8 = 1;
pub const IORING_OP_WRITEV: __u8 = 2;
pub const IORING_OP_FSYNC: __u8 = 3;
pub const IORING_OP_READ_FIXED: __u8 = 4;
pub const IORING_OP_WRITE_FIXED: __u8 = 5;
pub const IORING_OP_POLL_ADD: __u8 = 6;
pub const IORING_OP_POLL_REMOVE: __u8 = 7;
pub const IORING_OP_SYNC_FILE_RANGE: __u8 = 8;
pub const IORING_OP_SENDMSG: __u8 = 9;
pub const IORING_OP_RECVMSG: __u8 = 10;
pub const IORING_OP_TIMEOUT: __u8 = 11;
pub const IORING_OP_TIMEOUT_REMOVE: __u8 = 12;
pub const IORING_OP_ACCEPT: __u8 = 13;
pub const IORING_OP_ASYNC_CANCEL: __u8 = 14;
pub const IORING_OP_LINK_TIMEOUT: __u8 = 15;
pub const IORING_OP_CONNECT: __u8 = 16;
pub const IORING_OP_FALLOCATE: __u8 = 17;
pub const IORING_OP_OPENAT: __u8 = 18;
pub const IORING_OP_CLOSE: __u8 = 19;
pub const IORING_OP_FILES_UPDATE: __u8 = 20;
pub const IORING_OP_STATX: __u8 = 21;
pub const IORING_OP_READ: __u8 = 22;
pub const IORING_OP_WRITE: __u8 = 23;
pub const IORING_OP_FADVISE: __u8 = 24;
pub const IORING_OP_MADVISE: __u8 = 25;
pub const IORING_OP_SEND: __u8 = 26;
pub const IORING_OP_RECV: __u8 = 27;
pub const IORING_OP_OPENAT2: __u8 = 28;
pub const IORING_OP_EPOLL_CTL: __u8 = 29;
pub const IORING_OP_SPLICE: __u8 = 30;
pub const IORING_OP_PROVIDE_BUFFERS: __u8 = 31;
pub const IORING_OP_REMOVE_BUFFERS: __u8 = 32;
pub const IORING_OP_TEE: __u8 = 33;
pub const IORING_OP_SHUTDOWN: __u8 = 34;
pub const IORING_OP_RENAMEAT: __u8 = 35;
pub const IORING_OP_UNLINKAT: __u8 = 36;
pub const IORING_OP_MKDIRAT: __u8 = 37;
pub const IORING_OP_SYMLINKAT: __u8 = 38;
pub const IORING_OP_LINKAT: __u8 = 39;
pub const IORING_OP_LAST: __u8 = 40; // this goes last, obviously

// sqe.fsync_flags
pub const IORING_FSYNC_DATASYNC: __u32 = 1 << 0;

// sqe.timeout_flags
pub const IORING_TIMEOUT_ABS: __u32 = 1 << 0;
pub const IORING_TIMEOUT_UPDATE: __u32 = 1 << 1;
pub const IORING_TIMEOUT_BOOTTIME: u32 = 1 << 2;
pub const IORING_TIMEOUT_REALTIME: u32 = 1 << 3;
pub const IORING_LINK_TIMEOUT_UPDATE: u32 = 1 << 4;
pub const IORING_TIMEOUT_ETIME_SUCCESS: u32 = 1 << 5;
pub const IORING_TIMEOUT_CLOCK_MASK: u32 = IORING_TIMEOUT_BOOTTIME | IORING_TIMEOUT_REALTIME;
pub const IORING_TIMEOUT_UPDATE_MASK: u32 = IORING_TIMEOUT_UPDATE | IORING_LINK_TIMEOUT_UPDATE;

// sqe.splice_flags
// extends splice(2) flags
pub const SPLICE_F_FD_IN_FIXED: __u32 = 1 << 31; // the last bit of __u32

// POLL_ADD flags. Note that since sqe->poll_events is the flag space, the
// command flags for POLL_ADD are stored in sqe->len.
//
// IORING_POLL_ADD_MULTI	Multishot poll. Sets IORING_CQE_F_MORE if
//				the poll handler will continue to report
//				CQEs on behalf of the same SQE.
//
// IORING_POLL_UPDATE		Update existing poll request, matching
//				sqe->addr as the old user_data field.
pub const IORING_POLL_ADD_MULTI: __u32 = 1 << 0;
pub const IORING_POLL_UPDATE_EVENTS: __u32 = 1 << 1;
pub const IORING_POLL_UPDATE_USER_DATA: __u32 = 1 << 2;

// IO completion data structure (Completion Queue Entry)
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct io_uring_cqe {
    pub user_data: __u64, // io_uring_sqe.data submission passed back
    pub res: __s32,       // result code for this event
    pub flags: __u32,
}

// cqe.flags
pub const IORING_CQE_F_BUFFER: __u32 = 1 << 0; // the upper 16 bits are the buffer ID
pub const IORING_CQE_F_MORE: __u32 = 1 << 1; // parent SQE will generate more CQE entries
pub const IORING_CQE_BUFFER_SHIFT: ::std::os::raw::c_uint = 16;

/// io_uring_register(2) opcodes and arguments
pub const IORING_REGISTER_BUFFERS: ::std::os::raw::c_uint = 0;
pub const IORING_UNREGISTER_BUFFERS: ::std::os::raw::c_uint = 1;
pub const IORING_REGISTER_FILES: ::std::os::raw::c_uint = 2;
pub const IORING_UNREGISTER_FILES: ::std::os::raw::c_uint = 3;
pub const IORING_REGISTER_EVENTFD: ::std::os::raw::c_uint = 4;
pub const IORING_UNREGISTER_EVENTFD: ::std::os::raw::c_uint = 5;
pub const IORING_REGISTER_FILES_UPDATE: ::std::os::raw::c_uint = 6;
pub const IORING_REGISTER_EVENTFD_ASYNC: ::std::os::raw::c_uint = 7;
pub const IORING_REGISTER_PROBE: ::std::os::raw::c_uint = 8;
pub const IORING_REGISTER_PERSONALITY: ::std::os::raw::c_uint = 9;
pub const IORING_UNREGISTER_PERSONALITY: ::std::os::raw::c_uint = 10;
pub const IORING_REGISTER_RESTRICTIONS: ::std::os::raw::c_uint = 11;
pub const IORING_REGISTER_ENABLE_RINGS: ::std::os::raw::c_uint = 12;
// extended with tagging
pub const IORING_REGISTER_FILES2: ::std::os::raw::c_uint = 13;
pub const IORING_REGISTER_FILES_UPDATE2: ::std::os::raw::c_uint = 14;
pub const IORING_REGISTER_BUFFERS2: ::std::os::raw::c_uint = 15;
pub const IORING_REGISTER_BUFFERS_UPDATE: ::std::os::raw::c_uint = 16;
// set/clear io-wq thread affinities
pub const IORING_REGISTER_IOWQ_AFF: ::std::os::raw::c_uint = 17;
pub const IORING_UNREGISTER_IOWQ_AFF: ::std::os::raw::c_uint = 18;
// set/get max number of async workers
pub const IORING_REGISTER_IOWQ_MAX_WORKERS: ::std::os::raw::c_uint = 19;
// this goes last
pub const IORING_REGISTER_LAST: ::std::os::raw::c_uint = 20;
