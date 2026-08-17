#![allow(deprecated)]
use std::borrow::Cow;
use crate::dirtyalpha::blurred_dirty_alpha;
use crate::error::Error;
#[cfg(not(feature = "threading"))]
use crate::rayoff as rayon;
use imgref::{Img, ImgVec};
use rav1e::prelude::*;
use rgb::{RGB8, RGBA8};

/// For [`Encoder::with_internal_color_model`]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ColorModel {
    /// Standard color model for photographic content. Usually the best choice.
    /// This library always uses full-resolution color (4:4:4).
    /// This library will automatically choose between BT.601 or BT.709.
    YCbCr,
    /// RGB channels are encoded without color space transformation.
    /// Usually results in larger file sizes, and is less compatible than `YCbCr`.
    /// Use only if the content really makes use of RGB, e.g. anaglyph images or RGB subpixel anti-aliasing.
    RGB,
}

/// Handling of color channels in transparent images. For [`Encoder::with_alpha_color_mode`]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AlphaColorMode {
    /// Use unassociated alpha channel and leave color channels unchanged, even if there's redundant color data in transparent areas.
    UnassociatedDirty,
    /// Use unassociated alpha channel, but set color channels of transparent areas to a solid color to eliminate invisible data and improve compression.
    UnassociatedClean,
    /// Store color channels of transparent images in premultiplied form.
    /// This requires support for premultiplied alpha in AVIF decoders.
    ///
    /// It may reduce file sizes due to clearing of fully-transparent pixels, but
    /// may also increase file sizes due to creation of new edges in the color channels.
    ///
    /// Note that this is only internal detail for the AVIF file.
    /// It does not change meaning of `RGBA` in this library — it's always unassociated.
    Premultiplied,
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum BitDepth {
    Eight,
    Ten,
    /// Same as `Ten`
    #[default]
    Auto,
}

/// The newly-created image file + extra info FYI
#[non_exhaustive]
#[derive(Clone)]
pub struct EncodedImage {
    /// AVIF (HEIF+AV1) encoded image data
    pub avif_file: Vec<u8>,
    /// FYI: number of bytes of AV1 payload used for the color
    pub color_byte_size: usize,
    /// FYI: number of bytes of AV1 payload used for the alpha channel
    pub alpha_byte_size: usize,
}

/// Encoder config builder
///
/// The lifetime is relevant only for [`Encoder::with_exif()`]. Use `Encoder<'static>` if Rust complains.
#[derive(Debug, Clone)]
pub struct Encoder<'exif_slice> {
    /// 0-255 scale
    quantizer: u8,
    /// 0-255 scale
    alpha_quantizer: u8,
    /// rav1e preset 1 (slow) 10 (fast but crappy)
    speed: u8,
    /// True if RGBA input has already been premultiplied. It inserts appropriate metadata.
    premultiplied_alpha: bool,
    /// Which pixel format to use in AVIF file. RGB tends to give larger files.
    color_model: ColorModel,
    /// How many threads should be used (0 = match core count), None - use global rayon thread pool
    threads: Option<usize>,
    /// [`AlphaColorMode`]
    alpha_color_mode: AlphaColorMode,
    /// 8 or 10
    output_depth: BitDepth,
    /// Dropped into MPEG infe BOX
    exif: Option<Cow<'exif_slice, [u8]>>,
}

/// Builder methods
impl<'exif_slice> Encoder<'exif_slice> {
    /// Start here
    #[must_use]
    pub fn new() -> Self {
        Self {
            quantizer: quality_to_quantizer(80.),
            alpha_quantizer: quality_to_quantizer(80.),
            speed: 5,
            output_depth: BitDepth::default(),
            premultiplied_alpha: false,
            color_model: ColorModel::YCbCr,
            threads: None,
            exif: None,
            alpha_color_mode: AlphaColorMode::UnassociatedClean,
        }
    }

