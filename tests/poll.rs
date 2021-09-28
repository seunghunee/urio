use std::{
    error::Error,
    io::Write,
    os::unix::{io::AsRawFd, net::UnixStream},
};

use urio::op::PollEvent;

#[test]
fn poll_add_stream() -> Result<(), Box<dyn Error>> {
    let (mut sq, mut cq, _) = urio::new(1)?;
    let (mut tx, rx) = UnixStream::pair()?;

    sq.alloc_sqe()?
        .packup_poll_add(rx.as_raw_fd(), PollEvent::IN);

    tx.write_all(b"ping")?;

    let submitted = sq.submit_and_wait(1)?;
    assert!(submitted > 0);

    let cqe = cq.reap_cqe()?;
    assert_ne!(cqe.result()? & PollEvent::IN.bits(), 0);

    Ok(())
}

#[test]
fn poll_add_ring() -> Result<(), Box<dyn Error>> {
    let (mut sq, ..) = urio::new(1)?;

    let ring_fd = sq.uring().as_raw_fd();
    sq.alloc_sqe()?.packup_poll_add(ring_fd, PollEvent::IN);

    let submitted = sq.submit()?;
    assert!(submitted > 0);
    Ok(())
}
