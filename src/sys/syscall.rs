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
