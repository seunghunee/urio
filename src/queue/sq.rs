use std::{
    ops::Deref,
    rc::Rc,
    sync::atomic::{AtomicU32, Ordering::Acquire},
};

use crate::sys::{io_sqring_offsets, io_uring_sqe};

use super::sqe::Packer;
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

    pub fn alloc_sqe(&mut self) -> Result<Packer, &'static str> {
        unsafe {
            let head = (*self.head).load(Acquire);
            let next = self.sqe_tail + 1;

            if next - head <= *self.ring_entries {
                let idx = self.sqe_tail & *self.ring_mask;
                let sqe = (*self.sqes.deref() as *mut io_uring_sqe)
                    .add(idx as _)
                    .as_mut()
                    .unwrap(); // sqes never be a null pointer
                self.sqe_tail = next;
                Ok(Packer::new(sqe))
            } else {
                Err("Submission Queue is full")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Uring;

    #[test]
    fn alloc_sqe_full_sq() {
        const NUM_ENTRIES: u32 = 16;
        let mut ring = Uring::new(NUM_ENTRIES).expect("Failed to setup ring");
        let mut num = 0;
        while let Ok(_builder) = ring.sq.alloc_sqe() {
            num += 1;
        }
        assert_eq!(num, NUM_ENTRIES);
    }
}
