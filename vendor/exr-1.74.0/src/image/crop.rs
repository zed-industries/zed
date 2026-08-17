//! Crop away unwanted pixels. Includes automatic detection of bounding rectangle.
//! Currently does not support deep data and resolution levels.

use crate::meta::attribute::{IntegerBounds, LevelMode, ChannelList};
use crate::math::{Vec2, RoundingMode};
use crate::image::{Layer, FlatSamples, SpecificChannels, AnyChannels, FlatSamplesPixel, AnyChannel};
use crate::image::write::channels::{GetPixel, WritableChannels, ChannelsWriter};
use crate::meta::header::{LayerAttributes, Header};
use crate::block::BlockIndex;

/// Something that has a two-dimensional rectangular shape
pub trait GetBounds {

    /// The bounding rectangle of this pixel grid.
    fn bounds(&self) -> IntegerBounds;
}

/// Inspect the pixels in this image to determine where to crop some away
pub trait InspectSample: GetBounds {

    /// The type of pixel in this pixel grid.
    type Sample;

    /// Index is not in world coordinates, but within the data window.
    /// Position `(0,0)` always represents the top left pixel.
    fn inspect_sample(&self, local_index: Vec2<usize>) -> Self::Sample;
}

/// Crop some pixels ways when specifying a smaller rectangle
pub trait Crop: Sized {

    /// The type of  this image after cropping (probably the same as before)
    type Cropped;

    /// Crop the image to exclude unwanted pixels.
    /// Panics for invalid (larger than previously) bounds.
    /// The bounds are specified in absolute coordinates.
    /// Does not reduce allocation size of the current image, but instead only adjust a few boundary numbers.
    /// Use `reallocate_cropped()` on the return value to actually reduce the memory footprint.
    fn crop(self, bounds: IntegerBounds) -> Self::Cropped;

    /// Reduce your image to a smaller part, usually to save memory.
    /// Crop if bounds are specified, return the original if no bounds are specified.
    /// Does not reduce allocation size of the current image, but instead only adjust a few boundary numbers.
    /// Use `reallocate_cropped()` on the return value to actually reduce the memory footprint.
    fn try_crop(self, bounds: Option<IntegerBounds>) -> CropResult<Self::Cropped, Self> {
        match bounds {
            Some(bounds) => CropResult::Cropped(self.crop(bounds)),
            None => CropResult::Empty { original: self },
        }
    }
}

/// Cropping an image fails if the image is fully transparent.
/// Use [`or_crop_to_1x1_if_empty`] or [`or_none_if_empty`] to obtain a normal image again.
#[must_use]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CropResult<Cropped, Old> {

    /// The image contained some pixels and has been cropped or left untouched
    Cropped (Cropped),

    /// All pixels in the image would be discarded, removing the whole image
    Empty {

        /// The fully discarded image which caused the cropping to fail
        original: Old
    }
}

/// Crop away unwanted pixels from the border if they match the specified rule.
pub trait CropWhere<Sample>: Sized {

    /// The type of the cropped image (probably the same as the original image).
    type Cropped;

    /// Crop away unwanted pixels from the border if they match the specified rule.
    /// Does not reduce allocation size of the current image, but instead only adjust a few boundary numbers.
    /// Use `reallocate_cropped()` on the return value to actually reduce the memory footprint.
    fn crop_where(self, discard_if: impl Fn(Sample) -> bool) -> CropResult<Self::Cropped, Self>;

    /// Crop away unwanted pixels from the border if they match the specified color.
    /// If you want discard based on a rule, use `crop_where` with a closure instead.
    /// Does not reduce allocation size of the current image, but instead only adjust a few boundary numbers.
    /// Use `reallocate_cropped()` on the return value to actually reduce the memory footprint.
    fn crop_where_eq(self, discard_color: impl Into<Sample>) -> CropResult<Self::Cropped, Self> where Sample: PartialEq;

    /// Convert this data to cropped data without discarding any pixels.
    fn crop_nowhere(self) -> Self::Cropped;
}

impl<Channels> Crop for Layer<Channels> {
    type Cropped = Layer<CroppedChannels<Channels>>;

    fn crop(self, bounds: IntegerBounds) -> Self::Cropped {
        CroppedChannels::crop_layer(bounds, self)
    }
}

