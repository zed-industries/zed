use std::fmt::Debug;

use clock::ReplicaId;
use collections::{BTreeMap, HashSet};

pub struct Network<T: Clone, R: rand::Rng> {
    inboxes: BTreeMap<ReplicaId, Vec<Envelope<T>>>,
    disconnected_peers: HashSet<ReplicaId>,
    rng: R,
}

#[derive(Clone, Debug)]
struct Envelope<T: Clone> {
    message: T,
}

impl<T: Clone, R: rand::Rng> Network<T, R> {
    pub fn new(rng: R) -> Self {
        Network {
            inboxes: BTreeMap::default(),
            disconnected_peers: HashSet::default(),
            rng,
        }
    }

    pub fn add_peer(&mut self, id: ReplicaId) {
        self.inboxes.insert(id, Vec::new());
    }

    pub fn disconnect_peer(&mut self, id: ReplicaId) {
        self.disconnected_peers.insert(id);
        self.inboxes.get_mut(&id).unwrap().clear();
    }

    pub fn reconnect_peer(&mut self, id: ReplicaId, replicate_from: ReplicaId) {
        assert!(self.disconnected_peers.remove(&id));
        self.replicate(replicate_from, id);
    }

    pub fn is_disconnected(&self, id: ReplicaId) -> bool {
        self.disconnected_peers.contains(&id)
    }

    pub fn contains_disconnected_peers(&self) -> bool {
        !self.disconnected_peers.is_empty()
    }

    pub fn replicate(&mut self, old_replica_id: ReplicaId, new_replica_id: ReplicaId) {
        self.inboxes
            .insert(new_replica_id, self.inboxes[&old_replica_id].clone());
    }

    pub fn is_idle(&self) -> bool {
        self.inboxes.values().all(|i| i.is_empty())
    }

    pub fn broadcast(&mut self, sender: ReplicaId, messages: Vec<T>) {
        // Drop messages from disconnected peers.
        if self.disconnected_peers.contains(&sender) {
            return;
        }

        for (replica, inbox) in self.inboxes.iter_mut() {
            if *replica != sender && !self.disconnected_peers.contains(replica) {
                for message in &messages {
                    // Insert one or more duplicates of this message, potentially *before* the previous
                    // message sent by this peer to simulate out-of-order delivery.
                    for _ in 0..self.rng.random_range(1..4) {
                        let insertion_index = self.rng.random_range(0..inbox.len() + 1);
                        inbox.insert(
                            insertion_index,
                            Envelope {
                                message: message.clone(),
                            },
                        );
                    }
                }
            }
        }
    }

    pub fn has_unreceived(&self, receiver: ReplicaId) -> bool {
        !self.inboxes[&receiver].is_empty()
    }

    pub fn receive(&mut self, receiver: ReplicaId) -> Vec<T> {
        let inbox = self.inboxes.get_mut(&receiver).unwrap();
        let count = self.rng.random_range(0..inbox.len() + 1);
        inbox
            .drain(0..count)
            .map(|envelope| envelope.message)
            .collect()
    }
}
