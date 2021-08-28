use std::{
    error::Error,
    io::{IoSliceMut, Write},
    os::unix::io::AsRawFd,
};

use urio::Uring;

const TEXT: &[u8] = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec eu ultricies
turpis, eget dapibus elit. Nulla auctor eget metus eget maximus. Nam diam
sapien, vestibulum vitae libero nec, faucibus venenatis augue. Nulla
facilisi. Nullam tristique velit id bibendum mattis. In tincidunt ultrices
pellentesque. Sed aliquam tellus eget sem bibendum, efficitur tincidunt sem
laoreet. Nam luctus eros id neque consectetur posuere. Aliquam bibendum
lacinia nulla sed finibus. Mauris auctor libero nec consequat vehicula.

Vestibulum non maximus mi. Nulla pellentesque, nisl non imperdiet semper,
nunc erat cursus purus, ut consectetur odio massa ut odio. Etiam tempor
placerat massa et accumsan. Praesent vulputate augue vitae mi tempor feugiat.
Curabitur maximus elit ex, sed aliquam metus lacinia ut. Praesent ut odio id
est laoreet volutpat ut ac ligula. Morbi egestas ac justo ut tempor. Ut augue
magna, ultrices in venenatis ut, placerat sed urna. Donec et ultricies
sapien. Donec vitae scelerisque eros.";

#[test]
fn read_vectored() -> Result<(), Box<dyn Error>> {
    let mut ring = Uring::new(8)?;

    let mut tmpfile = tempfile::tempfile()?;
    tmpfile.write_all(&TEXT)?;
    tmpfile.flush()?;

    let mut buf = [0; 4096];
    ring.alloc_sqe()?.packup_read_vectored(
        tmpfile.as_raw_fd(),
        &mut [IoSliceMut::new(&mut buf)],
        0,
    );

    let submitted = ring.submit_and_wait(1)?;
    assert!(submitted == 1);

    let cqe = ring.reap_cqe()?;
    let len = cqe.res as usize;
    assert_eq!(len, TEXT.len());
    assert_eq!(&buf[..len], &TEXT[..len]);

    Ok(())
}
