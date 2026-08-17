//! How to read arbitrary channels and rgb channels.

use crate::prelude::*;
use crate::io::*;
use crate::math::*;
use crate::meta::{header::*, attribute::*};
use crate::block::*;
use crate::image::recursive::*;
use crate::block::samples::*;
use crate::image::write::samples::*;

use std::marker::PhantomData;


/// Enables an image containing this list of channels to be written to a file.
pub trait WritableChannels<'slf> {

    /// Generate the file meta data for this list of channel
    fn infer_channel_list(&self) -> ChannelList;

    ///  Generate the file meta data of whether and how resolution levels should be stored in the file
    fn infer_level_modes(&self) -> (LevelMode, RoundingMode);

    /// The type of temporary writer
    type Writer: ChannelsWriter;

    /// Create a temporary writer for this list of channels
    fn create_writer(&'slf self, header: &Header) -> Self::Writer;
}

/// A temporary writer for a list of channels
pub trait ChannelsWriter: Sync {

    /// Deliver a block of pixels, containing all channel data, to be stored in the file
    fn extract_uncompressed_block(&self, header: &Header, block: BlockIndex) -> Vec<u8>; // TODO return uncompressed block?
}


/// Define how to get a pixel from your custom pixel storage.
/// Can be a closure of type [`Sync + Fn(Vec2<usize>) -> YourPixel`].
pub trait GetPixel: Sync {

    /// The pixel tuple containing `f32`, `f16`, `u32` and `Sample` values.
    /// The length of the tuple must match the number of channels in the image.
    type Pixel;

    /// Inspect a single pixel at the requested position.
    /// Will be called exactly once for each pixel in the image.
    /// The position will not exceed the image dimensions.
    /// Might be called from multiple threads at the same time.
    fn get_pixel(&self, position: Vec2<usize>) -> Self::Pixel;
}

impl<F, P> GetPixel for F where F: Sync + Fn(Vec2<usize>) -> P {
    type Pixel = P;
    fn get_pixel(&self, position: Vec2<usize>) -> P { self(position) }
}

impl<'samples, Samples> WritableChannels<'samples> for AnyChannels<Samples>
    where Samples: 'samples + WritableSamples<'samples>
{
    fn infer_channel_list(&self) -> ChannelList {
        ChannelList::new(self.list.iter().map(|channel| ChannelDescription {
            name: channel.name.clone(),
            sample_type: channel.sample_data.sample_type(),
            quantize_linearly: channel.quantize_linearly,
            sampling: channel.sampling
        }).collect())
    }

    fn infer_level_modes(&self) -> (LevelMode, RoundingMode) {
        let mode = self.list.iter().next().expect("zero channels in list").sample_data.infer_level_modes();

        debug_assert!(
            std::iter::repeat(mode).zip(self.list.iter().skip(1))
                .all(|(first, other)| other.sample_data.infer_level_modes() == first),

            "level mode must be the same across all levels (do not nest resolution levels!)"
        );

        mode
    }

    type Writer = AnyChannelsWriter<Samples::Writer>;
    fn create_writer(&'samples self, header: &Header) -> Self::Writer {
        let channels = self.list.iter()
            .map(|chan| chan.sample_data.create_samples_writer(header))
            .collect();

        AnyChannelsWriter { channels }
    }
}

/// A temporary writer for an arbitrary list of channels
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AnyChannelsWriter<SamplesWriter> {
    channels: SmallVec<[SamplesWriter; 4]>
}

impl<Samples> ChannelsWriter for AnyChannelsWriter<Samples> where Samples: SamplesWriter {
    fn extract_uncompressed_block(&self, header: &Header, block_index: BlockIndex) -> Vec<u8> {
        UncompressedBlock::collect_block_data_from_lines(&header.channels, block_index, |line_ref| {
            self.channels[line_ref.location.channel].extract_line(line_ref)
        })
    }
}






