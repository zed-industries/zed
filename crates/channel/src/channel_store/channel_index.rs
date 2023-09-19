use std::{ops::Deref, sync::Arc};

use crate::{Channel, ChannelId};
use collections::HashMap;
use rpc::proto;

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

    pub fn bulk_insert(&mut self) -> ChannelPathsInsertGuard {
        ChannelPathsInsertGuard {
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
#[derive(Debug)]
pub struct ChannelPathsInsertGuard<'a> {
    paths: &'a mut Vec<ChannelPath>,
    channels_by_id: &'a mut ChannelsById,
}

impl<'a> ChannelPathsInsertGuard<'a> {
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

    pub fn insert(&mut self, channel_proto: proto::Channel) {
        if let Some(existing_channel) = self.channels_by_id.get_mut(&channel_proto.id) {
            Arc::make_mut(existing_channel).name = channel_proto.name;
        } else {
            self.channels_by_id.insert(
                channel_proto.id,
                Arc::new(Channel {
                    id: channel_proto.id,
                    name: channel_proto.name,
                }),
            );
            self.insert_root(channel_proto.id);
        }
    }

    pub fn insert_edge(&mut self, channel_id: ChannelId, parent_id: ChannelId) {
        let mut parents = Vec::new();
        let mut descendants = Vec::new();
        let mut ixs_to_remove = Vec::new();

        for (ix, path) in self.paths.iter().enumerate() {
            if path
                .windows(2)
                .any(|window| window[0] == parent_id && window[1] == channel_id)
            {
                // We already have this edge in the index
                return;
            }
            if path.ends_with(&[parent_id]) {
                parents.push(path);
            } else if let Some(position) = path.iter().position(|id| id == &channel_id) {
                if position == 0 {
                    ixs_to_remove.push(ix);
                }
                descendants.push(path.split_at(position).1);
            }
        }

        let mut new_paths = Vec::new();
        for parent in parents.iter() {
            if descendants.is_empty() {
                let mut new_path = Vec::with_capacity(parent.len() + 1);
                new_path.extend_from_slice(parent);
                new_path.push(channel_id);
                new_paths.push(ChannelPath(new_path.into()));
            } else {
                for descendant in descendants.iter() {
                    let mut new_path = Vec::with_capacity(parent.len() + descendant.len());
                    new_path.extend_from_slice(parent);
                    new_path.extend_from_slice(descendant);
                    new_paths.push(ChannelPath(new_path.into()));
                }
            }
        }

        for ix in ixs_to_remove.into_iter().rev() {
            self.paths.swap_remove(ix);
        }
        self.paths.extend(new_paths)
    }

    fn insert_root(&mut self, channel_id: ChannelId) {
        self.paths.push(ChannelPath::new(Arc::from([channel_id])));
    }
}

impl<'a> Drop for ChannelPathsInsertGuard<'a> {
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