    /// Quality `1..=100`. Panics if out of range.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn with_quality(mut self, quality: f32) -> Self {
        assert!(quality >= 1. && quality <= 100.);
        self.quantizer = quality_to_quantizer(quality);
        self
    }

    #[doc(hidden)]
    #[deprecated(note = "Renamed to with_bit_depth")]
    #[must_use]
    pub fn with_depth(self, depth: Option<u8>) -> Self {
        self.with_bit_depth(depth.map(|d| if d >= 10 { BitDepth::Ten } else { BitDepth::Eight }).unwrap_or(BitDepth::Auto))
    }

    /// Internal precision to use in the encoded AV1 data, for both color and alpha. 10-bit depth works best, even for 8-bit inputs/outputs.
    ///
    /// Use 8-bit depth only as a workaround for decoders that need it.
    ///
    /// This setting does not affect pixel inputs for this library.
    #[inline(always)]
    #[must_use]
    pub fn with_bit_depth(mut self, depth: BitDepth) -> Self {
        self.output_depth = depth;
        self
    }

    /// Quality for the alpha channel only. `1..=100`. Panics if out of range.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn with_alpha_quality(mut self, quality: f32) -> Self {
        assert!(quality >= 1. && quality <= 100.);
        self.alpha_quantizer = quality_to_quantizer(quality);
        self
    }

    /// * 1 = very very slow, but max compression.
    /// * 10 = quick, but larger file sizes and lower quality.
    ///
    /// Panics if outside `1..=10`.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn with_speed(mut self, speed: u8) -> Self {
        assert!(speed >= 1 && speed <= 10);
        self.speed = speed;
        self
    }

    /// Changes how color channels are stored in the image. The default is YCbCr.
    ///
    /// Note that this is only internal detail for the AVIF file, and doesn't
    /// change color model of inputs to encode functions.
    #[inline(always)]
    #[must_use]
    pub fn with_internal_color_model(mut self, color_model: ColorModel) -> Self {
        self.color_model = color_model;
        self
    }

    #[doc(hidden)]
    #[deprecated = "Renamed to `with_internal_color_model()`"]
    #[must_use]
    pub fn with_internal_color_space(self, color_model: ColorModel) -> Self {
        self.with_internal_color_model(color_model)
    }

    /// Configures `rayon` thread pool size.
    /// The default `None` is to use all threads in the default `rayon` thread pool.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn with_num_threads(mut self, num_threads: Option<usize>) -> Self {
        assert!(num_threads.is_none_or(|n| n > 0));
        self.threads = num_threads;
        self
    }

    /// Configure handling of color channels in transparent images
    ///
    /// Note that this doesn't affect input format for this library,
    /// which for RGBA is always uncorrelated alpha.
    #[inline(always)]
    #[must_use]
    pub fn with_alpha_color_mode(mut self, mode: AlphaColorMode) -> Self {
        self.alpha_color_mode = mode;
        self.premultiplied_alpha = mode == AlphaColorMode::Premultiplied;
        self
    }

    /// Embedded into AVIF file as-is
    ///
    /// The data can be `Vec<u8>`, or `&[u8]` if the encoder instance doesn't leave its scope.
    pub fn with_exif(mut self, exif_data: impl Into<Cow<'exif_slice, [u8]>>) -> Self {
        self.exif = Some(exif_data.into());
        self
    }
}

