use crate::{Channel, ChannelId};
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

    pub fn acknowledge_note_version(
        &mut self,
        channel_id: ChannelId,
        epoch: u64,
        version: &clock::Global,
    ) {
        if let Some(channel) = self.channels_by_id.get_mut(&channel_id) {
            let channel = Arc::make_mut(channel);
            if let Some((unseen_epoch, unseen_version)) = &channel.unseen_note_version {
                if epoch > *unseen_epoch
                    || epoch == *unseen_epoch && version.observed_all(unseen_version)
                {
                    channel.unseen_note_version = None;
                }
            }
        }
    }

    pub fn acknowledge_message_id(&mut self, channel_id: ChannelId, message_id: u64) {
        if let Some(channel) = self.channels_by_id.get_mut(&channel_id) {
            let channel = Arc::make_mut(channel);
            if let Some(unseen_message_id) = channel.unseen_message_id {
                if message_id >= unseen_message_id {
                    channel.unseen_message_id = None;
                }
            }
        }
    }

    pub fn note_changed(&mut self, channel_id: ChannelId, epoch: u64, version: &clock::Global) {
        insert_note_changed(&mut self.channels_by_id, channel_id, epoch, version);
    }

    pub fn new_message(&mut self, channel_id: ChannelId, message_id: u64) {
        insert_new_message(&mut self.channels_by_id, channel_id, message_id)
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
    pub fn note_changed(&mut self, channel_id: ChannelId, epoch: u64, version: &clock::Global) {
        insert_note_changed(self.channels_by_id, channel_id, epoch, version);
    }

    pub fn new_messages(&mut self, channel_id: ChannelId, message_id: u64) {
        insert_new_message(self.channels_by_id, channel_id, message_id)
    }

    pub fn insert(&mut self, channel_proto: proto::Channel) -> bool {
        let mut ret = false;
        if let Some(existing_channel) = self.channels_by_id.get_mut(&channel_proto.id) {
            let existing_channel = Arc::make_mut(existing_channel);

            ret = existing_channel.visibility != channel_proto.visibility()
                || existing_channel.role != channel_proto.role()
                || existing_channel.name != channel_proto.name;

            existing_channel.visibility = channel_proto.visibility();
            existing_channel.role = channel_proto.role();
            existing_channel.name = channel_proto.name.into();
        } else {
            self.channels_by_id.insert(
                channel_proto.id,
                Arc::new(Channel {
                    id: channel_proto.id,
                    visibility: channel_proto.visibility(),
                    role: channel_proto.role(),
                    name: channel_proto.name.into(),
                    unseen_note_version: None,
                    unseen_message_id: None,
                    parent_path: channel_proto.parent_path,
                }),
            );
            self.insert_root(channel_proto.id);
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

fn channel_path_sorting_key<'a>(
    id: ChannelId,
    channels_by_id: &'a BTreeMap<ChannelId, Arc<Channel>>,
) -> impl Iterator<Item = &str> {
    let (parent_path, name) = channels_by_id
        .get(&id)
        .map_or((&[] as &[_], None), |channel| {
            (channel.parent_path.as_slice(), Some(channel.name.as_ref()))
        });
    parent_path
        .iter()
        .filter_map(|id| Some(channels_by_id.get(id)?.name.as_ref()))
        .chain(name)
}

fn insert_note_changed(
    channels_by_id: &mut BTreeMap<ChannelId, Arc<Channel>>,
    channel_id: u64,
    epoch: u64,
    version: &clock::Global,
) {
    if let Some(channel) = channels_by_id.get_mut(&channel_id) {
        let unseen_version = Arc::make_mut(channel)
            .unseen_note_version
            .get_or_insert((0, clock::Global::new()));
        if epoch > unseen_version.0 {
            *unseen_version = (epoch, version.clone());
        } else {
            unseen_version.1.join(version);
        }
    }
}

fn insert_new_message(
    channels_by_id: &mut BTreeMap<ChannelId, Arc<Channel>>,
    channel_id: u64,
    message_id: u64,
) {
    if let Some(channel) = channels_by_id.get_mut(&channel_id) {
        let unseen_message_id = Arc::make_mut(channel).unseen_message_id.get_or_insert(0);
        *unseen_message_id = message_id.max(*unseen_message_id);
    }
}
