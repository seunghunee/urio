#![allow(non_upper_case_globals)]
#![allow(clippy::missing_safety_doc)]

use libc::*;
use std::mem;

use super::io_uring_params;

pub unsafe fn io_uring_setup(entries: c_uint, p: *mut io_uring_params) -> c_int {
    syscall(SYS_io_uring_setup, entries, p) as _
}

pub unsafe fn io_uring_enter(
    fd: c_int,
    to_submit: c_uint,
    min_complete: c_uint,
    flags: c_uint,
    arg: *const c_void,
    sz: size_t,
) -> c_int {
    syscall(
        SYS_io_uring_enter,
        fd,
        to_submit,
        min_complete,
        flags,
        arg,
        sz,
    ) as _
}

pub unsafe fn enter(
    fd: c_int,
    to_submit: c_uint,
    min_complete: c_uint,
    flags: c_uint,
    sig: *const sigset_t,
) -> c_int {
    io_uring_enter(
        fd,
        to_submit,
        min_complete,
        flags,
        sig as _,
        mem::size_of::<sigset_t>(),
    ) as _
}

pub unsafe fn io_uring_register(
    fd: c_int,
    opcode: c_uint,
    arg: *const c_void,
    nr_args: c_uint,
) -> c_int {
    syscall(SYS_io_uring_register, fd, opcode, arg, nr_args) as _
}

#[cfg(test)]
mod tests {
    use std::{
        error::Error,
        fs::File,
        io::{self, IoSliceMut},
        os::unix::io::AsRawFd,
        ptr,
    };

    use super::*;
    use crate::{
        sys::{
            IORING_ENTER_GETEVENTS, IORING_REGISTER_BUFFERS, IORING_SETUP_SQPOLL,
            IORING_SETUP_SQ_AFF, IORING_UNREGISTER_BUFFERS,
        },
        Uring,
    };

    #[test]
    fn io_uring_setup_no_entries() {
        let mut p: io_uring_params = Default::default();
        assert_err_setup(|| unsafe { io_uring_setup(0, &mut p) }, EINVAL);
    }
    #[test]
    fn io_uring_setup_null_ptr() {
        assert_err_setup(|| unsafe { io_uring_setup(1, ptr::null_mut()) }, EFAULT);
    }
    #[test]
    fn io_uring_setup_non_zero_resv() {
        let mut p: io_uring_params = Default::default();
        p.resv = [1; 3];
        assert_err_setup(|| unsafe { io_uring_setup(1, &mut p) }, EINVAL);
    }
    #[test]
    fn io_uring_setup_invalid_flags() {
        let mut p: io_uring_params = Default::default();
        p.flags = u32::MAX;
        assert_err_setup(|| unsafe { io_uring_setup(1, &mut p) }, EINVAL);
    }
    #[test]
    fn io_uring_setup_bind_poll_thread_to_cpu_without_poll_thread() {
        let mut p: io_uring_params = Default::default();
        p.flags = IORING_SETUP_SQ_AFF;
        assert_err_setup(|| unsafe { io_uring_setup(1, &mut p) }, EINVAL);
    }

