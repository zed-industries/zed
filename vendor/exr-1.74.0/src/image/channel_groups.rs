
use std::collections::HashMap;
use crate::image::write::channels::{WritableChannels, ChannelsWriter};
use crate::meta::attribute::{LevelMode, ChannelList, Text, TextSlice, ChannelInfo};
use crate::meta::header::Header;
use crate::image::read::layers::{ReadChannels, ChannelsReader};
use crate::block::{BlockIndex, UncompressedBlock};
use crate::block::lines::{collect_uncompressed_block_from_lines, LineIndex};
use std::io::{Cursor, Read};
use crate::error::{Result, UnitResult};
use crate::block::chunk::TileCoordinates;
use crate::prelude::SmallVec;





pub struct ChannelGroups<ChannelGroup> {
    channel_group: Option<ChannelGroup>,
    children: HashMap<Text, Self>
}


impl<ChannelGroup> ChannelGroups<ChannelGroup>  {


    // pub fn visit_groups_mut(&mut self, visitor: impl Fn(&mut Channels)) {
    // }



    pub fn groups(&self) -> SmallVec<[&ChannelGroup; 12]> {
        let children = self.children.iter().flat_map(|group| group.groups());
        self.channel_group.iter().chain(children).collect()
    }

    pub fn lookup_group(&self, group_name: &TextSlice) -> Option<&ChannelGroup> {
        let dot_index = group_name.iter().position('.');
        if let Some(dot_index) = dot_index {
            let group_name = &group_name[.. dot_index];
            let child_name = &group_name[dot_index + 1 ..];
            self.children.get(group_name)
                .and_then(|child| child.lookup(child_name))
        }
        else {
            self.channel_group.lookup(name)
        }
    }


    /*pub fn insert_group(&mut self, full_name: &TextSlice, value: ChannelGroup) {
        let dot_index = full_name.iter().position('.');
        if let Some(dot_index) = dot_index {
            let group_name = &group_name[.. dot_index];
            let name_rest = &group_name[dot_index + 1 ..];

            self.children.entry(Text::from_slice_unchecked(group_name))
                .or_insert(|| );

            // self.children.insert(Text::from_slice_unchecked(group_name), value)
            //     .and_then(|child| child.lookup(name_rest));
        }
        else {
            self.channel_group.lookup(name);
        }
    }*/

    pub fn map<T>(self, mapper: impl FnMut(ChannelGroup) -> T) -> ChannelGroups<T> {
        ChannelGroups {
            children: self.channel_group.iter().map(&mapper).collect(),
            channel_group: self.channel_group.map(mapper),
        }
    }
}


pub fn parse_channel_list_groups<T>(channels: impl Iterator<Item=(Text, T)>)
    -> ChannelGroups<SmallVec<(Text, T)>>
{
    fn insert_into_groups(groups: &mut ChannelGroups<SmallVec<(Text, T)>>, name: Text, value: T) {
        let dot_index = name.as_slice().iter().position('.');

        if let Some(dot_index) = dot_index {
            // insert into child group

            let group_name = Text::from_slice_unchecked(&name.as_slice()[.. dot_index]);
            let child_channel = Text::from_slice_unchecked(&name.as_slice()[dot_index + 1 ..]);

            let child_group = groups.children.entry(group_name)
                .or_insert(ChannelGroups { channel_group: None, children: Default::default() });

            insert_into_groups(child_group, child_channel, value);
        }

        else {
            // insert directly into group

            if groups.channel_group.is_none() {
                groups.channel_group = Some(SmallVec::new());
            }

            groups.channel_group.unwrap().push(value);
        }
    }

    let mut result = ChannelGroups { channel_group: None, children: HashMap::default() };
    for (name, value) in channels { insert_into_groups(&mut result, name, value); }
    result
}


