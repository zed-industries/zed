//! The last wrapper of image readers, finally containing the [`from_file(path)`] method.
//! This completes the builder and reads a complete image.

use crate::image::*;
use crate::meta::header::{Header, ImageAttributes};
use crate::error::{Result, UnitResult};
use crate::block::{UncompressedBlock, BlockIndex};
use crate::block::chunk::TileCoordinates;
use std::path::Path;
use std::io::{Read, BufReader};
use std::io::Seek;
use crate::meta::MetaData;
use crate::block::reader::ChunksReader;

/// Specify whether to read the image in parallel,
/// whether to use pedantic error handling,
/// and a callback for the reading progress.
#[derive(Debug, Clone)]
pub struct ReadImage<OnProgress, ReadLayers> {
    on_progress: OnProgress,
    read_layers: ReadLayers,
    pedantic: bool,
    parallel: bool,
}

impl<F, L> ReadImage<F, L> where F: FnMut(f64)
{
    /// Uses relaxed error handling and parallel decompression.
    pub fn new(read_layers: L, on_progress: F) -> Self {
        Self {
            on_progress, read_layers,
            pedantic: false,
            #[cfg(not(feature = "rayon"))]
            parallel: false,
            #[cfg(feature = "rayon")]
            parallel: true,
        }
    }

    /// Specify that any missing or unusual information should result in an error.
    /// Otherwise, `exrs` will try to compute or ignore missing information.
    ///
    /// If pedantic is true, then an error will be returned as soon as anything is missing in the file,
    /// or two values in the image contradict each other. If pedantic is false,
    /// then only fatal errors will be thrown. By default, reading an image is not pedantic,
    /// which means that slightly invalid files might still be readable.
    /// For example, if some attribute is missing but can be recomputed, this flag decides whether an error is thrown.
    /// Or if the pedantic flag is true and there are still bytes left after the decompression algorithm finished,
    /// an error is thrown, because this should not happen and something might be wrong with the file.
    /// Or if your application is a target of attacks, or if you want to emulate the original C++ library,
    /// you might want to switch to pedantic reading.
    pub fn pedantic(self) -> Self { Self { pedantic: true, ..self } }

    /// Specify that multiple pixel blocks should never be decompressed using multiple threads at once.
    /// This might be slower but uses less memory and less synchronization.
    pub fn non_parallel(self) -> Self { Self { parallel: false, ..self } }

    /// Specify a function to be called regularly throughout the loading process.
    /// Replaces all previously specified progress functions in this reader.
    pub fn on_progress<OnProgress>(self, on_progress: OnProgress) -> ReadImage<OnProgress, L>
        where OnProgress: FnMut(f64)
    {
        ReadImage {
            on_progress,
            read_layers: self.read_layers,
            pedantic: self.pedantic,
            parallel: self.parallel
        }
    }


    /// Read the exr image from a file.
    /// Use [`ReadImage::read_from_unbuffered`] instead, if you do not have a file.
    #[inline]
    #[must_use]
    pub fn from_file<Layers>(self, path: impl AsRef<Path>) -> Result<Image<Layers>>
        where for<'s> L: ReadLayers<'s, Layers = Layers>
    {
        self.from_unbuffered(std::fs::File::open(path)?)
    }

    /// Buffer the reader and then read the exr image from it.
    /// Use [`ReadImage::read_from_buffered`] instead, if your reader is an in-memory reader.
    /// Use [`ReadImage::read_from_file`] instead, if you have a file path.
    #[inline]
    #[must_use]
    pub fn from_unbuffered<Layers>(self, unbuffered: impl Read + Seek) -> Result<Image<Layers>>
        where for<'s> L: ReadLayers<'s, Layers = Layers>
    {
        self.from_buffered(BufReader::new(unbuffered))
    }

    /// Read the exr image from a buffered reader.
    /// Use [`ReadImage::read_from_file`] instead, if you have a file path.
    /// Use [`ReadImage::read_from_unbuffered`] instead, if this is not an in-memory reader.
    // TODO Use Parallel<> Wrapper to only require sendable byte source where parallel decompression is required
    #[must_use]
    pub fn from_buffered<Layers>(self, buffered: impl Read + Seek) -> Result<Image<Layers>>
        where for<'s> L: ReadLayers<'s, Layers = Layers>
    {
        let chunks = crate::block::read(buffered, self.pedantic)?;
        self.from_chunks(chunks)
    }

