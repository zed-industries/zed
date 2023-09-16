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
#[derive(Debug)]
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

    pub fn insert_edge(&mut self, parent_id: ChannelId, channel_id: ChannelId) {
        debug_assert!(self.channels_by_id.contains_key(&parent_id));
        let mut ix = 0;
        println!("*********** INSERTING EDGE {}, {} ***********", channel_id, parent_id);
        dbg!(&self.paths);
        while ix < self.paths.len() {
            let path = &self.paths[ix];
            println!("*********");
            dbg!(path);

            if path.ends_with(&[parent_id]) {
                dbg!("Appending to parent path");
                let mut new_path = Vec::with_capacity(path.len() + 1);
                new_path.extend_from_slice(path);
                new_path.push(channel_id);
                self.paths.insert(ix + 1, dbg!(ChannelPath::new(new_path.into())));
                ix += 2;
            } else if let Some(path_ix) = path.iter().position(|c| c == &channel_id) {
                if path.contains(&parent_id) {
                    dbg!("Doing nothing");
                    ix += 1;
                    continue;
                }
                if path_ix == 0 && path.len() == 1 {
                    dbg!("Removing path that is just this");
                    self.paths.swap_remove(ix);
                    continue;
                }
                // This is the busted section rn
                // We're trying to do this weird, unsorted context
                // free insertion thing, but we can't insert 'parent_id',
                // we have to _prepend_ with _parent path to_,
                // or something like that.
                // It's a bit busted rn, I think I need to keep this whole thing
                // sorted now, as this is a huge mess.
                // Basically, we want to do the exact thing we do in the
                // server, except explicitly.
                // Also, rethink the bulk edit abstraction, it's use may no longer
                // be as needed with the channel names and edges seperated.
                dbg!("Expanding path which contains");
                let (left, right) = path.split_at(path_ix);
                let mut new_path = Vec::with_capacity(left.len() + right.len() + 1);

                /// WRONG WRONG WRONG
                new_path.extend_from_slice(left);
                new_path.push(parent_id);
                /// WRONG WRONG WRONG

                new_path.extend_from_slice(right);
                if path_ix == 0 {
                    dbg!("Replacing path that starts with this");
                    self.paths[ix] = dbg!(ChannelPath::new(new_path.into()));
                } else {
                    dbg!("inserting new path");
                    self.paths.insert(ix + 1, dbg!(ChannelPath::new(new_path.into())));
                    ix += 1;
                }
                ix += 1;
            } else {
                dbg!("Doing nothing");
                ix += 1;
            }
        }
    }

    fn insert_root(&mut self, channel_id: ChannelId) {
        self.paths.push(ChannelPath::new(Arc::from([channel_id])));
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
