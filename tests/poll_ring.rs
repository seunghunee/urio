use std::os::unix::io::AsRawFd;

use urio::{sqe::PollEvent, Uring};

#[test]
fn poll_ring() {
    let mut ring = Uring::new(1).expect("Failed to setup ring");

    let ring_fd = ring.as_raw_fd();
    ring.alloc_sqe()
        .expect("Failed to allocate a sqe")
        .packup_poll_add(ring_fd, PollEvent::IN);

    let submitted = ring.submit().expect("Failed to submit sqe");
    assert!(submitted > 0);
}
