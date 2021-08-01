#![allow(non_upper_case_globals)]
#![allow(clippy::missing_safety_doc)]

use libc::*;
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
    use std::{
        io::Error,
        ptr::{null, null_mut},
    };

    use super::*;
    use crate::{
        sys::{IORING_SETUP_SQPOLL, IORING_SETUP_SQ_AFF},
        Uring,
    };

    #[test]
    fn io_uring_setup_no_entries() {
        let mut p: io_uring_params = Default::default();
        assert_err(|| unsafe { io_uring_setup(0, &mut p) }, EINVAL);
    }
    #[test]
    fn io_uring_setup_null_ptr() {
        assert_err(|| unsafe { io_uring_setup(1, null_mut()) }, EFAULT);
    }
    #[test]
    fn io_uring_setup_non_zero_resv() {
        let mut p: io_uring_params = Default::default();
        p.resv = [1; 3];
        assert_err(|| unsafe { io_uring_setup(1, &mut p) }, EINVAL);
    }
    #[test]
    fn io_uring_setup_invalid_flags() {
        let mut p: io_uring_params = Default::default();
        p.flags = u32::MAX;
        assert_err(|| unsafe { io_uring_setup(1, &mut p) }, EINVAL);
    }
    #[test]
    fn io_uring_setup_bind_poll_thread_to_cpu_without_poll_thread() {
        let mut p: io_uring_params = Default::default();
        p.flags = IORING_SETUP_SQ_AFF;
        assert_err(|| unsafe { io_uring_setup(1, &mut p) }, EINVAL);
    }

    #[test]
    // require root privilege
    #[ignore]
    fn io_uring_setup_bind_poll_thread_to_invalid_cpu() {
        let mut p: io_uring_params = Default::default();
        p.flags = IORING_SETUP_SQPOLL | IORING_SETUP_SQ_AFF;
        p.sq_thread_cpu = unsafe { sysconf(_SC_NPROCESSORS_CONF) as _ };
        assert_err(|| unsafe { io_uring_setup(1, &mut p) }, EINVAL);
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

    const IORING_MAX_ENTRIES: u32 = 4096;
    #[test]
    fn io_uring_enter_invalid_fd() {
        assert_err(|| unsafe { io_uring_enter(-1, 0, 0, 0, null()) }, EBADF);
    }
    #[test]
    fn io_uring_enter_valid_non_ring_fd() {
        assert_err(|| unsafe { io_uring_enter(0, 0, 0, 0, null()) }, EOPNOTSUPP);
    }
    #[test]
    fn io_uring_enter_invalid_flags() {
        let ring = Uring::new(IORING_MAX_ENTRIES).expect("Failed to build an Uring");
        assert_err(
            || unsafe { io_uring_enter(ring.fd, 1, 0, c_uint::MAX, null()) },
            EINVAL,
        );
    }
    #[test]
    fn io_uring_enter_no_submit_no_flags() {
        let ring = Uring::new(IORING_MAX_ENTRIES).expect("Failed to build an Uring");
        let ret = unsafe { io_uring_enter(ring.fd, 0, 0, 0, null()) };
        assert_eq!(ret, 0);
    }

    fn assert_err(f: impl FnOnce() -> c_int, err: c_int) {
        let ret = f();
        assert_eq!(ret, -1);
        let raw_os_err = Error::last_os_error().raw_os_error().unwrap();
        assert_eq!(raw_os_err, err);
    }
}