/// Once done with config, call one of the `encode_*` functions
impl Encoder<'_> {
    /// Make a new AVIF image from RGBA pixels (non-premultiplied, alpha last)
    ///
    /// Make the `Img` for the `buffer` like this:
    ///
    /// ```rust,ignore
    /// Img::new(&pixels_rgba[..], width, height)
    /// ```
    ///
    /// If you have pixels as `u8` slice, then use the `rgb` crate, and do:
    ///
    /// ```rust,ignore
    /// use rgb::ComponentSlice;
    /// let pixels_rgba = pixels_u8.as_rgba();
    /// ```
    ///
    /// If all pixels are opaque, the alpha channel will be left out automatically.
    ///
    /// This function takes 8-bit inputs, but will generate an AVIF file using 10-bit depth.
    ///
    /// returns AVIF file with info about sizes about AV1 payload.
    pub fn encode_rgba(&self, in_buffer: Img<&[rgb::RGBA<u8>]>) -> Result<EncodedImage, Error> {
        let new_alpha = self.convert_alpha_8bit(in_buffer);
        let buffer = new_alpha.as_ref().map(|b| b.as_ref()).unwrap_or(in_buffer);
        let use_alpha = buffer.pixels().any(|px| px.a != 255);
        if !use_alpha {
            return self.encode_rgb_internal_from_8bit(buffer.width(), buffer.height(), buffer.pixels().map(|px| px.rgb()));
        }

        let width = buffer.width();
        let height = buffer.height();
        let matrix_coefficients = match self.color_model {
            ColorModel::YCbCr => MatrixCoefficients::BT601,
            ColorModel::RGB => MatrixCoefficients::Identity,
        };
        match self.output_depth {
            BitDepth::Eight => {
                let planes = buffer.pixels().map(|px| match self.color_model {
                    ColorModel::YCbCr => rgb_to_8_bit_ycbcr(px.rgb(), BT601).into(),
                    ColorModel::RGB => rgb_to_8_bit_gbr(px.rgb()).into(),
                });
                let alpha = buffer.pixels().map(|px| px.a);
                self.encode_raw_planes_8_bit(width, height, planes, Some(alpha), PixelRange::Full, matrix_coefficients)
            },
            BitDepth::Ten | BitDepth::Auto => {
                let planes = buffer.pixels().map(|px| match self.color_model {
                    ColorModel::YCbCr => rgb_to_10_bit_ycbcr(px.rgb(), BT601).into(),
                    ColorModel::RGB => rgb_to_10_bit_gbr(px.rgb()).into(),
                });
                let alpha = buffer.pixels().map(|px| to_ten(px.a));
                self.encode_raw_planes_10_bit(width, height, planes, Some(alpha), PixelRange::Full, matrix_coefficients)
            },
        }
    }

    fn convert_alpha_8bit(&self, in_buffer: Img<&[RGBA8]>) -> Option<ImgVec<RGBA8>> {
        match self.alpha_color_mode {
            AlphaColorMode::UnassociatedDirty => None,
            AlphaColorMode::UnassociatedClean => blurred_dirty_alpha(in_buffer),
            AlphaColorMode::Premultiplied => {
                let prem = in_buffer.pixels()
                    .map(|px| {
                        if px.a == 0 || px.a == 255 {
                            RGBA8::default()
                        } else {
                            RGBA8::new(
                                (u16::from(px.r) * 255 / u16::from(px.a)) as u8,
                                (u16::from(px.g) * 255 / u16::from(px.a)) as u8,
                                (u16::from(px.b) * 255 / u16::from(px.a)) as u8,
                                px.a,
                            )
                        }
                    })
                    .collect();
                Some(ImgVec::new(prem, in_buffer.width(), in_buffer.height()))
            },
        }
    }

    /// Make a new AVIF image from RGB pixels
    ///
    /// Make the `Img` for the `buffer` like this:
    ///
    /// ```rust,ignore
    /// Img::new(&pixels_rgb[..], width, height)
    /// ```
    ///
    /// If you have pixels as `u8` slice, then first do:
    ///
    /// ```rust,ignore
    /// use rgb::ComponentSlice;
    /// let pixels_rgb = pixels_u8.as_rgb();
    /// ```
    ///
    /// returns AVIF file, size of color metadata
    #[inline]
    pub fn encode_rgb(&self, buffer: Img<&[RGB8]>) -> Result<EncodedImage, Error> {
        self.encode_rgb_internal_from_8bit(buffer.width(), buffer.height(), buffer.pixels())
    }

    fn encode_rgb_internal_from_8bit(&self, width: usize, height: usize, pixels: impl Iterator<Item = RGB8> + Send + Sync) -> Result<EncodedImage, Error> {
        let matrix_coefficients = match self.color_model {
            ColorModel::YCbCr => MatrixCoefficients::BT601,
            ColorModel::RGB => MatrixCoefficients::Identity,
        };

        match self.output_depth {
            BitDepth::Eight => {
                let planes = pixels.map(|px| {
                    let (y, u, v) = match self.color_model {
                        ColorModel::YCbCr => rgb_to_8_bit_ycbcr(px, BT601),
                        ColorModel::RGB => rgb_to_8_bit_gbr(px),
                    };
                    [y, u, v]
                });
                self.encode_raw_planes_8_bit(width, height, planes, None::<[_; 0]>, PixelRange::Full, matrix_coefficients)
            },
            BitDepth::Ten | BitDepth::Auto => {
                let planes = pixels.map(|px| {
                    let (y, u, v) = match self.color_model {
                        ColorModel::YCbCr => rgb_to_10_bit_ycbcr(px, BT601),
                        ColorModel::RGB => rgb_to_10_bit_gbr(px),
                    };
                    [y, u, v]
                });
                self.encode_raw_planes_10_bit(width, height, planes, None::<[_; 0]>, PixelRange::Full, matrix_coefficients)
            },
        }
    }

    /// Encodes AVIF from 3 planar channels that are in the color space described by `matrix_coefficients`,
    /// with sRGB transfer characteristics and color primaries.
    ///
    /// Alpha always uses full range. Chroma subsampling is not supported, and it's a bad idea for AVIF anyway.
    /// If there's no alpha, use `None::<[_; 0]>`.
    ///
    /// `color_pixel_range` should be `PixelRange::Full` to avoid worsening already small 8-bit dynamic range.
    /// Support for limited range may be removed in the future.
    ///
    /// If `AlphaColorMode::Premultiplied` has been set, the alpha pixels must be premultiplied.
    /// `AlphaColorMode::UnassociatedClean` has no effect in this function, and is equivalent to `AlphaColorMode::UnassociatedDirty`.
    ///
    /// returns AVIF file, size of color metadata, size of alpha metadata overhead
    #[inline]
    pub fn encode_raw_planes_8_bit(
        &self, width: usize, height: usize,
        planes: impl IntoIterator<Item = [u8; 3]> + Send,
        alpha: Option<impl IntoIterator<Item = u8> + Send>,
        color_pixel_range: PixelRange, matrix_coefficients: MatrixCoefficients,
    ) -> Result<EncodedImage, Error> {
        self.encode_raw_planes_internal(width, height, planes, alpha, color_pixel_range, matrix_coefficients, 8)
    }

    /// Encodes AVIF from 3 planar channels that are in the color space described by `matrix_coefficients`,
    /// with sRGB transfer characteristics and color primaries.
    ///
    /// The pixels are 10-bit (values `0.=1023`) in host's native endian.
    ///
    /// Alpha always uses full range. Chroma subsampling is not supported, and it's a bad idea for AVIF anyway.
    /// If there's no alpha, use `None::<[_; 0]>`.
    ///
    /// `color_pixel_range` should be `PixelRange::Full`. Support for limited range may be removed in the future.
    ///
    /// If `AlphaColorMode::Premultiplied` has been set, the alpha pixels must be premultiplied.
    /// `AlphaColorMode::UnassociatedClean` has no effect in this function, and is equivalent to `AlphaColorMode::UnassociatedDirty`.
    ///
    /// returns AVIF file, size of color metadata, size of alpha metadata overhead
    #[inline]
    pub fn encode_raw_planes_10_bit(
        &self, width: usize, height: usize,
        planes: impl IntoIterator<Item = [u16; 3]> + Send,
        alpha: Option<impl IntoIterator<Item = u16> + Send>,
        color_pixel_range: PixelRange, matrix_coefficients: MatrixCoefficients,
    ) -> Result<EncodedImage, Error> {
        self.encode_raw_planes_internal(width, height, planes, alpha, color_pixel_range, matrix_coefficients, 10)
    }

    #[inline(never)]
    fn encode_raw_planes_internal<P: rav1e::Pixel + Default>(
        &self, width: usize, height: usize,
        planes: impl IntoIterator<Item = [P; 3]> + Send,
        alpha: Option<impl IntoIterator<Item = P> + Send>,
        color_pixel_range: PixelRange, matrix_coefficients: MatrixCoefficients,
        input_pixels_bit_depth: u8,
    ) -> Result<EncodedImage, Error> {
        let color_description = Some(ColorDescription {
            transfer_characteristics: TransferCharacteristics::SRGB,
            color_primaries: ColorPrimaries::BT709, // sRGB-compatible
            matrix_coefficients,
        });

        let threads = self.threads.map(|threads| {
            if threads > 0 { threads } else { rayon::current_num_threads() }
        });

        let encode_color = move || {
            encode_to_av1::<P>(
                &Av1EncodeConfig {
                    width,
                    height,
                    bit_depth: input_pixels_bit_depth.into(),
                    quantizer: self.quantizer.into(),
                    speed: SpeedTweaks::from_my_preset(self.speed, self.quantizer),
                    threads,
                    pixel_range: color_pixel_range,
                    chroma_sampling: ChromaSampling::Cs444,
                    color_description,
                },
                move |frame| init_frame_3(width, height, planes, frame),
            )
        };
        let encode_alpha = move || {
            alpha.map(|alpha| {
                encode_to_av1::<P>(
                    &Av1EncodeConfig {
                        width,
                        height,
                        bit_depth: input_pixels_bit_depth.into(),
                        quantizer: self.alpha_quantizer.into(),
                        speed: SpeedTweaks::from_my_preset(self.speed, self.alpha_quantizer),
                        threads,
                        pixel_range: PixelRange::Full,
                        chroma_sampling: ChromaSampling::Cs400,
                        color_description: None,
                    },
                    |frame| init_frame_1(width, height, alpha, frame),
                )
            })
        };
        #[cfg(all(target_arch = "wasm32", not(target_feature = "atomics")))]
        let (color, alpha) = (encode_color(), encode_alpha());
        #[cfg(not(all(target_arch = "wasm32", not(target_feature = "atomics"))))]
        let (color, alpha) = rayon::join(encode_color, encode_alpha);
        let (color, alpha) = (color?, alpha.transpose()?);

        let mut serializer_config = avif_serialize::Aviffy::new();
        serializer_config
            .matrix_coefficients(match matrix_coefficients {
                MatrixCoefficients::Identity => avif_serialize::constants::MatrixCoefficients::Rgb,
                MatrixCoefficients::BT709 => avif_serialize::constants::MatrixCoefficients::Bt709,
                MatrixCoefficients::Unspecified => avif_serialize::constants::MatrixCoefficients::Unspecified,
                MatrixCoefficients::BT601 => avif_serialize::constants::MatrixCoefficients::Bt601,
                MatrixCoefficients::YCgCo => avif_serialize::constants::MatrixCoefficients::Ycgco,
                MatrixCoefficients::BT2020NCL => avif_serialize::constants::MatrixCoefficients::Bt2020Ncl,
                MatrixCoefficients::BT2020CL => avif_serialize::constants::MatrixCoefficients::Bt2020Cl,
                _ => return Err(Error::Unsupported("matrix coefficients")),
            })
            .premultiplied_alpha(self.premultiplied_alpha);
        if let Some(exif) = &self.exif {
            serializer_config.set_exif(exif.to_vec());
        }
        let avif_file = serializer_config.to_vec(&color, alpha.as_deref(), width as u32, height as u32, input_pixels_bit_depth);
        let color_byte_size = color.len();
        let alpha_byte_size = alpha.as_ref().map_or(0, |a| a.len());

        Ok(EncodedImage {
            avif_file, color_byte_size, alpha_byte_size,
        })
    }
}

