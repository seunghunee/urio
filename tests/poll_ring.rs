use std::{error::Error, os::unix::io::AsRawFd};

use urio::{sqe::PollEvent, Uring};

#[test]
fn poll_ring() -> Result<(), Box<dyn Error>> {
    let mut ring = Uring::new(1)?;

    let ring_fd = ring.as_raw_fd();
    ring.alloc_sqe()?.packup_poll_add(ring_fd, PollEvent::IN);

    let submitted = ring.submit()?;
    assert!(submitted > 0);
    Ok(())
}
