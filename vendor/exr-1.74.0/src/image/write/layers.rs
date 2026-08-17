//! How to write either a single or a list of layers.

use crate::meta::header::{ImageAttributes, Header};
use crate::meta::{Headers, compute_chunk_count};
use crate::block::BlockIndex;
use crate::image::{Layers, Layer};
use crate::meta::attribute::{TileDescription};
use crate::prelude::{SmallVec};
use crate::image::write::channels::{WritableChannels, ChannelsWriter};
use crate::image::recursive::{Recursive, NoneMore};

/// Enables an image containing this list of layers to be written to a file.
pub trait WritableLayers<'slf> {

    /// Generate the file meta data for this list of layers
    fn infer_headers(&self, image_attributes: &ImageAttributes) -> Headers;

    /// The type of temporary writer
    type Writer: LayersWriter;

    /// Create a temporary writer for this list of layers
    fn create_writer(&'slf self, headers: &[Header]) -> Self::Writer;
}

/// A temporary writer for a list of channels
pub trait LayersWriter: Sync {

    /// Deliver a block of pixels from a single layer to be stored in the file
    fn extract_uncompressed_block(&self, headers: &[Header], block: BlockIndex) -> Vec<u8>;
}

/// A temporary writer for an arbitrary list of layers
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AllLayersWriter<ChannelsWriter> {
    layers: SmallVec<[LayerWriter<ChannelsWriter>; 2]>
}

/// A temporary writer for a single layer
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LayerWriter<ChannelsWriter> {
    channels: ChannelsWriter, // impl ChannelsWriter
}

// impl for smallvec
impl<'slf, Channels: 'slf> WritableLayers<'slf> for Layers<Channels> where Channels: WritableChannels<'slf> {
    fn infer_headers(&self, image_attributes: &ImageAttributes) -> Headers {
        slice_infer_headers(self.as_slice(), image_attributes)
    }

    type Writer = AllLayersWriter<Channels::Writer>;
    fn create_writer(&'slf self, headers: &[Header]) -> Self::Writer {
        slice_create_writer(self.as_slice(), headers)
    }
}

fn slice_infer_headers<'slf, Channels:'slf + WritableChannels<'slf>>(
    slice: &[Layer<Channels>], image_attributes: &ImageAttributes
) -> Headers
{
    slice.iter().map(|layer| layer.infer_headers(image_attributes).remove(0)).collect() // TODO no array-vs-first
}

fn slice_create_writer<'slf, Channels:'slf + WritableChannels<'slf>>(
    slice: &'slf [Layer<Channels>], headers: &[Header]
) -> AllLayersWriter<Channels::Writer>
{
    AllLayersWriter {
        layers: slice.iter().zip(headers.chunks_exact(1)) // TODO no array-vs-first
            .map(|(layer, header)| layer.create_writer(header))
            .collect()
    }
}


impl<'slf, Channels: WritableChannels<'slf>> WritableLayers<'slf> for Layer<Channels> {
    fn infer_headers(&self, image_attributes: &ImageAttributes) -> Headers {
        let blocks = match self.encoding.blocks {
            crate::image::Blocks::ScanLines => crate::meta::BlockDescription::ScanLines,
            crate::image::Blocks::Tiles(tile_size) => {
                let (level_mode, rounding_mode) = self.channel_data.infer_level_modes();
                crate::meta::BlockDescription::Tiles(TileDescription { level_mode, rounding_mode, tile_size, })
            },
        };

        let chunk_count = compute_chunk_count(
            self.encoding.compression, self.size, blocks
        );

        let header = Header {
            channels: self.channel_data.infer_channel_list(),
            compression: self.encoding.compression,

            blocks,
            chunk_count,

            line_order: self.encoding.line_order,
            layer_size: self.size,
            shared_attributes: image_attributes.clone(),
            own_attributes: self.attributes.clone(),


            deep: false, // TODO deep data
            deep_data_version: None,
            max_samples_per_pixel: None,
        };

        smallvec![ header ]// TODO no array-vs-first
    }

    type Writer = LayerWriter</*'l,*/ Channels::Writer>;
    fn create_writer(&'slf self, headers: &[Header]) -> Self::Writer {
        let channels = self.channel_data
            .create_writer(headers.first().expect("inferred header error")); // TODO no array-vs-first

        LayerWriter { channels }
    }
}

impl<C> LayersWriter for AllLayersWriter<C> where C: ChannelsWriter {
    fn extract_uncompressed_block(&self, headers: &[Header], block: BlockIndex) -> Vec<u8> {
        self.layers[block.layer].extract_uncompressed_block(std::slice::from_ref(&headers[block.layer]), block) // TODO no array-vs-first
    }
}

impl<C> LayersWriter for LayerWriter<C> where C: ChannelsWriter {
    fn extract_uncompressed_block(&self, headers: &[Header], block: BlockIndex) -> Vec<u8> {
        self.channels.extract_uncompressed_block(headers.first().expect("invalid inferred header"), block) // TODO no array-vs-first
    }
}





impl<'slf> WritableLayers<'slf> for NoneMore {
    fn infer_headers(&self, _: &ImageAttributes) -> Headers { SmallVec::new() }

    type Writer = NoneMore;
    fn create_writer(&'slf self, _: &[Header]) -> Self::Writer { NoneMore }
}

impl<'slf, InnerLayers, Channels> WritableLayers<'slf> for Recursive<InnerLayers, Layer<Channels>>
    where InnerLayers: WritableLayers<'slf>, Channels: WritableChannels<'slf>
{
    fn infer_headers(&self, image_attributes: &ImageAttributes) -> Headers {
        let mut headers = self.inner.infer_headers(image_attributes);
        headers.push(self.value.infer_headers(image_attributes).remove(0)); // TODO no unwrap
        headers
    }

    type Writer = RecursiveLayersWriter<InnerLayers::Writer, Channels::Writer>;

    fn create_writer(&'slf self, headers: &[Header]) -> Self::Writer {
        let (own_header, inner_headers) = headers.split_last()
            .expect("header has not been inferred correctly");

        let layer_index = inner_headers.len();
        RecursiveLayersWriter {
            inner: self.inner.create_writer(inner_headers),
            value: (layer_index, self.value.create_writer(std::slice::from_ref(own_header))) // TODO no slice
        }
    }
}

type RecursiveLayersWriter<InnerLayersWriter, ChannelsWriter> = Recursive<InnerLayersWriter, (usize, LayerWriter<ChannelsWriter>)>;

impl LayersWriter for NoneMore {
    fn extract_uncompressed_block(&self, _: &[Header], _: BlockIndex) -> Vec<u8> {
        panic!("recursive length mismatch bug");
    }
}

impl<InnerLayersWriter, Channels> LayersWriter for RecursiveLayersWriter<InnerLayersWriter, Channels>
    where InnerLayersWriter: LayersWriter, Channels: ChannelsWriter
{
    fn extract_uncompressed_block(&self, headers: &[Header], block: BlockIndex) -> Vec<u8> {
        let (layer_index, layer) = &self.value;
        if *layer_index == block.layer {
            let header = headers.get(*layer_index).expect("layer index bug");
            layer.extract_uncompressed_block(std::slice::from_ref(header), block) // TODO no slice?
        }
        else {
            self.inner.extract_uncompressed_block(headers, block)
        }
    }
}


