use super::ifd::Value;
use super::stream::PackBitsReader;
use super::tag_reader::TagReader;
use super::ChunkType;
use super::{predict_f16, predict_f32, predict_f64, ValueReader};
use crate::tags::{
    CompressionMethod, ExtraSamples, PhotometricInterpretation, PlanarConfiguration, Predictor,
    SampleFormat, Tag,
};
use crate::{
    ColorType, Directory, TiffError, TiffFormatError, TiffResult, TiffUnsupportedError, UsageError,
};

use std::io::{self, Cursor, Read, Seek};
use std::sync::Arc;

#[derive(Debug)]
pub(crate) struct StripDecodeState {
    pub rows_per_strip: u32,
}

#[derive(Debug)]
/// Computed values useful for tile decoding
pub(crate) struct TileAttributes {
    pub image_width: usize,
    pub image_height: usize,

    pub tile_width: usize,
    pub tile_length: usize,
}

impl TileAttributes {
    pub fn tiles_across(&self) -> usize {
        self.image_width.div_ceil(self.tile_width)
    }
    pub fn tiles_down(&self) -> usize {
        self.image_height.div_ceil(self.tile_length)
    }
    fn padding_right(&self) -> usize {
        (self.tile_width - self.image_width % self.tile_width) % self.tile_width
    }
    fn padding_down(&self) -> usize {
        (self.tile_length - self.image_height % self.tile_length) % self.tile_length
    }
    pub fn get_padding(&self, tile: usize) -> (usize, usize) {
        let row = tile / self.tiles_across();
        let column = tile % self.tiles_across();

        let padding_right = if column == self.tiles_across() - 1 {
            self.padding_right()
        } else {
            0
        };

        let padding_down = if row == self.tiles_down() - 1 {
            self.padding_down()
        } else {
            0
        };

        (padding_right, padding_down)
    }
}

#[derive(Debug)]
pub(crate) struct Image {
    pub ifd: Option<Directory>,
    pub width: u32,
    pub height: u32,
    pub bits_per_sample: u8,
    pub samples: u16,
    /// The `ExtraSamples`, defaulting to empty if not given.
    pub extra_samples: Vec<ExtraSamples>,
    /// Number of samples that belong to the photometric interpretation, samples except
    /// `ExtraSamples` (338, 0x0152) tag.
    pub photometric_samples: u16,
    pub sample_format: SampleFormat,
    pub photometric_interpretation: PhotometricInterpretation,
    pub compression_method: CompressionMethod,
    pub predictor: Predictor,
    pub jpeg_tables: Option<Arc<Vec<u8>>>,
    pub chunk_type: ChunkType,
    pub planar_config: PlanarConfiguration,
    pub strip_decoder: Option<StripDecodeState>,
    pub tile_attributes: Option<TileAttributes>,
    pub chunk_offsets: Vec<u64>,
    pub chunk_bytes: Vec<u64>,
    pub chroma_subsampling: (u16, u16),
}

/// Describes how to read a tile-aligned portion of the image.
#[derive(Clone)]
pub(crate) struct ReadoutLayout {
    /// The planar configuration, which applies to both the underlying image and the output buffer.
    /// This may be relaxed if we find a clean enough way to provide it.
    pub planar_config: PlanarConfiguration,

    /// The sample interpretation (interpret with planar_config).
    ///
    /// FIXME: we should not require this here. The ability to turn out the raw bytes from the
    /// sample arrays is very different from turning out interpretable color. Firstly we can always
    /// readout `Multiband` but currently only use that ColorType in special circumstances (it must
    /// not overlap cases where actually want to use a ColorType).
    ///
    /// And then we have CIE Lab, which uses a tuple of `(u8, i8, i8)`, that is still filterable
    /// but still not represented by any of our `DecoderResult` variants. Other color variants
    /// depend on extra tags (YCbCrCoefficients/0x0211) and we don't have a good side channel to
    /// tag the output with all that TIFF specific information, so arguably we should process and
    /// apply those to the data so it becomes a self-contained representation.
    ///
    /// This should be computed at a higher level, in `Decoder`, instead.
    pub color: ColorType,
    /// The number of bytes from one row to another.
    pub minimum_row_stride: usize,
    /// The format of samples (assumed uniform for now, same with depth of `ColorType`).
    pub sample_format: SampleFormat,

    /// Number of bytes to advance in output per row.
    pub row_stride: usize,
    /// Number of bytes to advance in output per chunk in width.
    pub chunk_row_stride: usize,
    /// Number of bytes to advance in output per chunk in height.
    pub chunk_col_stride: usize,
    /// Number of bytes in output from one plane to another.
    pub plane_stride: usize,

    /// Bits per sample in the encoded data.
    pub tiff_bits_per_sample: u8,
    /// Number of samples in the encoded data.
    pub tiff_samples: u16,
    /// Dimensions of the underlying rectangular chunks (tile or strips).
    pub tiff_chunk_dimensions: (u32, u32),
    /// Number of bytes in the underlying data with all samples per row of chunks.
    pub tiff_row_bytes: usize,

    /// Chunks until wrapping to the next row of chunks.
    pub chunks_across: u32,
    /// Chunks to advance to get to the next plane of chunks.
    pub chunks_per_plane: u32,
}

