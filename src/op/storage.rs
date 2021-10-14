use parking_lot::Mutex;
use std::{mem, sync::Arc};

use crate::Reaper;

enum Slot {
    Vacant,
    Reserved,
    Occupied(Box<dyn Unpack>),
}

pub(super) type Id = usize;

pub(crate) struct UnpackerStorage {
    slots: Vec<Slot>,
    reusable_ids: Vec<Id>,
}

impl UnpackerStorage {
    #[inline]
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            slots: Vec::with_capacity(capacity),
            reusable_ids: Vec::with_capacity(capacity),
        }
    }

    #[inline]
    fn ticket(&mut self) -> Ticket {
        let id = self.reusable_ids.pop().unwrap_or_else(|| {
            self.slots.push(Slot::Vacant);
            self.slots.len() - 1
        });
        self.slots[id] = Slot::Reserved;
        Ticket { storage: self, id }
    }

    #[inline]
    fn store(&mut self, unpacker: Box<dyn Unpack>) {
        let id = unpacker.id();
        self.slots[id] = Slot::Occupied(unpacker);
    }

    fn release(&mut self, reaper: Reaper) {
        for cqe in reaper {
            let id = cqe.user_data() as Id;
            let state = mem::replace(&mut self.slots[id], Slot::Vacant);
            if let Slot::Occupied(unpacker) = state {
                unpacker.unpack();
                self.slots[id] = Slot::Vacant;
            }
        }
    }
}

pub(crate) struct Ticket {
    storage: Arc<Mutex<UnpackerStorage>>,
    id: Id,
}

impl Ticket {
    #[inline]
    pub(super) fn id(&self) -> Id {
        self.id
    }
}

impl Drop for Ticket {
    fn drop(&mut self) {
        let storage = self.storage.lock();
        storage.slots[self.id] = Slot::Vacant;
        storage.reusable_ids.push(self.id);
    }
}

pub(super) trait Unpack {
    fn id(&self) -> Id;
    fn unpack(self: Box<Self>);
}
