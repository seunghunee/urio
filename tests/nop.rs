use std::error::Error;

#[test]
fn nop_single() -> Result<(), Box<dyn Error>> {
    let (mut sq, mut cq, _) = urio::new(8)?;

    sq.alloc_sqe()?.packup_nop();

    let submitted = sq.submit_and_wait(1)?;
    assert_eq!(submitted, 1);

    cq.reap_cqe()?;

    Ok(())
}
