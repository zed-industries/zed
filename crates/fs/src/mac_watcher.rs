use crate::Watcher;
use anyhow::{Context as _, Result};
use collections::{BTreeMap, Bound};
use fsevent::EventStream;
use parking_lot::Mutex;
use std::{
    path::{Path, PathBuf},
    sync::Weak,
    time::Duration,
};

pub struct MacWatcher {
    events_tx: smol::channel::Sender<Vec<fsevent::Event>>,
    handles: Weak<Mutex<BTreeMap<PathBuf, fsevent::Handle>>>,
    latency: Duration,
}

impl MacWatcher {
    pub fn new(
        events_tx: smol::channel::Sender<Vec<fsevent::Event>>,
        handles: Weak<Mutex<BTreeMap<PathBuf, fsevent::Handle>>>,
        latency: Duration,
    ) -> Self {
        Self {
            events_tx,
            handles,
            latency,
        }
    }
}

impl Watcher for MacWatcher {
    fn add(&self, path: &Path) -> Result<()> {
        let handles = self
            .handles
            .upgrade()
            .context("unable to watch path, receiver dropped")?;
        let mut handles = handles.lock();

        // Return early if an ancestor of this path was already being watched.
        if let Some((watched_path, _)) = handles
            .range::<Path, _>((Bound::Unbounded, Bound::Included(path)))
            .next_back()
            && path.starts_with(watched_path)
        {
            return Ok(());
        }

        let (stream, handle) = EventStream::new(&[path], self.latency);
        let tx = self.events_tx.clone();
        std::thread::spawn(move || {
            stream.run(move |events| smol::block_on(tx.send(events)).is_ok());
        });
        handles.insert(path.into(), handle);

        Ok(())
    }

    fn remove(&self, path: &Path) -> anyhow::Result<()> {
        let handles = self
            .handles
            .upgrade()
            .context("unable to remove path, receiver dropped")?;

        let mut handles = handles.lock();
        handles.remove(path);
        Ok(())
    }
}