impl Image {
    pub fn from_reader<R: Read + Seek>(
        decoder: &mut ValueReader<R>,
        ifd: Directory,
    ) -> TiffResult<Image> {
        let mut tag_reader = TagReader { decoder, ifd: &ifd };

        let width = tag_reader.require_tag(Tag::ImageWidth)?.into_u32()?;
        let height = tag_reader.require_tag(Tag::ImageLength)?.into_u32()?;
        if width == 0 || height == 0 {
            return Err(TiffError::FormatError(TiffFormatError::InvalidDimensions(
                width, height,
            )));
        }

        let photometric_interpretation = tag_reader
            .find_tag(Tag::PhotometricInterpretation)?
            .map(Value::into_u16)
            .transpose()?
            .and_then(PhotometricInterpretation::from_u16)
            .ok_or(TiffUnsupportedError::UnknownInterpretation)?;

        // Try to parse both the compression method and the number, format, and bits of the included samples.
        // If they are not explicitly specified, those tags are reset to their default values and not carried from previous images.
        let compression_method = match tag_reader.find_tag(Tag::Compression)? {
            Some(val) => CompressionMethod::from_u16_exhaustive(val.into_u16()?),
            None => CompressionMethod::None,
        };

        let jpeg_tables = if compression_method == CompressionMethod::ModernJPEG
            && ifd.contains(Tag::JPEGTables)
        {
            let vec = tag_reader
                .find_tag(Tag::JPEGTables)?
                .unwrap()
                .into_u8_vec()?;
            if vec.len() < 2 {
                return Err(TiffError::FormatError(
                    TiffFormatError::InvalidTagValueType(Tag::JPEGTables),
                ));
            }

            Some(Arc::new(vec))
        } else {
            None
        };

        let samples: u16 = tag_reader
            .find_tag(Tag::SamplesPerPixel)?
            .map(Value::into_u16)
            .transpose()?
            .unwrap_or(1);

        if samples == 0 {
            return Err(TiffFormatError::SamplesPerPixelIsZero.into());
        }

        let extra_samples = match tag_reader.find_tag(Tag::ExtraSamples)? {
            Some(n) => n.into_u16_vec()?,
            None => vec![],
        };

        let extra_samples = extra_samples
            .into_iter()
            .map(|x| ExtraSamples::from_u16(x).unwrap_or(ExtraSamples::Unspecified))
            .collect::<Vec<_>>();

        let photometric_samples = match usize::from(samples).checked_sub(extra_samples.len()) {
            None => {
                return Err(TiffError::FormatError(
                    TiffFormatError::InconsistentSizesEncountered,
                ));
            }
            Some(n) => n as u16,
        };

        let sample_format = match tag_reader.find_tag_uint_vec(Tag::SampleFormat)? {
            Some(vals) => {
                let sample_format: Vec<_> = vals
                    .into_iter()
                    .map(SampleFormat::from_u16_exhaustive)
                    .collect();

                let Some(format) = sample_format.first().copied() else {
                    // Reject empty sample formats
                    return Err(TiffFormatError::InvalidTagValueType(Tag::SampleFormat).into());
                };
                // TODO: for now, only homogenous formats across samples are supported.
                if !sample_format.iter().all(|&s| s == format) {
                    return Err(TiffUnsupportedError::UnsupportedSampleFormat(sample_format).into());
                }
                format
            }
            None => SampleFormat::Uint,
        };

        let bits_per_sample: Vec<u8> = tag_reader
            .find_tag_uint_vec(Tag::BitsPerSample)?
            .unwrap_or_else(|| vec![1]);

        // Technically bits_per_sample.len() should be *equal* to samples, but libtiff also allows
        // it to be a single value that applies to all samples.
        if bits_per_sample.len() != usize::from(samples) && bits_per_sample.len() != 1 {
            return Err(TiffError::FormatError(
                TiffFormatError::InconsistentSizesEncountered,
            ));
        }

        // This library (and libtiff) do not support mixed sample formats and zero bits per sample
        // doesn't make sense.
        if bits_per_sample.iter().any(|&b| b != bits_per_sample[0]) || bits_per_sample[0] == 0 {
            return Err(TiffUnsupportedError::InconsistentBitsPerSample(bits_per_sample).into());
        }

        let predictor = tag_reader
            .find_tag(Tag::Predictor)?
            .map(Value::into_u16)
            .transpose()?
            .map(|p| {
                Predictor::from_u16(p)
                    .ok_or(TiffError::FormatError(TiffFormatError::UnknownPredictor(p)))
            })
            .transpose()?
            .unwrap_or(Predictor::None);

        let planar_config = tag_reader
            .find_tag(Tag::PlanarConfiguration)?
            .map(Value::into_u16)
            .transpose()?
            .map(|p| {
                PlanarConfiguration::from_u16(p).ok_or(TiffError::FormatError(
                    TiffFormatError::UnknownPlanarConfiguration(p),
                ))
            })
            .transpose()?
            .unwrap_or(PlanarConfiguration::Chunky);

        let ycbcr_subsampling = tag_reader.find_tag_uint_vec::<u16>(Tag::ChromaSubsampling)?;

        let chroma_subsampling = if let Some(subsamples) = &ycbcr_subsampling {
            let [a, b] = subsamples.as_slice() else {
                return Err(TiffError::FormatError(TiffFormatError::InvalidCountForTag(
                    Tag::ChromaSubsampling,
                    subsamples.len(),
                )));
            };

            // ImageWidth and ImageLength are constrained to be integer multiples of
            // YCbCrSubsampleHoriz and YCbCrSubsampleVert respectively. TileWidth and TileLength
            // have the same constraints. RowsPerStrip must be an integer multiple of
            // YCbCrSubsampleVert.
            (*a, *b)
        } else {
            (2, 2)
        };

        let planes = match planar_config {
            PlanarConfiguration::Chunky => 1,
            PlanarConfiguration::Planar => samples,
        };

        let chunk_type;
        let chunk_offsets;
        let chunk_bytes;
        let strip_decoder;
        let tile_attributes;
        match (
            ifd.contains(Tag::StripByteCounts),
            ifd.contains(Tag::StripOffsets),
            ifd.contains(Tag::TileByteCounts),
            ifd.contains(Tag::TileOffsets),
        ) {
            (true, true, false, false) => {
                chunk_type = ChunkType::Strip;

                chunk_offsets = tag_reader
                    .find_tag(Tag::StripOffsets)?
                    .unwrap()
                    .into_u64_vec()?;
                chunk_bytes = tag_reader
                    .find_tag(Tag::StripByteCounts)?
                    .unwrap()
                    .into_u64_vec()?;
                let rows_per_strip = tag_reader
                    .find_tag(Tag::RowsPerStrip)?
                    .map(Value::into_u32)
                    .transpose()?
                    .unwrap_or(height);
                strip_decoder = Some(StripDecodeState { rows_per_strip });
                tile_attributes = None;

                if chunk_offsets.len() != chunk_bytes.len()
                    || rows_per_strip == 0
                    || u32::try_from(chunk_offsets.len())?
                        != (height.saturating_sub(1) / rows_per_strip + 1) * planes as u32
                {
                    return Err(TiffError::FormatError(
                        TiffFormatError::InconsistentSizesEncountered,
                    ));
                }
            }
            (false, false, true, true) => {
                chunk_type = ChunkType::Tile;

                let tile_width =
                    usize::try_from(tag_reader.require_tag(Tag::TileWidth)?.into_u32()?)?;
                let tile_length =
                    usize::try_from(tag_reader.require_tag(Tag::TileLength)?.into_u32()?)?;

                if tile_width == 0 {
                    return Err(TiffFormatError::InvalidTagValueType(Tag::TileWidth).into());
                } else if tile_length == 0 {
                    return Err(TiffFormatError::InvalidTagValueType(Tag::TileLength).into());
                }

                strip_decoder = None;
                tile_attributes = Some(TileAttributes {
                    image_width: usize::try_from(width)?,
                    image_height: usize::try_from(height)?,
                    tile_width,
                    tile_length,
                });
                chunk_offsets = tag_reader
                    .find_tag(Tag::TileOffsets)?
                    .unwrap()
                    .into_u64_vec()?;
                chunk_bytes = tag_reader
                    .find_tag(Tag::TileByteCounts)?
                    .unwrap()
                    .into_u64_vec()?;

                let tile = tile_attributes.as_ref().unwrap();
                if chunk_offsets.len() != chunk_bytes.len()
                    || chunk_offsets.len()
                        != tile.tiles_down() * tile.tiles_across() * planes as usize
                {
                    return Err(TiffError::FormatError(
                        TiffFormatError::InconsistentSizesEncountered,
                    ));
                }
            }
            (_, _, _, _) => {
                return Err(TiffError::FormatError(
                    TiffFormatError::StripTileTagConflict,
                ))
            }
        };

        Ok(Image {
            ifd: Some(ifd),
            width,
            height,
            bits_per_sample: bits_per_sample[0],
            samples,
            extra_samples,
            photometric_samples,
            sample_format,
            photometric_interpretation,
            compression_method,
            jpeg_tables,
            predictor,
            chunk_type,
            planar_config,
            strip_decoder,
            tile_attributes,
            chunk_offsets,
            chunk_bytes,
            chroma_subsampling,
        })
    }