impl<T> CropWhere<T::Sample> for T where T: Crop + InspectSample {
    type Cropped = <Self as Crop>::Cropped;

    fn crop_where(self, discard_if: impl Fn(T::Sample) -> bool) -> CropResult<Self::Cropped, Self> {
        let smaller_bounds = {
            let keep_if = |position| !discard_if(self.inspect_sample(position));
            try_find_smaller_bounds(self.bounds(), keep_if)
        };

        self.try_crop(smaller_bounds)
    }

    fn crop_where_eq(self, discard_color: impl Into<T::Sample>) -> CropResult<Self::Cropped, Self> where T::Sample: PartialEq {
        let discard_color: T::Sample = discard_color.into();
        self.crop_where(|sample| sample == discard_color)
    }

    fn crop_nowhere(self) -> Self::Cropped {
        let current_bounds = self.bounds();
        self.crop(current_bounds)
    }
}

/// A smaller window into an existing pixel storage
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CroppedChannels<Channels> {

    /// The uncropped pixel storage
    pub full_channels: Channels,

    /// The uncropped pixel storage bounds
    pub full_bounds: IntegerBounds,

    /// The cropped pixel storage bounds
    pub cropped_bounds: IntegerBounds,
}

impl<Channels> CroppedChannels<Channels> {

    /// Wrap a layer in a cropped view with adjusted bounds, but without reallocating your pixels
    pub fn crop_layer(new_bounds: IntegerBounds, layer: Layer<Channels>) -> Layer<CroppedChannels<Channels>> {
        Layer {
            channel_data: CroppedChannels {
                cropped_bounds: new_bounds,
                full_bounds: layer.absolute_bounds(),
                full_channels: layer.channel_data,
            },

            size: new_bounds.size,

            attributes: LayerAttributes {
                layer_position: new_bounds.position,
                .. layer.attributes
            },

            encoding: layer.encoding
        }
    }
}

// TODO make cropped view readable if you only need a specific section of the image?

// make cropped view writable:

impl<'slf, Channels:'slf> WritableChannels<'slf> for CroppedChannels<Channels> where Channels: WritableChannels<'slf> {
    fn infer_channel_list(&self) -> ChannelList {
        self.full_channels.infer_channel_list() // no need for adjustments, as the layer content already reflects the changes
    }

    fn infer_level_modes(&self) -> (LevelMode, RoundingMode) {
        self.full_channels.infer_level_modes()
    }

    type Writer = CroppedWriter<Channels::Writer>;

    fn create_writer(&'slf self, header: &Header) -> Self::Writer {
        let offset = (self.cropped_bounds.position - self.full_bounds.position)
            .to_usize("invalid cropping bounds for cropped view").unwrap();

        CroppedWriter { channels: self.full_channels.create_writer(header), offset }
    }
}

/// A writer for the cropped view layer
#[derive(Debug, Clone, PartialEq)]
pub struct CroppedWriter<ChannelsWriter> {
    channels: ChannelsWriter,
    offset: Vec2<usize>
}

impl<'c, Channels> ChannelsWriter for CroppedWriter<Channels> where Channels: ChannelsWriter {
    fn extract_uncompressed_block(&self, header: &Header, block: BlockIndex) -> Vec<u8> {
        let block = BlockIndex {
            pixel_position: block.pixel_position + self.offset,
            .. block
        };

        self.channels.extract_uncompressed_block(header, block)
    }
}

impl<Samples, Channels> InspectSample for Layer<SpecificChannels<Samples, Channels>> where Samples: GetPixel {
    type Sample = Samples::Pixel;
    fn inspect_sample(&self, local_index: Vec2<usize>) -> Samples::Pixel {
        self.channel_data.pixels.get_pixel(local_index)
    }
}

impl InspectSample for Layer<AnyChannels<FlatSamples>> {
    type Sample = FlatSamplesPixel;

    fn inspect_sample(&self, local_index: Vec2<usize>) -> FlatSamplesPixel {
        self.sample_vec_at(local_index)
    }
}

// ALGORITHM IDEA: for arbitrary channels, find the most desired channel,
// and process that first, keeping the processed bounds as starting point for the other layers

/// Realize a cropped view of the original data,
/// by actually removing the unwanted original pixels,
/// reducing the memory consumption.
/// Currently not supported for `SpecificChannels`.
pub trait ApplyCroppedView {

