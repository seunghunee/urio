use std::os::unix::io::AsRawFd;
use std::{error::Error, fs::File, io::IoSlice};

fn main() -> Result<(), Box<dyn Error>> {
    let (mut sq, mut cq, _) = urio::new(8)?;
    let file = File::create("hello.txt")?;

    let message = b"Hello, urio!";
    sq.alloc_sqe()?
        .packup_write_vectored(file.as_raw_fd(), &[IoSlice::new(message)], 0);
    sq.submit_and_wait(1)?;

    let cqe = cq.reap_cqe()?;
    let n = cqe.result()?;
    assert_eq!(n, message.len() as _);

    Ok(())
}
