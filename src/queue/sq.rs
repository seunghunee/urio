use std::{
    io,
    ops::Deref,
    ptr,
    sync::{
        atomic::{AtomicU32, Ordering::Acquire, Ordering::Relaxed, Ordering::Release},
        Arc,
    },
};

use crate::{
    sys::{
        self, io_sqring_offsets, io_uring_sqe, IORING_ENTER_GETEVENTS, IORING_ENTER_SQ_WAKEUP,
        IORING_SQ_CQ_OVERFLOW, IORING_SQ_NEED_WAKEUP,
    },
    Packer, Uring,
};

use super::util::Mmap;

/// Submission Queue.
pub struct Sq {
    uring: Arc<Uring>,

    head: *const AtomicU32,
    tail: *mut AtomicU32,
    ring_mask: *const u32,
    ring_entries: *const u32,
    flags: *const AtomicU32,
    dropped: *const AtomicU32,
    array: *mut u32,
    ring: Arc<Mmap>,

    sqe_head: u32,
    sqe_tail: u32,
    sqes: Mmap,
}

impl Sq {
    pub(crate) fn new(
        uring: Arc<Uring>,
        ring: Arc<Mmap>,
        offset: io_sqring_offsets,
        sqes: Mmap,
    ) -> Self {
        unsafe {
            Self {
                uring,

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

    /// Returns the reference to the [`Uring`].
    pub fn uring(&self) -> &Uring {
        &self.uring
    }

    /// Allocate a vacant SQE(Submission Queue Entry) and push it to the end of
    /// the SQ(Submission Queue). Returns a new sqe data [`Packer`].
    ///
    /// # Errors
    ///
    /// If the SQ is full, then an error is returned.
    pub fn alloc_sqe(&mut self) -> Result<Packer, &'static str> {
        unsafe {
            let head = (*self.head).load(Acquire);
            let next = self.sqe_tail.wrapping_add(1);

            if next.wrapping_sub(head) <= *self.ring_entries {
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

    /// Flush SQEs to the SQ ring for preparing submission. Returns the number
    /// of pending items in the SQ ring.
    fn flush(&mut self) -> u32 {
        unsafe {
            let mut tail = *(self.tail as *const u32);
            let to_submit = self.sqe_tail.wrapping_sub(self.sqe_head);

            if to_submit > 0 {
                let mask = *self.ring_mask;
                for _ in 0..to_submit {
                    *(self.array.add((tail & mask) as _)) = self.sqe_head & mask;
                    tail = tail.wrapping_add(1);
                    self.sqe_head = self.sqe_head.wrapping_add(1);
                }
                (*self.tail).store(tail, Release);
            }

            // Loading head without `Acquire` is ok. There's no race.
            // but, self.head can be potentially out-of-date regardless
            // of atomicity.
            tail.wrapping_sub(*(self.head as *const u32))
        }
    }

    /// Submit pending sqes in the SQ ring to the kernel. Returns number of sqes
    /// submitted.
    pub fn submit(&mut self) -> io::Result<usize> {
        self.submit_and_wait(0)
    }

    /// Like [`submit`], but allows waiting for events as well. Returns number
    /// of sqes submitted.
    ///
    /// [`submit`]: method@Self::submit
    pub fn submit_and_wait(&mut self, min_complete: u32) -> io::Result<usize> {
        let mut flags = 0;
        let to_submit = self.flush();

        if self.uring.has_sqpoll() {
            if self.needs_wakeup() {
                flags |= IORING_ENTER_SQ_WAKEUP;
            } else if min_complete == 0 {
                return Ok(to_submit as _);
            }
        }

        if min_complete > 0 || self.uring.is_io_polled() {
            flags |= IORING_ENTER_GETEVENTS;
        }

        let ret = unsafe { sys::enter(self.uring.fd, to_submit, min_complete, flags, ptr::null()) };
        if ret < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(ret as _)
    }

    /// Returns the number of entries the SQ can hold.
    #[inline]
    pub fn capacity(&self) -> usize {
        unsafe { *self.ring_entries as _ }
    }

    /// Return `true` if the kernel side polling thread has gone to sleep
    /// when it has been idle for a while.
    #[inline]
    pub fn needs_wakeup(&self) -> bool {
        unsafe { (*self.flags).load(Relaxed) & IORING_SQ_NEED_WAKEUP != 0 }
    }

    /// Returns `true` if the CQ(Completion Queue) ring is overflown.
    #[inline]
    pub fn is_cq_overflown(&self) -> bool {
        unsafe { (*self.flags).load(Relaxed) & IORING_SQ_CQ_OVERFLOW != 0 }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn alloc_sqe_full_sq() {
        const NUM_ENTRIES: u32 = 16;
        let (mut sq, _) = crate::new(NUM_ENTRIES).expect("Failed to setup ring");
        let mut num = 0;
        while let Ok(_builder) = sq.alloc_sqe() {
            num += 1;
        }
        assert_eq!(num, NUM_ENTRIES);
    }
}