impl<'c, Channels, Storage>
WritableChannels<'c> for SpecificChannels<Storage, Channels>
where
    Storage: 'c + GetPixel,
    Storage::Pixel: IntoRecursive,
    Channels: 'c + Sync + Clone + IntoRecursive,
    <Channels as IntoRecursive>::Recursive: WritableChannelsDescription<<Storage::Pixel as IntoRecursive>::Recursive>,
{
    fn infer_channel_list(&self) -> ChannelList {
        let mut vec = self.channels.clone().into_recursive().channel_descriptions_list();
        vec.sort_unstable_by_key(|channel:&ChannelDescription| channel.name.clone()); // TODO no clone?

        debug_assert!(
            // check for equal neighbors in sorted vec
            vec.iter().zip(vec.iter().skip(1)).all(|(prev, next)| prev.name != next.name),
            "specific channels contain duplicate channel names"
        );

        ChannelList::new(vec)
    }

    fn infer_level_modes(&self) -> (LevelMode, RoundingMode) {
        (LevelMode::Singular, RoundingMode::Down) // TODO
    }

    type Writer = SpecificChannelsWriter<
        'c,
        <<Channels as IntoRecursive>::Recursive as WritableChannelsDescription<<Storage::Pixel as IntoRecursive>::Recursive>>::RecursiveWriter,
        Storage,
        Channels
    >;

    fn create_writer(&'c self, header: &Header) -> Self::Writer {
        SpecificChannelsWriter {
            channels: self,
            recursive_channel_writer: self.channels.clone().into_recursive().create_recursive_writer(&header.channels),
        }
    }
}



/// A temporary writer for a layer of channels, alpha being optional
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SpecificChannelsWriter<'channels, PixelWriter, Storage, Channels> {
    channels: &'channels SpecificChannels<Storage, Channels>, // TODO this need not be a reference?? impl writer for specific_channels directly?
    recursive_channel_writer: PixelWriter,
}


impl<'channels, PxWriter, Storage, Channels> ChannelsWriter
for SpecificChannelsWriter<'channels, PxWriter, Storage, Channels>
    where
        Channels: Sync,
        Storage: GetPixel,
        Storage::Pixel: IntoRecursive,
        PxWriter: Sync + RecursivePixelWriter<<Storage::Pixel as IntoRecursive>::Recursive>,
{
    fn extract_uncompressed_block(&self, header: &Header, block_index: BlockIndex) -> Vec<u8> {
        let block_bytes = block_index.pixel_size.area() * header.channels.bytes_per_pixel;
        let mut block_bytes = vec![0_u8; block_bytes];

        let width = block_index.pixel_size.0;
        let line_bytes = width * header.channels.bytes_per_pixel;
        let byte_lines = block_bytes.chunks_exact_mut(line_bytes);
        assert_eq!(byte_lines.len(), block_index.pixel_size.height(), "invalid block line splits");

        //dbg!(width, line_bytes, header.channels.bytes_per_pixel, byte_lines.len());

        let mut pixel_line = Vec::with_capacity(width);

        for (y, line_bytes) in byte_lines.enumerate() {
            pixel_line.clear();
            pixel_line.extend((0 .. width).map(|x|
                self.channels.pixels.get_pixel(block_index.pixel_position + Vec2(x, y)).into_recursive()
            ));

            self.recursive_channel_writer.write_pixels(line_bytes, pixel_line.as_slice(), |px| px);
        }

        block_bytes
    }
}

/// A tuple containing either `ChannelsDescription` or `Option<ChannelsDescription>` entries.
/// Use an `Option` if you want to dynamically omit a single channel (probably only for roundtrip tests).
/// The number of entries must match the number of channels.
pub trait WritableChannelsDescription<Pixel>: Sync {

    /// A type that has a recursive entry for each channel in the image,
    /// which must accept the desired pixel type.
    type RecursiveWriter: RecursivePixelWriter<Pixel>;

    /// Create the temporary writer, accepting the sorted list of channels from `channel_descriptions_list`.
    fn create_recursive_writer(&self, channels: &ChannelList) -> Self::RecursiveWriter;

    /// Return all the channels that should actually end up in the image, in any order.
    fn channel_descriptions_list(&self) -> SmallVec<[ChannelDescription; 5]>;
}