/// Native endian
#[inline(always)]
fn to_ten(x: u8) -> u16 {
    (u16::from(x) << 2) | (u16::from(x) >> 6)
}

/// Native endian
#[inline(always)]
fn rgb_to_10_bit_gbr(px: rgb::RGB<u8>) -> (u16, u16, u16) {
    (to_ten(px.g), to_ten(px.b), to_ten(px.r))
}

#[inline(always)]
fn rgb_to_8_bit_gbr(px: rgb::RGB<u8>) -> (u8, u8, u8) {
    (px.g, px.b, px.r)
}

// const REC709: [f32; 3] = [0.2126, 0.7152, 0.0722];
const BT601: [f32; 3] = [0.2990, 0.5870, 0.1140];

#[inline(always)]
fn rgb_to_ycbcr(px: rgb::RGB<u8>, depth: u8, matrix: [f32; 3]) -> (f32, f32, f32) {
    let max_value = ((1 << depth) - 1) as f32;
    let scale = max_value / 255.;
    let shift = (max_value * 0.5).round();
    let y = (scale * matrix[2]).mul_add(f32::from(px.b), (scale * matrix[0]).mul_add(f32::from(px.r), scale * matrix[1] * f32::from(px.g)));
    let cb = f32::from(px.b).mul_add(scale, -y).mul_add(0.5 / (1. - matrix[2]), shift);
    let cr = f32::from(px.r).mul_add(scale, -y).mul_add(0.5 / (1. - matrix[0]), shift);
    (y.round(), cb.round(), cr.round())
}