impl<'slf, ChannelGroup> WritableChannels<'slf> for ChannelGroups<ChannelGroup>
    where ChannelGroup: WritableChannels<'slf>
{
    fn infer_channel_list(&self) -> ChannelList {
        // TODO what about empty groups with NO channels??

        let child_channels = self.children.iter().flat_map(|(group_name, child)| {
            let mut child_channels = child.infer_channel_list().list;
            for channel in &mut child_channels { channel.name.push_front(group_name) };
            child_channels
        });

        let mut own_channels = self.channel_group
            .map(|chans| chans.infer_channel_list().list)
            .unwrap_or_default();

        own_channels.extend(child_channels);
        own_channels.sort_unstable(); // TODO only once at end
        ChannelList::new(own_channels) // might be empty, but will be checked in MetaData::validate()
    }

    fn level_mode(&self) -> LevelMode {
        fn find_mode_or_none(channels: &Self) -> Option<LevelMode> {
            channels.channel_group.map(WritableChannels::level_mode).or_else(|| {
                channels.children.iter().map(find_mode_or_none).next()
            })
        }

        let mode = find_mode_or_none(self)
            .expect("empty channel groups (check failed)"); // TODO only happens for empty channels, right? panic maybe?

        if let Some(chans) = self.channel_group.as_ref() {
            debug_assert_eq!(chans.level_mode(), mode, "level mode must be equal for all legacy channel groups")
        }

        debug_assert!(
            self.children.values()
                .flat_map(find_mode_or_none)
                .all(|child_mode| child_mode == mode),

            "level mode must be equal for all legacy channel groups"
        );

        mode
    }

    type Writer = GroupChannelsWriter<'slf, ChannelGroup>;

    fn create_writer(&'slf self, header: &Header) -> Self::Writer {
        let channels = header.channels.list.iter()
            .map(|channel_info|{
                // hashmap order is not guaranteed? so look up each channel group manually instead of generating new
                let channels = self.lookup_group(channel_info.name.as_slice())
                    .expect("channels not found bug");

                channels.create_writer(header) // channel_info.name.clone()
            })
            .collect();

        GroupChannelsWriter { channels_list: channels }
    }
}

struct GroupChannelsWriter<'c, ChannelGroupWriter> {
    channels_list: Vec<&'c ChannelGroupWriter>,
}

impl<'c, Channels> ChannelsWriter for GroupChannelsWriter<'c, Channels> where Channels: ChannelsWriter {
    fn extract_uncompressed_block(&self, header: &Header, block: BlockIndex) -> Vec<u8> {
        let mut blocks_per_channel: Vec<Cursor<Vec<u8>>> = self
            .channels_list.iter()
            .map(|channels| Cursor::new(channels.extract_uncompressed_block(header, block)))
            .collect();

        UncompressedBlock::uncompressed_block_from_lines(header, block, |line|{
            let channel_reader = &mut blocks_per_channel[line.location.channel]; // TODO subsampling

            // read from specific channel into total byte block
            // this assumes that the lines in the callback are iterated in strictly increasing order
            // because each channel reader is consumed
            channel_reader.read_exact(line.value)
                .expect("collecting grouped channel byte block failed");
        })
    }
}


struct ReadChannelGroups<ReadChannelGroup> {
    read_channels: ReadChannelGroup
}

struct ChannelGroupsReader<ChannelGroupReader> {
    channels: ChannelGroups<usize>,
    indexed_channels: Vec<ChannelGroupReader>,
}

impl<'s, ReadChannelGroup> ReadChannels<'s> for ReadChannelGroups<ReadChannelGroup>
    where ReadChannelGroup: ReadChannels<'s>
{
    type Reader = ChannelGroupsReader<ReadChannelGroup::Reader>;

    fn create_channels_reader(&'s self, header: &Header) -> Result<Self::Reader> {
        let swap = |(a,b)| (b,a);
        let channel_groups = parse_channel_list_groups(
            header.channels.list.iter().enumerate().map(swap)
        );

        let mut indexed_channels = Vec::new();
        let channel_groups = channel_groups.map(|channels| {

            let mut channels_header = header.clone(); // TODO no clone?
            channels_header.channels = ChannelList::new(channels.iter().map(|(name, index)|{
                let mut channel_info = header.channels.list[index].clone();
                channel_info.name = name;
                channel_info
            }).collect()); // FIXME does not comply to `header.chunk_count` and that stuff?? change ReadChannels fn signature?

            indexed_channels.push(self.read_channels.create_channels_reader(&channels_header));

            // FIXME this is not the original order indexed_channels.len() - 1
            indexed_channels[]
        });

        Ok(ChannelGroupsReader {
            channels: channel_groups,
            indexed_channels,
        })

        /*Ok(ChannelGroupsReader {
            channels: header.channels.list.iter().map(|channel| {
                let mut channels_header = header.clone();

                let reader = self.read_channels.create_channels_reader(&channels_header);
                (channels_header, reader)
            }).collect(),
        })*/
    }
}

impl<ChannelGroupReader> ChannelsReader for ChannelGroupsReader<ChannelGroupReader> where ChannelGroupReader: ChannelsReader {
    type Channels = ChannelGroups<ChannelGroupReader::Channels>;

    fn filter_block(&self, tile: (usize, &TileCoordinates)) -> bool {
        self.indexed_channels.iter().any(|channel| channel.filter_block(tile))
    }

    fn read_block(&mut self, header: &Header, block: UncompressedBlock) -> UnitResult {
        block.for_lines(|line|{

        })
    }

    fn into_channels(self) -> Self::Channels {

    }
}