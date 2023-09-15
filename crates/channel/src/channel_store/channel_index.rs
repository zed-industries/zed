use std::{ops::Deref, sync::Arc};

use collections::HashMap;
use rpc::proto;

use crate::{Channel, ChannelId};

use super::ChannelPath;

pub type ChannelsById = HashMap<ChannelId, Arc<Channel>>;

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

    /// Delete the given channels from this index.
    pub fn delete_channels(&mut self, channels: &[ChannelId]) {
        self.channels_by_id
            .retain(|channel_id, _| !channels.contains(channel_id));
        self.paths.retain(|path| {
            path.iter()
                .all(|channel_id| self.channels_by_id.contains_key(channel_id))
        });
    }

    pub fn bulk_edit(&mut self) -> ChannelPathsEditGuard {
        ChannelPathsEditGuard {
            paths: &mut self.paths,
            channels_by_id: &mut self.channels_by_id,
        }
    }
}

impl Deref for ChannelIndex {
    type Target = [ChannelPath];

    fn deref(&self) -> &Self::Target {
        &self.paths
    }
}

/// A guard for ensuring that the paths index maintains its sort and uniqueness
/// invariants after a series of insertions
pub struct ChannelPathsEditGuard<'a> {
    paths: &'a mut Vec<ChannelPath>,
    channels_by_id: &'a mut ChannelsById,
}

impl<'a> ChannelPathsEditGuard<'a> {
    /// Remove the given edge from this index. This will not remove the channel.
    /// If this operation would result in a dangling edge, re-insert it.
    pub fn delete_edge(&mut self, parent_id: ChannelId, channel_id: ChannelId) {
        self.paths.retain(|path| {
            !path
                .windows(2)
                .any(|window| window == [parent_id, channel_id])
        });

        // Ensure that there is at least one channel path in the index
        if !self
            .paths
            .iter()
            .any(|path| path.iter().any(|id| id == &channel_id))
        {
            self.insert_root(channel_id);
        }
    }

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
        debug_assert!(self.channels_by_id.contains_key(&parent_id));
        let mut ix = 0;
        while ix < self.paths.len() {
            let path = &self.paths[ix];
            if path.ends_with(&[parent_id]) {
                let mut new_path = path.to_vec();
                new_path.push(channel_id);
                self.paths.insert(ix + 1, ChannelPath(new_path.into()));
                ix += 2;
            } else if path.get(0) == Some(&channel_id) {
                // Clear out any paths that have this chahnnel as their root
                self.paths.swap_remove(ix);
            } else {
                ix += 1;
            }
        }
    }

    fn insert_root(&mut self, channel_id: ChannelId) {
        self.paths.push(ChannelPath(Arc::from([channel_id])));
    }
}

impl<'a> Drop for ChannelPathsEditGuard<'a> {
    fn drop(&mut self) {
        self.paths.sort_by(|a, b| {
            let a = channel_path_sorting_key(a, &self.channels_by_id);
            let b = channel_path_sorting_key(b, &self.channels_by_id);
            a.cmp(b)
        });
        self.paths.dedup();
    }
}

fn channel_path_sorting_key<'a>(
    path: &'a [ChannelId],
    channels_by_id: &'a ChannelsById,
) -> impl 'a + Iterator<Item = Option<&'a str>> {
    path.iter()
        .map(|id| Some(channels_by_id.get(id)?.name.as_str()))
}
