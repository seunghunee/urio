#![allow(non_upper_case_globals)]
#![allow(clippy::missing_safety_doc)]

use libc::{c_int, c_long, c_uint, syscall};

use super::io_uring_params;

const __NR_io_uring_setup: c_long = 425;
const __NR_io_uring_enter: c_long = 426;
const __NR_io_uring_register: c_long = 427;

pub unsafe fn io_uring_setup(entries: c_uint, p: *mut io_uring_params) -> c_int {
    syscall(__NR_io_uring_setup, entries, p) as _
}
