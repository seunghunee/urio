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
#[repr(C)]
#[derive(Copy, Clone)]
pub union io_uring_sqe__bindgen_ty_4 {
    pub __bindgen_anon_1: io_uring_sqe__bindgen_ty_4__bindgen_ty_1,
    pub __pad2: [__u64; 3usize],
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
pub struct io_uring_sqe__bindgen_ty_4__bindgen_ty_1 {
    pub __bindgen_anon_1: io_uring_sqe__bindgen_ty_4__bindgen_ty_1__bindgen_ty_1,
    // personality to use, if used
    pub personality: __u16,
    pub splice_fd_in: __s32,
}
impl Default for io_uring_sqe__bindgen_ty_4__bindgen_ty_1 {
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
pub union io_uring_sqe__bindgen_ty_4__bindgen_ty_1__bindgen_ty_1 {
    pub buf_index: __u16, // index into fixed buffers, if used
    pub buf_group: __u16, // for grouped buffer selection
}
impl Default for io_uring_sqe__bindgen_ty_4__bindgen_ty_1__bindgen_ty_1 {
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

#[repr(C)]
#[derive(Debug)]
pub enum OpCode {
    IORING_OP_NOP,
    IORING_OP_READV,
    IORING_OP_WRITEV,
    IORING_OP_FSYNC,
    IORING_OP_READ_FIXED,
    IORING_OP_WRITE_FIXED,
    IORING_OP_POLL_ADD,
    IORING_OP_POLL_REMOVE,
    IORING_OP_SYNC_FILE_RANGE,
    IORING_OP_SENDMSG,
    IORING_OP_RECVMSG,
    IORING_OP_TIMEOUT,
    IORING_OP_TIMEOUT_REMOVE,
    IORING_OP_ACCEPT,
    IORING_OP_ASYNC_CANCEL,
    IORING_OP_LINK_TIMEOUT,
    IORING_OP_CONNECT,
    IORING_OP_FALLOCATE,
    IORING_OP_OPENAT,
    IORING_OP_CLOSE,
    IORING_OP_FILES_UPDATE,
    IORING_OP_STATX,
    IORING_OP_READ,
    IORING_OP_WRITE,
    IORING_OP_FADVISE,
    IORING_OP_MADVISE,
    IORING_OP_SEND,
    IORING_OP_RECV,
    IORING_OP_OPENAT2,
    IORING_OP_EPOLL_CTL,
    IORING_OP_SPLICE,
    IORING_OP_PROVIDE_BUFFERS,
    IORING_OP_REMOVE_BUFFERS,
    IORING_OP_TEE,
    IORING_OP_SHUTDOWN,
    IORING_OP_RENAMEAT,
    IORING_OP_UNLINKAT,
    IORING_OP_MKDIRAT,
    IORING_OP_SYMLINKAT,
    IORING_OP_LINKAT,

