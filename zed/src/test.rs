use crate::{
    channel::ChannelList,
    fs::RealFs,
    language::LanguageRegistry,
    rpc,
    settings::{self, ThemeRegistry},
    time::ReplicaId,
    user::UserStore,
    AppState, Settings,
};
use gpui::{AppContext, Entity, ModelHandle, MutableAppContext};
use parking_lot::Mutex;
use postage::watch;
use smol::channel;
use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
    sync::Arc,
};
use tempdir::TempDir;

#[cfg(feature = "test-support")]
pub use zrpc::test::Channel;

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
pub(crate) struct Network<T: Clone, R: rand::Rng> {
    inboxes: std::collections::BTreeMap<ReplicaId, Vec<Envelope<T>>>,
    all_messages: Vec<T>,
    rng: R,
}

#[cfg(test)]
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

    pub fn is_idle(&self) -> bool {
        self.inboxes.values().all(|i| i.is_empty())
    }

    pub fn broadcast(&mut self, sender: ReplicaId, messages: Vec<T>) {
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
                    for _ in 0..self.rng.gen_range(1..4) {
                        let insertion_index = self.rng.gen_range(min_index..inbox.len() + 1);
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

    pub fn receive(&mut self, receiver: ReplicaId) -> Vec<T> {
        let inbox = self.inboxes.get_mut(&receiver).unwrap();
        let count = self.rng.gen_range(0..inbox.len() + 1);
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

pub fn build_settings(cx: &AppContext) -> watch::Receiver<Settings> {
    settings::channel(&cx.font_cache()).unwrap().1
}

pub fn build_app_state(cx: &mut MutableAppContext) -> Arc<AppState> {
    let (settings_tx, settings) = settings::channel(&cx.font_cache()).unwrap();
    let languages = Arc::new(LanguageRegistry::new());
    let themes = ThemeRegistry::new(());
    let rpc = rpc::Client::new();
    let user_store = Arc::new(UserStore::new(rpc.clone()));
    Arc::new(AppState {
        settings_tx: Arc::new(Mutex::new(settings_tx)),
        settings,
        themes,
        languages: languages.clone(),
        channel_list: cx.add_model(|cx| ChannelList::new(user_store, rpc.clone(), cx)),
        rpc,
        fs: Arc::new(RealFs),
    })
}

pub struct Observer<T>(PhantomData<T>);

impl<T: 'static> Entity for Observer<T> {
    type Event = ();
}

impl<T: Entity> Observer<T> {
    pub fn new(
        handle: &ModelHandle<T>,
        cx: &mut gpui::TestAppContext,
    ) -> (ModelHandle<Self>, channel::Receiver<()>) {
        let (notify_tx, notify_rx) = channel::unbounded();
        let observer = cx.add_model(|cx| {
            cx.observe(handle, move |_, _, _| {
                let _ = notify_tx.try_send(());
            })
            .detach();
            Observer(PhantomData)
        });
        (observer, notify_rx)
    }
}
