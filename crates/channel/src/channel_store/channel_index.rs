use crate::Channel;
use client::ChannelId;
use collections::BTreeMap;
use rpc::proto;
use std::sync::Arc;

#[derive(Default, Debug)]
pub struct ChannelIndex {
    channels_ordered: Vec<ChannelId>,
    channels_by_id: BTreeMap<ChannelId, Arc<Channel>>,
}

impl ChannelIndex {
    pub fn by_id(&self) -> &BTreeMap<ChannelId, Arc<Channel>> {
        &self.channels_by_id
    }

    pub fn ordered_channels(&self) -> &[ChannelId] {
        &self.channels_ordered
    }

    pub fn clear(&mut self) {
        self.channels_ordered.clear();
        self.channels_by_id.clear();
    }

    /// Delete the given channels from this index.
    pub fn delete_channels(&mut self, channels: &[ChannelId]) {
        self.channels_by_id
            .retain(|channel_id, _| !channels.contains(channel_id));
        self.channels_ordered
            .retain(|channel_id| !channels.contains(channel_id));
    }

    pub fn bulk_insert(&mut self) -> ChannelPathsInsertGuard {
        ChannelPathsInsertGuard {
            channels_ordered: &mut self.channels_ordered,
            channels_by_id: &mut self.channels_by_id,
        }
    }
}

/// A guard for ensuring that the paths index maintains its sort and uniqueness
/// invariants after a series of insertions
#[derive(Debug)]
pub struct ChannelPathsInsertGuard<'a> {
    channels_ordered: &'a mut Vec<ChannelId>,
    channels_by_id: &'a mut BTreeMap<ChannelId, Arc<Channel>>,
}

impl<'a> ChannelPathsInsertGuard<'a> {
    pub fn insert(&mut self, channel_proto: proto::Channel) -> bool {
        let mut ret = false;
        let parent_path = channel_proto
            .parent_path
            .iter()
            .map(|cid| ChannelId(*cid))
            .collect();
        if let Some(existing_channel) = self.channels_by_id.get_mut(&ChannelId(channel_proto.id)) {
            let existing_channel = Arc::make_mut(existing_channel);

            ret = existing_channel.visibility != channel_proto.visibility()
                || existing_channel.name != channel_proto.name
                || existing_channel.parent_path != parent_path;

            existing_channel.visibility = channel_proto.visibility();
            existing_channel.name = channel_proto.name.into();
            existing_channel.parent_path = parent_path;
        } else {
            self.channels_by_id.insert(
                ChannelId(channel_proto.id),
                Arc::new(Channel {
                    id: ChannelId(channel_proto.id),
                    visibility: channel_proto.visibility(),
                    name: channel_proto.name.into(),
                    parent_path,
                }),
            );
            self.insert_root(ChannelId(channel_proto.id));
        }
        ret
    }

    fn insert_root(&mut self, channel_id: ChannelId) {
        self.channels_ordered.push(channel_id);
    }
}

impl<'a> Drop for ChannelPathsInsertGuard<'a> {
    fn drop(&mut self) {
        self.channels_ordered.sort_by(|a, b| {
            let a = channel_path_sorting_key(*a, self.channels_by_id);
            let b = channel_path_sorting_key(*b, self.channels_by_id);
            a.cmp(b)
        });
        self.channels_ordered.dedup();
    }
}

fn channel_path_sorting_key(
    id: ChannelId,
    channels_by_id: &BTreeMap<ChannelId, Arc<Channel>>,
) -> impl Iterator<Item = (&str, ChannelId)> {
    let (parent_path, name) = channels_by_id
        .get(&id)
        .map_or((&[] as &[_], None), |channel| {
            (
                channel.parent_path.as_slice(),
                Some((channel.name.as_ref(), channel.id)),
            )
        });
    parent_path
        .iter()
        .filter_map(|id| Some((channels_by_id.get(id)?.name.as_ref(), *id)))
        .chain(name)
}