#[inline(always)]
fn rgb_to_10_bit_ycbcr(px: rgb::RGB<u8>, matrix: [f32; 3]) -> (u16, u16, u16) {
    let (y, u, v) = rgb_to_ycbcr(px, 10, matrix);
    (y as u16, u as u16, v as u16)
}

#[inline(always)]
fn rgb_to_8_bit_ycbcr(px: rgb::RGB<u8>, matrix: [f32; 3]) -> (u8, u8, u8) {
    let (y, u, v) = rgb_to_ycbcr(px, 8, matrix);
    (y as u8, u as u8, v as u8)
}

fn quality_to_quantizer(quality: f32) -> u8 {
    let q = quality / 100.;
    let x = if q >= 0.82 { (1. - q) * 2.6 } else if q > 0.25 { q.mul_add(-0.5, 1. - 0.125) } else { 1. - q };
    (x * 255.).round() as u8
}

#[derive(Debug, Copy, Clone)]
struct SpeedTweaks {
    pub speed_preset: u8,

    pub fast_deblock: Option<bool>,
    pub reduced_tx_set: Option<bool>,
    pub tx_domain_distortion: Option<bool>,
    pub tx_domain_rate: Option<bool>,
    pub encode_bottomup: Option<bool>,
    pub rdo_tx_decision: Option<bool>,
    pub cdef: Option<bool>,
    /// loop restoration filter
    pub lrf: Option<bool>,
    pub sgr_complexity_full: Option<bool>,
    pub use_satd_subpel: Option<bool>,
    pub inter_tx_split: Option<bool>,
    pub fine_directional_intra: Option<bool>,
    pub complex_prediction_modes: Option<bool>,
    pub partition_range: Option<(u8, u8)>,
    pub min_tile_size: u16,
}