    pub(crate) fn colortype(&self) -> TiffResult<ColorType> {
        let is_alpha_extra_samples = matches!(
            self.extra_samples.as_slice(),
            [ExtraSamples::AssociatedAlpha, ..] | [ExtraSamples::UnassociatedAlpha, ..]
        );

        match self.photometric_interpretation {
            PhotometricInterpretation::RGB => match self.photometric_samples {
                3 => Ok(if is_alpha_extra_samples {
                    ColorType::RGBA(self.bits_per_sample)
                } else {
                    ColorType::RGB(self.bits_per_sample)
                }),
                4 => Ok(ColorType::RGBA(self.bits_per_sample)),
                _ => Err(TiffError::UnsupportedError(
                    TiffUnsupportedError::InterpretationWithBits(
                        self.photometric_interpretation,
                        vec![self.bits_per_sample; self.samples as usize],
                    ),
                )),
            },
            PhotometricInterpretation::CMYK => match self.photometric_samples {
                4 => Ok(if is_alpha_extra_samples {
                    ColorType::CMYKA(self.bits_per_sample)
                } else {
                    ColorType::CMYK(self.bits_per_sample)
                }),
                5 => Ok(ColorType::CMYKA(self.bits_per_sample)),
                _ => Err(TiffError::UnsupportedError(
                    TiffUnsupportedError::InterpretationWithBits(
                        self.photometric_interpretation,
                        vec![self.bits_per_sample; self.samples as usize],
                    ),
                )),
            },
            PhotometricInterpretation::YCbCr => match self.photometric_samples {
                3 => Ok(ColorType::YCbCr(self.bits_per_sample)),
                _ => Err(TiffError::UnsupportedError(
                    TiffUnsupportedError::InterpretationWithBits(
                        self.photometric_interpretation,
                        vec![self.bits_per_sample; self.samples as usize],
                    ),
                )),
            },
            // TODO: treatment of WhiteIsZero is not quite consistent with `invert_colors` that is
            // later called when that interpretation is read. That function does not support
            // Multiband as a color type and will error. It's unclear how to resolve that exactly.
            PhotometricInterpretation::BlackIsZero | PhotometricInterpretation::WhiteIsZero => {
                // Note: compatibility with previous implementation requires us to return extra
                // samples as `Multiband`. For gray images however the better choice would be
                // returning a `Gray` color, i.e. matching on `photometric_samples` instead.
                match self.samples {
                    1 => Ok(ColorType::Gray(self.bits_per_sample)),
                    _ => Ok(ColorType::Multiband {
                        bit_depth: self.bits_per_sample,
                        num_samples: self.samples,
                    }),
                }
            }
            // ```
            // struct IccLab /* Interpretation 9* {
            //     pub L: u8, // SampleFormat::Uint
            //     pub a: u8, // SampleFormat::Uint, defined as TiffLab::a + 128
            //     pub b: u8, // SampleFormat::Uint, defined as TiffLab::b + 128
            // }
            // ```
            PhotometricInterpretation::IccLab => match self.photometric_samples {
                3 if matches!(self.sample_format, SampleFormat::Uint) => {
                    Ok(ColorType::Lab(self.bits_per_sample))
                }
                _ => Err(TiffError::UnsupportedError(
                    TiffUnsupportedError::InterpretationWithBits(
                        self.photometric_interpretation,
                        vec![self.bits_per_sample; self.samples as usize],
                    ),
                )),
            },
            // Unsupported due to inherently heterogeneous sample types. This is represented as:
            // ```
            // struct TiffLab /* Interpretation 8* {
            //     pub L: u8, // SampleFormat::Uint
            //     pub a: i8, // SampleFormat::Int
            //     pub b: i8, // SampleFormat::Int
            // }
            // ```
            PhotometricInterpretation::CIELab => Err(TiffError::UnsupportedError(
                TiffUnsupportedError::InterpretationWithBits(
                    PhotometricInterpretation::CIELab,
                    vec![self.bits_per_sample; self.samples as usize],
                ),
            )),
            // Unsupported due to extra unfiltering and conversion steps. We need to find the
            // Decode tag (SRATIONAL; 2 * SamplesPerPixel) and apply the following conversion:
            //
            // L* = Decode[0] + Lsample x (Decode[1] - Decode[0]) / (2^n -1)
            // …
            //
            // So we'll have a larger depth in the output and either worry about reducing fractions
            // or turn everything into floats. That's a lot of decisions.
            PhotometricInterpretation::ItuLab => Err(TiffError::UnsupportedError(
                TiffUnsupportedError::InterpretationWithBits(
                    PhotometricInterpretation::CIELab,
                    vec![self.bits_per_sample; self.samples as usize],
                ),
            )),
            PhotometricInterpretation::RGBPalette | PhotometricInterpretation::TransparencyMask => {
                Err(TiffError::UnsupportedError(
                    TiffUnsupportedError::InterpretationWithBits(
                        self.photometric_interpretation,
                        vec![self.bits_per_sample; self.samples as usize],
                    ),
                ))
            }
        }
    }

