use clock::ReplicaId;
use std::path::{Path, PathBuf};
use tempdir::TempDir;

#[derive(Clone)]
struct Envelope<T: Clone> {
    message: T,
}

pub struct Network<T: Clone, R: rand::Rng> {
    inboxes: std::collections::BTreeMap<ReplicaId, Vec<Envelope<T>>>,
    all_messages: Vec<T>,
    rng: R,
}

impl<T: Clone, R: rand::Rng> Network<T, R> {
    pub fn new(rng: R) -> Self {
        Network {
            inboxes: Default::default(),
            all_messages: Vec::new(),
            rng,
        }
    }

    pub fn add_peer(&mut self, id: ReplicaId) {
        self.inboxes.insert(id, Vec::new());
    }

    pub fn replicate(&mut self, old_replica_id: ReplicaId, new_replica_id: ReplicaId) {
        self.inboxes
            .insert(new_replica_id, self.inboxes[&old_replica_id].clone());
    }

    pub fn is_idle(&self) -> bool {
        self.inboxes.values().all(|i| i.is_empty())
    }

    pub fn broadcast(&mut self, sender: ReplicaId, messages: Vec<T>) {
        for (replica, inbox) in self.inboxes.iter_mut() {
            if *replica != sender {
                for message in &messages {
                    // Insert one or more duplicates of this message, potentially *before* the previous
                    // message sent by this peer to simulate out-of-order delivery.
                    for _ in 0..self.rng.gen_range(1..4) {
                        let insertion_index = self.rng.gen_range(0..inbox.len() + 1);
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
        self.all_messages.extend(messages);
    }

    pub fn has_unreceived(&self, receiver: ReplicaId) -> bool {
        !self.inboxes[&receiver].is_empty()
    }

    pub fn receive(&mut self, receiver: ReplicaId) -> Vec<T> {
        let inbox = self.inboxes.get_mut(&receiver).unwrap();
        let count = self.rng.gen_range(0..inbox.len() + 1);
        inbox
            .drain(0..count)
            .map(|envelope| envelope.message)
            .collect()
    }
}

pub fn temp_tree(tree: serde_json::Value) -> TempDir {
    let dir = TempDir::new("").unwrap();
    write_tree(dir.path(), tree);
    dir
}

fn write_tree(path: &Path, tree: serde_json::Value) {
    use serde_json::Value;
    use std::fs;

    if let Value::Object(map) = tree {
        for (name, contents) in map {
            let mut path = PathBuf::from(path);
            path.push(name);
            match contents {
                Value::Object(_) => {
                    fs::create_dir(&path).unwrap();
                    write_tree(&path, contents);
                }
                Value::Null => {
                    fs::create_dir(&path).unwrap();
                }
                Value::String(contents) => {
                    fs::write(&path, contents).unwrap();
                }
                _ => {
                    panic!("JSON object must contain only objects, strings, or null");
                }
            }
        }
    } else {
        panic!("You must pass a JSON object to this helper")
    }
}

pub fn sample_text(rows: usize, cols: usize, start_char: char) -> String {
    let mut text = String::new();
    for row in 0..rows {
        let c: char = (start_char as u32 + row as u32) as u8 as char;
        let mut line = c.to_string().repeat(cols);
        if row < rows - 1 {
            line.push('\n');
        }
        text += &line;
    }
    text
}
