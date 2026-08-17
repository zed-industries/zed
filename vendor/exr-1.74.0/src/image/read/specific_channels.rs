//! How to read arbitrary but specific selection of arbitrary channels.
//! This is not a zero-cost abstraction.

use crate::image::recursive::*;
use crate::block::samples::*;
use crate::image::*;
use crate::math::*;
use crate::meta::header::*;
use crate::error::*;
use crate::block::UncompressedBlock;
use crate::image::read::layers::{ChannelsReader, ReadChannels};
use crate::block::chunk::TileCoordinates;

use std::marker::PhantomData;
use crate::io::Read;


/// Can be attached one more channel reader.
/// Call `required` or `optional` on this object to declare another channel to be read from the file.
/// Call `collect_pixels` at last to define how the previously declared pixels should be stored.
pub trait ReadSpecificChannel: Sized + CheckDuplicates {

    /// A separate internal reader for the pixels. Will be of type `Recursive<_, SampleReader<_>>`,
    /// depending on the pixels of the specific channel combination.
    type RecursivePixelReader: RecursivePixelReader;

    /// Create a separate internal reader for the pixels of the specific channel combination.
    fn create_recursive_reader(&self, channels: &ChannelList) -> Result<Self::RecursivePixelReader>;

    /// Plan to read an additional channel from the image, with the specified name.
    /// If the channel cannot be found in the image when the image is read, the image will not be loaded.
    /// The generic parameter can usually be inferred from the closure in `collect_pixels`.
    fn required<Sample>(self, channel_name: impl Into<Text>) -> ReadRequiredChannel<Self, Sample> {
        let channel_name = channel_name.into();
        assert!(self.already_contains(&channel_name).not(), "a channel with the name `{}` is already defined", channel_name);
        ReadRequiredChannel { channel_name, previous_channels: self, px: Default::default() }
    }

    /// Plan to read an additional channel from the image, with the specified name.
    /// If the file does not contain this channel, the specified default sample will be returned instead.
    /// You can check whether the channel has been loaded by
    /// checking the presence of the optional channel description before instantiating your own image.
    /// The generic parameter can usually be inferred from the closure in `collect_pixels`.
    fn optional<Sample>(self, channel_name: impl Into<Text>, default_sample: Sample)
        -> ReadOptionalChannel<Self, Sample>
    {
        let channel_name = channel_name.into();
        assert!(self.already_contains(&channel_name).not(), "a channel with the name `{}` is already defined", channel_name);
        ReadOptionalChannel { channel_name, previous_channels: self, default_sample }
    }

    /// Using two closures, define how to store the pixels.
    /// The first closure creates an image, and the second closure inserts a single pixel.
    /// The type of the pixel can be defined by the second closure;
    /// it must be a tuple containing `f16`, `f32`, `u32` or `Sample` values.
    /// See the examples for more information.
    fn collect_pixels<Pixel, PixelStorage, CreatePixels, SetPixel>(
        self, create_pixels: CreatePixels, set_pixel: SetPixel
    ) -> CollectPixels<Self, Pixel, PixelStorage, CreatePixels, SetPixel>
        where
            <Self::RecursivePixelReader as RecursivePixelReader>::RecursivePixel: IntoTuple<Pixel>,
            <Self::RecursivePixelReader as RecursivePixelReader>::RecursiveChannelDescriptions: IntoNonRecursive,
            CreatePixels: Fn(
                Vec2<usize>,
                &<<Self::RecursivePixelReader as RecursivePixelReader>::RecursiveChannelDescriptions as IntoNonRecursive>::NonRecursive
            ) -> PixelStorage,
            SetPixel: Fn(&mut PixelStorage, Vec2<usize>, Pixel),
    {
        CollectPixels { read_channels: self, set_pixel, create_pixels, px: Default::default() }
    }
}

/// A reader containing sub-readers for reading the pixel content of an image.
pub trait RecursivePixelReader {

    /// The channel descriptions from the image.
    /// Will be converted to a tuple before being stored in `SpecificChannels<_, ChannelDescriptions>`.
    type RecursiveChannelDescriptions;

    /// Returns the channel descriptions based on the channels in the file.
    fn get_descriptions(&self) -> Self::RecursiveChannelDescriptions;

