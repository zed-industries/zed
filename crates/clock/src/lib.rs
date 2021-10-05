use smallvec::SmallVec;
use std::{
    cmp::{self, Ordering},
    fmt,
    ops::{Add, AddAssign},
    slice,
};

pub type ReplicaId = u16;
pub type Seq = u32;

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Local {
    pub replica_id: ReplicaId,
    pub value: Seq,
}

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
pub struct Lamport {
    pub replica_id: ReplicaId,
    pub value: Seq,
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

#[derive(Clone, Default, Hash, Eq, PartialEq)]
pub struct Global(SmallVec<[Local; 3]>);

impl From<Vec<rpc::proto::VectorClockEntry>> for Global {
    fn from(message: Vec<rpc::proto::VectorClockEntry>) -> Self {
        let mut version = Self::new();
        for entry in message {
            version.observe(Local {
                replica_id: entry.replica_id as ReplicaId,
                value: entry.timestamp,
            });
        }
        version
    }
}

impl<'a> From<&'a Global> for Vec<rpc::proto::VectorClockEntry> {
    fn from(version: &'a Global) -> Self {
        version
            .iter()
            .map(|entry| rpc::proto::VectorClockEntry {
                replica_id: entry.replica_id as u32,
                timestamp: entry.value,
            })
            .collect()
    }
}

impl Global {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, replica_id: ReplicaId) -> Seq {
        self.0
            .iter()
            .find(|t| t.replica_id == replica_id)
            .map_or(0, |t| t.value)
    }

    pub fn observe(&mut self, timestamp: Local) {
        if let Some(entry) = self
            .0
            .iter_mut()
            .find(|t| t.replica_id == timestamp.replica_id)
        {
            entry.value = cmp::max(entry.value, timestamp.value);
        } else {
            self.0.push(timestamp);
        }
    }

    pub fn join(&mut self, other: &Self) {
        for timestamp in other.0.iter() {
            self.observe(*timestamp);
        }
    }

    pub fn meet(&mut self, other: &Self) {
        for timestamp in other.0.iter() {
            if let Some(entry) = self
                .0
                .iter_mut()
                .find(|t| t.replica_id == timestamp.replica_id)
            {
                entry.value = cmp::min(entry.value, timestamp.value);
            } else {
                self.0.push(*timestamp);
            }
        }
    }

    pub fn observed(&self, timestamp: Local) -> bool {
        self.get(timestamp.replica_id) >= timestamp.value
    }

    pub fn changed_since(&self, other: &Self) -> bool {
        self.0.iter().any(|t| t.value > other.get(t.replica_id))
    }

    pub fn iter(&self) -> slice::Iter<Local> {
        self.0.iter()
    }
}

impl PartialOrd for Global {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut global_ordering = Ordering::Equal;

        for timestamp in self.0.iter().chain(other.0.iter()) {
            let ordering = self
                .get(timestamp.replica_id)
                .cmp(&other.get(timestamp.replica_id));
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

impl Ord for Lamport {
    fn cmp(&self, other: &Self) -> Ordering {
        // Use the replica id to break ties between concurrent events.
        self.value
            .cmp(&other.value)
            .then_with(|| self.replica_id.cmp(&other.replica_id))
    }
}

impl PartialOrd for Lamport {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
}

impl fmt::Debug for Local {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Local {{{}: {}}}", self.replica_id, self.value)
    }
}

impl fmt::Debug for Lamport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lamport {{{}: {}}}", self.replica_id, self.value)
    }
}

impl fmt::Debug for Global {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Global {{")?;
        for (i, element) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", element.replica_id, element.value)?;
        }
        write!(f, "}}")
    }
}
