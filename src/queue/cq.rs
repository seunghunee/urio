use std::{
    rc::Rc,
    sync::atomic::{
        AtomicU32,
        Ordering::{Acquire, Release},
    },
};

use crate::{
    sys::{io_cqring_offsets, io_uring_cqe},
    Cqe,
};

use super::util::Mmap;

// Completion Queue.
pub struct Cq {
    head: *const AtomicU32,
    tail: *const AtomicU32,
    ring_mask: *const u32,
    ring_entries: *const u32,
    flags: Option<*const AtomicU32>,
    overflow: *const AtomicU32,
    cqes: *const io_uring_cqe,
    ring: Rc<Mmap>,
}

impl Cq {
    pub fn new(ring: Rc<Mmap>, offset: io_cqring_offsets) -> Self {
        unsafe {
            Self {
                head: ring.add(offset.head as _) as _,
                tail: ring.add(offset.tail as _) as _,
                ring_mask: ring.add(offset.ring_mask as _) as _,
                ring_entries: ring.add(offset.ring_entries as _) as _,
                flags: if offset.flags > 0 {
                    Some(ring.add(offset.flags as _) as _)
                } else {
                    None
                },
                overflow: ring.add(offset.overflow as _) as _,
                cqes: ring.add(offset.cqes as _) as _,
                ring,
            }
        }
    }

    pub fn reap(&mut self, want: usize) -> Result<Reaper, &'static str> {
        let (ptr, available) = self.available_cqes();
        if available < want {
            return Err("Failed to get cqes as much as you want");
        }

        Ok(Reaper::new(self, ptr, want as _))
    }

    fn available_cqes(&self) -> (*const io_uring_cqe, usize) {
        unsafe {
            let tail = (*self.tail).load(Acquire);
            let head = *(self.head as *const u32);

            let len = tail - head;
            if len == 0 {
                return (self.cqes, 0);
            }

            let idx = head & *self.ring_mask;
            let cqe = self.cqes.add(idx as _);
            (cqe, len as _)
        }
    }

    /// Returns the number of events the CQ can hold.
    #[inline]
    pub fn capacity(&self) -> usize {
        unsafe { *self.ring_entries as _ }
    }
}

/// Reap CQEs(Completion Queue Event).
pub struct Reaper<'a> {
    cq: &'a mut Cq,
    ptr: *const io_uring_cqe,
    len: u32,
    off: u32,
}

impl<'a> Reaper<'a> {
    fn new(cq: &'a mut Cq, ptr: *const io_uring_cqe, len: u32) -> Self {
        Self {
            cq,
            ptr,
            len,
            off: 0,
        }
    }
}

impl Iterator for Reaper<'_> {
    type Item = Cqe;

    fn next(&mut self) -> Option<Self::Item> {
        if self.off < self.len {
            unsafe {
                let idx = self.off & *self.cq.ring_mask;
                let cqe = self.ptr.add(idx as _).as_ref().expect("cqe is null");
                self.off += 1;
                Some(Cqe::new(cqe))
            }
        } else {
            None
        }
    }
}

impl Drop for Reaper<'_> {
    fn drop(&mut self) {
        unsafe {
            let head = *(self.cq.head as *const u32);
            (*self.cq.head).store(head + self.len, Release);
        }
    }
}
