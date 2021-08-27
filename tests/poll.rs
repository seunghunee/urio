use std::{
    error::Error,
    io::Write,
    net::{TcpListener, TcpStream},
    os::unix::io::AsRawFd,
};

use urio::{sqe::PollEvent, Uring};

#[test]
fn poll_add_socket() -> Result<(), Box<dyn Error>> {
    let mut ring = Uring::new(1)?;

    let listener = TcpListener::bind("127.0.0.1:0")?;
    ring.alloc_sqe()?
        .packup_poll_add(listener.as_raw_fd(), PollEvent::IN);

    let addr = listener.local_addr()?;
    let mut stream = TcpStream::connect(addr).unwrap();
    stream.write_all(b"ping").unwrap();

    let submitted = ring.submit_and_wait(1)?;
    assert!(submitted > 0);

    let cqe = ring.reap_cqe()?;
    assert_eq!(
        cqe.res & PollEvent::IN.bits() as i32,
        PollEvent::IN.bits() as _
    );

    Ok(())
}

#[test]
fn poll_add_ring() -> Result<(), Box<dyn Error>> {
    let mut ring = Uring::new(1)?;

    let ring_fd = ring.as_raw_fd();
    ring.alloc_sqe()?.packup_poll_add(ring_fd, PollEvent::IN);

    let submitted = ring.submit()?;
    assert!(submitted > 0);
    Ok(())
}