    fn create_reader<'r, R: 'r + Read + Seek>(
        reader: R,
        compression_method: CompressionMethod,
        compressed_length: u64,
        // FIXME: these should be `expect` attributes or we choose another way of passing them.
        #[cfg_attr(not(feature = "jpeg"), allow(unused_variables))] jpeg_tables: Option<&[u8]>,
        #[cfg_attr(not(feature = "fax"), allow(unused_variables))] dimensions: (u32, u32),
        #[cfg_attr(not(feature = "webp"), allow(unused_variables))] samples: u16,
    ) -> TiffResult<Box<dyn Read + 'r>> {
        Ok(match compression_method {
            CompressionMethod::None => Box::new(reader),
            #[cfg(feature = "lzw")]
            CompressionMethod::LZW => Box::new(super::stream::LZWReader::new(
                reader,
                usize::try_from(compressed_length)?,
            )),
            #[cfg(feature = "zstd")]
            CompressionMethod::ZSTD => Box::new(zstd::Decoder::new(reader)?),
            CompressionMethod::PackBits => Box::new(PackBitsReader::new(reader, compressed_length)),
            #[cfg(feature = "deflate")]
            CompressionMethod::Deflate | CompressionMethod::OldDeflate => {
                Box::new(super::stream::DeflateReader::new(reader))
            }
            #[cfg(feature = "jpeg")]
            CompressionMethod::ModernJPEG => {
                use zune_jpeg::zune_core;

                if jpeg_tables.is_some() && compressed_length < 2 {
                    return Err(TiffError::FormatError(
                        TiffFormatError::InvalidTagValueType(Tag::JPEGTables),
                    ));
                }

                // Construct new jpeg_reader wrapping a SmartReader.
                //
                // JPEG compression in TIFF allows saving quantization and/or huffman tables in one
                // central location. These `jpeg_tables` are simply prepended to the remaining jpeg image data.
                // Because these `jpeg_tables` start with a `SOI` (HEX: `0xFFD8`) or __start of image__ marker
                // which is also at the beginning of the remaining JPEG image data and would
                // confuse the JPEG renderer, one of these has to be taken off. In this case the first two
                // bytes of the remaining JPEG data is removed because it follows `jpeg_tables`.
                // Similary, `jpeg_tables` ends with a `EOI` (HEX: `0xFFD9`) or __end of image__ marker,
                // this has to be removed as well (last two bytes of `jpeg_tables`).
                let mut jpeg_reader = match jpeg_tables {
                    Some(jpeg_tables) => {
                        let mut reader = reader.take(compressed_length);
                        reader.read_exact(&mut [0; 2])?;

                        Box::new(
                            Cursor::new(&jpeg_tables[..jpeg_tables.len() - 2])
                                .chain(reader.take(compressed_length)),
                        ) as Box<dyn Read>
                    }
                    None => Box::new(reader.take(compressed_length)),
                };

                let mut jpeg_data = Vec::new();
                jpeg_reader.read_to_end(&mut jpeg_data)?;

                let mut decoder =
                    zune_jpeg::JpegDecoder::new(zune_core::bytestream::ZCursor::new(jpeg_data));
                let mut options: zune_core::options::DecoderOptions = Default::default();

                // Disable color conversion by setting the output colorspace to the input
                // colorspace.
                decoder.decode_headers()?;
                if let Some(colorspace) = decoder.input_colorspace() {
                    options = options.jpeg_set_out_colorspace(colorspace);
                }

                decoder.set_options(options);

                let data = decoder.decode()?;

                Box::new(Cursor::new(data))
            }
            #[cfg(feature = "fax")]
            CompressionMethod::Fax4 => Box::new(super::stream::Group4Reader::new(
                dimensions,
                reader,
                compressed_length,
            )?),
            #[cfg(feature = "webp")]
            CompressionMethod::WebP => Box::new(super::stream::WebPReader::new(
                reader,
                compressed_length,
                samples,
            )?),

            method => {
                return Err(TiffError::UnsupportedError(
                    TiffUnsupportedError::UnsupportedCompressionMethod(method),
                ))
            }
        })
    }

    /// Samples per pixel within chunk.
    ///
    /// In planar config, samples are stored in separate strips/chunks, also called bands.
    ///
    /// Example with `bits_per_sample = [8, 8, 8]` and `PhotometricInterpretation::RGB`:
    /// * `PlanarConfiguration::Chunky` -> 3 (RGBRGBRGB...)
    /// * `PlanarConfiguration::Planar` -> 1 (RRR...) (GGG...) (BBB...)
    pub(crate) fn samples_per_pixel(&self) -> u16 {
        match self.planar_config {
            PlanarConfiguration::Chunky => self.samples,
            PlanarConfiguration::Planar => 1,
        }
    }

    pub(crate) fn samples_per_out_texel(&self, color: ColorType) -> u16 {
        match self.planar_config {
            PlanarConfiguration::Chunky => color.num_samples(),
            PlanarConfiguration::Planar => 1,
        }
    }

    /// Number of strips per pixel.
    pub(crate) fn strips_per_pixel(&self) -> u16 {
        match self.planar_config {
            PlanarConfiguration::Chunky => 1,
            PlanarConfiguration::Planar => self.samples,
        }
    }

    pub(crate) fn chunk_file_range(&self, chunk: u32) -> TiffResult<(u64, u64)> {
        let file_offset = self
            .chunk_offsets
            .get(chunk as usize)
            .ok_or(TiffError::FormatError(
                TiffFormatError::InconsistentSizesEncountered,
            ))?;

        let compressed_bytes =
            self.chunk_bytes
                .get(chunk as usize)
                .ok_or(TiffError::FormatError(
                    TiffFormatError::InconsistentSizesEncountered,
                ))?;

        Ok((*file_offset, *compressed_bytes))
    }

    pub(crate) fn chunk_dimensions(&self) -> TiffResult<(u32, u32)> {
        match self.chunk_type {
            ChunkType::Strip => {
                let strip_attrs = self.strip_decoder.as_ref().unwrap();
                Ok((self.width, strip_attrs.rows_per_strip))
            }
            ChunkType::Tile => {
                let tile_attrs = self.tile_attributes.as_ref().unwrap();
                Ok((
                    u32::try_from(tile_attrs.tile_width)?,
                    u32::try_from(tile_attrs.tile_length)?,
                ))
            }
        }
    }

    pub(crate) fn readout_for_image(&self) -> TiffResult<ReadoutLayout> {
        let Image { width, height, .. } = *self;
        self.readout_for_size(width, height)
    }

    /// Get the layout for reading out a tile-aligned portion of the image.
    ///
    /// The provided width and height should be less than or equal to the image dimensions.
    pub(crate) fn readout_for_size(&self, width: u32, height: u32) -> TiffResult<ReadoutLayout> {
        let color = self.colortype()?;

        let tiff_samples = self.samples_per_pixel();
        let tiff_bits_per_sample = self.bits_per_sample;
        let data_samples = self.samples_per_out_texel(color);
        let tiff_chunk_dimensions = self.chunk_dimensions()?;
        let strips_per_pixel = self.strips_per_pixel();

        let data_dimensions = (width, height);

        let tiff_row_bits = (u64::from(tiff_chunk_dimensions.0) * u64::from(tiff_bits_per_sample))
            .checked_mul(u64::from(tiff_samples))
            .ok_or(TiffError::LimitsExceeded)?;
        let tiff_row_bytes: usize = tiff_row_bits.div_ceil(8).try_into()?;

        let chunk_row_bits = (u64::from(tiff_chunk_dimensions.0) * u64::from(tiff_bits_per_sample))
            .checked_mul(u64::from(data_samples))
            .ok_or(TiffError::LimitsExceeded)?;
        let chunk_row_bytes: usize = chunk_row_bits.div_ceil(8).try_into()?;

        let data_row_bits = (u64::from(data_dimensions.0) * u64::from(tiff_bits_per_sample))
            .checked_mul(u64::from(data_samples))
            .ok_or(TiffError::LimitsExceeded)?;
        let data_row_bytes: usize = data_row_bits.div_ceil(8).try_into()?;

        let chunk_col_stride: usize = data_row_bits
            .div_ceil(8)
            .checked_mul(u64::from(tiff_chunk_dimensions.1))
            .ok_or(TiffError::LimitsExceeded)?
            .try_into()?;

        let plane_stride: usize = data_row_bits
            .div_ceil(8)
            .checked_mul(u64::from(data_dimensions.1))
            .ok_or(TiffError::LimitsExceeded)?
            .try_into()?;

        let minimum_row_stride = data_row_bytes;

        let chunks_across: u32 = data_dimensions.0.div_ceil(tiff_chunk_dimensions.0);
        let chunks_per_plane = (self.chunk_offsets.len() as u32) / u32::from(strips_per_pixel);

        // We would not get an offset in byte units, sorry, no bit interleaving in the output.
        if chunks_across > 1 && chunk_row_bits % 8 != 0 {
            return Err(TiffError::UnsupportedError(
                TiffUnsupportedError::MisalignedTileBoundaries,
            ));
        }

        // Only this color type interprets the tag, which is defined with a default of (2, 2)
        if matches!(color, ColorType::YCbCr(_)) && self.chroma_subsampling != (1, 1) {
            // The JPEG library does upsampling for us and defines its buffers correctly
            // (presumably). All other compression schemes are not supported..
            //
            // NOTE: as explained in <fa225e820b96bef35f01bf4685654beeb4a8df0c> we may be better
            // off supporting this tag by consistently upsampling, not by adjusting the buffer
            // size. At least as a default this makes more sense and is much more permissive in
            // case the compression stream disagrees with the tags (we would not have enough / or
            // the wrong buffer layout if we only asked for subsampled planes in a planar layout).
            if !matches!(self.compression_method, CompressionMethod::ModernJPEG) {
                return Err(TiffError::UnsupportedError(
                    TiffUnsupportedError::ChromaSubsampling,
                ));
            }
        }

        Ok(ReadoutLayout {
            planar_config: self.planar_config,
            color,
            minimum_row_stride,
            sample_format: self.sample_format,
            row_stride: data_row_bytes,
            chunk_row_stride: chunk_row_bytes,
            chunk_col_stride,
            plane_stride,
            tiff_bits_per_sample,
            tiff_samples,
            tiff_chunk_dimensions,
            tiff_row_bytes,
            chunks_across,
            chunks_per_plane,
        })
    }

    pub(crate) fn chunk_data_dimensions(&self, chunk_index: u32) -> TiffResult<(u32, u32)> {
        let dims = self.chunk_dimensions()?;

        match self.chunk_type {
            ChunkType::Strip => {
                let rows_per_strip = dims.1;
                let strips_per_band = self.height.div_ceil(rows_per_strip);

                let strip_height_without_padding = (chunk_index % strips_per_band)
                    .checked_mul(dims.1)
                    .and_then(|x| self.height.checked_sub(x))
                    .ok_or(TiffError::UsageError(UsageError::InvalidChunkIndex(
                        chunk_index,
                    )))?;

                // Ignore potential vertical padding on the bottommost strip
                let strip_height = dims.1.min(strip_height_without_padding);

                Ok((dims.0, strip_height))
            }
            ChunkType::Tile => {
                let tile_attrs = self.tile_attributes.as_ref().unwrap();
                let (padding_right, padding_down) = tile_attrs.get_padding(chunk_index as usize);

                let tile_width = tile_attrs.tile_width - padding_right;
                let tile_length = tile_attrs.tile_length - padding_down;

                Ok((u32::try_from(tile_width)?, u32::try_from(tile_length)?))
            }
        }
    }

    pub(crate) fn expand_chunk(
        &self,
        reader: &mut ValueReader<impl Read + Seek>,
        buf: &mut [u8],
        layout: &ReadoutLayout,
        chunk_index: u32,
    ) -> TiffResult<()> {
        let ValueReader {
            reader,
            bigtiff: _,
            limits,
        } = reader;

        let byte_order = reader.byte_order;

        // Validate that the color type is supported.
        let color_type = layout.color;

        match color_type {
            ColorType::RGB(n)
            | ColorType::RGBA(n)
            | ColorType::CMYK(n)
            | ColorType::CMYKA(n)
            | ColorType::YCbCr(n)
            | ColorType::Gray(n)
            | ColorType::Multiband {
                bit_depth: n,
                num_samples: _,
            } if n == 8 || n == 16 || n == 32 || n == 64 => {}
            ColorType::Gray(n)
            | ColorType::Multiband {
                bit_depth: n,
                num_samples: _,
            } if n < 8 => match self.predictor {
                Predictor::None => {}
                Predictor::Horizontal => {
                    return Err(TiffError::UnsupportedError(
                        TiffUnsupportedError::HorizontalPredictor(color_type),
                    ));
                }
                Predictor::FloatingPoint => {
                    return Err(TiffError::UnsupportedError(
                        TiffUnsupportedError::FloatingPointPredictor(color_type),
                    ));
                }
            },
            type_ => {
                return Err(TiffError::UnsupportedError(
                    TiffUnsupportedError::UnsupportedColorType(type_),
                ));
            }
        }

        // Validate that the predictor is supported for the sample type.
        match (self.predictor, self.sample_format) {
            (
                Predictor::Horizontal,
                SampleFormat::Int | SampleFormat::Uint | SampleFormat::IEEEFP,
            ) => {}
            (Predictor::Horizontal, _) => {
                return Err(TiffError::UnsupportedError(
                    TiffUnsupportedError::HorizontalPredictor(color_type),
                ));
            }
            (Predictor::FloatingPoint, SampleFormat::IEEEFP) => {}
            (Predictor::FloatingPoint, _) => {
                return Err(TiffError::UnsupportedError(
                    TiffUnsupportedError::FloatingPointPredictor(color_type),
                ));
            }
            _ => {}
        }

        let compressed_bytes =
            self.chunk_bytes
                .get(chunk_index as usize)
                .ok_or(TiffError::FormatError(
                    TiffFormatError::InconsistentSizesEncountered,
                ))?;

        if *compressed_bytes > limits.intermediate_buffer_size as u64 {
            return Err(TiffError::LimitsExceeded);
        }

        let compression_method = self.compression_method;
        let photometric_interpretation = self.photometric_interpretation;
        let predictor = self.predictor;

        let samples = layout.tiff_samples;
        let data_samples = layout.samples_per_out_texel();

        // We have two dimensions: the 2d rectangle of encoded data and the 2d rectangle this
        // takes up in the output. Each has an associated count of bits per pixel. The first
        // dimension, i.e. a ''row'', is the number of pixels that are encoded with bit packing
        // while the second is the byte-padded array of each so encoded slices.
        //
        // During decoding we map the relevant bits from one to the other.
        let chunk_dims = self.chunk_dimensions()?;
        let data_dims = self.chunk_data_dimensions(chunk_index)?;

        let chunk_row_bytes: usize = layout.tiff_row_bytes;
        let data_row_bytes: usize = layout.chunk_row_bytes(data_dims.0)?;

        // TODO: Should these return errors instead?
        assert!(layout.minimum_row_stride >= data_row_bytes);
        assert!(buf.len() >= layout.row_stride * (data_dims.1 as usize - 1) + data_row_bytes);

        let is_all_bits = samples == data_samples;
        let is_output_chunk_rows = layout.row_stride == chunk_row_bytes;

        let mut reader = Self::create_reader(
            reader.inner(),
            compression_method,
            *compressed_bytes,
            self.jpeg_tables.as_deref().map(|a| &**a),
            chunk_dims,
            self.samples,
        )?;

        if is_output_chunk_rows && is_all_bits {
            // Here we can read directly into the output buffer itself.
            let tile = &mut buf[..chunk_row_bytes * data_dims.1 as usize];
            reader.read_exact(tile)?;

            for row in tile.chunks_mut(chunk_row_bytes) {
                super::fix_endianness_and_predict(
                    row,
                    color_type.bit_depth(),
                    samples,
                    byte_order,
                    predictor,
                );
            }

            if photometric_interpretation == PhotometricInterpretation::WhiteIsZero {
                super::invert_colors(tile, color_type, self.sample_format)?;
            }
        } else if chunk_row_bytes > data_row_bytes && self.predictor == Predictor::FloatingPoint {
            // The floating point predictor shuffles the padding bytes into the encoded output, so
            // this case is handled specially when needed.
            let mut encoded = vec![0u8; chunk_row_bytes];
            for row in buf.chunks_mut(layout.row_stride).take(data_dims.1 as usize) {
                reader.read_exact(&mut encoded)?;

                let row = &mut row[..data_row_bytes];
                match color_type.bit_depth() {
                    16 => predict_f16(&mut encoded, row, samples),
                    32 => predict_f32(&mut encoded, row, samples),
                    64 => predict_f64(&mut encoded, row, samples),
                    _ => unreachable!(),
                }
                if photometric_interpretation == PhotometricInterpretation::WhiteIsZero {
                    super::invert_colors(row, color_type, self.sample_format)?;
                }
            }
        } else if is_all_bits {
            // We read row-by-row but each row fits in its output buffer.
            for row in buf.chunks_mut(layout.row_stride).take(data_dims.1 as usize) {
                let row = &mut row[..data_row_bytes];
                let used = data_row_bytes.min(chunk_row_bytes);

                // Two ways how we get here: we have more bytes in our chunk data than in the image
                // we are to read. Then we need to skip the rest of the data. Or we have a bigger
                // row stride than the chunk contains data, then we  need to fill only the front.
                reader.read_exact(&mut row[..used])?;
                // Skip horizontal padding
                if chunk_row_bytes > data_row_bytes {
                    let len = u64::try_from(chunk_row_bytes - data_row_bytes)?;
                    io::copy(&mut reader.by_ref().take(len), &mut io::sink())?;
                }

                super::fix_endianness_and_predict(
                    row,
                    color_type.bit_depth(),
                    samples,
                    byte_order,
                    predictor,
                );

                if photometric_interpretation == PhotometricInterpretation::WhiteIsZero {
                    super::invert_colors(row, color_type, self.sample_format)?;
                }
            }
        } else {
            // The encoded data potentially takes up more space than the output data so we must be
            // prepared to discard some of it. That decision is bit-by-bit.
            let bits_per_pixel = u32::from(self.bits_per_sample) * u32::from(self.samples);
            // Assumes the photometric samples are always the start.. This is slightly problematic.
            // To expand spport we should instead have different methods of transforming the read
            // buffer data, not only the `compact_photometric_bytes` method below and then choose
            // from the right one with supplied parameters. Then we can also bit-for-bit copy with
            // a selection for better performance.
            let photometric_bit_end = u32::from(self.bits_per_sample) * data_samples as u32;

            debug_assert!(bits_per_pixel >= photometric_bit_end);

            if bits_per_pixel % 8 != 0 || photometric_bit_end % 8 != 0 {
                return Err(TiffError::UnsupportedError(
                    TiffUnsupportedError::InterpretationWithBits(
                        self.photometric_interpretation,
                        vec![self.bits_per_sample; self.samples as usize],
                    ),
                ));
            }

            let photo_range = photometric_bit_end / 8..bits_per_pixel / 8;
            let mut encoded = vec![0u8; chunk_row_bytes];
            for row in buf.chunks_mut(layout.row_stride).take(data_dims.1 as usize) {
                reader.read_exact(&mut encoded)?;

                Self::compact_photometric_bytes(&mut encoded, row, &photo_range);

                super::fix_endianness_and_predict(
                    row,
                    color_type.bit_depth(),
                    samples,
                    byte_order,
                    predictor,
                );

                if photometric_interpretation == PhotometricInterpretation::WhiteIsZero {
                    super::invert_colors(row, color_type, self.sample_format)?;
                }
            }
        }

        Ok(())
    }

    /// Turn a contiguous buffer of a whole number of raw sample arrays into a whole number of
    /// photometric sample arrays by removing the extra samples in-between.
    fn compact_photometric_bytes(
        raw: &mut [u8],
        row: &mut [u8],
        photo_range: &std::ops::Range<u32>,
    ) {
        raw.chunks_exact_mut(photo_range.end as usize)
            .zip(row.chunks_exact_mut(photo_range.start as usize))
            .for_each(|(src, dst)| {
                dst.copy_from_slice(&src[..photo_range.start as usize]);
            });
    }
}