    /// The simpler type after cropping is realized
    type Reallocated;

    /// Make the cropping real by reallocating the underlying storage,
    /// with the goal of reducing total memory usage.
    /// Currently not supported for `SpecificChannels`.
    fn reallocate_cropped(self) -> Self::Reallocated;
}

impl ApplyCroppedView for Layer<CroppedChannels<AnyChannels<FlatSamples>>> {
    type Reallocated = Layer<AnyChannels<FlatSamples>>;

    fn reallocate_cropped(self) -> Self::Reallocated {
        let cropped_absolute_bounds = self.channel_data.cropped_bounds;
        let cropped_relative_bounds = cropped_absolute_bounds.with_origin(-self.channel_data.full_bounds.position);

        assert!(self.absolute_bounds().contains(cropped_absolute_bounds), "bounds not valid for layer dimensions");
        assert!(cropped_relative_bounds.size.area() > 0, "the cropped image would be empty");

        Layer {
            channel_data: if cropped_relative_bounds.size == self.channel_data.full_bounds.size {
                assert_eq!(cropped_absolute_bounds.position, self.channel_data.full_bounds.position, "crop bounds size equals, but position does not");

                // the cropping would not remove any pixels
                self.channel_data.full_channels
            }
            else {
                let start_x = cropped_relative_bounds.position.x() as usize; // safe, because just checked above
                let start_y = cropped_relative_bounds.position.y() as usize; // safe, because just checked above
                let x_range = start_x .. start_x + cropped_relative_bounds.size.width();
                let old_width = self.channel_data.full_bounds.size.width();
                let new_height = cropped_relative_bounds.size.height();

                let channels = self.channel_data.full_channels.list.into_iter().map(|channel: AnyChannel<FlatSamples>| {
                    fn crop_samples<T:Copy>(samples: Vec<T>, old_width: usize, new_height: usize, x_range: std::ops::Range<usize>, y_start: usize) -> Vec<T> {
                        let filtered_lines = samples.chunks_exact(old_width).skip(y_start).take(new_height);
                        let trimmed_lines = filtered_lines.map(|line| &line[x_range.clone()]);
                        trimmed_lines.flatten().map(|x|*x).collect() // TODO does this use memcpy?
                    }

                    let samples = match channel.sample_data {
                        FlatSamples::F16(samples) => FlatSamples::F16(crop_samples(
                            samples, old_width, new_height, x_range.clone(), start_y
                        )),

                        FlatSamples::F32(samples) => FlatSamples::F32(crop_samples(
                            samples, old_width, new_height, x_range.clone(), start_y
                        )),

                        FlatSamples::U32(samples) => FlatSamples::U32(crop_samples(
                            samples, old_width, new_height, x_range.clone(), start_y
                        )),
                    };

                    AnyChannel { sample_data: samples, ..channel }
                }).collect();

                AnyChannels { list: channels }
            },

            attributes: self.attributes,
            encoding: self.encoding,
            size: self.size,
        }
    }
}