    /// The pixel type. Will be converted to a tuple at the end of the process.
    type RecursivePixel: Copy + Default + 'static;

    /// Read the line of pixels.
    fn read_pixels<'s, FullPixel>(
        &self, bytes: &'s[u8], pixels: &mut [FullPixel],
        get_pixel: impl Fn(&mut FullPixel) -> &mut Self::RecursivePixel
    );
}

// does not use the generic `Recursive` struct to reduce the number of angle brackets in the public api
/// Used to read another specific channel from an image.
/// Contains the previous `ReadChannels` objects.
#[derive(Clone, Debug)]
pub struct ReadOptionalChannel<ReadChannels, Sample> {
    previous_channels: ReadChannels,
    channel_name: Text,
    default_sample: Sample,
}

// does not use the generic `Recursive` struct to reduce the number of angle brackets in the public api
/// Used to read another specific channel from an image.
/// Contains the previous `ReadChannels` objects.
#[derive(Clone, Debug)]
pub struct ReadRequiredChannel<ReadChannels, Sample> {
    previous_channels: ReadChannels,
    channel_name: Text,
    px: PhantomData<Sample>,
}

/// Specifies how to collect all the specified channels into a number of individual pixels.
#[derive(Copy, Clone, Debug)]
pub struct CollectPixels<ReadChannels, Pixel, PixelStorage, CreatePixels, SetPixel> {
    read_channels: ReadChannels,
    create_pixels: CreatePixels,
    set_pixel: SetPixel,
    px: PhantomData<(Pixel, PixelStorage)>,
}

impl<Inner: CheckDuplicates, Sample> CheckDuplicates for ReadRequiredChannel<Inner, Sample> {
    fn already_contains(&self, name: &Text) -> bool {
        &self.channel_name == name || self.previous_channels.already_contains(name)
    }
}

impl<Inner: CheckDuplicates, Sample> CheckDuplicates for ReadOptionalChannel<Inner, Sample> {
    fn already_contains(&self, name: &Text) -> bool {
        &self.channel_name == name || self.previous_channels.already_contains(name)
    }
}

impl<'s, InnerChannels, Pixel, PixelStorage, CreatePixels, SetPixel: 's>
ReadChannels<'s> for CollectPixels<InnerChannels, Pixel, PixelStorage, CreatePixels, SetPixel>
    where
        InnerChannels: ReadSpecificChannel,
        <InnerChannels::RecursivePixelReader as RecursivePixelReader>::RecursivePixel: IntoTuple<Pixel>,
        <InnerChannels::RecursivePixelReader as RecursivePixelReader>::RecursiveChannelDescriptions: IntoNonRecursive,
        CreatePixels: Fn(Vec2<usize>, &<<InnerChannels::RecursivePixelReader as RecursivePixelReader>::RecursiveChannelDescriptions as IntoNonRecursive>::NonRecursive) -> PixelStorage,
        SetPixel: Fn(&mut PixelStorage, Vec2<usize>, Pixel),
{
    type Reader = SpecificChannelsReader<
        PixelStorage, &'s SetPixel,
        InnerChannels::RecursivePixelReader,
        Pixel,
    >;

    fn create_channels_reader(&'s self, header: &Header) -> Result<Self::Reader> {
        if header.deep { return Err(Error::invalid("`SpecificChannels` does not support deep data yet")) }

        let pixel_reader = self.read_channels.create_recursive_reader(&header.channels)?;
        let channel_descriptions = pixel_reader.get_descriptions().into_non_recursive();// TODO not call this twice

        let create = &self.create_pixels;
        let pixel_storage = create(header.layer_size, &channel_descriptions);

        Ok(SpecificChannelsReader {
            set_pixel: &self.set_pixel,
            pixel_storage,
            pixel_reader,
            px: Default::default()
        })
    }
}

/// The reader that holds the temporary data that is required to read some specified channels.
#[derive(Copy, Clone, Debug)]
pub struct SpecificChannelsReader<PixelStorage, SetPixel, PixelReader, Pixel> {
    set_pixel: SetPixel,
    pixel_storage: PixelStorage,
    pixel_reader: PixelReader,
    px: PhantomData<Pixel>
}