impl WritableChannelsDescription<NoneMore> for NoneMore {
    type RecursiveWriter = NoneMore;
    fn create_recursive_writer(&self, _: &ChannelList) -> Self::RecursiveWriter { NoneMore }
    fn channel_descriptions_list(&self) -> SmallVec<[ChannelDescription; 5]> { SmallVec::new() }
}

impl<InnerDescriptions, InnerPixel, Sample: IntoNativeSample>
    WritableChannelsDescription<Recursive<InnerPixel, Sample>>
    for Recursive<InnerDescriptions, ChannelDescription>
    where InnerDescriptions: WritableChannelsDescription<InnerPixel>
{
    type RecursiveWriter = RecursiveWriter<InnerDescriptions::RecursiveWriter, Sample>;

    fn create_recursive_writer(&self, channels: &ChannelList) -> Self::RecursiveWriter {
        // this linear lookup is required because the order of the channels changed, due to alphabetical sorting
        let (start_byte_offset, target_sample_type) = channels.channels_with_byte_offset()
            .find(|(_offset, channel)| channel.name == self.value.name)
            .map(|(offset, channel)| (offset, channel.sample_type))
            .expect("a channel has not been put into channel list");

        Recursive::new(self.inner.create_recursive_writer(channels), SampleWriter {
            start_byte_offset, target_sample_type,
            px: PhantomData::default()
        })
    }

    fn channel_descriptions_list(&self) -> SmallVec<[ChannelDescription; 5]> {
        let mut inner_list = self.inner.channel_descriptions_list();
        inner_list.push(self.value.clone());
        inner_list
    }
}

impl<InnerDescriptions, InnerPixel, Sample: IntoNativeSample>
WritableChannelsDescription<Recursive<InnerPixel, Sample>>
for Recursive<InnerDescriptions, Option<ChannelDescription>>
    where InnerDescriptions: WritableChannelsDescription<InnerPixel>
{
    type RecursiveWriter = OptionalRecursiveWriter<InnerDescriptions::RecursiveWriter, Sample>;

    fn create_recursive_writer(&self, channels: &ChannelList) -> Self::RecursiveWriter {
        // this linear lookup is required because the order of the channels changed, due to alphabetical sorting

        let channel = self.value.as_ref().map(|required_channel|
            channels.channels_with_byte_offset()
                .find(|(_offset, channel)| channel == &required_channel)
                .map(|(offset, channel)| (offset, channel.sample_type))
                .expect("a channel has not been put into channel list")
        );

        Recursive::new(
            self.inner.create_recursive_writer(channels),
            channel.map(|(start_byte_offset, target_sample_type)| SampleWriter {
                start_byte_offset, target_sample_type,
                px: PhantomData::default(),
            })
        )
    }

    fn channel_descriptions_list(&self) -> SmallVec<[ChannelDescription; 5]> {
        let mut inner_list = self.inner.channel_descriptions_list();
        if let Some(value) = &self.value { inner_list.push(value.clone()); }
        inner_list
    }
}

/// Write pixels to a slice of bytes. The top level writer contains all the other channels,
/// the most inner channel is `NoneMore`.
pub trait RecursivePixelWriter<Pixel>: Sync {

    /// Write pixels to a slice of bytes. Recursively do this for all channels.
    fn write_pixels<FullPixel>(&self, bytes: &mut [u8], pixels: &[FullPixel], get_pixel: impl Fn(&FullPixel) -> &Pixel);
}

type RecursiveWriter<Inner, Sample> = Recursive<Inner, SampleWriter<Sample>>;
type OptionalRecursiveWriter<Inner, Sample> = Recursive<Inner, Option<SampleWriter<Sample>>>;

/// Write the pixels of a single channel, unconditionally. Generic over the concrete sample type (f16, f32, u32).
#[derive(Debug, Clone)]
pub struct SampleWriter<Sample> {
    target_sample_type: SampleType,
    start_byte_offset: usize,
    px: PhantomData<Sample>,
}

