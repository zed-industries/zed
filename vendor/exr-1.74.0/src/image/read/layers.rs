//! How to read either a single or a list of layers.

use crate::image::*;
use crate::meta::header::{Header, LayerAttributes};
use crate::error::{Result, UnitResult, Error};
use crate::block::{UncompressedBlock, BlockIndex};
use crate::math::Vec2;
use crate::image::read::image::{ReadLayers, LayersReader};
use crate::block::chunk::TileCoordinates;
use crate::meta::MetaData;

/// Specify to read all channels, aborting if any one is invalid.
/// [`ReadRgbaChannels`] or [`ReadAnyChannels<ReadFlatSamples>`].
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReadAllLayers<ReadChannels> {

    /// The channel reading specification
    pub read_channels: ReadChannels,
}

/// Specify to read only the first layer which meets the previously specified requirements
// FIXME do not throw error on deep data but just skip it!
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ReadFirstValidLayer<ReadChannels> {

    /// The channel reading specification
    pub read_channels: ReadChannels,
}

/// A template that creates a [`ChannelsReader`] once for all channels per layer.
pub trait ReadChannels<'s> {

    /// The type of the temporary channels reader
    type Reader: ChannelsReader;

    /// Create a single reader for all channels of a specific layer
    fn create_channels_reader(&'s self, header: &Header) -> Result<Self::Reader>;


    /// Read only the first layer which meets the previously specified requirements
    /// For example, skips layers with deep data, if specified earlier.
    /// Aborts if the image contains no layers.
    // TODO test if this filters non-deep layers while ignoring deep data layers!
    fn first_valid_layer(self) -> ReadFirstValidLayer<Self> where Self:Sized { ReadFirstValidLayer { read_channels: self } }

// FIXME do not throw error on deep data but just skip it!


    /// Reads all layers, including an empty list. Aborts if any of the layers are invalid,
    /// even if only one of the layers contains unexpected data.
    fn all_layers(self) -> ReadAllLayers<Self> where Self:Sized { ReadAllLayers { read_channels: self } }

    // TODO pub fn all_valid_layers(self) -> ReadAllValidLayers<Self> { ReadAllValidLayers { read_channels: self } }
}


/// Processes pixel blocks from a file and accumulates them into a list of layers.
/// For example, `ChannelsReader` can be
/// [`SpecificChannelsReader`] or [`AnyChannelsReader<FlatSamplesReader>`].
#[derive(Debug, Clone, PartialEq)]
pub struct AllLayersReader<ChannelsReader> {
    layer_readers: SmallVec<[LayerReader<ChannelsReader>; 2]>, // TODO unpack struct?
}

/// Processes pixel blocks from a file and accumulates them into a single layers, using only the first.
/// For example, `ChannelsReader` can be
/// `SpecificChannelsReader` or `AnyChannelsReader<FlatSamplesReader>`.
#[derive(Debug, Clone, PartialEq)]
pub struct FirstValidLayerReader<ChannelsReader> {
    layer_reader: LayerReader<ChannelsReader>,
    layer_index: usize,
}

/// Processes pixel blocks from a file and accumulates them into a single layers.
/// For example, `ChannelsReader` can be
/// `SpecificChannelsReader` or `AnyChannelsReader<FlatSamplesReader>`.
#[derive(Debug, Clone, PartialEq)]
pub struct LayerReader<ChannelsReader> {
    channels_reader: ChannelsReader,
    attributes: LayerAttributes,
    size: Vec2<usize>,
    encoding: Encoding
}

/// Processes pixel blocks from a file and accumulates them into multiple channels per layer.
pub trait ChannelsReader {

    /// The type of the resulting channel collection
    type Channels;

    /// Specify whether a single block of pixels should be loaded from the file
    fn filter_block(&self, tile: TileCoordinates) -> bool;

    /// Load a single pixel block, which has not been filtered, into the reader, accumulating the channel data
    fn read_block(&mut self, header: &Header, block: UncompressedBlock) -> UnitResult;

