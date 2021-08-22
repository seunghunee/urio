use std::error::Error;

use urio::Uring;

#[test]
fn nop_single() -> Result<(), Box<dyn Error>> {
    let mut ring = Uring::new(8)?;

    ring.alloc_sqe()?.packup_nop();

    let submitted = ring.submit_and_wait(1)?;
    assert!(submitted > 0);

    ring.reap_cqe()?;

    Ok(())
}
