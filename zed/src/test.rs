use rand::Rng;
use std::collections::BTreeMap;

use crate::time::ReplicaId;

#[derive(Clone)]
struct Envelope<T: Clone> {
    message: T,
    sender: ReplicaId,
}

pub(crate) struct Network<T: Clone> {
    inboxes: BTreeMap<ReplicaId, Vec<Envelope<T>>>,
    all_messages: Vec<T>,
}

impl<T: Clone> Network<T> {
    pub fn new() -> Self {
        Network {
            inboxes: BTreeMap::new(),
            all_messages: Vec::new(),
        }
    }

    pub fn add_peer(&mut self, id: ReplicaId) {
        self.inboxes.insert(id, Vec::new());
    }

    pub fn is_idle(&self) -> bool {
        self.inboxes.values().all(|i| i.is_empty())
    }

    pub fn broadcast<R>(&mut self, sender: ReplicaId, messages: Vec<T>, rng: &mut R)
    where
        R: Rng,
    {
        for (replica, inbox) in self.inboxes.iter_mut() {
            if *replica != sender {
                for message in &messages {
                    let min_index = inbox
                        .iter()
                        .enumerate()
                        .rev()
                        .find_map(|(index, envelope)| {
                            if sender == envelope.sender {
                                Some(index + 1)
                            } else {
                                None
                            }
                        })
                        .unwrap_or(0);

                    // Insert one or more duplicates of this message *after* the previous
                    // message delivered by this replica.
                    for _ in 0..rng.gen_range(1, 4) {
                        let insertion_index = rng.gen_range(min_index, inbox.len() + 1);
                        inbox.insert(
                            insertion_index,
                            Envelope {
                                message: message.clone(),
                                sender,
                            },
                        );
                    }
                }
            }
        }
        self.all_messages.extend(messages);
    }

    pub fn has_unreceived(&self, receiver: ReplicaId) -> bool {
        !self.inboxes[&receiver].is_empty()
    }

    pub fn receive<R>(&mut self, receiver: ReplicaId, rng: &mut R) -> Vec<T>
    where
        R: Rng,
    {
        let inbox = self.inboxes.get_mut(&receiver).unwrap();
        let count = rng.gen_range(0, inbox.len() + 1);
        inbox
            .drain(0..count)
            .map(|envelope| envelope.message)
            .collect()
    }
}

pub fn sample_text(rows: usize, cols: usize) -> String {
    let mut text = String::new();
    for row in 0..rows {
        let c: char = ('a' as u32 + row as u32) as u8 as char;
        let mut line = c.to_string().repeat(cols);
        if row < rows - 1 {
            line.push('\n');
        }
        text += &line;
    }
    text
}