impl SpeedTweaks {
    pub fn from_my_preset(speed: u8, quantizer: u8) -> Self {
        let low_quality = quantizer < quality_to_quantizer(55.);
        let high_quality = quantizer > quality_to_quantizer(80.);
        let max_block_size = if high_quality { 16 } else { 64 };

        Self {
            speed_preset: speed,

            partition_range: Some(match speed {
                0 => (4, 64.min(max_block_size)),
                1 if low_quality => (4, 64.min(max_block_size)),
                2 if low_quality => (4, 32.min(max_block_size)),
                1..=4 => (4, 16),
                5..=8 => (8, 16),
                _ => (16, 16),
            }),

            complex_prediction_modes: Some(speed <= 1), // 2x-3x slower, 2% better
            sgr_complexity_full: Some(speed <= 2), // 15% slower, barely improves anything -/+1%

            encode_bottomup: Some(speed <= 2), // may be costly (+60%), may even backfire

            // big blocks disabled at 3

            // these two are together?
            rdo_tx_decision: Some(speed <= 4 && !high_quality), // it tends to blur subtle textures
            reduced_tx_set: Some(speed == 4 || speed >= 9), // It interacts with tx_domain_distortion too?

            // 4px blocks disabled at 5

            fine_directional_intra: Some(speed <= 6),
            fast_deblock: Some(speed >= 7 && !high_quality), // mixed bag?

            // 8px blocks disabled at 8
            lrf: Some(low_quality && speed <= 8), // hardly any help for hi-q images. recovers some q at low quality
            cdef: Some(low_quality && speed <= 9), // hardly any help for hi-q images. recovers some q at low quality

            inter_tx_split: Some(speed >= 9), // mixed bag even when it works, and it backfires if not used together with reduced_tx_set
            tx_domain_rate: Some(speed >= 10), // 20% faster, but also 10% larger files!

            tx_domain_distortion: None, // very mixed bag, sometimes helps speed sometimes it doesn't
            use_satd_subpel: Some(false), // doesn't make sense
            min_tile_size: match speed {
                0 => 4096,
                1 => 2048,
                2 => 1024,
                3 => 512,
                4 => 256,
                _ => 128,
            } * if high_quality { 2 } else { 1 },
        }
    }

