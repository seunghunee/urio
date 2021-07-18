#![allow(deref_nullptr)]

pub mod syscall;

use libc::{__u32, __u64};

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
}
