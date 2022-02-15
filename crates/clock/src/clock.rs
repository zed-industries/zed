use smallvec::SmallVec;
use std::{
    cmp::{self, Ordering},
    fmt, iter,
    ops::{Add, AddAssign},
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
    pub const MIN: Self = Self {
        replica_id: ReplicaId::MIN,
        value: Seq::MIN,
    };
    pub const MAX: Self = Self {
        replica_id: ReplicaId::MAX,
        value: Seq::MAX,
    };

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
pub struct Global(SmallVec<[u32; 8]>);

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

impl From<Global> for Vec<rpc::proto::VectorClockEntry> {
    fn from(version: Global) -> Self {
        (&version).into()
    }
}

impl Global {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, replica_id: ReplicaId) -> Seq {
        self.0.get(replica_id as usize).copied().unwrap_or(0) as Seq
    }

    pub fn observe(&mut self, timestamp: Local) {
        if timestamp.value > 0 {
            let new_len = timestamp.replica_id as usize + 1;
            if new_len > self.0.len() {
                self.0.resize(new_len, 0);
            }

            let entry = &mut self.0[timestamp.replica_id as usize];
            *entry = cmp::max(*entry, timestamp.value);
        }
    }

    pub fn join(&mut self, other: &Self) {
        if other.0.len() > self.0.len() {
            self.0.resize(other.0.len(), 0);
        }

        for (left, right) in self.0.iter_mut().zip(&other.0) {
            *left = cmp::max(*left, *right);
        }
    }

    pub fn meet(&mut self, other: &Self) {
        if other.0.len() > self.0.len() {
            self.0.resize(other.0.len(), 0);
        }

        let mut new_len = 0;
        for (ix, (left, right)) in self
            .0
            .iter_mut()
            .zip(other.0.iter().chain(iter::repeat(&0)))
            .enumerate()
        {
            if *left == 0 {
                *left = *right;
            } else if *right > 0 {
                *left = cmp::min(*left, *right);
            }

            if *left != 0 {
                new_len = ix + 1;
            }
        }
        self.0.resize(new_len, 0);
    }

    pub fn observed(&self, timestamp: Local) -> bool {
        self.get(timestamp.replica_id) >= timestamp.value
    }

    pub fn observed_any(&self, other: &Self) -> bool {
        let mut lhs = self.0.iter();
        let mut rhs = other.0.iter();
        loop {
            if let Some(left) = lhs.next() {
                if let Some(right) = rhs.next() {
                    if *right > 0 && left >= right {
                        return true;
                    }
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
    }

    pub fn observed_all(&self, other: &Self) -> bool {
        let mut lhs = self.0.iter();
        let mut rhs = other.0.iter();
        loop {
            if let Some(left) = lhs.next() {
                if let Some(right) = rhs.next() {
                    if left < right {
                        return false;
                    }
                } else {
                    return true;
                }
            } else {
                return rhs.next().is_none();
            }
        }
    }

    pub fn changed_since(&self, other: &Self) -> bool {
        if self.0.len() > other.0.len() {
            return true;
        }
        for (left, right) in self.0.iter().zip(other.0.iter()) {
            if left > right {
                return true;
            }
        }
        false
    }

    pub fn iter<'a>(&'a self) -> impl 'a + Iterator<Item = Local> {
        self.0.iter().enumerate().map(|(replica_id, seq)| Local {
            replica_id: replica_id as ReplicaId,
            value: *seq,
        })
    }
}

impl FromIterator<Local> for Global {
    fn from_iter<T: IntoIterator<Item = Local>>(locals: T) -> Self {
        let mut result = Self::new();
        for local in locals {
            result.observe(local);
        }
        result
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
        for timestamp in self.iter() {
            if timestamp.replica_id > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", timestamp.replica_id, timestamp.value)?;
        }
        write!(f, "}}")
    }
}
