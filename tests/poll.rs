use std::{
    error::Error,
    io::Write,
    net::{TcpListener, TcpStream},
    os::unix::io::AsRawFd,
};

use urio::op::PollEvent;

#[test]
fn poll_add_socket() -> Result<(), Box<dyn Error>> {
    let (mut sq, mut cq, _) = urio::new(1)?;

    let listener = TcpListener::bind("127.0.0.1:0")?;
    sq.alloc_sqe()?
        .packup_poll_add(listener.as_raw_fd(), PollEvent::IN);

    let addr = listener.local_addr()?;
    let mut stream = TcpStream::connect(addr).unwrap();
    stream.write_all(b"ping").unwrap();

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
