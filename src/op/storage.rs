use std::mem;

use crate::Reaper;

pub(super) type Id = u64;

pub(crate) struct UnpackerStorage(Vec<Option<Box<dyn Unpack>>>);

impl UnpackerStorage {
    #[inline]
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    #[inline]
    pub(crate) fn issue_id(&mut self) -> Id {
        self.0
            .iter()
            .position(|slot| slot.is_none())
            .unwrap_or_else(|| {
                self.0.push(None);
                self.0.len() - 1
            }) as _
    }

    #[inline]
    pub(crate) fn store(&mut self, unpacker: Box<dyn Unpack>) {
        let id = unpacker.id();
        self.0[id as usize] = Some(unpacker);
    }

    pub(crate) fn release(&mut self, reaper: Reaper) {
        for cqe in reaper {
            let id = cqe.user_data() as usize;
            let state = mem::replace(&mut self.0[id], None);
            if let Some(unpacker) = state {
                unpacker.unpack();
            }
        }
    }
}

pub(crate) trait Unpack {
    fn id(&self) -> Id;
    fn unpack(self: Box<Self>);
}
