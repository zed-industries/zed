use std::{sync::Arc, ops::Deref};

use collections::HashMap;
use rpc::proto;
use serde_derive::{Serialize, Deserialize};

use crate::{ChannelId, Channel};

pub type ChannelsById = HashMap<ChannelId, Arc<Channel>>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Serialize, Deserialize)]
pub struct ChannelPath(Arc<[ChannelId]>);

impl Deref for ChannelPath {
    type Target = [ChannelId];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ChannelPath {
    pub fn parent_id(&self) -> Option<ChannelId> {
        self.0.len().checked_sub(2).map(|i| {
            self.0[i]
        })
    }
}

impl Default for ChannelPath {
    fn default() -> Self {
        ChannelPath(Arc::from([]))
    }
}

#[derive(Default, Debug)]
pub struct ChannelIndex {
    paths: Vec<ChannelPath>,
    channels_by_id: ChannelsById,
}


impl ChannelIndex {
    pub fn by_id(&self) -> &ChannelsById {
        &self.channels_by_id
    }

    pub fn clear(&mut self) {
        self.paths.clear();
        self.channels_by_id.clear();
    }

    pub fn len(&self) -> usize {
        self.paths.len()
    }

    pub fn get(&self, idx: usize) -> Option<&ChannelPath> {
        self.paths.get(idx)
    }

    pub fn iter(&self) -> impl Iterator<Item = &ChannelPath> {
        self.paths.iter()
    }

    /// Remove the given edge from this index. This will not remove the channel
    /// and may result in dangling channels.
    pub fn delete_edge(&mut self, parent_id: ChannelId, channel_id: ChannelId) {
        self.paths.retain(|path| {
            !path
                .windows(2)
                .any(|window| window == [parent_id, channel_id])
        });
    }

    /// Delete the given channels from this index.
    pub fn delete_channels(&mut self, channels: &[ChannelId]) {
        self.channels_by_id.retain(|channel_id, _| !channels.contains(channel_id));
        self.paths.retain(|channel_path| !channel_path.iter().any(|channel_id| {channels.contains(channel_id)}))
    }

    /// Upsert one or more channels into this index.
    pub fn start_upsert(& mut self) -> ChannelPathsUpsertGuard {
        ChannelPathsUpsertGuard {
            paths: &mut self.paths,
            channels_by_id: &mut self.channels_by_id,
        }
    }
}

/// A guard for ensuring that the paths index maintains its sort and uniqueness
/// invariants after a series of insertions
pub struct ChannelPathsUpsertGuard<'a> {
    paths:  &'a mut Vec<ChannelPath>,
    channels_by_id: &'a mut ChannelsById,
}

impl<'a> ChannelPathsUpsertGuard<'a> {
    pub fn upsert(&mut self, channel_proto: proto::Channel) {
        if let Some(existing_channel) = self.channels_by_id.get_mut(&channel_proto.id) {
            Arc::make_mut(existing_channel).name = channel_proto.name;

            if let Some(parent_id) = channel_proto.parent_id {
                self.insert_edge(parent_id, channel_proto.id)
            }
        } else {
            let channel = Arc::new(Channel {
                id: channel_proto.id,
                name: channel_proto.name,
            });
            self.channels_by_id.insert(channel.id, channel.clone());

            if let Some(parent_id) = channel_proto.parent_id {
                self.insert_edge(parent_id, channel.id);
            } else {
                self.insert_root(channel.id);
            }
        }
    }

    fn insert_edge(&mut self, parent_id: ChannelId, channel_id: ChannelId) {
        let mut ix = 0;
        while ix < self.paths.len() {
            let path = &self.paths[ix];
            if path.ends_with(&[parent_id]) {
                let mut new_path = path.to_vec();
                new_path.push(channel_id);
                self.paths.insert(ix + 1, ChannelPath(new_path.into()));
                ix += 1;
            }
            ix += 1;
        }
    }

    fn insert_root(&mut self, channel_id: ChannelId) {
        self.paths.push(ChannelPath(Arc::from([channel_id])));
    }
}

impl<'a> Drop for ChannelPathsUpsertGuard<'a> {
    fn drop(&mut self) {
        self.paths.sort_by(|a, b| {
            let a = channel_path_sorting_key(a, &self.channels_by_id);
            let b = channel_path_sorting_key(b, &self.channels_by_id);
            a.cmp(b)
        });
        self.paths.dedup();
        self.paths.retain(|path| {
            path.iter()
                .all(|channel_id| self.channels_by_id.contains_key(channel_id))
        });
    }
}


fn channel_path_sorting_key<'a>(
    path: &'a [ChannelId],
    channels_by_id: &'a ChannelsById,
) -> impl 'a + Iterator<Item = Option<&'a str>> {
    path.iter()
        .map(|id| Some(channels_by_id.get(id)?.name.as_str()))
}
