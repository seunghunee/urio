use std::{
    rc::Rc,
    slice,
    sync::atomic::{AtomicU32, Ordering::Acquire},
};

use crate::sys::{io_cqring_offsets, io_uring_cqe};

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

    pub fn available_cqes(&mut self) -> &[io_uring_cqe] {
        unsafe {
            let tail = (*self.tail).load(Acquire);
            let head = *(self.head as *const u32);

            let len = tail - head;
            if len == 0 {
                return &[];
            }

            let idx = head & *self.ring_mask;
            let cqe = self.cqes.add(idx as _);
            slice::from_raw_parts(cqe, len as _)
        }
    }
}
