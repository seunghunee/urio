use std::io;
use std::sync::Arc;

use crate::queue::{self, Cq, Sq};
use crate::sys::{self, io_uring_params};
use crate::Uring;

/// io_uring factory, which can be used in order to configure the properties of
/// a new io_uring instance.
///
/// Methods can be chained on it in order to configure it. The [`Sq`] and [`Cq`]
/// are constructed by calling [`build`]. The [`urio::new`] methods are aliases
/// for default options using this builder.
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

    /// Build the configured [`Sq`] and [`Cq`].
    pub fn build(&mut self) -> io::Result<(Sq, Cq)> {
        let fd = unsafe { sys::io_uring_setup(self.entries, &mut self.p) };
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }

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
                Ok((
                    Sq::new(Arc::clone(&uring), sqring, self.p.sq_off, sqes),
                    Cq::new(Arc::clone(&uring), cqring, self.p.cq_off),
                ))
            },
        )
    }
}