impl ReadoutLayout {
    pub(crate) fn samples_per_out_texel(&self) -> u16 {
        match self.planar_config {
            PlanarConfiguration::Chunky => self.color.num_samples(),
            PlanarConfiguration::Planar => 1,
        }
    }

    // For a concrete chunk, which may be a partial border chunk, the byte length of one row of its
    // pixel data.
    pub(crate) fn chunk_row_bytes(&self, width: u32) -> TiffResult<usize> {
        let data_samples = self.samples_per_out_texel();
        let data_row_bits = (u64::from(width) * u64::from(self.tiff_bits_per_sample))
            .checked_mul(u64::from(data_samples))
            .ok_or(TiffError::LimitsExceeded)?;
        Ok(data_row_bits.div_ceil(8).try_into()?)
    }

    pub(crate) fn set_row_stride(&mut self, row_stride: usize) -> Result<(), TiffError> {
        if row_stride < self.minimum_row_stride {
            return Err(TiffError::UsageError(
                UsageError::InsufficientOutputRowStride {
                    needed: self.minimum_row_stride,
                    requested: row_stride,
                },
            ));
        }

        let data_row_bytes = u64::try_from(row_stride)?;

        let chunk_col_stride = data_row_bytes
            .checked_mul(u64::from(self.tiff_chunk_dimensions.1))
            .ok_or(TiffError::LimitsExceeded)?
            .try_into()?;

        let height = self.plane_stride.checked_div(self.row_stride);

        let plane_stride = height
            .and_then(|h| data_row_bytes.checked_mul(h as u64))
            // If height was zero, or the previous stride was zero, there are no bytes in a plane
            .unwrap_or(0)
            .try_into()?;

        self.row_stride = row_stride;
        self.chunk_col_stride = chunk_col_stride;
        self.plane_stride = plane_stride;

        Ok(())
    }