impl<PixelStorage, SetPixel, PxReader, Pixel>
ChannelsReader for SpecificChannelsReader<PixelStorage, SetPixel, PxReader, Pixel>
    where PxReader: RecursivePixelReader,
          PxReader::RecursivePixel: IntoTuple<Pixel>,
          PxReader::RecursiveChannelDescriptions: IntoNonRecursive,
          SetPixel: Fn(&mut PixelStorage, Vec2<usize>, Pixel),
{
    type Channels = SpecificChannels<PixelStorage, <PxReader::RecursiveChannelDescriptions as IntoNonRecursive>::NonRecursive>;

    fn filter_block(&self, tile: TileCoordinates) -> bool { tile.is_largest_resolution_level() } // TODO all levels

    fn read_block(&mut self, header: &Header, block: UncompressedBlock) -> UnitResult {
        let mut pixels = vec![PxReader::RecursivePixel::default(); block.index.pixel_size.width()]; // TODO allocate once in self

        let byte_lines = block.data.chunks_exact(header.channels.bytes_per_pixel * block.index.pixel_size.width());
        debug_assert_eq!(byte_lines.len(), block.index.pixel_size.height(), "invalid block lines split");

        for (y_offset, line_bytes) in byte_lines.enumerate() { // TODO sampling
            // this two-step copy method should be very cache friendly in theory, and also reduce sample_type lookup count
            self.pixel_reader.read_pixels(line_bytes, &mut pixels, |px| px);

            for (x_offset, pixel) in pixels.iter().enumerate() {
                let set_pixel = &self.set_pixel;
                set_pixel(&mut self.pixel_storage, block.index.pixel_position + Vec2(x_offset, y_offset), pixel.into_tuple());
            }
        }

        Ok(())
    }

    fn into_channels(self) -> Self::Channels {
        SpecificChannels { channels: self.pixel_reader.get_descriptions().into_non_recursive(), pixels: self.pixel_storage }
    }
}


/// Read zero channels from an image. Call `with_named_channel` on this object
/// to read as many channels as desired.
pub type ReadZeroChannels = NoneMore;

impl ReadSpecificChannel for NoneMore {
    type RecursivePixelReader = NoneMore;
    fn create_recursive_reader(&self, _: &ChannelList) -> Result<Self::RecursivePixelReader> { Ok(NoneMore) }
}

impl<DefaultSample, ReadChannels> ReadSpecificChannel for ReadOptionalChannel<ReadChannels, DefaultSample>
    where ReadChannels: ReadSpecificChannel, DefaultSample: FromNativeSample + 'static,
{
    type RecursivePixelReader = Recursive<ReadChannels::RecursivePixelReader, OptionalSampleReader<DefaultSample>>;

    fn create_recursive_reader(&self, channels: &ChannelList) -> Result<Self::RecursivePixelReader> {
        debug_assert!(self.previous_channels.already_contains(&self.channel_name).not(), "duplicate channel name: {}", self.channel_name);

        let inner_samples_reader = self.previous_channels.create_recursive_reader(channels)?;
        let reader = channels.channels_with_byte_offset()
            .find(|(_, channel)| channel.name == self.channel_name)
            .map(|(channel_byte_offset, channel)| SampleReader {
                channel_byte_offset, channel: channel.clone(),
                px: Default::default()
            });

        Ok(Recursive::new(inner_samples_reader, OptionalSampleReader {
            reader, default_sample: self.default_sample,
        }))
    }
}

impl<Sample, ReadChannels> ReadSpecificChannel for ReadRequiredChannel<ReadChannels, Sample>
    where ReadChannels: ReadSpecificChannel, Sample: FromNativeSample + 'static
{
    type RecursivePixelReader = Recursive<ReadChannels::RecursivePixelReader, SampleReader<Sample>>;

    fn create_recursive_reader(&self, channels: &ChannelList) -> Result<Self::RecursivePixelReader> {
        let previous_samples_reader = self.previous_channels.create_recursive_reader(channels)?;
        let (channel_byte_offset, channel) = channels.channels_with_byte_offset()
                .find(|(_, channel)| channel.name == self.channel_name)
                .ok_or_else(|| Error::invalid(format!(
                    "layer does not contain all of your specified channels (`{}` is missing)",
                    self.channel_name
                )))?;

        Ok(Recursive::new(previous_samples_reader, SampleReader { channel_byte_offset, channel: channel.clone(), px: Default::default() }))
    }
}

