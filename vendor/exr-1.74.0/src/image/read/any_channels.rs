//! How to read arbitrary channels.

use crate::image::*;
use crate::meta::header::{Header};
use crate::error::{Result, UnitResult};
use crate::block::UncompressedBlock;
use crate::block::lines::{LineRef};
use crate::math::Vec2;
use crate::meta::attribute::{Text, ChannelDescription};
use crate::image::read::layers::{ReadChannels, ChannelsReader};
use crate::block::chunk::TileCoordinates;

/// A template that creates an [AnyChannelsReader] for each layer in the image.
/// This loads all channels for each layer.
/// The `ReadSamples` can, for example, be [ReadFlatSamples] or [ReadAllLevels<ReadFlatSamples>].
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReadAnyChannels<ReadSamples> {

    /// The sample reading specification
    pub read_samples: ReadSamples
}

/// A template that creates a new [`SampleReader`] for each channel in each layer.
pub trait ReadSamples {

    /// The type of the temporary samples reader
    type Reader: SamplesReader;

    /// Create a single reader for a single channel of a layer
    fn create_sample_reader(&self, header: &Header, channel: &ChannelDescription) -> Result<Self::Reader>;
}

/// Processes pixel blocks from a file and accumulates them into a collection of arbitrary channels.
/// Loads all channels for each layer.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AnyChannelsReader<SamplesReader> {

    /// Stores a separate sample reader per channel in the layer
    sample_channels_reader: SmallVec<[AnyChannelReader<SamplesReader>; 4]>,
}

/// Processes pixel blocks from a file and accumulates them into a single arbitrary channel.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AnyChannelReader<SamplesReader> {

    /// The custom reader that accumulates the pixel data for a single channel
    samples: SamplesReader,

    /// Temporarily accumulated meta data.
    name: Text,

    /// Temporarily accumulated meta data.
    sampling_rate: Vec2<usize>,

    /// Temporarily accumulated meta data.
    quantize_linearly: bool,
}

/// Processes pixel blocks from a file and accumulates them into a single pixel channel.
/// For example, stores thousands of "Red" pixel values for a single layer.
pub trait SamplesReader {

    /// The type of resulting sample storage
    type Samples;

    /// Specify whether a single block of pixels should be loaded from the file
    fn filter_block(&self, tile: TileCoordinates) -> bool;

    /// Load a single pixel line, which has not been filtered, into the reader, accumulating the sample data
    fn read_line(&mut self, line: LineRef<'_>) -> UnitResult;

    /// Deliver the final accumulated sample storage for the image
    fn into_samples(self) -> Self::Samples;
}


impl<'s, S: 's + ReadSamples> ReadChannels<'s> for ReadAnyChannels<S> {
    type Reader = AnyChannelsReader<S::Reader>;

    fn create_channels_reader(&self, header: &Header) -> Result<Self::Reader> {
        let samples: Result<_> = header.channels.list.iter()
            .map(|channel: &ChannelDescription| Ok(AnyChannelReader {
                samples: self.read_samples.create_sample_reader(header, channel)?,
                name: channel.name.clone(),
                sampling_rate: channel.sampling,
                quantize_linearly: channel.quantize_linearly
            }))
            .collect();

        Ok(AnyChannelsReader { sample_channels_reader: samples? })
    }
}

impl<S: SamplesReader> ChannelsReader for AnyChannelsReader<S> {
    type Channels = AnyChannels<S::Samples>;

    fn filter_block(&self, tile: TileCoordinates) -> bool {
        self.sample_channels_reader.iter().any(|channel| channel.samples.filter_block(tile))
    }

    fn read_block(&mut self, header: &Header, decompressed: UncompressedBlock) -> UnitResult {
        /*for (bytes, line) in LineIndex::lines_in_block(decompressed.index, header) {
            let channel = self.sample_channels_reader.get_mut(line.channel).unwrap();
            channel.samples.read_line(LineSlice { location: line, value: &decompressed.data[bytes] })?;
        }

        Ok(())*/
        for line in decompressed.lines(&header.channels) {
            self.sample_channels_reader[line.location.channel].samples.read_line(line)?;
        }

        Ok(())
    }

    fn into_channels(self) -> Self::Channels {
        AnyChannels { // not using `new()` as the channels are already sorted
            list: self.sample_channels_reader.into_iter()
                .map(|channel| AnyChannel {
                    sample_data: channel.samples.into_samples(),

                    name: channel.name,
                    quantize_linearly: channel.quantize_linearly,
                    sampling: channel.sampling_rate
                })
                .collect()
        }
    }
}
