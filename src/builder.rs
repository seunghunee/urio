use std::io;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::{
    op::storage::UnpackerStorage,
    queue::{self, Cq, Sq},
    register::Registrar,
    resultify,
    sys::{self, io_uring_params},
    Uring,
};

/// io_uring factory, which can be used in order to configure the properties of
/// a new io_uring instance.
///
/// Methods can be chained on it in order to configure it. The [`Sq`], [`Cq`]
/// and [`Registrar`] are constructed by calling [`build`]. The [`urio::new`]
/// methods are aliases for default options using this builder.
///
/// [`build`]: method@Self::build
/// [`urio::new`]: function@crate::new
pub struct Builder {
    entries: u32,
    p: io_uring_params,
}

impl Builder {
    /// Create a new [`Builder`] with given `entries` entries.
    ///
    /// `entries` denote the number of sqes and it must be a power of 2, in the
    /// range `1..=4096`
    pub fn new(entries: u32) -> Self {
        Self {
            entries,
            p: Default::default(),
        }
    }

    /// Build the configured [`Sq`], [`Cq`] and [`Registrar`].
    pub fn build(&mut self) -> io::Result<(Sq, Cq, Registrar)> {
        let fd = unsafe { sys::io_uring_setup(self.entries, &mut self.p) };
        let fd = resultify(fd)? as _;

        queue::mmap(fd, &self.p).map_or_else(
            |err| unsafe {
                libc::close(fd);
                Err(err)
            },
            |(sqring, cqring, sqes)| {
                let uring = Arc::new(Uring {
                    fd,
                    flags: self.p.flags,
                    features: self.p.features,
                });
                let cq_capacity =
                    unsafe { *{ cqring.add(self.p.cq_off.ring_entries as _) as *const u32 } };
                let storage =
                    Arc::new(Mutex::new(UnpackerStorage::with_capacity(cq_capacity as _)));

                Ok((
                    Sq::new(
                        Arc::clone(&uring),
                        Arc::clone(&storage),
                        sqring,
                        self.p.sq_off,
                        sqes,
                    ),
                    Cq::new(
                        Arc::clone(&uring),
                        Arc::clone(&storage),
                        cqring,
                        self.p.cq_off,
                    ),
                    Registrar::new(Arc::clone(&uring)),
                ))
            },
        )
    }
}