/// Reader for a single channel. Generic over the concrete sample type (f16, f32, u32).
#[derive(Clone, Debug)]
pub struct SampleReader<Sample> {

    /// to be multiplied with line width!
    channel_byte_offset: usize,

    channel: ChannelDescription,
    px: PhantomData<Sample>
}

/// Reader for a single channel. Generic over the concrete sample type (f16, f32, u32).
/// Can also skip reading a channel if it could not be found in the image.
#[derive(Clone, Debug)]
pub struct OptionalSampleReader<DefaultSample> {
    reader: Option<SampleReader<DefaultSample>>,
    default_sample: DefaultSample,
}

impl<Sample: FromNativeSample> SampleReader<Sample> {
    fn read_own_samples<'s, FullPixel>(
        &self, bytes: &'s[u8], pixels: &mut [FullPixel],
        get_sample: impl Fn(&mut FullPixel) -> &mut Sample
    ){
        let start_index = pixels.len() * self.channel_byte_offset;
        let byte_count = pixels.len() * self.channel.sample_type.bytes_per_sample();
        let mut own_bytes_reader = &mut &bytes[start_index .. start_index + byte_count]; // TODO check block size somewhere
        let mut samples_out = pixels.iter_mut().map(|pixel| get_sample(pixel));

        // match the type once for the whole line, not on every single sample
        match self.channel.sample_type {
            SampleType::F16 => read_and_convert_all_samples_batched(
                &mut own_bytes_reader, &mut samples_out,
                Sample::from_f16s
            ),

            SampleType::F32 => read_and_convert_all_samples_batched(
                &mut own_bytes_reader, &mut samples_out,
                Sample::from_f32s
            ),

            SampleType::U32 => read_and_convert_all_samples_batched(
                &mut own_bytes_reader, &mut samples_out,
                Sample::from_u32s
            ),
        }

        debug_assert!(samples_out.next().is_none(), "not all samples have been converted");
        debug_assert!(own_bytes_reader.is_empty(), "bytes left after reading all samples");
    }
}


/// Does the same as `convert_batch(in_bytes.chunks().map(From::from_bytes))`, but vectorized.
/// Reads the samples for one line, using the sample type specified in the file,
/// and then converts those to the desired sample types.
/// Uses batches to allow vectorization, converting multiple values with one instruction.
/// Does not convert endianness.
fn read_and_convert_all_samples_batched<'t, From, To>(
    mut in_bytes: impl Read,
    out_samples: &mut impl ExactSizeIterator<Item=&'t mut To>,
    convert_batch: fn(&[From], &mut [To])
) where From: Data + Default + Copy, To: 't + Default + Copy
{
    // this is not a global! why is this warning triggered?
    #[allow(non_upper_case_globals)]
    const batch_size: usize = 16;

    let total_sample_count = out_samples.len();
    let batch_count = total_sample_count / batch_size;
    let remaining_samples_count = total_sample_count % batch_size;

    let len_error_msg = "sample count was miscalculated";
    let byte_error_msg = "error when reading from in-memory slice";

    // write samples from a given slice to the output iterator. should be inlined.
    let output_n_samples = &mut move |samples: &[To]| {
        for converted_sample in samples {
            *out_samples.next().expect(len_error_msg) = *converted_sample;
        }
    };

    // read samples from the byte source into a given slice. should be inlined.
    // todo: use #[inline] when available
    // error[E0658]: attributes on expressions are experimental,
    // see issue #15701 <https://github.com/rust-lang/rust/issues/15701> for more information
    let read_n_samples = &mut move |samples: &mut [From]| {
        Data::read_slice_ne(&mut in_bytes, samples).expect(byte_error_msg);
    };

    // temporary arrays with fixed size, operations should be vectorized within these arrays
    let mut source_samples_batch: [From; batch_size] = Default::default();
    let mut desired_samples_batch: [To; batch_size] = Default::default();

    // first convert all whole batches, size statically known to be 16 element arrays
    for _ in 0 .. batch_count {
        read_n_samples(&mut source_samples_batch);
        convert_batch(source_samples_batch.as_slice(), desired_samples_batch.as_mut_slice());
        output_n_samples(&desired_samples_batch);
    }

    // then convert a partial remaining batch, size known only at runtime
    if remaining_samples_count != 0 {
        let source_samples_batch = &mut source_samples_batch[..remaining_samples_count];
        let desired_samples_batch = &mut desired_samples_batch[..remaining_samples_count];

        read_n_samples(source_samples_batch);
        convert_batch(source_samples_batch, desired_samples_batch);
        output_n_samples(desired_samples_batch);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn equals_naive_f32(){
        for total_array_size in [3, 7, 30, 41, 120, 10_423] {
            let input_f32s = (0..total_array_size).map(|_| rand::random::<f32>()).collect::<Vec<f32>>();
            let in_f32s_bytes = input_f32s.iter().cloned().flat_map(f32::to_ne_bytes).collect::<Vec<u8>>();

            let mut out_f16_samples_batched = vec![
                f16::from_f32(rand::random::<f32>());
                total_array_size
            ];

            read_and_convert_all_samples_batched(
                &mut in_f32s_bytes.as_slice(),
                &mut out_f16_samples_batched.iter_mut(),
                f16::from_f32s
            );

            let out_f16_samples_naive = input_f32s.iter()
                .cloned().map(f16::from_f32);

            assert!(out_f16_samples_naive.eq(out_f16_samples_batched));
        }
    }
}


impl RecursivePixelReader for NoneMore {
    type RecursiveChannelDescriptions = NoneMore;
    fn get_descriptions(&self) -> Self::RecursiveChannelDescriptions { NoneMore }

    type RecursivePixel = NoneMore;

    fn read_pixels<'s, FullPixel>(
        &self, _: &'s[u8], _: &mut [FullPixel],
        _: impl Fn(&mut FullPixel) -> &mut NoneMore
    ){}
}