/// Return the smallest bounding rectangle including all pixels that satisfy the predicate.
/// Worst case: Fully transparent image, visits each pixel once.
/// Best case: Fully opaque image, visits two pixels.
/// Returns `None` if the image is fully transparent.
/// Returns `[(0,0), size]` if the image is fully opaque.
/// Designed to be cache-friendly linear search. Optimized for row-major image vectors.
pub fn try_find_smaller_bounds(current_bounds: IntegerBounds, pixel_at: impl Fn(Vec2<usize>) -> bool) -> Option<IntegerBounds> {
    assert_ne!(current_bounds.size.area(), 0, "cannot find smaller bounds of an image with zero width or height");
    let Vec2(width, height) = current_bounds.size;

    // scans top to bottom (left to right)
    let first_top_left_pixel = (0 .. height)
        .flat_map(|y| (0 .. width).map(move |x| Vec2(x,y)))
        .find(|&position| pixel_at(position))?; // return none if no pixel should be kept

    // scans bottom to top (right to left)
    let first_bottom_right_pixel = (first_top_left_pixel.y() + 1 .. height) // excluding the top line
        .flat_map(|y| (0 .. width).map(move |x| Vec2(x, y))) // x search cannot start at first_top.x, because this must catch all bottom pixels
        .rev().find(|&position| pixel_at(position))
        .unwrap_or(first_top_left_pixel); // did not find any at bottom, but we know top has some pixel

    // now we know exactly how much we can throw away top and bottom,
    // but we don't know exactly about left or right
    let top = first_top_left_pixel.y();
    let bottom = first_bottom_right_pixel.y();

    // we only now some arbitrary left and right bounds which we need to refine.
    // because the actual image contents might be wider than the corner points.
    // we know that we do not need to look in the center between min x and max x,
    // as these must be included in any case.
    let mut min_left_x = first_top_left_pixel.x().min(first_bottom_right_pixel.x());
    let mut max_right_x = first_bottom_right_pixel.x().max(first_top_left_pixel.x());

    // requires for loop, because bounds change while searching
    for y in top ..= bottom {

        // escape the loop if there is nothing left to crop
        if min_left_x == 0 && max_right_x == width - 1 { break; }

        // search from right image edge towards image center, until known max x, for existing pixels,
        // possibly including some pixels that would have been cropped otherwise
        if max_right_x != width - 1 {
            max_right_x = (max_right_x + 1 .. width).rev() // excluding current max
                .find(|&x| pixel_at(Vec2(x, y)))
                .unwrap_or(max_right_x);
        }

        // search from left image edge towards image center, until known min x, for existing pixels,
        // possibly including some pixels that would have been cropped otherwise
        if min_left_x != 0 {
            min_left_x = (0 .. min_left_x) // excluding current min
                .find(|&x| pixel_at(Vec2(x, y)))
                .unwrap_or(min_left_x);
        }
    }

    // TODO add 1px margin to avoid interpolation issues?
    let local_start = Vec2(min_left_x, top);
    let local_end = Vec2(max_right_x + 1, bottom + 1);
    Some(IntegerBounds::new(
        current_bounds.position + local_start.to_i32(),
        local_end - local_start
    ))
}

impl<S> GetBounds for Layer<S> {
    fn bounds(&self) -> IntegerBounds {
        self.absolute_bounds()
    }
}

impl<Cropped, Original> CropResult<Cropped, Original> {

    /// If the image was fully empty, return `None`, otherwise return `Some(cropped_image)`.
    pub fn or_none_if_empty(self) -> Option<Cropped> {
        match self {
            CropResult::Cropped (cropped) => Some(cropped),
            CropResult::Empty { .. } => None,
        }
    }

