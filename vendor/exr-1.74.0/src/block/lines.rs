//! Extract lines from a block of pixel bytes.

use crate::math::*;
use std::io::{Cursor};
use crate::error::{Result, UnitResult};
use smallvec::SmallVec;
use std::ops::Range;
use crate::block::{BlockIndex};
use crate::meta::attribute::ChannelList;


/// A single line of pixels.
/// Use [LineRef] or [LineRefMut] for easier type names.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct LineSlice<T> {

    // TODO also store enum SampleType, as it would always be matched in every place it is used

    /// Where this line is located inside the image.
    pub location: LineIndex,

    /// The raw bytes of the pixel line, either `&[u8]` or `&mut [u8]`.
    /// Must be re-interpreted as slice of f16, f32, or u32,
    /// according to the channel data type.
    pub value: T,
}


/// An reference to a single line of pixels.
/// May go across the whole image or just a tile section of it.
///
/// This line contains an immutable slice that all samples will be read from.
pub type LineRef<'s> = LineSlice<&'s [u8]>;

/// A reference to a single mutable line of pixels.
/// May go across the whole image or just a tile section of it.
///
/// This line contains a mutable slice that all samples will be written to.
pub type LineRefMut<'s> = LineSlice<&'s mut [u8]>;


/// Specifies where a row of pixels lies inside an image.
/// This is a globally unique identifier which includes
/// the layer, channel index, and pixel location.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct LineIndex {

    /// Index of the layer.
    pub layer: usize,

    /// The channel index of the layer.
    pub channel: usize,

    /// Index of the mip or rip level in the image.
    pub level: Vec2<usize>,

    /// Position of the most left pixel of the row.
    pub position: Vec2<usize>,

    /// The width of the line; the number of samples in this row,
    /// that is, the number of f16, f32, or u32 values.
    pub sample_count: usize,
}


impl LineIndex {

    /// Iterates the lines of this block index in interleaved fashion:
    /// For each line in this block, this iterator steps once through each channel.
    /// This is how lines are stored in a pixel data block.
    ///
    /// Does not check whether `self.layer_index`, `self.level`, `self.size` and `self.position` are valid indices.__
    // TODO be sure this cannot produce incorrect data, as this is not further checked but only handled with panics
    #[inline]
    #[must_use]
    pub fn lines_in_block(block: BlockIndex, channels: &ChannelList) -> impl Iterator<Item=(Range<usize>, LineIndex)> {
        struct LineIter {
            layer: usize, level: Vec2<usize>, width: usize,
            end_y: usize, x: usize, channel_sizes: SmallVec<[usize; 8]>,
            byte: usize, channel: usize, y: usize,
        }

        // FIXME what about sub sampling??

        impl Iterator for LineIter {
            type Item = (Range<usize>, LineIndex);
            // TODO size hint?

            fn next(&mut self) -> Option<Self::Item> {
                if self.y < self.end_y {

                    // compute return value before incrementing
                    let byte_len = self.channel_sizes[self.channel];
                    let return_value = (
                        (self.byte .. self.byte + byte_len),
                        LineIndex {
                            channel: self.channel,
                            layer: self.layer,
                            level: self.level,
                            position: Vec2(self.x, self.y),
                            sample_count: self.width,
                        }
                    );

                    { // increment indices
                        self.byte += byte_len;
                        self.channel += 1;

                        if self.channel == self.channel_sizes.len() {
                            self.channel = 0;
                            self.y += 1;
                        }
                    }

                    Some(return_value)
                }

                else {
                    None
                }
            }
        }

        let channel_line_sizes: SmallVec<[usize; 8]> = channels.list.iter()
            .map(move |channel| block.pixel_size.0 * channel.sample_type.bytes_per_sample()) // FIXME is it fewer samples per tile or just fewer tiles for sampled images???
            .collect();

        LineIter {
            layer: block.layer,
            level: block.level,
            width: block.pixel_size.0,
            x: block.pixel_position.0,
            end_y: block.pixel_position.y() + block.pixel_size.height(),
            channel_sizes: channel_line_sizes,

            byte: 0,
            channel: 0,
            y: block.pixel_position.y()
        }
    }
}



impl<'s> LineRefMut<'s> {

    /// Writes the samples (f16, f32, u32 values) into this line value reference.
    /// Use `write_samples` if there is no slice available.
    #[inline]
    #[must_use]
    pub fn write_samples_from_slice<T: crate::io::Data>(self, slice: &[T]) -> UnitResult {
        debug_assert_eq!(slice.len(), self.location.sample_count, "slice size does not match the line width");
        debug_assert_eq!(self.value.len(), self.location.sample_count * T::BYTE_SIZE, "sample type size does not match line byte size");

        T::write_slice_ne(&mut Cursor::new(self.value), slice)
    }

    /// Iterate over all samples in this line, from left to right.
    /// The supplied `get_line` function returns the sample value
    /// for a given sample index within the line,
    /// which starts at zero for each individual line.
    /// Use `write_samples_from_slice` if you already have a slice of samples.
    #[inline]
    #[must_use]
    pub fn write_samples<T: crate::io::Data>(self, mut get_sample: impl FnMut(usize) -> T) -> UnitResult {
        debug_assert_eq!(self.value.len(), self.location.sample_count * T::BYTE_SIZE, "sample type size does not match line byte size");

        let mut write = Cursor::new(self.value);

        for index in 0..self.location.sample_count {
            T::write_ne(get_sample(index), &mut write)?;
        }

        Ok(())
    }
}

impl LineRef<'_> {

    /// Read the samples (f16, f32, u32 values) from this line value reference.
    /// Use `read_samples` if there is not slice available.
    pub fn read_samples_into_slice<T: crate::io::Data>(self, slice: &mut [T]) -> UnitResult {
        debug_assert_eq!(slice.len(), self.location.sample_count, "slice size does not match the line width");
        debug_assert_eq!(self.value.len(), self.location.sample_count * T::BYTE_SIZE, "sample type size does not match line byte size");

        T::read_slice_ne(&mut Cursor::new(self.value), slice)
    }

    /// Iterate over all samples in this line, from left to right.
    /// Use `read_sample_into_slice` if you already have a slice of samples.
    pub fn read_samples<T: crate::io::Data>(&self) -> impl Iterator<Item = Result<T>> + '_ {
        debug_assert_eq!(self.value.len(), self.location.sample_count * T::BYTE_SIZE, "sample type size does not match line byte size");

        let mut read = self.value; // FIXME deep data
        (0..self.location.sample_count).map(move |_| T::read_ne(&mut read))
    }
}