    #[test]
    // require root privilege
    #[ignore]
    fn io_uring_setup_bind_poll_thread_to_invalid_cpu() {
        let mut p: io_uring_params = Default::default();
        p.flags = IORING_SETUP_SQPOLL | IORING_SETUP_SQ_AFF;
        p.sq_thread_cpu = unsafe { sysconf(_SC_NPROCESSORS_CONF) as _ };
        assert_err_setup(|| unsafe { io_uring_setup(1, &mut p) }, EINVAL);
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

    const RING_SIZE: u32 = 128;
    #[test]
    fn io_uring_enter_invalid_fd() {
        assert_err(|| unsafe { enter(-1, 0, 0, 0, ptr::null()) }, EBADF);
    }
    #[test]
    fn io_uring_enter_valid_non_ring_fd() {
        assert_err(|| unsafe { enter(0, 0, 0, 0, ptr::null()) }, EOPNOTSUPP);
    }
    #[test]
    fn io_uring_enter_invalid_flags() {
        let ring = Uring::new(RING_SIZE).expect("Failed to build an Uring");
        assert_err(
            || unsafe { enter(ring.fd, 1, 0, c_uint::MAX, ptr::null()) },
            EINVAL,
        );
    }
    #[test]
    fn io_uring_enter_no_submit_no_flags() {
        let ring = Uring::new(RING_SIZE).expect("Failed to build an Uring");
        let ret = unsafe { enter(ring.fd, 0, 0, 0, ptr::null()) };
        assert_eq!(ret, 0);
    }
    const BLOCK_SIZE: usize = 4096;
    #[test]
    fn io_uring_enter_wait_sq_size_cqes() -> Result<(), Box<dyn Error>> {
        let mut ring = Uring::new(RING_SIZE)?;

        // Setup File
        let sq_capacity = ring.sq_capacity();
        let file_len = sq_capacity * BLOCK_SIZE;
        let tmpfile = tempfile::tempfile()?;
        tmpfile.set_len(file_len as _)?;

        // Allocate sqes as much as the ring could hold.
        let mut bufs = Vec::with_capacity(sq_capacity);
        for i in 0..sq_capacity {
            let mut buf = [0; BLOCK_SIZE];
            ring.alloc_sqe()?.packup_read_vectored(
                tmpfile.as_raw_fd(),
                &mut [IoSliceMut::new(&mut buf)],
                i as u64 * BLOCK_SIZE as u64,
            );
            bufs.push(buf);
        }

        // Submit the I/Os
        let submitted = ring.submit()?;
        assert_eq!(submitted, sq_capacity);

        // Wait for all events
        let ret = unsafe {
            enter(
                ring.as_raw_fd(),
                0,
                sq_capacity as _,
                IORING_ENTER_GETEVENTS,
                ptr::null(),
            )
        };
        assert!(ret == 0);
        assert_eq!(ring.cq_len(), sq_capacity);

        // Reap cqes
        let reaper = ring.reap_exact_cqes(sq_capacity)?;
        for cqe in reaper {
            let ret = cqe.result()?;
            assert_eq!(ret, BLOCK_SIZE as _);
        }

        Ok(())
    }

    #[test]
    fn io_uring_register_invalid_fd() {
        assert_err(
            || unsafe { io_uring_register(-1, 0, ptr::null(), 0) },
            EBADF,
        );
    }
    #[test]
    fn io_uring_register_null_dev_fd() {
        let null_device = File::open("/dev/null").expect("Failed to open the null device");
        assert_err(
            || unsafe { io_uring_register(null_device.as_raw_fd(), 0, ptr::null(), 0) },
            EOPNOTSUPP,
        );
    }
    #[test]
    fn io_uring_register_invalid_opcode() {
        let ring = Uring::new(RING_SIZE).expect("Failed to build an Uring");
        assert_err(
            || unsafe { io_uring_register(ring.as_raw_fd(), c_uint::MAX, ptr::null(), 0) },
            EINVAL,
        );
    }
    #[test]
    fn io_uring_register_null_iovec() {
        let ring = Uring::new(RING_SIZE).expect("Failed to build an Uring");
        let iov = libc::iovec {
            iov_base: 0 as _,
            iov_len: 4096,
        };

        assert_err_with_drop(
            || unsafe {
                io_uring_register(
                    ring.as_raw_fd(),
                    IORING_REGISTER_BUFFERS,
                    &iov as *const libc::iovec as _,
                    1,
                )
            },
            EFAULT,
            |_| unsafe {
                io_uring_register(ring.as_raw_fd(), IORING_UNREGISTER_BUFFERS, ptr::null(), 1);
            },
        );
    }

    fn assert_err_setup(f: impl FnOnce() -> c_int, err: c_int) {
        assert_err_with_drop(f, err, |fd| unsafe {
            libc::close(fd);
        });
    }
    fn assert_err(f: impl FnOnce() -> c_int, err: c_int) {
        assert_err_with_drop(f, err, |_| {});
    }

    fn assert_err_with_drop(f: impl FnOnce() -> c_int, err: c_int, drop: impl FnOnce(c_int)) {
        let ret = f();
        if ret != -1 {
            drop(ret);
            panic!("Expected to failed, but syscall succeeded");
        }
        let raw_os_err = io::Error::last_os_error().raw_os_error().unwrap();
        assert_eq!(raw_os_err, err);
    }
}
