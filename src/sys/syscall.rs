#![allow(non_upper_case_globals)]
#![allow(clippy::missing_safety_doc)]

use libc::{c_int, c_uint, sigset_t, syscall, SYS_io_uring_enter, SYS_io_uring_setup};
use std::mem::size_of;

use super::io_uring_params;

pub unsafe fn io_uring_setup(entries: c_uint, p: *mut io_uring_params) -> c_int {
    syscall(SYS_io_uring_setup, entries, p) as _
}

pub unsafe fn io_uring_enter(
    fd: c_int,
    to_submit: c_uint,
    min_complete: c_uint,
    flags: c_uint,
    sig: *const sigset_t,
) -> c_int {
    syscall(
        SYS_io_uring_enter,
        fd,
        to_submit,
        min_complete,
        flags,
        sig,
        size_of::<sigset_t>(), // TODO: _NSIG / 8 in liburing
    ) as _
}

#[cfg(test)]
mod tests {
    use libc::{read, sysconf, EFAULT, EINVAL, _SC_NPROCESSORS_CONF};
    use std::{io::Error, ptr};

    use super::*;
    use crate::sys::{IORING_SETUP_SQPOLL, IORING_SETUP_SQ_AFF};

    #[test]
    fn io_uring_setup_no_entries() {
        let mut p: io_uring_params = Default::default();
        try_io_uring_setup_err(0, &mut p, EINVAL);
    }
    #[test]
    fn io_uring_setup_null_ptr() {
        try_io_uring_setup_err(1, ptr::null_mut(), EFAULT);
    }
    #[test]
    fn io_uring_setup_non_zero_resv() {
        let mut p: io_uring_params = Default::default();
        p.resv = [1; 3];
        try_io_uring_setup_err(1, &mut p, EINVAL);
    }
    #[test]
    fn io_uring_setup_invalid_flags() {
        let mut p: io_uring_params = Default::default();
        p.flags = u32::MAX;
        try_io_uring_setup_err(1, &mut p, EINVAL);
    }
    #[test]
    fn io_uring_setup_bind_poll_thread_to_cpu_without_poll_thread() {
        let mut p: io_uring_params = Default::default();
        p.flags = IORING_SETUP_SQ_AFF;
        try_io_uring_setup_err(1, &mut p, EINVAL);
    }

    #[test]
    // require root privilege
    #[ignore]
    fn io_uring_setup_bind_poll_thread_to_invalid_cpu() {
        let mut p: io_uring_params = Default::default();
        p.flags = IORING_SETUP_SQPOLL | IORING_SETUP_SQ_AFF;
        p.sq_thread_cpu = unsafe { sysconf(_SC_NPROCESSORS_CONF) as _ };
        try_io_uring_setup_err(1, &mut p, EINVAL);
    }

    fn try_io_uring_setup_err(entries: c_uint, p: *mut io_uring_params, err: c_int) {
        let ret = unsafe { io_uring_setup(entries, p) };
        assert_eq!(ret, -1);
        let raw_os_err = Error::last_os_error().raw_os_error().unwrap();
        assert_eq!(raw_os_err, err);
    }

    #[test]
    fn io_uring_setup_read_on_io_uring_fd() {
        let mut p: io_uring_params = Default::default();
        let fd = unsafe { io_uring_setup(1, &mut p) };
        assert!(fd >= 0);

        let mut buf = [0; 4096];
        let ret = unsafe { read(fd, buf.as_mut_ptr() as _, 4096) };
        assert!(ret < 0);
    }
}