impl<Sample, InnerReader: RecursivePixelReader>
    RecursivePixelReader
    for Recursive<InnerReader, SampleReader<Sample>>
    where Sample: FromNativeSample + 'static
{
    type RecursiveChannelDescriptions = Recursive<InnerReader::RecursiveChannelDescriptions, ChannelDescription>;
    fn get_descriptions(&self) -> Self::RecursiveChannelDescriptions { Recursive::new(self.inner.get_descriptions(), self.value.channel.clone()) }

    type RecursivePixel = Recursive<InnerReader::RecursivePixel, Sample>;

    fn read_pixels<'s, FullPixel>(
        &self, bytes: &'s[u8], pixels: &mut [FullPixel],
        get_pixel: impl Fn(&mut FullPixel) -> &mut Self::RecursivePixel
    ) {
        self.value.read_own_samples(bytes, pixels, |px| &mut get_pixel(px).value);
        self.inner.read_pixels(bytes, pixels, |px| &mut get_pixel(px).inner);
    }
}

impl<Sample, InnerReader: RecursivePixelReader>
RecursivePixelReader
for Recursive<InnerReader, OptionalSampleReader<Sample>>
    where Sample: FromNativeSample + 'static
{
    type RecursiveChannelDescriptions = Recursive<InnerReader::RecursiveChannelDescriptions, Option<ChannelDescription>>;
    fn get_descriptions(&self) -> Self::RecursiveChannelDescriptions { Recursive::new(
        self.inner.get_descriptions(), self.value.reader.as_ref().map(|reader| reader.channel.clone())
    ) }

    type RecursivePixel = Recursive<InnerReader::RecursivePixel, Sample>;

    fn read_pixels<'s, FullPixel>(
        &self, bytes: &'s[u8], pixels: &mut [FullPixel],
        get_pixel: impl Fn(&mut FullPixel) -> &mut Self::RecursivePixel
    ) {
        if let Some(reader) = &self.value.reader {
            reader.read_own_samples(bytes, pixels, |px| &mut get_pixel(px).value);
        }
        else {
            // if this channel is optional and was not found in the file, fill the default sample
            for pixel in pixels.iter_mut() {
                get_pixel(pixel).value = self.value.default_sample;
            }
        }

        self.inner.read_pixels(bytes, pixels, |px| &mut get_pixel(px).inner);
    }
}


