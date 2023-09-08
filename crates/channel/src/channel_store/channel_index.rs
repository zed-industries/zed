use std::{ops::{Deref, DerefMut}, sync::Arc};

use collections::HashMap;
use rpc::proto;

use crate::{ChannelId, Channel};

pub type ChannelPath = Vec<ChannelId>;
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

    /// Insert or update all of the given channels into the index
    pub fn insert_channels(&mut self, channels: Vec<proto::Channel>) {
        let mut insert = self.insert();

        for channel_proto in channels {
            if let Some(existing_channel) = insert.channels_by_id.get_mut(&channel_proto.id) {
                Arc::make_mut(existing_channel).name = channel_proto.name;

                if let Some(parent_id) = channel_proto.parent_id {
                    insert.insert_edge(parent_id, channel_proto.id)
                }
            } else {
                let channel = Arc::new(Channel {
                    id: channel_proto.id,
                    name: channel_proto.name,
                });
                insert.channels_by_id.insert(channel.id, channel.clone());

                if let Some(parent_id) = channel_proto.parent_id {
                    insert.insert_edge(parent_id, channel.id);
                } else {
                    insert.insert_root(channel.id);
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.paths.clear();
        self.channels_by_id.clear();
    }

    /// Remove the given edge from this index. This will not remove the channel
    /// and may result in dangling channels.
    pub fn remove_edge(&mut self, parent_id: ChannelId, channel_id: ChannelId) {
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

    fn insert(& mut self) -> ChannelPathsInsertGuard {
        ChannelPathsInsertGuard {
            paths: &mut self.paths,
            channels_by_id: &mut self.channels_by_id,
        }
    }
}

impl Deref for ChannelIndex {
    type Target = Vec<ChannelPath>;

    fn deref(&self) -> &Self::Target {
        &self.paths
    }
}

/// A guard for ensuring that the paths index maintains its sort and uniqueness
/// invariants after a series of insertions
struct ChannelPathsInsertGuard<'a> {
    paths:  &'a mut Vec<ChannelPath>,
    channels_by_id: &'a mut ChannelsById,
}

impl Deref for ChannelPathsInsertGuard<'_> {
    type Target = ChannelsById;

    fn deref(&self) -> &Self::Target {
        &self.channels_by_id
    }
}

impl DerefMut for ChannelPathsInsertGuard<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.channels_by_id
    }
}


impl<'a> ChannelPathsInsertGuard<'a> {
    pub fn insert_edge(&mut self, parent_id: ChannelId, channel_id: ChannelId) {
        let mut ix = 0;
        while ix < self.paths.len() {
            let path = &self.paths[ix];
            if path.ends_with(&[parent_id]) {
                let mut new_path = path.clone();
                new_path.push(channel_id);
                self.paths.insert(ix + 1, new_path);
                ix += 1;
            }
            ix += 1;
        }
    }

    pub fn insert_root(&mut self, channel_id: ChannelId) {
        self.paths.push(vec![channel_id]);
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
