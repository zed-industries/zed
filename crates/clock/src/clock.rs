mod system_clock;

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::{
    cmp::{self, Ordering},
    fmt,
};

pub use system_clock::*;

/// A unique identifier for each distributed node.
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct ReplicaId(u16);

impl ReplicaId {
    /// The local replica
    pub const LOCAL: ReplicaId = ReplicaId(0);
    /// The remote replica of the connected remote server.
    pub const REMOTE_SERVER: ReplicaId = ReplicaId(1);
    /// The agent's unique identifier.
    pub const AGENT: ReplicaId = ReplicaId(2);
    /// A local branch.
    pub const LOCAL_BRANCH: ReplicaId = ReplicaId(3);
    /// The first collaborative replica ID, any replica equal or greater than this is a collaborative replica.
    pub const FIRST_COLLAB_ID: ReplicaId = ReplicaId(8);

    pub fn new(id: u16) -> Self {
        ReplicaId(id)
    }

    pub fn as_u16(&self) -> u16 {
        self.0
    }

    pub fn is_remote(self) -> bool {
        self == ReplicaId::REMOTE_SERVER || self >= ReplicaId::FIRST_COLLAB_ID
    }
}

impl fmt::Debug for ReplicaId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == ReplicaId::LOCAL {
            write!(f, "<local>")
        } else if *self == ReplicaId::REMOTE_SERVER {
            write!(f, "<remote>")
        } else if *self == ReplicaId::AGENT {
            write!(f, "<agent>")
        } else if *self == ReplicaId::LOCAL_BRANCH {
            write!(f, "<branch>")
        } else {
            write!(f, "{}", self.0)
        }
    }
}

/// A [Lamport sequence number](https://en.wikipedia.org/wiki/Lamport_timestamp).
pub type Seq = u32;

/// A [Lamport timestamp](https://en.wikipedia.org/wiki/Lamport_timestamp),
/// used to determine the ordering of events in the editor.
#[derive(Clone, Copy, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Lamport {
    pub replica_id: ReplicaId,
    pub value: Seq,
}

/// A [version vector](https://en.wikipedia.org/wiki/Version_vector).
#[derive(Clone, Default, Hash, Eq, PartialEq)]
pub struct Global {
    // 4 is chosen as it is the biggest count that does not increase the size of the field itself.
    // Coincidentally, it also covers all the important non-collab replica ids.
    values: SmallVec<[u32; 4]>,
}

impl Global {
    pub fn new() -> Self {
        Self::default()
    }

    /// Fetches the sequence number for the given replica ID.
    pub fn get(&self, replica_id: ReplicaId) -> Seq {
        self.values.get(replica_id.0 as usize).copied().unwrap_or(0) as Seq
    }

    /// Observe the lamport timestamp.
    ///
    /// This sets the current sequence number of the observed replica ID to the maximum of this global's observed sequence and the observed timestamp.
    pub fn observe(&mut self, timestamp: Lamport) {
        debug_assert_ne!(timestamp.replica_id, Lamport::MAX.replica_id);
        if timestamp.value > 0 {
            let new_len = timestamp.replica_id.0 as usize + 1;
            if new_len > self.values.len() {
                self.values.resize(new_len, 0);
            }

            let entry = &mut self.values[timestamp.replica_id.0 as usize];
            *entry = cmp::max(*entry, timestamp.value);
        }
    }

    /// Join another global.
    ///
    /// This observes all timestamps from the other global.
    #[doc(alias = "synchronize")]
    pub fn join(&mut self, other: &Self) {
        if other.values.len() > self.values.len() {
            self.values.resize(other.values.len(), 0);
        }

        for (left, right) in self.values.iter_mut().zip(&other.values) {
            *left = cmp::max(*left, *right);
        }
    }

    /// Meet another global.
    ///
    /// Sets all unobserved timestamps of this global to the sequences of other and sets all observed timestamps of this global to the minimum observed of both globals.
    pub fn meet(&mut self, other: &Self) {
        if other.values.len() > self.values.len() {
            self.values.resize(other.values.len(), 0);
        }

        let mut new_len = 0;
        for (ix, (left, &right)) in self.values.iter_mut().zip(&other.values).enumerate() {
            match (*left, right) {
                // left has not observed the replica
                (0, _) => *left = right,
                // right has not observed the replica
                (_, 0) => (),
                (_, _) => *left = cmp::min(*left, right),
            }
            if *left != 0 {
                new_len = ix + 1;
            }
        }
        if other.values.len() == self.values.len() {
            // only truncate if other was equal or shorter (which at this point
            // cant be due to the resize above) to `self` as otherwise we would
            // truncate the unprocessed tail that is guaranteed to contain
            // non-null timestamps
            self.values.truncate(new_len);
        }
    }

    pub fn observed(&self, timestamp: Lamport) -> bool {
        self.get(timestamp.replica_id) >= timestamp.value
    }

    pub fn observed_any(&self, other: &Self) -> bool {
        self.iter()
            .zip(other.iter())
            .any(|(left, right)| right.value > 0 && left.value >= right.value)
    }

    pub fn observed_all(&self, other: &Self) -> bool {
        if self.values.len() < other.values.len() {
            return false;
        }
        self.iter()
            .zip(other.iter())
            .all(|(left, right)| left.value >= right.value)
    }

    pub fn changed_since(&self, other: &Self) -> bool {
        self.values.len() > other.values.len()
            || self
                .values
                .iter()
                .zip(other.values.iter())
                .any(|(left, right)| left > right)
    }

    pub fn most_recent(&self) -> Option<Lamport> {
        self.iter().max_by_key(|timestamp| timestamp.value)
    }

    /// Iterates all replicas observed by this global as well as any unobserved replicas whose ID is lower than the highest observed replica.
    pub fn iter(&self) -> impl Iterator<Item = Lamport> + '_ {
        self.values
            .iter()
            .enumerate()
            .map(|(replica_id, seq)| Lamport {
                replica_id: ReplicaId(replica_id as u16),
                value: *seq,
            })
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
        replica_id: ReplicaId(u16::MIN),
        value: Seq::MIN,
    };

    pub const MAX: Self = Self {
        replica_id: ReplicaId(u16::MAX),
        value: Seq::MAX,
    };

    pub fn new(replica_id: ReplicaId) -> Self {
        Self {
            value: 1,
            replica_id,
        }
    }

    pub fn as_u64(self) -> u64 {
        ((self.value as u64) << 32) | (self.replica_id.0 as u64)
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
        if *self == Self::MAX {
            write!(f, "Lamport {{MAX}}")
        } else if *self == Self::MIN {
            write!(f, "Lamport {{MIN}}")
        } else {
            write!(f, "Lamport {{{:?}: {}}}", self.replica_id, self.value)
        }
    }
}

impl fmt::Debug for Global {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Global {{")?;
        for timestamp in self.iter() {
            if timestamp.replica_id.0 > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}: {}", timestamp.replica_id, timestamp.value)?;
        }
        write!(f, "}}")
    }
}
