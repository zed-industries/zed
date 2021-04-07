use std::cmp::{self, Ordering};
use std::collections::HashMap;
use std::mem;
use std::ops::{Add, AddAssign};
use std::sync::Arc;

pub type ReplicaId = u16;
pub type Seq = u64;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Local {
    pub replica_id: ReplicaId,
    pub value: Seq,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Global(Arc<HashMap<ReplicaId, u64>>);

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Lamport {
    pub value: Seq,
    pub replica_id: ReplicaId,
}

impl Local {
    pub fn new(replica_id: ReplicaId) -> Self {
        Self {
            replica_id,
            value: 1,
        }
    }

    pub fn tick(&mut self) -> Self {
        let timestamp = *self;
        self.value += 1;
        timestamp
    }

    pub fn observe(&mut self, timestamp: Self) {
        if timestamp.replica_id == self.replica_id {
            self.value = cmp::max(self.value, timestamp.value + 1);
        }
    }
}

impl<'a> Add<&'a Self> for Local {
    type Output = Local;

    fn add(self, other: &'a Self) -> Self::Output {
        cmp::max(&self, other).clone()
    }
}

impl<'a> AddAssign<&'a Local> for Local {
    fn add_assign(&mut self, other: &Self) {
        if *self < *other {
            *self = other.clone();
        }
    }
}

impl Global {
    pub fn new() -> Self {
        Global(Arc::new(HashMap::new()))
    }

    pub fn get(&self, replica_id: ReplicaId) -> Seq {
        *self.0.get(&replica_id).unwrap_or(&0)
    }

    pub fn observe(&mut self, timestamp: Local) {
        let map = Arc::make_mut(&mut self.0);
        let value = map.entry(timestamp.replica_id).or_insert(0);
        *value = cmp::max(*value, timestamp.value);
    }

    pub fn observe_all(&mut self, other: &Self) {
        for (replica_id, value) in other.0.as_ref() {
            self.observe(Local {
                replica_id: *replica_id,
                value: *value,
            });
        }
    }

    pub fn observed(&self, timestamp: Local) -> bool {
        self.get(timestamp.replica_id) >= timestamp.value
    }

    pub fn changed_since(&self, other: &Self) -> bool {
        self.0
            .iter()
            .any(|(replica_id, value)| *value > other.get(*replica_id))
    }
}

impl PartialOrd for Global {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut global_ordering = Ordering::Equal;

        for replica_id in self.0.keys().chain(other.0.keys()) {
            let ordering = self.get(*replica_id).cmp(&other.get(*replica_id));
            if ordering != Ordering::Equal {
                if global_ordering == Ordering::Equal {
                    global_ordering = ordering;
                } else if ordering != global_ordering {
                    return None;
                }
            }
        }

        Some(global_ordering)
    }
}

impl Lamport {
    pub fn new(replica_id: ReplicaId) -> Self {
        Self {
            value: 1,
            replica_id,
        }
    }

    pub fn tick(&mut self) -> Self {
        let timestamp = *self;
        self.value += 1;
        timestamp
    }

    pub fn observe(&mut self, timestamp: Self) {
        self.value = cmp::max(self.value, timestamp.value) + 1;
    }

    pub fn to_bytes(&self) -> [u8; 24] {
        let mut bytes = [0; 24];
        bytes[0..8].copy_from_slice(unsafe { &mem::transmute::<u64, [u8; 8]>(self.value.to_be()) });
        bytes[8..10]
            .copy_from_slice(unsafe { &mem::transmute::<u16, [u8; 2]>(self.replica_id.to_be()) });
        bytes
    }
}
