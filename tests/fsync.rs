use std::{error::Error, os::unix::io::AsRawFd};

use urio::{op::FsyncFlags, Uring};

#[test]
fn fsync_single() -> Result<(), Box<dyn Error>> {
    let mut ring = Uring::new(8)?;

    let tmpfile = tempfile::tempfile()?;
    ring.alloc_sqe()?
        .packup_fsync(tmpfile.as_raw_fd(), FsyncFlags::DATASYNC);

    let submitted = ring.submit_and_wait(1)?;
    assert!(submitted == 1);

    ring.reap_cqe()?;

    Ok(())
}