    /* this goes last, obviously */
    IORING_OP_LAST,
}

// sqe.fsync_flags
pub const IORING_FSYNC_DATASYNC: __u32 = 1 << 0;

// sqe.timeout_flags
pub const IORING_TIMEOUT_ABS: __u32 = 1 << 0;
pub const IORING_TIMEOUT_UPDATE: __u32 = 1 << 1;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bindgen_test_layout_io_uring_params() {
        assert_eq!(
            ::std::mem::size_of::<io_uring_params>(),
            120usize,
            concat!("Size of: ", stringify!(io_uring_params))
        );
        assert_eq!(
            ::std::mem::align_of::<io_uring_params>(),
            8usize,
            concat!("Alignment of ", stringify!(io_uring_params))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_uring_params>())).sq_entries as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_params),
                "::",
                stringify!(sq_entries)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_uring_params>())).cq_entries as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_params),
                "::",
                stringify!(cq_entries)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_uring_params>())).flags as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_params),
                "::",
                stringify!(flags)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<io_uring_params>())).sq_thread_cpu as *const _ as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_params),
                "::",
                stringify!(sq_thread_cpu)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<io_uring_params>())).sq_thread_idle as *const _ as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_params),
                "::",
                stringify!(sq_thread_idle)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_uring_params>())).features as *const _ as usize },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_params),
                "::",
                stringify!(features)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_uring_params>())).wq_fd as *const _ as usize },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_params),
                "::",
                stringify!(wq_fd)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_uring_params>())).resv as *const _ as usize },
            28usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_params),
                "::",
                stringify!(resv)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_uring_params>())).sq_off as *const _ as usize },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_params),
                "::",
                stringify!(sq_off)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_uring_params>())).cq_off as *const _ as usize },
            80usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_params),
                "::",
                stringify!(cq_off)
            )
        );
    }
    #[test]
    fn bindgen_test_layout_io_sqring_offsets() {
        assert_eq!(
            ::std::mem::size_of::<io_sqring_offsets>(),
            40usize,
            concat!("Size of: ", stringify!(io_sqring_offsets))
        );
        assert_eq!(
            ::std::mem::align_of::<io_sqring_offsets>(),
            8usize,
            concat!("Alignment of ", stringify!(io_sqring_offsets))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_sqring_offsets>())).head as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(io_sqring_offsets),
                "::",
                stringify!(head)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_sqring_offsets>())).tail as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(io_sqring_offsets),
                "::",
                stringify!(tail)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_sqring_offsets>())).ring_mask as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(io_sqring_offsets),
                "::",
                stringify!(ring_mask)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<io_sqring_offsets>())).ring_entries as *const _ as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(io_sqring_offsets),
                "::",
                stringify!(ring_entries)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_sqring_offsets>())).flags as *const _ as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(io_sqring_offsets),
                "::",
                stringify!(flags)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_sqring_offsets>())).dropped as *const _ as usize },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(io_sqring_offsets),
                "::",
                stringify!(dropped)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_sqring_offsets>())).array as *const _ as usize },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(io_sqring_offsets),
                "::",
                stringify!(array)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_sqring_offsets>())).resv1 as *const _ as usize },
            28usize,
            concat!(
                "Offset of field: ",
                stringify!(io_sqring_offsets),
                "::",
                stringify!(resv1)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_sqring_offsets>())).resv2 as *const _ as usize },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(io_sqring_offsets),
                "::",
                stringify!(resv2)
            )
        );
    }
    #[test]
    fn bindgen_test_layout_io_cqring_offsets() {
        assert_eq!(
            ::std::mem::size_of::<io_cqring_offsets>(),
            40usize,
            concat!("Size of: ", stringify!(io_cqring_offsets))
        );
        assert_eq!(
            ::std::mem::align_of::<io_cqring_offsets>(),
            8usize,
            concat!("Alignment of ", stringify!(io_cqring_offsets))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_cqring_offsets>())).head as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(io_cqring_offsets),
                "::",
                stringify!(head)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_cqring_offsets>())).tail as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(io_cqring_offsets),
                "::",
                stringify!(tail)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_cqring_offsets>())).ring_mask as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(io_cqring_offsets),
                "::",
                stringify!(ring_mask)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<io_cqring_offsets>())).ring_entries as *const _ as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(io_cqring_offsets),
                "::",
                stringify!(ring_entries)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_cqring_offsets>())).overflow as *const _ as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(io_cqring_offsets),
                "::",
                stringify!(overflow)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_cqring_offsets>())).cqes as *const _ as usize },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(io_cqring_offsets),
                "::",
                stringify!(cqes)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_cqring_offsets>())).flags as *const _ as usize },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(io_cqring_offsets),
                "::",
                stringify!(flags)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_cqring_offsets>())).resv1 as *const _ as usize },
            28usize,
            concat!(
                "Offset of field: ",
                stringify!(io_cqring_offsets),
                "::",
                stringify!(resv1)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_cqring_offsets>())).resv2 as *const _ as usize },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(io_cqring_offsets),
                "::",
                stringify!(resv2)
            )
        );
    }
    #[test]
    fn bindgen_test_layout_io_uring_cqe() {
        assert_eq!(
            ::std::mem::size_of::<io_uring_cqe>(),
            16usize,
            concat!("Size of: ", stringify!(io_uring_cqe))
        );
        assert_eq!(
            ::std::mem::align_of::<io_uring_cqe>(),
            8usize,
            concat!("Alignment of ", stringify!(io_uring_cqe))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_uring_cqe>())).user_data as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_cqe),
                "::",
                stringify!(user_data)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_uring_cqe>())).res as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_cqe),
                "::",
                stringify!(res)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_uring_cqe>())).flags as *const _ as usize },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_cqe),
                "::",
                stringify!(flags)
            )
        );
    }
    #[test]
    fn bindgen_test_layout_io_uring_sqe__bindgen_ty_1() {
        assert_eq!(
            ::std::mem::size_of::<io_uring_sqe__bindgen_ty_1>(),
            8usize,
            concat!("Size of: ", stringify!(io_uring_sqe__bindgen_ty_1))
        );
        assert_eq!(
            ::std::mem::align_of::<io_uring_sqe__bindgen_ty_1>(),
            8usize,
            concat!("Alignment of ", stringify!(io_uring_sqe__bindgen_ty_1))
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<io_uring_sqe__bindgen_ty_1>())).off as *const _ as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_sqe__bindgen_ty_1),
                "::",
                stringify!(off)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<io_uring_sqe__bindgen_ty_1>())).addr2 as *const _ as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_sqe__bindgen_ty_1),
                "::",
                stringify!(addr2)
            )
        );
    }
    #[test]
    fn bindgen_test_layout_io_uring_sqe__bindgen_ty_2() {
        assert_eq!(
            ::std::mem::size_of::<io_uring_sqe__bindgen_ty_2>(),
            8usize,
            concat!("Size of: ", stringify!(io_uring_sqe__bindgen_ty_2))
        );
        assert_eq!(
            ::std::mem::align_of::<io_uring_sqe__bindgen_ty_2>(),
            8usize,
            concat!("Alignment of ", stringify!(io_uring_sqe__bindgen_ty_2))
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<io_uring_sqe__bindgen_ty_2>())).addr as *const _ as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_sqe__bindgen_ty_2),
                "::",
                stringify!(addr)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<io_uring_sqe__bindgen_ty_2>())).splice_off_in as *const _
                    as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_sqe__bindgen_ty_2),
                "::",
                stringify!(splice_off_in)
            )
        );
    }
    #[test]
    fn bindgen_test_layout_io_uring_sqe__bindgen_ty_3() {
        assert_eq!(
            ::std::mem::size_of::<io_uring_sqe__bindgen_ty_3>(),
            4usize,
            concat!("Size of: ", stringify!(io_uring_sqe__bindgen_ty_3))
        );
        assert_eq!(
            ::std::mem::align_of::<io_uring_sqe__bindgen_ty_3>(),
            4usize,
            concat!("Alignment of ", stringify!(io_uring_sqe__bindgen_ty_3))
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<io_uring_sqe__bindgen_ty_3>())).rw_flags as *const _ as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_sqe__bindgen_ty_3),
                "::",
                stringify!(rw_flags)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<io_uring_sqe__bindgen_ty_3>())).fsync_flags as *const _
                    as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_sqe__bindgen_ty_3),
                "::",
                stringify!(fsync_flags)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<io_uring_sqe__bindgen_ty_3>())).poll_events as *const _
                    as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_sqe__bindgen_ty_3),
                "::",
                stringify!(poll_events)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<io_uring_sqe__bindgen_ty_3>())).poll32_events as *const _
                    as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_sqe__bindgen_ty_3),
                "::",
                stringify!(poll32_events)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<io_uring_sqe__bindgen_ty_3>())).sync_range_flags as *const _
                    as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_sqe__bindgen_ty_3),
                "::",
                stringify!(sync_range_flags)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<io_uring_sqe__bindgen_ty_3>())).msg_flags as *const _
                    as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_sqe__bindgen_ty_3),
                "::",
                stringify!(msg_flags)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<io_uring_sqe__bindgen_ty_3>())).timeout_flags as *const _
                    as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_sqe__bindgen_ty_3),
                "::",
                stringify!(timeout_flags)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<io_uring_sqe__bindgen_ty_3>())).accept_flags as *const _
                    as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_sqe__bindgen_ty_3),
                "::",
                stringify!(accept_flags)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<io_uring_sqe__bindgen_ty_3>())).cancel_flags as *const _
                    as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_sqe__bindgen_ty_3),
                "::",
                stringify!(cancel_flags)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<io_uring_sqe__bindgen_ty_3>())).open_flags as *const _
                    as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_sqe__bindgen_ty_3),
                "::",
                stringify!(open_flags)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<io_uring_sqe__bindgen_ty_3>())).statx_flags as *const _
                    as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_sqe__bindgen_ty_3),
                "::",
                stringify!(statx_flags)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<io_uring_sqe__bindgen_ty_3>())).fadvise_advice as *const _
                    as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_sqe__bindgen_ty_3),
                "::",
                stringify!(fadvise_advice)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<io_uring_sqe__bindgen_ty_3>())).splice_flags as *const _
                    as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_sqe__bindgen_ty_3),
                "::",
                stringify!(splice_flags)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<io_uring_sqe__bindgen_ty_3>())).rename_flags as *const _
                    as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_sqe__bindgen_ty_3),
                "::",
                stringify!(rename_flags)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<io_uring_sqe__bindgen_ty_3>())).unlink_flags as *const _
                    as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_sqe__bindgen_ty_3),
                "::",
                stringify!(unlink_flags)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<io_uring_sqe__bindgen_ty_3>())).hardlink_flags as *const _
                    as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_sqe__bindgen_ty_3),
                "::",
                stringify!(hardlink_flags)
            )
        );
    }
    #[test]
    fn bindgen_test_layout_io_uring_sqe__bindgen_ty_4__bindgen_ty_1__bindgen_ty_1() {
        assert_eq!(
            ::std::mem::size_of::<io_uring_sqe__bindgen_ty_4__bindgen_ty_1__bindgen_ty_1>(),
            2usize,
            concat!(
                "Size of: ",
                stringify!(io_uring_sqe__bindgen_ty_4__bindgen_ty_1__bindgen_ty_1)
            )
        );
        assert_eq!(
            ::std::mem::align_of::<io_uring_sqe__bindgen_ty_4__bindgen_ty_1__bindgen_ty_1>(),
            1usize,
            concat!(
                "Alignment of ",
                stringify!(io_uring_sqe__bindgen_ty_4__bindgen_ty_1__bindgen_ty_1)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<io_uring_sqe__bindgen_ty_4__bindgen_ty_1__bindgen_ty_1>()))
                    .buf_index as *const _ as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_sqe__bindgen_ty_4__bindgen_ty_1__bindgen_ty_1),
                "::",
                stringify!(buf_index)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<io_uring_sqe__bindgen_ty_4__bindgen_ty_1__bindgen_ty_1>()))
                    .buf_group as *const _ as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_sqe__bindgen_ty_4__bindgen_ty_1__bindgen_ty_1),
                "::",
                stringify!(buf_group)
            )
        );
    }
    #[test]
    fn bindgen_test_layout_io_uring_sqe__bindgen_ty_4__bindgen_ty_1() {
        assert_eq!(
            ::std::mem::size_of::<io_uring_sqe__bindgen_ty_4__bindgen_ty_1>(),
            8usize,
            concat!(
                "Size of: ",
                stringify!(io_uring_sqe__bindgen_ty_4__bindgen_ty_1)
            )
        );
        assert_eq!(
            ::std::mem::align_of::<io_uring_sqe__bindgen_ty_4__bindgen_ty_1>(),
            4usize,
            concat!(
                "Alignment of ",
                stringify!(io_uring_sqe__bindgen_ty_4__bindgen_ty_1)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<io_uring_sqe__bindgen_ty_4__bindgen_ty_1>())).personality
                    as *const _ as usize
            },
            2usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_sqe__bindgen_ty_4__bindgen_ty_1),
                "::",
                stringify!(personality)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<io_uring_sqe__bindgen_ty_4__bindgen_ty_1>())).splice_fd_in
                    as *const _ as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_sqe__bindgen_ty_4__bindgen_ty_1),
                "::",
                stringify!(splice_fd_in)
            )
        );
    }
    #[test]
    fn bindgen_test_layout_io_uring_sqe__bindgen_ty_4() {
        assert_eq!(
            ::std::mem::size_of::<io_uring_sqe__bindgen_ty_4>(),
            24usize,
            concat!("Size of: ", stringify!(io_uring_sqe__bindgen_ty_4))
        );
        assert_eq!(
            ::std::mem::align_of::<io_uring_sqe__bindgen_ty_4>(),
            8usize,
            concat!("Alignment of ", stringify!(io_uring_sqe__bindgen_ty_4))
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<io_uring_sqe__bindgen_ty_4>())).__pad2 as *const _ as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_sqe__bindgen_ty_4),
                "::",
                stringify!(__pad2)
            )
        );
    }
    #[test]
    fn bindgen_test_layout_io_uring_sqe() {
        assert_eq!(
            ::std::mem::size_of::<io_uring_sqe>(),
            64usize,
            concat!("Size of: ", stringify!(io_uring_sqe))
        );
        assert_eq!(
            ::std::mem::align_of::<io_uring_sqe>(),
            8usize,
            concat!("Alignment of ", stringify!(io_uring_sqe))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_uring_sqe>())).opcode as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_sqe),
                "::",
                stringify!(opcode)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_uring_sqe>())).flags as *const _ as usize },
            1usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_sqe),
                "::",
                stringify!(flags)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_uring_sqe>())).ioprio as *const _ as usize },
            2usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_sqe),
                "::",
                stringify!(ioprio)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_uring_sqe>())).fd as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_sqe),
                "::",
                stringify!(fd)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_uring_sqe>())).len as *const _ as usize },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_sqe),
                "::",
                stringify!(len)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<io_uring_sqe>())).user_data as *const _ as usize },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(io_uring_sqe),
                "::",
                stringify!(user_data)
            )
        );
    }
}
