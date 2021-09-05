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
        if self.len() < want {
            return Err("Failed to get cqes as much as you want");
        }

        Ok(Reaper::new(self, want as _))
    }

    /// Returns the number of events the CQ can hold.
    #[inline]
    pub fn capacity(&self) -> usize {
        unsafe { *self.ring_entries as _ }
    }

    /// Returns the number of events in the CQ.
    #[inline]
    pub fn len(&self) -> usize {
        (unsafe {
            let tail = (*self.tail).load(Acquire);
            let head = *(self.head as *const u32);
            tail.wrapping_sub(head)
        }) as _
    }
}

/// Reap CQEs(Completion Queue Event).
pub struct Reaper<'a> {
    cq: &'a mut Cq,
    len: u32,
    reaped: u32,
}

impl<'a> Reaper<'a> {
    fn new(cq: &'a mut Cq, len: u32) -> Self {
        Self { cq, len, reaped: 0 }
    }
}

impl Iterator for Reaper<'_> {
    type Item = Cqe;

    fn next(&mut self) -> Option<Self::Item> {
        if self.reaped < self.len {
            unsafe {
                let head = *(self.cq.head as *const u32);
                let idx = head.wrapping_add(self.reaped) & *self.cq.ring_mask;
                let cqe = self.cq.cqes.add(idx as _).as_ref().expect("cqe is null");
                self.reaped += 1;
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
            (*self.cq.head).store(head.wrapping_add(self.len), Release);
        }
    }
}
