use std::{rc::Rc, sync::atomic::AtomicU32};

use crate::sys::io_sqring_offsets;

use super::util::Mmap;

// Submission Queue.
pub struct Sq {
    head: *const AtomicU32,
    tail: *const AtomicU32,
    ring_mask: *const u32,
    ring_entries: *const u32,
    flags: *const AtomicU32,
    dropped: *const AtomicU32,
    array: *const AtomicU32,
    ring: Rc<Mmap>,

    sqe_head: u32,
    sqe_tail: u32,
    sqes: Mmap,
}

impl Sq {
    pub fn new(ring: Rc<Mmap>, offset: io_sqring_offsets, sqes: Mmap) -> Self {
        unsafe {
            Self {
                head: ring.add(offset.head as _) as _,
                tail: ring.add(offset.tail as _) as _,
                ring_mask: ring.add(offset.ring_mask as _) as _,
                ring_entries: ring.add(offset.ring_entries as _) as _,
                flags: ring.add(offset.flags as _) as _,
                dropped: ring.add(offset.dropped as _) as _,
                array: ring.add(offset.array as _) as _,
                ring,

                sqe_head: 0,
                sqe_tail: 0,
                sqes,
            }
        }
    }
}
