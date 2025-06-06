mod system_clock;

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::{
    cmp::{self, Ordering},
    fmt, iter,
};

pub use system_clock::*;

pub const LOCAL_BRANCH_REPLICA_ID: u16 = u16::MAX;
pub const AGENT_REPLICA_ID: u16 = u16::MAX - 1;

/// A unique identifier for each distributed node.
pub type ReplicaId = u16;

/// A [Lamport sequence number](https://en.wikipedia.org/wiki/Lamport_timestamp).
pub type Seq = u32;

/// A [Lamport timestamp](https://en.wikipedia.org/wiki/Lamport_timestamp),
/// used to determine the ordering of events in the editor.
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Lamport {
    pub replica_id: ReplicaId,
    pub value: Seq,
}

/// A [vector clock](https://en.wikipedia.org/wiki/Vector_clock).
#[derive(Clone, Default, Hash, Eq, PartialEq)]
pub struct Global {
    values: SmallVec<[u32; 8]>,
    local_branch_value: u32,
}

impl Global {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, replica_id: ReplicaId) -> Seq {
        if replica_id == LOCAL_BRANCH_REPLICA_ID {
            self.local_branch_value
        } else {
            self.values.get(replica_id as usize).copied().unwrap_or(0) as Seq
        }
    }

    pub fn observe(&mut self, timestamp: Lamport) {
        if timestamp.value > 0 {
            if timestamp.replica_id == LOCAL_BRANCH_REPLICA_ID {
                self.local_branch_value = cmp::max(self.local_branch_value, timestamp.value);
            } else {
                let new_len = timestamp.replica_id as usize + 1;
                if new_len > self.values.len() {
                    self.values.resize(new_len, 0);
                }

                let entry = &mut self.values[timestamp.replica_id as usize];
                *entry = cmp::max(*entry, timestamp.value);
            }
        }
    }

    pub fn join(&mut self, other: &Self) {
        if other.values.len() > self.values.len() {
            self.values.resize(other.values.len(), 0);
        }

        for (left, right) in self.values.iter_mut().zip(&other.values) {
            *left = cmp::max(*left, *right);
        }

        self.local_branch_value = cmp::max(self.local_branch_value, other.local_branch_value);
    }

    pub fn meet(&mut self, other: &Self) {
        if other.values.len() > self.values.len() {
            self.values.resize(other.values.len(), 0);
        }

        let mut new_len = 0;
        for (ix, (left, right)) in self
            .values
            .iter_mut()
            .zip(other.values.iter().chain(iter::repeat(&0)))
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
        self.values.resize(new_len, 0);
        self.local_branch_value = cmp::min(self.local_branch_value, other.local_branch_value);
    }

    pub fn observed(&self, timestamp: Lamport) -> bool {
        self.get(timestamp.replica_id) >= timestamp.value
    }

    pub fn observed_any(&self, other: &Self) -> bool {
        self.values
            .iter()
            .zip(other.values.iter())
            .any(|(left, right)| *right > 0 && left >= right)
            || (other.local_branch_value > 0 && self.local_branch_value >= other.local_branch_value)
    }

    pub fn observed_all(&self, other: &Self) -> bool {
        let mut rhs = other.values.iter();
        self.values.iter().all(|left| match rhs.next() {
            Some(right) => left >= right,
            None => true,
        }) && rhs.next().is_none()
            && self.local_branch_value >= other.local_branch_value
    }

    pub fn changed_since(&self, other: &Self) -> bool {
        self.values.len() > other.values.len()
            || self
                .values
                .iter()
                .zip(other.values.iter())
                .any(|(left, right)| left > right)
            || self.local_branch_value > other.local_branch_value
    }

    pub fn iter(&self) -> impl Iterator<Item = Lamport> + '_ {
        self.values
            .iter()
            .enumerate()
            .map(|(replica_id, seq)| Lamport {
                replica_id: replica_id as ReplicaId,
                value: *seq,
            })
            .chain((self.local_branch_value > 0).then_some(Lamport {
                replica_id: LOCAL_BRANCH_REPLICA_ID,
                value: self.local_branch_value,
            }))
    }
}

impl FromIterator<Lamport> for Global {
    fn from_iter<T: IntoIterator<Item = Lamport>>(locals: T) -> Self {
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
            value: 1,
            replica_id,
        }
    }

    pub fn as_u64(self) -> u64 {
        ((self.value as u64) << 32) | (self.replica_id as u64)
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
            if timestamp.replica_id == LOCAL_BRANCH_REPLICA_ID {
                write!(f, "<branch>: {}", timestamp.value)?;
            } else {
                write!(f, "{}: {}", timestamp.replica_id, timestamp.value)?;
            }
        }
        write!(f, "}}")
    }
}
