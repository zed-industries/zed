
//! Write an exr image to a file.
//!
//! First, call `my_image.write()`. The resulting value can be customized, like this:
//! ```no_run
//!     use exr::prelude::*;
//! #   let my_image: FlatImage = unimplemented!();
//!
//!     my_image.write()
//!            .on_progress(|progress| println!("progress: {:.1}", progress*100.0))
//!            .to_file("image.exr").unwrap();
//! ```
//!

pub mod layers;
pub mod samples;
pub mod channels;



use crate::meta::Headers;
use crate::error::UnitResult;
use std::io::{Seek, BufWriter};
use crate::io::Write;
use crate::image::{Image, ignore_progress, SpecificChannels, IntoSample};
use crate::image::write::layers::{WritableLayers, LayersWriter};
use crate::math::Vec2;
use crate::block::writer::ChunksWriter;

/// An oversimplified function for "just write the damn file already" use cases.
/// Have a look at the examples to see how you can write an image with more flexibility (it's not that hard).
/// Use `write_rgb_file` if you do not need an alpha channel.
///
/// Each of `R`, `G`, `B` and `A` can be either `f16`, `f32`, `u32`, or `Sample`.
// TODO explain pixel tuple f32,f16,u32
pub fn write_rgba_file<R,G,B,A>(
    path: impl AsRef<std::path::Path>, width: usize, height: usize,
    colors: impl Sync + Fn(usize, usize) -> (R, G, B, A)
) -> UnitResult
    where R: IntoSample, G: IntoSample, B: IntoSample, A: IntoSample,
{
    let channels = SpecificChannels::rgba(|Vec2(x,y)| colors(x,y));
    Image::from_channels((width, height), channels).write().to_file(path)
}

/// An oversimplified function for "just write the damn file already" use cases.
/// Have a look at the examples to see how you can write an image with more flexibility (it's not that hard).
/// Use `write_rgb_file` if you do not need an alpha channel.
///
/// Each of `R`, `G`, and `B` can be either `f16`, `f32`, `u32`, or `Sample`.
// TODO explain pixel tuple f32,f16,u32
pub fn write_rgb_file<R,G,B>(
    path: impl AsRef<std::path::Path>, width: usize, height: usize,
    colors: impl Sync + Fn(usize, usize) -> (R, G, B)
) -> UnitResult
    where R: IntoSample, G: IntoSample, B: IntoSample
{
    let channels = SpecificChannels::rgb(|Vec2(x,y)| colors(x,y));
    Image::from_channels((width, height), channels).write().to_file(path)
}



/// Enables an image to be written to a file. Call `image.write()` where this trait is implemented.
pub trait WritableImage<'img, WritableLayers>: Sized {

    /// Create a temporary writer which can be configured and used to write the image to a file.
    fn write(self) -> WriteImageWithOptions<'img, WritableLayers, fn(f64)>;
}

impl<'img, WritableLayers> WritableImage<'img, WritableLayers> for &'img Image<WritableLayers> {
    fn write(self) -> WriteImageWithOptions<'img, WritableLayers, fn(f64)> {
        WriteImageWithOptions {
            image: self,
            check_compatibility: true,

            #[cfg(not(feature = "rayon"))]
            parallel: false,

            #[cfg(feature = "rayon")]
            parallel: true,

            on_progress: ignore_progress
        }
    }
}

/// A temporary writer which can be configured and used to write an image to a file.
// temporary writer with options
#[derive(Debug, Clone, PartialEq)]
pub struct WriteImageWithOptions<'img, Layers, OnProgress> {
    image: &'img Image<Layers>,
    on_progress: OnProgress,
    check_compatibility: bool,
    parallel: bool,
}


impl<'img, L, F> WriteImageWithOptions<'img, L, F>
    where L: WritableLayers<'img>, F: FnMut(f64)
{
    /// Generate file meta data for this image. The meta data structure is close to the data in the file.
    pub fn infer_meta_data(&self) -> Headers { // TODO this should perform all validity checks? and none after that?
        self.image.layer_data.infer_headers(&self.image.attributes)
    }

    /// Do not compress multiple pixel blocks on multiple threads at once.
    /// Might use less memory and synchronization, but will be slower in most situations.
    pub fn non_parallel(self) -> Self { Self { parallel: false, ..self } }

    /// Skip some checks that ensure a file can be opened by other exr software.
    /// For example, it is no longer checked that no two headers or two attributes have the same name,
    /// which might be an expensive check for images with an exorbitant number of headers.
    ///
    /// If you write an uncompressed file and need maximum speed, it might save a millisecond to disable the checks,
    /// if you know that your file is not invalid any ways. I do not recommend this though,
    /// as the file might not be readably by any other exr library after that.
    /// __You must care for not producing an invalid file yourself.__
    pub fn skip_compatibility_checks(self) -> Self { Self { check_compatibility: false, ..self } }

    /// Specify a function to be called regularly throughout the writing process.
    /// Replaces all previously specified progress functions in this reader.
    pub fn on_progress<OnProgress>(self, on_progress: OnProgress) -> WriteImageWithOptions<'img, L, OnProgress>
        where OnProgress: FnMut(f64)
    {
        WriteImageWithOptions {
            on_progress,
            image: self.image,
            check_compatibility: self.check_compatibility,
            parallel: self.parallel
        }
    }

    /// Write the exr image to a file.
    /// Use `to_unbuffered` instead, if you do not have a file.
    /// If an error occurs, attempts to delete the partially written file.
    #[inline]
    #[must_use]
    pub fn to_file(self, path: impl AsRef<std::path::Path>) -> UnitResult {
        crate::io::attempt_delete_file_on_write_error(path.as_ref(), move |write|
            self.to_unbuffered(write)
        )
    }

    /// Buffer the writer and then write the exr image to it.
    /// Use `to_buffered` instead, if your writer is an in-memory buffer.
    /// Use `to_file` instead, if you have a file path.
    /// If your writer cannot seek, you can write to an in-memory vector of bytes first, using `to_buffered`.
    #[inline]
    #[must_use]
    pub fn to_unbuffered(self, unbuffered: impl Write + Seek) -> UnitResult {
        self.to_buffered(BufWriter::new(unbuffered))
    }

    /// Write the exr image to a writer.
    /// Use `to_file` instead, if you have a file path.
    /// Use `to_unbuffered` instead, if this is not an in-memory writer.
    /// If your writer cannot seek, you can write to an in-memory vector of bytes first.
    #[must_use]
    pub fn to_buffered(self, write: impl Write + Seek) -> UnitResult {
        let headers = self.infer_meta_data();
        let layers = self.image.layer_data.create_writer(&headers);

        crate::block::write(
            write, headers, self.check_compatibility,
            move |meta, chunk_writer|{
                let blocks = meta.collect_ordered_block_data(|block_index|
                     layers.extract_uncompressed_block(&meta.headers, block_index)
                );

                let chunk_writer = chunk_writer.on_progress(self.on_progress);
                if self.parallel {
                    #[cfg(not(feature = "rayon"))]
                    return Err(crate::error::Error::unsupported("parallel compression requires the rayon feature"));

                    #[cfg(feature = "rayon")]
                    chunk_writer.compress_all_blocks_parallel(&meta, blocks)?;
                }
                else { chunk_writer.compress_all_blocks_sequential(&meta, blocks)?; }
                Ok(())
            }
        )
    }
}