impl<Sample> SampleWriter<Sample> where Sample: IntoNativeSample {
    fn write_own_samples(&self, bytes: &mut [u8], samples: impl ExactSizeIterator<Item=Sample>) {
        let byte_start_index = samples.len() * self.start_byte_offset;
        let byte_count = samples.len() * self.target_sample_type.bytes_per_sample();
        let ref mut byte_writer = &mut bytes[byte_start_index..byte_start_index + byte_count];

        let write_error_msg = "invalid memory buffer length when writing";

        // match outside the loop to avoid matching on every single sample
        match self.target_sample_type {
            // TODO does this boil down to a `memcpy` where the sample type equals the type parameter?
            SampleType::F16 => for sample in samples { sample.to_f16().write_ne(byte_writer).expect(write_error_msg); },
            SampleType::F32 => for sample in samples { sample.to_f32().write_ne(byte_writer).expect(write_error_msg); },
            SampleType::U32 => for sample in samples { sample.to_u32().write_ne(byte_writer).expect(write_error_msg); },
        };

        debug_assert!(byte_writer.is_empty(), "all samples are written, but more were expected");
    }
}

impl RecursivePixelWriter<NoneMore> for NoneMore {
    fn write_pixels<FullPixel>(&self, _: &mut [u8], _: &[FullPixel], _: impl Fn(&FullPixel) -> &NoneMore) {}
}

impl<Inner, InnerPixel, Sample: IntoNativeSample>
    RecursivePixelWriter<Recursive<InnerPixel, Sample>>
    for RecursiveWriter<Inner, Sample>
    where Inner: RecursivePixelWriter<InnerPixel>
{
    // TODO impl exact size iterator <item = Self::Pixel>
    fn write_pixels<FullPixel>(&self, bytes: &mut [u8], pixels: &[FullPixel], get_pixel: impl Fn(&FullPixel) -> &Recursive<InnerPixel, Sample>){
        self.value.write_own_samples(bytes, pixels.iter().map(|px| get_pixel(px).value));
        self.inner.write_pixels(bytes, pixels, |px| &get_pixel(px).inner);
    }
}

impl<Inner, InnerPixel, Sample> RecursivePixelWriter<Recursive<InnerPixel, Sample>>
    for OptionalRecursiveWriter<Inner, Sample>
    where Inner: RecursivePixelWriter<InnerPixel>,
        Sample: IntoNativeSample
{
    fn write_pixels<FullPixel>(&self, bytes: &mut [u8], pixels: &[FullPixel], get_pixel: impl Fn(&FullPixel) -> &Recursive<InnerPixel, Sample>) {
        if let Some(writer) = &self.value {
            writer.write_own_samples(bytes, pixels.iter().map(|px| get_pixel(px).value));
        }

        self.inner.write_pixels(bytes, pixels, |px| &get_pixel(px).inner);
    }
}







#[cfg(test)]
mod test {
    use crate::image::write::channels::WritableChannels;
    use crate::image::SpecificChannels;
    use crate::prelude::{f16};
    use crate::meta::attribute::{ChannelDescription, SampleType};
    use crate::image::pixel_vec::PixelVec;

    #[test]
    fn compiles(){
        let x = 3_f32;
        let y = f16::from_f32(4.0);
        let z = 2_u32;
        let s = 1.3_f32;
        let px = (x,y,z,s);

        assert_is_writable_channels(
            SpecificChannels::rgba(|_pos| px)
        );

        assert_is_writable_channels(SpecificChannels::rgba(
            PixelVec::new((3, 2), vec![px, px, px, px, px, px])
        ));

        let px = (2333_u32, 4_f32);
        assert_is_writable_channels(
            SpecificChannels::build()
                .with_channel("A")
                .with_channel("C")
                .with_pixels(PixelVec::new((3, 2), vec![px, px, px, px, px, px]))
        );

        let px = (3_f32, f16::ONE, 2333_u32, 4_f32);
        assert_is_writable_channels(SpecificChannels::new(
            (
                ChannelDescription::named("x", SampleType::F32),
                ChannelDescription::named("y", SampleType::F16),
                Some(ChannelDescription::named("z", SampleType::U32)),
                Some(ChannelDescription::named("p", SampleType::F32)),
            ),

            PixelVec::new((3, 2), vec![px, px, px, px, px, px])
        ));



        fn assert_is_writable_channels<'s>(_channels: impl WritableChannels<'s>){}

    }
}




