use crate::{language::LanguageRegistry, rpc, settings, time::ReplicaId, AppState};
use gpui::AppContext;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tempdir::TempDir;

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    env_logger::init();
}

#[derive(Clone)]
struct Envelope<T: Clone> {
    message: T,
    sender: ReplicaId,
}

#[cfg(test)]
pub(crate) struct Network<T: Clone> {
    inboxes: std::collections::BTreeMap<ReplicaId, Vec<Envelope<T>>>,
    all_messages: Vec<T>,
}

#[cfg(test)]
impl<T: Clone> Network<T> {
    pub fn new() -> Self {
        Network {
            inboxes: Default::default(),
            all_messages: Vec::new(),
        }
    }

    pub fn add_peer(&mut self, id: ReplicaId) {
        self.inboxes.insert(id, Vec::new());
    }

    pub fn is_idle(&self) -> bool {
        self.inboxes.values().all(|i| i.is_empty())
    }

    pub fn broadcast<R: rand::Rng>(&mut self, sender: ReplicaId, messages: Vec<T>, rng: &mut R) {
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
                    for _ in 0..rng.gen_range(1..4) {
                        let insertion_index = rng.gen_range(min_index..inbox.len() + 1);
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

    pub fn receive<R: rand::Rng>(&mut self, receiver: ReplicaId, rng: &mut R) -> Vec<T> {
        let inbox = self.inboxes.get_mut(&receiver).unwrap();
        let count = rng.gen_range(0..inbox.len() + 1);
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

pub fn build_app_state(cx: &AppContext) -> AppState {
    let settings = settings::channel(&cx.font_cache()).unwrap().1;
    let language_registry = Arc::new(LanguageRegistry::new());
    AppState {
        settings,
        language_registry: language_registry.clone(),
        rpc: rpc::Client::new(language_registry),
    }
}