    /// Deliver the final accumulated channel collection for the image
    fn into_channels(self) -> Self::Channels;
}


impl<C> LayerReader<C> {
    fn new(header: &Header, channels_reader: C) -> Result<Self> {
        Ok(LayerReader {
            channels_reader,
            attributes: header.own_attributes.clone(),
            size: header.layer_size,
            encoding: Encoding {
                compression: header.compression,
                line_order: header.line_order,
                blocks: match header.blocks {
                    crate::meta::BlockDescription::ScanLines => Blocks::ScanLines,
                    crate::meta::BlockDescription::Tiles(TileDescription { tile_size, .. }) => Blocks::Tiles(tile_size)
                },
            },
        })
    }
}

impl<'s, C> ReadLayers<'s> for ReadAllLayers<C> where C: ReadChannels<'s> {
    type Layers = Layers<<C::Reader as ChannelsReader>::Channels>;
    type Reader = AllLayersReader<C::Reader>;

    fn create_layers_reader(&'s self, headers: &[Header]) -> Result<Self::Reader> {
        let readers: Result<_> = headers.iter()
            .map(|header| LayerReader::new(header, self.read_channels.create_channels_reader(header)?))
            .collect();

        Ok(AllLayersReader {
            layer_readers: readers?
        })
    }
}

impl<C> LayersReader for AllLayersReader<C> where C: ChannelsReader {
    type Layers = Layers<C::Channels>;

    fn filter_block(&self, _: &MetaData, tile: TileCoordinates, block: BlockIndex) -> bool {
        let layer = self.layer_readers.get(block.layer).expect("invalid layer index argument");
        layer.channels_reader.filter_block(tile)
    }

    fn read_block(&mut self, headers: &[Header], block: UncompressedBlock) -> UnitResult {
        self.layer_readers
            .get_mut(block.index.layer).expect("invalid layer index argument")
            .channels_reader.read_block(headers.get(block.index.layer).expect("invalid header index in block"), block)
    }

    fn into_layers(self) -> Self::Layers {
        self.layer_readers
            .into_iter()
            .map(|layer| Layer {
                channel_data: layer.channels_reader.into_channels(),
                attributes: layer.attributes,
                size: layer.size,
                encoding: layer.encoding
            })
            .collect()
    }
}


impl<'s, C> ReadLayers<'s> for ReadFirstValidLayer<C> where C: ReadChannels<'s> {
    type Layers = Layer<<C::Reader as ChannelsReader>::Channels>;
    type Reader = FirstValidLayerReader<C::Reader>;

    fn create_layers_reader(&'s self, headers: &[Header]) -> Result<Self::Reader> {
        headers.iter().enumerate()
            .flat_map(|(index, header)|
                self.read_channels.create_channels_reader(header)
                    .and_then(|reader| Ok(FirstValidLayerReader {
                        layer_reader: LayerReader::new(header, reader)?,
                        layer_index: index
                    }))
                    .ok()
            )
            .next()
            .ok_or(Error::invalid("no layer in the image matched your specified requirements"))
    }
}


impl<C> LayersReader for FirstValidLayerReader<C> where C: ChannelsReader {
    type Layers = Layer<C::Channels>;

    fn filter_block(&self, _: &MetaData, tile: TileCoordinates, block: BlockIndex) -> bool {
        block.layer == self.layer_index && self.layer_reader.channels_reader.filter_block(tile)
    }

    fn read_block(&mut self, headers: &[Header], block: UncompressedBlock) -> UnitResult {
        debug_assert_eq!(block.index.layer, self.layer_index, "block should have been filtered out");
        self.layer_reader.channels_reader.read_block(&headers[self.layer_index], block)
    }

    fn into_layers(self) -> Self::Layers {
        Layer {
            channel_data: self.layer_reader.channels_reader.into_channels(),
            attributes: self.layer_reader.attributes,
            size: self.layer_reader.size,
            encoding: self.layer_reader.encoding
        }
    }
}