    /// Read the exr image from an initialized chunks reader
    /// that has already extracted the meta data from the file.
    /// Use [`ReadImage::read_from_file`] instead, if you have a file path.
    /// Use [`ReadImage::read_from_buffered`] instead, if this is an in-memory reader.
    // TODO Use Parallel<> Wrapper to only require sendable byte source where parallel decompression is required
    #[must_use]
    pub fn from_chunks<Layers>(mut self, chunks_reader: crate::block::reader::Reader<impl Read + Seek>) -> Result<Image<Layers>>
        where for<'s> L: ReadLayers<'s, Layers = Layers>
    {
        let Self { pedantic, parallel, ref mut on_progress, ref mut read_layers } = self;

        let layers_reader = read_layers.create_layers_reader(chunks_reader.headers())?;
        let mut image_collector = ImageWithAttributesReader::new(chunks_reader.headers(), layers_reader)?;

        let block_reader = chunks_reader
            .filter_chunks(pedantic, |meta, tile, block| {
                image_collector.filter_block(meta, tile, block)
            })?
            .on_progress(on_progress);

        // TODO propagate send requirement further upwards
        if parallel {
            #[cfg(not(feature = "rayon"))]
            return Err(crate::error::Error::unsupported("parallel decompression requires the rayon feature"));

            #[cfg(feature = "rayon")]
            block_reader.decompress_parallel(pedantic, |meta_data, block|{
                image_collector.read_block(&meta_data.headers, block)
            })?;
        }
        else {
            block_reader.decompress_sequential(pedantic, |meta_data, block|{
                image_collector.read_block(&meta_data.headers, block)
            })?;
        }

        Ok(image_collector.into_image())
    }
}

/// Processes blocks from a file and collects them into a complete `Image`.
#[derive(Debug, Clone, PartialEq)]
pub struct ImageWithAttributesReader<L> {
    image_attributes: ImageAttributes,
    layers_reader: L,
}

impl<L> ImageWithAttributesReader<L> where L: LayersReader {

    /// A new image reader with image attributes.
    pub fn new(headers: &[Header], layers_reader: L) -> Result<Self>
    {
        Ok(ImageWithAttributesReader {
            image_attributes: headers.first().as_ref().expect("invalid headers").shared_attributes.clone(),
            layers_reader,
        })
    }

    /// Specify whether a single block of pixels should be loaded from the file
    fn filter_block(&self, meta: &MetaData, tile: TileCoordinates, block: BlockIndex) -> bool {
        self.layers_reader.filter_block(meta, tile, block)
    }

    /// Load a single pixel block, which has not been filtered, into the reader, accumulating the image
    fn read_block(&mut self, headers: &[Header], block: UncompressedBlock) -> UnitResult {
        self.layers_reader.read_block(headers, block)
    }

    /// Deliver the complete accumulated image
    fn into_image(self) -> Image<L::Layers> {
        Image {
            attributes: self.image_attributes,
            layer_data: self.layers_reader.into_layers()
        }
    }
}


/// A template that creates a `LayerReader` for each layer in the file.
pub trait ReadLayers<'s> {

    /// The type of the resulting Layers
    type Layers;

    /// The type of the temporary layer reader
    type Reader: LayersReader<Layers = Self::Layers>;

    /// Create a single reader for a single layer
    fn create_layers_reader(&'s self, headers: &[Header]) -> Result<Self::Reader>;

    /// Specify that all attributes should be read from an image.
    /// Use `from_file(path)` on the return value of this method to actually decode an image.
    fn all_attributes(self) -> ReadImage<fn(f64), Self> where Self: Sized {
        ReadImage::new(self, ignore_progress)
    }
}

/// Processes pixel blocks from a file and accumulates them into a single image layer.
pub trait LayersReader {

    /// The type of resulting layers
    type Layers;

    /// Specify whether a single block of pixels should be loaded from the file
    fn filter_block(&self, meta: &MetaData, tile: TileCoordinates, block: BlockIndex) -> bool;

    /// Load a single pixel block, which has not been filtered, into the reader, accumulating the layer
    fn read_block(&mut self, headers: &[Header], block: UncompressedBlock) -> UnitResult;

    /// Deliver the final accumulated layers for the image
    fn into_layers(self) -> Self::Layers;
}