    /// If the image was fully empty, crop to one single pixel of all the transparent pixels instead,
    /// leaving the layer intact while reducing memory usage.
    pub fn or_crop_to_1x1_if_empty(self) -> Cropped where Original: Crop<Cropped=Cropped> + GetBounds {
        match self {
            CropResult::Cropped (cropped) => cropped,
            CropResult::Empty { original } => {
                let bounds = original.bounds();
                if bounds.size == Vec2(0,0) { panic!("layer has width and height of zero") }
                original.crop(IntegerBounds::new(bounds.position, Vec2(1,1)))
            },
        }
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_bounds() {
        fn find_bounds(offset: Vec2<i32>, lines: &Vec<Vec<i32>>) -> IntegerBounds {
            if let Some(first_line) = lines.first() {
                assert!(lines.iter().all(|line| line.len() == first_line.len()), "invalid test input");
                IntegerBounds::new(offset, (first_line.len(), lines.len()))
            }
            else {
                IntegerBounds::new(offset, (0,0))
            }
        }

        fn assert_found_smaller_bounds(offset: Vec2<i32>, uncropped_lines: Vec<Vec<i32>>, expected_cropped_lines: Vec<Vec<i32>>) {
            let old_bounds = find_bounds(offset, &uncropped_lines);

            let found_bounds = try_find_smaller_bounds(
                old_bounds,
                |position| uncropped_lines[position.y()][position.x()] != 0
            ).unwrap();

            let found_bounds = found_bounds.with_origin(-offset); // make indices local

            let cropped_lines: Vec<Vec<i32>> =
                uncropped_lines[found_bounds.position.y() as usize .. found_bounds.end().y() as usize]
                .iter().map(|uncropped_line|{
                    uncropped_line[found_bounds.position.x() as usize .. found_bounds.end().x() as usize].to_vec()
                }).collect();

            assert_eq!(cropped_lines, expected_cropped_lines);
        }

        assert_found_smaller_bounds(
            Vec2(-3,-3),

            vec![
                vec![ 2, 3, 4 ],
                vec![ 2, 3, 4 ],
            ],

            vec![
                vec![ 2, 3, 4 ],
                vec![ 2, 3, 4 ],
            ]
        );

        assert_found_smaller_bounds(
            Vec2(-3,-3),

            vec![
                vec![ 2 ],
            ],

            vec![
                vec![ 2 ],
            ]
        );

        assert_found_smaller_bounds(
            Vec2(-3,-3),

            vec![
                vec![ 0 ],
                vec![ 2 ],
                vec![ 0 ],
                vec![ 0 ],
            ],

            vec![
                vec![ 2 ],
            ]
        );

        assert_found_smaller_bounds(
            Vec2(-3,-3),

            vec![
                vec![ 0, 0, 0, 3, 0 ],
            ],

            vec![
                vec![ 3 ],
            ]
        );

        assert_found_smaller_bounds(
            Vec2(3,3),

            vec![
                vec![ 0, 1, 1, 2, 1, 0 ],
                vec![ 0, 1, 3, 1, 1, 0 ],
                vec![ 0, 1, 1, 1, 1, 0 ],
            ],

            vec![
                vec![ 1, 1, 2, 1 ],
                vec![ 1, 3, 1, 1 ],
                vec![ 1, 1, 1, 1 ],
            ]
        );

        assert_found_smaller_bounds(
            Vec2(3,3),

            vec![
                vec![ 0, 0, 0, 0 ],
                vec![ 1, 1, 2, 1 ],
                vec![ 1, 3, 1, 1 ],
                vec![ 1, 1, 1, 1 ],
                vec![ 0, 0, 0, 0 ],
            ],

            vec![
                vec![ 1, 1, 2, 1 ],
                vec![ 1, 3, 1, 1 ],
                vec![ 1, 1, 1, 1 ],
            ]
        );

        assert_found_smaller_bounds(
            Vec2(3,3),

            vec![
                vec![ 0, 1, 1, 2, 1, 0 ],
                vec![ 0, 0, 3, 1, 0, 0 ],
                vec![ 0, 1, 1, 1, 1, 0 ],
            ],

            vec![
                vec![ 1, 1, 2, 1 ],
                vec![ 0, 3, 1, 0 ],
                vec![ 1, 1, 1, 1 ],
            ]
        );

        assert_found_smaller_bounds(
            Vec2(3,3),

            vec![
                vec![ 0, 0, 1, 2, 0, 0 ],
                vec![ 0, 1, 3, 1, 1, 0 ],
                vec![ 0, 0, 1, 1, 0, 0 ],
            ],

            vec![
                vec![ 0, 1, 2, 0 ],
                vec![ 1, 3, 1, 1 ],
                vec![ 0, 1, 1, 0 ],
            ]
        );

        assert_found_smaller_bounds(
            Vec2(1,3),

            vec![
                vec![ 1, 0, 0, 0, ],
                vec![ 0, 0, 0, 0, ],
                vec![ 0, 0, 0, 0, ],
            ],

            vec![
                vec![ 1 ],
            ]
        );

        assert_found_smaller_bounds(
            Vec2(1,3),

            vec![
                vec![ 0, 0, 0, 0, ],
                vec![ 0, 1, 0, 0, ],
                vec![ 0, 0, 0, 0, ],
            ],

            vec![
                vec![ 1 ],
            ]
        );

        assert_found_smaller_bounds(
            Vec2(-1,-3),

            vec![
                vec![ 0, 0, 0, 0, ],
                vec![ 0, 0, 0, 1, ],
                vec![ 0, 0, 0, 0, ],
            ],

            vec![
                vec![ 1 ],
            ]
        );

        assert_found_smaller_bounds(
            Vec2(-1,-3),

            vec![
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
                vec![ 0, 0, 1, 1, 1, 0, 0 ],
                vec![ 0, 0, 1, 1, 1, 0, 0 ],
                vec![ 0, 0, 1, 1, 1, 0, 0 ],
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
            ],

            vec![
                vec![ 1, 1, 1 ],
                vec![ 1, 1, 1 ],
                vec![ 1, 1, 1 ],
            ]
        );

        assert_found_smaller_bounds(
            Vec2(1000,-300),

            vec![
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
                vec![ 0, 0, 1, 1, 1, 0, 0 ],
                vec![ 0, 1, 1, 1, 1, 1, 0 ],
                vec![ 0, 0, 1, 1, 1, 0, 0 ],
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
            ],

            vec![
                vec![ 0, 1, 1, 1, 0 ],
                vec![ 1, 1, 1, 1, 1 ],
                vec![ 0, 1, 1, 1, 0 ],
            ]
        );

        assert_found_smaller_bounds(
            Vec2(-10,-300),

            vec![
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
                vec![ 0, 0, 1, 0, 1, 0, 0 ],
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
                vec![ 0, 0, 1, 0, 1, 0, 0 ],
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
            ],

            vec![
                vec![ 1, 0, 1 ],
                vec![ 0, 0, 0 ],
                vec![ 1, 0, 1 ],
            ]
        );

        assert_found_smaller_bounds(
            Vec2(-10,-300),

            vec![
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
                vec![ 0, 0, 1, 0, 1, 0, 0 ],
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
            ],

            vec![
                vec![ 1, 0, 1 ],
            ]
        );

        assert_found_smaller_bounds(
            Vec2(-10,-300),

            vec![
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
                vec![ 0, 0, 0, 1, 0, 0, 0 ],
                vec![ 0, 0, 0, 2, 0, 0, 0 ],
                vec![ 0, 0, 3, 3, 3, 0, 0 ],
                vec![ 0, 0, 0, 4, 0, 0, 0 ],
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
            ],

            vec![
                vec![ 0, 1, 0 ],
                vec![ 0, 2, 0 ],
                vec![ 3, 3, 3 ],
                vec![ 0, 4, 0 ],
            ]
        );

        assert_found_smaller_bounds(
            Vec2(-10,-300),

            vec![
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
                vec![ 0, 0, 0, 0, 1, 0, 0 ],
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
                vec![ 0, 0, 1, 0, 0, 0, 0 ],
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
            ],

            vec![
                vec![ 0, 0, 1 ],
                vec![ 0, 0, 0 ],
                vec![ 0, 0, 0 ],
                vec![ 1, 0, 0 ],
            ]
        );

        assert_found_smaller_bounds(
            Vec2(-10,-300),

            vec![
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
                vec![ 0, 0, 1, 0, 0, 0, 0 ],
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
                vec![ 0, 0, 0, 0, 0, 1, 0 ],
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
            ],

            vec![
                vec![ 1, 0, 0, 0 ],
                vec![ 0, 0, 0, 0 ],
                vec![ 0, 0, 0, 1 ],
            ]
        );

        assert_found_smaller_bounds(
            Vec2(-10,-300),

            vec![
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
                vec![ 0, 0, 1, 0, 0, 0, 0 ],
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
                vec![ 0, 0, 1, 0, 0, 0, 0 ],
                vec![ 0, 0, 0, 0, 0, 0, 0 ],
            ],

            vec![
                vec![ 1 ],
                vec![ 0 ],
                vec![ 0 ],
                vec![ 1 ],
            ]
        );


        assert_found_smaller_bounds(
            Vec2(-1,-3),

            vec![
                vec![ 0, 0, 1, 0, ],
                vec![ 0, 0, 0, 1, ],
                vec![ 0, 0, 0, 0, ],
            ],

            vec![
                vec![ 1, 0, ],
                vec![ 0, 1, ],
            ]
        );

        assert_found_smaller_bounds(
            Vec2(-1,-3),

            vec![
                vec![ 1, 0, 0, 0, ],
                vec![ 0, 1, 0, 0, ],
                vec![ 0, 0, 0, 0, ],
                vec![ 0, 0, 0, 0, ],
            ],

            vec![
                vec![ 1, 0, ],
                vec![ 0, 1, ],
            ]
        );
    }


    #[test]
    fn find_no_bounds() {
        let pixels = vec![
            vec![ 0, 0, 0, 0 ],
            vec![ 0, 0, 0, 0 ],
            vec![ 0, 0, 0, 0 ],
        ];

        let bounds = try_find_smaller_bounds(
            IntegerBounds::new((0,0), (4,3)),
            |position| pixels[position.y()][position.x()] != 0
        );

        assert_eq!(bounds, None)
    }

}




