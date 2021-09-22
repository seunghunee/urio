pub mod cq;
pub use cq::{Cq, Reaper};

pub mod sq;
pub use sq::Sq;

mod util;

use std::{io, mem, rc::Rc};

use crate::sys::{
    io_uring_cqe, io_uring_params, io_uring_sqe, IORING_FEAT_SINGLE_MMAP, IORING_OFF_CQ_RING,
    IORING_OFF_SQES, IORING_OFF_SQ_RING,
};

use self::util::Mmap;

pub(crate) fn mmap(fd: i32, p: &io_uring_params) -> io::Result<(Rc<Mmap>, Rc<Mmap>, Mmap)> {
    // mmap rings
    let sqr_len = p.sq_off.array as usize + p.sq_entries as usize * mem::size_of::<u32>();
    let cqr_len = p.cq_off.cqes as usize + p.cq_entries as usize * mem::size_of::<io_uring_cqe>();

    let (sqring, cqring) = if p.features & IORING_FEAT_SINGLE_MMAP != 0 {
        let sqr = Rc::new(Mmap::new(fd, sqr_len.max(cqr_len), IORING_OFF_SQ_RING)?);
        let cqr = Rc::clone(&sqr);
        (sqr, cqr)
    } else {
        let sqr = Rc::new(Mmap::new(fd, sqr_len, IORING_OFF_SQ_RING)?);
        let cqr = Rc::new(Mmap::new(fd, cqr_len, IORING_OFF_CQ_RING)?);
        (sqr, cqr)
    };

    // mmap sqe array
    let sqes_len = p.sq_entries as usize * mem::size_of::<io_uring_sqe>();
    let sqes = Mmap::new(fd, sqes_len, IORING_OFF_SQES)?;

    Ok((sqring, cqring, sqes))
}
