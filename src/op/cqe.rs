use std::io;

use crate::{resultify, sys::io_uring_cqe};

use super::storage::{Id, Unpack};

pub struct Unpacker<T: 'static> {
    id: Id,
    data: T,
    handler: Box<dyn FnOnce(T)>,
}

impl<T: 'static> Unpacker<T> {
    pub(super) fn new(id: Id, data: T, handler: Box<dyn FnOnce(T)>) -> Self {
        Self { id, data, handler }
    }
}

impl<T: 'static> Unpack for Unpacker<T> {
    #[inline]
    fn id(&self) -> Id {
        self.id
    }

    fn unpack(self: Box<Self>) {
        (self.handler)(self.data)
    }
}

/// CQE(Completion Queue Event), which represents a completed IO event.
///
/// This is added by kernel to CQ(Completion Queue) for each SQE that is
/// submitted. It contains the result of the operation submitted as part of the
/// SQE.
pub struct Cqe(io_uring_cqe);

impl Cqe {
    #[inline]
    pub(crate) fn new(cqe: &io_uring_cqe) -> Self {
        Self(*cqe)
    }

    #[inline]
    pub fn user_data(&self) -> u64 {
        self.0.user_data
    }

    #[inline]
    pub fn result(&self) -> io::Result<u32> {
        resultify(self.0.res)
    }
}