    /// Reduce this down to the layout of the output planes.
    pub(crate) fn to_plane_layout(&self) -> Result<PlaneLayout, TiffError> {
        let num_planes = self.color.num_samples() / self.samples_per_out_texel();

        // Using the standard range iterator as checked_add on steroids.
        //
        // Note: for supporting subsampling, adjust as required.
        let mut offset = (0..=usize::MAX).step_by(self.plane_stride);

        let plane_offsets = offset.by_ref().take(usize::from(num_planes)).collect();

        // Get the past-the-end of the last plane.
        //
        // This also verifies the `take` above was not short.
        let Some(total_bytes) = offset.next() else {
            return Err(TiffError::LimitsExceeded);
        };

        Ok(PlaneLayout {
            plane_offsets,
            total_bytes,
            readout: self.clone(),
        })
    }
}

/// A `ReadoutLayout` with pre-calculated plane information.
pub(crate) struct PlaneLayout {
    /// The underlying readout layout.
    pub readout: ReadoutLayout,
    /// Buffer offset from one plane of output to the next.
    pub plane_offsets: Vec<usize>,
    /// Total number of bytes for all planes in given order.
    pub total_bytes: usize,
}

impl PlaneLayout {
    /// Return the number of planes to extract into the provided buffer.
    pub(crate) fn used_planes(&self, buffer: &[impl Sized]) -> TiffResult<u16> {
        self.readout.assert_min_layout(buffer)?;
        let buffer_len = core::mem::size_of_val(buffer);

        // Note: with differently sized planes this is dependent on the plane.
        let last_plane_start = buffer_len.checked_sub(self.readout.plane_stride);

        // Find how many planes fit into the output buffer.
        let used_plane_offsets = self
            .plane_offsets
            .iter()
            .enumerate()
            // Find the first plane that would not fit completely at its offset.
            .skip_while(|(_, &offset)| last_plane_start >= Some(offset))
            .nth(0)
            // If all planes fit, use all of them.
            .map_or(self.plane_offsets.len(), |(idx, _)| idx);

        debug_assert!(
            used_plane_offsets <= usize::from(u16::MAX),
            "Planes limited by number of samples, which is encoded as u16"
        );

        Ok(used_plane_offsets as u16)
    }
}
