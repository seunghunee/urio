use std::io;

use crate::queue::{self, cq::Cq, sq::Sq};
use crate::sys::{self, io_uring_params};
use crate::Uring;

/// [`Uring`] factory, which can be used in order to configure the properties
/// of a new [`Uring`].
///
/// Methods can be chained on it in order to configure it.
/// The [`Uring`] is constructed by calling [`build`].
/// The [`Uring::new`] methods are aliases for default options using this builder.
///
/// [`build`]: method@Self::build
pub struct Builder {
    entries: u32,
    p: io_uring_params,
}

impl Builder {
    /// Create a new [`Builder`] with given `entries` entries.
    ///
    /// `entries` denote the number of sqes and it must be a power of 2,
    /// in the range `1..=4096`
    pub fn new(entries: u32) -> Self {
        Self {
            entries,
            p: Default::default(),
        }
    }

    /// Build the configured [`Uring`].
    pub fn build(&mut self) -> io::Result<Uring> {
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
                Ok(Uring {
                    fd,
                    sq: Sq::new(sqring, self.p.sq_off, sqes),
                    cq: Cq::new(cqring, self.p.cq_off),
                    flags: self.p.flags,
                    features: self.p.features,
                })
            },
        )
    }
}
