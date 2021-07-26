use libc::{c_void, off_t, size_t, MAP_FAILED, MAP_POPULATE, MAP_SHARED, PROT_READ, PROT_WRITE};
use std::{io, ops::Deref, ptr};

/// A memory mapped io_uring component.
pub struct Mmap {
    ptr: *mut c_void,
    len: usize,
}

impl Mmap {
    /// Creates a memory map backed by a io_uring component.
    pub fn new(fd: i32, len: usize, offset: off_t) -> io::Result<Mmap> {
        match unsafe {
            libc::mmap(
                ptr::null_mut(),
                len as size_t,
                PROT_READ | PROT_WRITE,
                MAP_SHARED | MAP_POPULATE,
                fd,
                offset,
            )
        } {
            MAP_FAILED => Err(io::Error::last_os_error()),
            ptr => Ok(Mmap { ptr, len }),
        }
    }
}

impl Drop for Mmap {
    /// Unmap the mapping.
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.ptr, self.len as size_t);
        }
    }
}

impl Deref for Mmap {
    type Target = *mut c_void;

    fn deref(&self) -> &Self::Target {
        &self.ptr
    }
}