    pub(crate) fn speed_settings(&self) -> SpeedSettings {
        let mut speed_settings = SpeedSettings::from_preset(self.speed_preset);

        speed_settings.multiref = false;
        speed_settings.rdo_lookahead_frames = 1;
        speed_settings.scene_detection_mode = SceneDetectionSpeed::None;
        speed_settings.motion.include_near_mvs = false;

        if let Some(v) = self.fast_deblock { speed_settings.fast_deblock = v; }
        if let Some(v) = self.reduced_tx_set { speed_settings.transform.reduced_tx_set = v; }
        if let Some(v) = self.tx_domain_distortion { speed_settings.transform.tx_domain_distortion = v; }
        if let Some(v) = self.tx_domain_rate { speed_settings.transform.tx_domain_rate = v; }
        if let Some(v) = self.encode_bottomup { speed_settings.partition.encode_bottomup = v; }
        if let Some(v) = self.rdo_tx_decision { speed_settings.transform.rdo_tx_decision = v; }
        if let Some(v) = self.cdef { speed_settings.cdef = v; }
        if let Some(v) = self.lrf { speed_settings.lrf = v; }
        if let Some(v) = self.inter_tx_split { speed_settings.transform.enable_inter_tx_split = v; }
        if let Some(v) = self.sgr_complexity_full { speed_settings.sgr_complexity = if v { SGRComplexityLevel::Full } else { SGRComplexityLevel::Reduced } }
        if let Some(v) = self.use_satd_subpel { speed_settings.motion.use_satd_subpel = v; }
        if let Some(v) = self.fine_directional_intra { speed_settings.prediction.fine_directional_intra = v; }
        if let Some(v) = self.complex_prediction_modes { speed_settings.prediction.prediction_modes = if v { PredictionModesSetting::ComplexAll } else { PredictionModesSetting::Simple} }
        if let Some((min, max)) = self.partition_range {
            debug_assert!(min <= max);
            fn sz(s: u8) -> BlockSize {
                match s {
                    4 => BlockSize::BLOCK_4X4,
                    8 => BlockSize::BLOCK_8X8,
                    16 => BlockSize::BLOCK_16X16,
                    32 => BlockSize::BLOCK_32X32,
                    64 => BlockSize::BLOCK_64X64,
                    128 => BlockSize::BLOCK_128X128,
                    _ => panic!("bad size {s}"),
                }
            }
            speed_settings.partition.partition_range = PartitionRange::new(sz(min), sz(max));
        }

        speed_settings
    }
}

struct Av1EncodeConfig {
    pub width: usize,
    pub height: usize,
    pub bit_depth: usize,
    pub quantizer: usize,
    pub speed: SpeedTweaks,
    /// 0 means num_cpus
    pub threads: Option<usize>,
    pub pixel_range: PixelRange,
    pub chroma_sampling: ChromaSampling,
    pub color_description: Option<ColorDescription>,
}

