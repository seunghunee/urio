use std::{error::Error, os::unix::io::AsRawFd};

use urio::op::FsyncFlags;

#[test]
fn fsync_single() -> Result<(), Box<dyn Error>> {
    let (mut sq, mut cq, _) = urio::new(8)?;

    let tmpfile = tempfile::tempfile()?;
    sq.alloc_sqe()?
        .packup_fsync(tmpfile.as_raw_fd(), FsyncFlags::DATASYNC);

    let submitted = sq.submit_and_wait(1)?;
    assert_eq!(submitted, 1);

    cq.reap_cqe()?;

    Ok(())
}
