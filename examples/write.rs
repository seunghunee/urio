use std::os::unix::io::AsRawFd;
use std::{error::Error, fs::File, io::IoSlice};

use urio::Uring;

fn main() -> Result<(), Box<dyn Error>> {
    let mut ring = Uring::new(8)?;
    let file = File::create("hello.txt")?;

    let message = b"Hello, urio!";
    ring.alloc_sqe()?
        .packup_write_vectored(file.as_raw_fd(), &[IoSlice::new(message)], 0);
    ring.submit_and_wait(1)?;

    let cqe = ring.reap_cqe()?;
    let n = cqe.result()?;
    assert_eq!(n, message.len() as _);

    Ok(())
}