fn rav1e_config(p: &Av1EncodeConfig) -> Config {
    // AV1 needs all the CPU power you can give it,
    // except when it'd create inefficiently tiny tiles
    let tiles = {
        let threads = p.threads.unwrap_or_else(rayon::current_num_threads);
        threads.min((p.width * p.height) / (p.speed.min_tile_size as usize).pow(2))
    };
    let speed_settings = p.speed.speed_settings();
    let cfg = Config::new()
        .with_encoder_config(EncoderConfig {
        width: p.width,
        height: p.height,
        time_base: Rational::new(1, 1),
        sample_aspect_ratio: Rational::new(1, 1),
        bit_depth: p.bit_depth,
        chroma_sampling: p.chroma_sampling,
        chroma_sample_position: ChromaSamplePosition::Unknown,
        pixel_range: p.pixel_range,
        color_description: p.color_description,
        mastering_display: None,
        content_light: None,
        enable_timing_info: false,
        still_picture: true,
        error_resilient: false,
        switch_frame_interval: 0,
        min_key_frame_interval: 0,
        max_key_frame_interval: 0,
        reservoir_frame_delay: None,
        low_latency: false,
        quantizer: p.quantizer,
        min_quantizer: p.quantizer as _,
        bitrate: 0,
        tune: Tune::Psychovisual,
        tile_cols: 0,
        tile_rows: 0,
        tiles,
        film_grain_params: None,
        level_idx: None,
        speed_settings,
    });

    if let Some(threads) = p.threads {
        cfg.with_threads(threads)
    } else {
        cfg
    }
}

fn init_frame_3<P: rav1e::Pixel + Default>(
    width: usize, height: usize, planes: impl IntoIterator<Item = [P; 3]> + Send, frame: &mut Frame<P>,
) -> Result<(), Error> {
    let mut f = frame.planes.iter_mut();
    let mut planes = planes.into_iter();

    // it doesn't seem to be necessary to fill padding area
    let mut y = f.next().unwrap().mut_slice(Default::default());
    let mut u = f.next().unwrap().mut_slice(Default::default());
    let mut v = f.next().unwrap().mut_slice(Default::default());

    for ((y, u), v) in y.rows_iter_mut().zip(u.rows_iter_mut()).zip(v.rows_iter_mut()).take(height) {
        let y = &mut y[..width];
        let u = &mut u[..width];
        let v = &mut v[..width];
        for ((y, u), v) in y.iter_mut().zip(u).zip(v) {
            let px = planes.next().ok_or(Error::TooFewPixels)?;
            *y = px[0];
            *u = px[1];
            *v = px[2];
        }
    }
    Ok(())
}

fn init_frame_1<P: rav1e::Pixel + Default>(width: usize, height: usize, planes: impl IntoIterator<Item = P> + Send, frame: &mut Frame<P>) -> Result<(), Error> {
    let mut y = frame.planes[0].mut_slice(Default::default());
    let mut planes = planes.into_iter();

    for y in y.rows_iter_mut().take(height) {
        let y = &mut y[..width];
        for y in y.iter_mut() {
            *y = planes.next().ok_or(Error::TooFewPixels)?;
        }
    }
    Ok(())
}

#[inline(never)]
fn encode_to_av1<P: rav1e::Pixel>(p: &Av1EncodeConfig, init: impl FnOnce(&mut Frame<P>) -> Result<(), Error>) -> Result<Vec<u8>, Error> {
    let mut ctx: Context<P> = rav1e_config(p).new_context()?;
    let mut frame = ctx.new_frame();

    init(&mut frame)?;
    ctx.send_frame(frame)?;
    ctx.flush();

    let mut out = Vec::new();
    loop {
        match ctx.receive_packet() {
            Ok(mut packet) => match packet.frame_type {
                FrameType::KEY => {
                    out.append(&mut packet.data);
                },
                _ => continue,
            },
            Err(EncoderStatus::Encoded | EncoderStatus::LimitReached) => break,
            Err(err) => Err(err)?,
        }
    }
    Ok(out)
}
