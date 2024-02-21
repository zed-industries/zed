use std::fmt::Display;

use collections::HashMap;
use gpui::SharedString;
use rpc::proto;

use crate::ChannelId;

use super::{ChannelBufferId, NotesVersion};

#[derive(Clone, Debug)]
pub struct ChannelBufferHandle {
    pub id: ChannelBufferId,
    pub channel_id: ChannelId,
    pub name: SharedString,
}

impl PartialEq for ChannelBufferHandle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl ChannelBufferHandle {
    pub fn to_proto(&self) -> proto::ChannelBuffer {
        proto::ChannelBuffer {
            id: self.id.0,
            channel_id: self.channel_id,
            name: self.name.clone().into(),
            is_notes: false,
        }
    }

    pub fn from_proto(p: proto::ChannelBuffer) -> Self {
        Self {
            id: ChannelBufferId::from_proto(p.id),
            channel_id: p.channel_id.into(),
            name: p.name.into(),
        }
    }
}

impl Display for ChannelBufferHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.id.0)
    }
}

#[derive(Default, Debug)]
pub(crate) struct BufferIndex {
    by_channel_id: HashMap<ChannelId, Vec<ChannelBufferId>>,
    by_buffer_id: HashMap<ChannelBufferId, ChannelBufferHandle>,
    versions: HashMap<ChannelBufferId, Versions>,
}

impl BufferIndex {
    pub fn insert(&mut self, handle: ChannelBufferHandle) {
        if let Some(previous) = self.by_buffer_id.remove(&handle.id) {
            self.by_channel_id
                .get_mut(&previous.channel_id)
                .unwrap()
                .retain(|id| *id != previous.id)
        }

        let v = self.by_channel_id.entry(handle.channel_id).or_default();
        match v.binary_search_by(|id| self.by_buffer_id[id].name.cmp(&handle.name)) {
            Err(ix) => v.insert(ix, handle.id),
            Ok(_) => panic!("duplicate buffer ids"),
        };

        self.by_buffer_id.insert(handle.id, handle);
    }

    pub fn delete(&mut self, buffer_id: ChannelBufferId) {
        if let Some(previous) = self.by_buffer_id.remove(&buffer_id) {
            self.by_channel_id
                .get_mut(&previous.channel_id)
                .unwrap()
                .retain(|id| *id != previous.id);
            self.versions.remove(&buffer_id);
        }
    }

    pub fn for_channel(&self, channel_id: ChannelId) -> Vec<ChannelBufferHandle> {
        if let Some(results) = self.by_channel_id.get(&channel_id) {
            results
                .iter()
                .map(|id| self.by_buffer_id[id].clone())
                .collect()
        } else {
            vec![]
        }
    }

    pub fn update_observed_version(
        &mut self,
        buffer_id: ChannelBufferId,
        epoch: u64,
        version: &clock::Global,
    ) {
        self.versions
            .entry(buffer_id)
            .or_insert_with(|| Default::default())
            .update_observed(epoch, version);
    }

    pub fn update_latest_version(
        &mut self,
        buffer_id: ChannelBufferId,
        epoch: u64,
        version: &clock::Global,
    ) {
        self.versions
            .entry(buffer_id)
            .or_insert_with(|| Default::default())
            .update_latest(epoch, version);
    }

    pub fn is_unread(&self, buffer_id: ChannelBufferId) -> bool {
        if let Some(version) = self.versions.get(&buffer_id) {
            version.is_unread()
        } else {
            false
        }
    }
}

#[derive(Default, Debug)]
struct Versions {
    pub latest: Option<NotesVersion>,
    pub observed: Option<NotesVersion>,
}

impl Versions {
    fn update_observed(&mut self, epoch: u64, version: &clock::Global) {
        if let Some(existing) = &mut self.observed {
            if existing.epoch == epoch {
                existing.version.join(version);
                return;
            }
        }
        self.observed = Some(NotesVersion {
            epoch,
            version: version.clone(),
        });
    }

    fn update_latest(&mut self, epoch: u64, version: &clock::Global) {
        if let Some(existing) = &mut self.latest {
            if existing.epoch == epoch {
                existing.version.join(version);
                return;
            }
        }
        self.latest = Some(NotesVersion {
            epoch,
            version: version.clone(),
        });
    }

    fn is_unread(&self) -> bool {
        if let Some(latest) = &self.latest {
            if let Some(observed) = &self.observed {
                latest.epoch > observed.epoch
                    || (latest.epoch == observed.epoch
                        && latest.version.changed_since(&observed.version))
            } else {
                true
            }
        } else {
            false
        }
    }
}
