/*
 * Copyright (c) 2023.
 *
 * This software is free software;
 *
 * You can redistribute it or modify it under terms of the MIT, Apache License or Zlib license
 */

use alloc::vec::Vec;
use alloc::{format, vec};
use core::cmp::min;

use zune_core::bytestream::ZByteReaderTrait;
use zune_core::colorspace::ColorSpace;
use zune_core::colorspace::ColorSpace::Luma;
use zune_core::log::{error, trace, warn};

use crate::bitstream::BitStream;
use crate::components::SampleRatios;
use crate::decoder::MAX_COMPONENTS;
use crate::errors::DecodeErrors;
use crate::marker::Marker;
use crate::mcu_prog::get_marker;
use crate::misc::{calculate_padded_width, setup_component_params};
use crate::worker::{color_convert, upsample};
use crate::JpegDecoder;

/// The size of a DC block for a MCU.

pub const DCT_BLOCK: usize = 64;

impl<T: ZByteReaderTrait> JpegDecoder<T> {
    /// Check for existence of DC and AC Huffman Tables
    pub(crate) fn check_tables(&self) -> Result<(), DecodeErrors> {
        // check that dc and AC tables exist outside the hot path
        for component in &self.components {
            let _ = &self
                .dc_huffman_tables
                .get(component.dc_huff_table)
                .as_ref()
                .ok_or_else(|| {
                    DecodeErrors::HuffmanDecode(format!(
                        "No Huffman DC table for component {:?} ",
                        component.component_id
                    ))
                })?
                .as_ref()
                .ok_or_else(|| {
                    DecodeErrors::HuffmanDecode(format!(
                        "No DC table for component {:?}",
                        component.component_id
                    ))
                })?;

            let _ = &self
                .ac_huffman_tables
                .get(component.ac_huff_table)
                .as_ref()
                .ok_or_else(|| {
                    DecodeErrors::HuffmanDecode(format!(
                        "No Huffman AC table for component {:?} ",
                        component.component_id
                    ))
                })?
                .as_ref()
                .ok_or_else(|| {
                    DecodeErrors::HuffmanDecode(format!(
                        "No AC table for component {:?}",
                        component.component_id
                    ))
                })?;
        }
        Ok(())
    }

    /// Decode MCUs and carry out post processing.
    ///
    /// This is the main decoder loop for the library, the hot path.
    ///
    /// Because of this, we pull in some very crazy optimization tricks hence readability is a pinch
    /// here.
    #[allow(
        clippy::similar_names,
        clippy::too_many_lines,
        clippy::cast_possible_truncation
    )]
    #[inline(never)]
    pub(crate) fn decode_mcu_ycbcr_baseline(
        &mut self, pixels: &mut [u8]
    ) -> Result<(), DecodeErrors> {
        setup_component_params(self)?;

        // check dc and AC tables
        self.check_tables()?;

        let (mut mcu_width, mut mcu_height);

        if self.is_interleaved {
            // set upsampling functions
            self.set_upsampling()?;

            mcu_width = self.mcu_x;
            mcu_height = self.mcu_y;
        } else {
            // For non-interleaved images( (1*1) subsampling)
            // number of MCU's are the widths (+7 to account for paddings) divided bu 8.
            mcu_width = (self.info.width as usize + 7) / 8;
            mcu_height = (self.info.height as usize + 7) / 8;
        }
        if self.is_interleaved
            && self.input_colorspace.num_components() > 1
            && self.options.jpeg_get_out_colorspace().num_components() == 1
            && (self.info.sample_ratio == SampleRatios::V
                || self.info.sample_ratio == SampleRatios::HV)
        {
            // For a specific set of images, e.g interleaved,
            // when converting from YcbCr to grayscale, we need to
            // take into account mcu height since the MCU decoding needs to take
            // it into account for padding purposes and the post processor
            // parses two rows per mcu width.
            //
            // set coeff to be 2 to ensure that we increment two rows
            // for every mcu processed also
            mcu_height *= self.v_max;
            mcu_height /= self.h_max;
            self.coeff = 2;
        }

        if self.input_colorspace == ColorSpace::Luma && self.is_interleaved {
            warn!("Grayscale image with down-sampled component, resetting component details");

            self.reset_params();

            mcu_width = ((self.info.width + 7) / 8) as usize;
            mcu_height = ((self.info.height + 7) / 8) as usize;
        }
        let width = usize::from(self.info.width);

        let padded_width = calculate_padded_width(width, self.info.sample_ratio);

        let mut stream = BitStream::new();
        let mut tmp = [0_i32; DCT_BLOCK];

        let comp_len = self.components.len();

        for (pos, comp) in self.components.iter_mut().enumerate() {
            // Allocate only needed components.
            //
            // For special colorspaces i.e YCCK and CMYK, just allocate all of the needed
            // components.
            if min(
                self.options.jpeg_get_out_colorspace().num_components() - 1,
                pos
            ) == pos
                || comp_len == 4
            // Special colorspace
            {
                // allocate enough space to hold a whole MCU width
                // this means we should take into account sampling ratios
                // `*8` is because each MCU spans 8 widths.
                let len = comp.width_stride * comp.vertical_sample * 8;

                comp.needed = true;
                comp.raw_coeff = vec![0; len];
            } else {
                comp.needed = false;
            }
        }

        // If all components are contained in the first scan of MCUs, then we can process into
        // (upsampled) pixels immediately after each MCU, for convenience we use each row of MCUS.
        // Otherwise, we must first wait until following SOS provide the remaining components.
        let all_components_in_first_scan = usize::from(self.num_scans) == self.components.len();
        let mut progressive_mcus: [Vec<i16>; 4] = core::array::from_fn(|_| vec![]);

        if !all_components_in_first_scan {
            for (component, mcu) in self.components.iter().zip(&mut progressive_mcus) {
                let len = mcu_width
                    * component.vertical_sample
                    * component.horizontal_sample
                    * mcu_height
                    * 64;
                *mcu = vec![0; len];
            }
        }

        let mut pixels_written = 0;

        let is_hv = usize::from(self.is_interleaved);
        let upsampler_scratch_size = is_hv * self.components.iter().map(|x| x.width_stride).max().unwrap_or(0) * 8;
        let mut upsampler_scratch_space = vec![0; upsampler_scratch_size];

        'sos: loop {
            trace!(
                "Baseline decoding of components: {:?}",
                &self.z_order[..usize::from(self.num_scans)]
            );

            trace!("Decoding MCU width: {mcu_width}, height: {mcu_height}");

            for i in 0..mcu_height {
                if stream.overread_by > 0 {
                    pixels.get_mut(pixels_written..).map(|v| v.fill(128));
                    if self.options.strict_mode() {
                        return Err(DecodeErrors::FormatStatic("Premature end of buffer"));
                    };

                    error!("Premature end of buffer");
                    break;
                }

                // decode a whole MCU width,
                // this takes into account interleaved components.
                let terminate = if all_components_in_first_scan {
                    self.decode_mcu_width::<false>(
                        mcu_width,
                        i,
                        &mut tmp,
                        &mut stream,
                        &mut progressive_mcus
                    )?
                } else {
                    /* NB: (cae). This code was added due to the issue at https://github.com/etemesi254/zune-image/issues/277
                    *
                    * There is a particular set of images that interleave the start of scan (SOS) with the MCU,
                    * E.g if it's a three component image, we have SOS->MCU ->SOS->MCU ->SOS->MCU
                    * which presents a problem on decoding, we need to buffer the whole image before continuing since
                    * we won't have a row containing all the component data which will be needed e.g for color conversion.
                    *
                    * The mechanisms is that we decode the whole image upfront, which goes against the normal
                    * routine of decoding MCU width , so this requires more memory upfront than initial routines
                    * but it is a single image out of the many corpuses that exist, so its fine.
                    * (image in test-images/jpeg/sos_news.jpeg)

                    * Code contributed by  Aurelia Molzer (https://github.com/197g)

                    *
                    */

                    self.decode_mcu_width::<true>(
                        mcu_width,
                        i,
                        &mut tmp,
                        &mut stream,
                        &mut progressive_mcus
                    )?
                };

                // process that width up until it's impossible. This is faster than allocation the
                // full components, which we skipped earlier.
                if all_components_in_first_scan {
                    self.post_process(
                        pixels,
                        i,
                        mcu_height,
                        width,
                        padded_width,
                        &mut pixels_written,
                        &mut upsampler_scratch_space
                    )?;
                }

                match terminate {
                    McuContinuation::Ok => {}
                    McuContinuation::AnotherSos if all_components_in_first_scan => {
                        warn!("More than one SOS despite already having all components");
                        return Ok(());
                    }
                    McuContinuation::AnotherSos => continue 'sos,
                    McuContinuation::InterScanMarker(marker) => {
                        // Handle inter-scan markers (DHT/DQT/etc) uniformly here.
                        // This keeps all marker handling in the outer loop.
                        if self.advance_to_next_sos(marker, &mut stream)? {
                            continue 'sos;
                        } else {
                            // Hit EOI
                            break;
                        }
                    }
                    McuContinuation::Terminate => {
                        warn!("Got terminate signal, will not process further");
                        pixels.get_mut(pixels_written..).map(|v| v.fill(128));
                        return Ok(());
                    }
                }
            }

            // Breaks if we get here, looping only if we have restarted, i.e. found another SOS and
            // continued at `'sos'.
            break;
        }

        if !all_components_in_first_scan {
            self.finish_baseline_decoding(&progressive_mcus, mcu_width, pixels)?;
        }

        // it may happen that some images don't have the whole buffer
        // so we can't panic in case of that
        // assert_eq!(pixels_written, pixels.len());

        // For UHD usecases that tie two images separating them with EOI and
        // SOI markers, it may happen that we do not reach this image end of image
        // So this ensures we reach it
        // Ensure we read EOI
        if !stream.seen_eoi {
            let marker = get_marker(&mut self.stream, &mut stream);
            match marker {
                Ok(_m) => {
                    trace!("Found marker {:?}", _m);
                }
                Err(_) => {
                    // ignore error
                }
            }
        }

        trace!("Finished decoding image");

        Ok(())
    }

    /// Process all MCUs when baseline decoding has been processing them component-after-component.
    /// For simplicity this assembles the dequantized blocks in the order that the post processing
    /// of an interleaved baseline decoding would use.
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::cast_sign_loss)]
    pub(crate) fn finish_baseline_decoding(
        &mut self, block: &[Vec<i16>; MAX_COMPONENTS], _mcu_width: usize, pixels: &mut [u8]
    ) -> Result<(), DecodeErrors> {
        let mcu_height = self.mcu_y;

        // Size of our output image(width*height)
        let is_hv = usize::from(self.is_interleaved);
        let upsampler_scratch_size = is_hv * self.components[0].width_stride;
        let width = usize::from(self.info.width);
        let padded_width = calculate_padded_width(width, self.info.sample_ratio);

        let mut upsampler_scratch_space = vec![0; upsampler_scratch_size];

        for (pos, comp) in self.components.iter_mut().enumerate() {
            // Mark only needed components for computing output colors.
            if min(
                self.options.jpeg_get_out_colorspace().num_components() - 1,
                pos
            ) == pos
                || self.input_colorspace == ColorSpace::YCCK
                || self.input_colorspace == ColorSpace::CMYK
            {
                comp.needed = true;
            } else {
                comp.needed = false;
            }
        }

        let mut pixels_written = 0;

        // dequantize and idct have been performed, only color convert.
        for i in 0..mcu_height {
            // All the data is already in the right order, we just need to be able to pass it to
            // the post_process & upsample method. That expects all the data to be stored as one
            // row of MCUs in each component's `raw_coeff`.
            'component: for (position, component) in &mut self.components.iter_mut().enumerate() {
                if !component.needed {
                    continue 'component;
                }

                // step is the number of pixels this iteration wil be handling
                // Given by the number of mcu's height and the length of the component block
                // Since the component block contains the whole channel as raw pixels
                // we this evenly divides the pixels into MCU blocks
                //
                // For interleaved images, this gives us the exact pixels comprising a whole MCU
                // block
                let step = block[position].len() / mcu_height;

                // where we will be reading our pixels from.
                let slice = &block[position][i * step..][..step];
                let temp_channel = &mut component.raw_coeff;
                temp_channel[..step].copy_from_slice(slice);
            }

            // process that whole stripe of MCUs
            self.post_process(
                pixels,
                i,
                mcu_height,
                width,
                padded_width,
                &mut pixels_written,
                &mut upsampler_scratch_space
            )?;
        }

        return Ok(());
    }

    fn decode_mcu_width<const PROGRESSIVE: bool>(
        &mut self, mcu_width: usize, mcu_height: usize, tmp: &mut [i32; 64],
        stream: &mut BitStream, progressive: &mut [Vec<i16>; 4]
    ) -> Result<McuContinuation, DecodeErrors> {
        let is_one_by_one = !self.scan_subsampled;

        // The definition of MCU depends on the sampling factor of involved scans. When components
        // have different factors then each Minimal-Coding-Unit is the least common multiple such
        // that we have an integer number of blocks from each component. But the decoding of these
        // components differs from it otherwise, we need an inner loop with a dynamic amount of
        // coefficients per component, whereas otherwise we have exactly one block of coefficients
        // encoded for each component in the bitstream order.
        //
        // We statically specialize on this to improve code generation of the common case a little
        // bit. We could also special case common sub-sampling cases but be mindful of code bloat.
        if is_one_by_one {
            self.inner_decode_mcu_width::<PROGRESSIVE, false>(
                mcu_width,
                mcu_height,
                tmp,
                stream,
                progressive
            )
        } else {
            self.inner_decode_mcu_width::<PROGRESSIVE, true>(
                mcu_width,
                mcu_height,
                tmp,
                stream,
                progressive
            )
        }
    }

    // Inline-never ensures we do get this function optimize on its own, into two different
    // versions, without the optimizer tripping up over the complexity that comes with the
    // constant folding. And constant folding is quite important for performance here as
    // when `not SAMPLED` then the inner loop has exactly one iteration per component in
    // the scan. The difference was ~1% or a bit more.
    fn inner_decode_mcu_width<const PROGRESSIVE: bool, const SAMPLED: bool>(
        &mut self, mcu_width: usize, mcu_height: usize, tmp: &mut [i32; 64],
        stream: &mut BitStream, progressive: &mut [Vec<i16>; 4]
    ) -> Result<McuContinuation, DecodeErrors> {
        let z_order = self.z_order;
        let z_scans = &z_order[..usize::from(self.num_scans)];

        // How much of the head of `tmp` was written by the last MCU decoding? We only check for
        // two different cases and not all possible outcomes as this is only used to optimize the
        // bytes written in `fill`. Since the clobber happens in UNZIGZAG order we'd be straddling
        // most cache lines anyways even if we did a partial write with the exact length of the
        // coefficient data which was written into `tmp`.
        let mut clobber_more_than_4x4 = true;

        // For non-interleaved scans (PROGRESSIVE=true), each scan contains a single component
        // and we iterate over that component's actual data unit count, not the interleaved MCU
        // width multiplied by sampling factor.
        let mut scan_du_width = if PROGRESSIVE {
            let k = z_scans[0];
            let comp = &self.components[k];
            // Calculate actual data units for this component: ceil(width / (8 * subsampling_ratio))
            (self.info.width as usize * comp.horizontal_sample + self.h_max * 8 - 1)
                / (self.h_max * 8)
        } else {
            mcu_width
        };
        // In malformed scans that list multiple components, clamp to the smallest row capacity
        // to avoid writing past the row buffer.
        if PROGRESSIVE && z_scans.len() > 1 {
            let min_du = z_scans
                .iter()
                .map(|&k| self.components[k].width_stride / 8)
                .min()
                .unwrap_or(0);
            scan_du_width = scan_du_width.min(min_du);
        }

        for j in 0..scan_du_width {
            // iterate over components
            for &k in z_scans {
                // we made this loop body massive due to several different paths that depend on
                // static conditions. Note we (potentially) call into other functions so the
                // compiler will not unroll anything here anyways. The gains from separating
                // differently optimized loop bodies are much greater than a single additional jump
                // here.
                let component = &mut self.components[k];

                let dc_table = self.dc_huffman_tables[component.dc_huff_table % MAX_COMPONENTS]
                    .as_ref()
                    .ok_or(DecodeErrors::FormatStatic("DC table not found"))?;

                let ac_table = self.ac_huffman_tables[component.ac_huff_table % MAX_COMPONENTS]
                    .as_ref()
                    .ok_or(DecodeErrors::FormatStatic("AC table not found"))?;

                let qt_table = &component.quantization_table;
                let channel = if PROGRESSIVE {
                    let offset =
                        mcu_height * component.width_stride * 8 * component.vertical_sample;
                    // Small stopgap for https://github.com/etemesi254/zune-image/issues/362
                    if offset >= progressive[k].len(){
                        return Err(DecodeErrors::FormatStatic("Would panic on slice iteration"))
                    }
                    &mut progressive[k][offset..]
                } else {
                    &mut component.raw_coeff
                };

                let component_samples_needed = component.needed;

                // If image is interleaved iterate over scan components,
                // otherwise if it-s non-interleaved, these routines iterate in
                // trivial scanline order(Y,Cb,Cr)
                //
                // Turn the bounds into a compile time constant for a common special case. This
                // allows the compiler to unroll the loop and then do a bunch of interleaving.
                //
                // For PROGRESSIVE (non-interleaved), we iterate data units directly so
                // h_samp/v_samp loops run exactly once.
                let v_step =
                    if SAMPLED && !PROGRESSIVE { 0..component.vertical_sample } else { 0..1 };

                for v_samp in v_step {
                    let h_step =
                        if SAMPLED && !PROGRESSIVE { 0..component.horizontal_sample } else { 0..1 };

                    for h_samp in h_step {
                        let result = if component_samples_needed {
                            // Fill the array with zeroes, decode_mcu_block expects
                            // a zero based array. Clobber is in zig-zag order though.
                            // Writing consecutive entries is basically free in terms
                            // of memory throughput so we opt for a larger power of
                            // two which lets the compiler turn this into a repeated
                            // write of a zeroed vector register, which does not have
                            // any branches, instead of a more difficult pattern where
                            // we attempt to overwrite exactly one coefficient.
                            let clobber_len = if !clobber_more_than_4x4 { 32 } else { 64 };

                            tmp[..clobber_len].fill(0);

                            stream.decode_mcu_block(
                                &mut self.stream,
                                dc_table,
                                ac_table,
                                qt_table,
                                tmp,
                                &mut component.dc_pred
                            )
                        } else {
                            // We do not touch tmp so there is no need to reset it.
                            stream.discard_mcu_block(&mut self.stream, dc_table, ac_table)
                        };

                        // If an error occurs we can either propagate it
                        // as an error or print it and call terminate.
                        //
                        // This allows even corrupt images to render something,
                        // even if its bad, matching browsers.
                        //
                        // See example in https://github.com/etemesi254/zune-image/issues/293
                        let len = if let Ok(len) = result {
                            len
                        } else {
                            // result.is_err()
                            return if self.options.strict_mode() {
                                Err(result.err().unwrap())
                            } else {
                                error!("{}", result.err().unwrap());
                                Ok(McuContinuation::Terminate)
                            };
                        };

                        if component_samples_needed {
                            // tmp was only written partially, note that len is in ZigZag order.
                            clobber_more_than_4x4 = len > 10;

                            let idct_position = if PROGRESSIVE {
                                // For non-interleaved, j indexes data units directly
                                j * 8
                            } else {
                                // derived from stb and rewritten for my tastes
                                let c2 = v_samp * 8;
                                let c3 = ((j * component.horizontal_sample) + h_samp) * 8;

                                component.width_stride * c2 + c3
                            };

                            let idct_pos = channel.get_mut(idct_position..).unwrap();

                            if len <= 1 {
                                (self.idct_1x1_func)(tmp, idct_pos, component.width_stride);
                            } else if len <= 10 {
                                (self.idct_4x4_func)(tmp, idct_pos, component.width_stride);
                            } else {
                                //  call idct.
                                (self.idct_func)(tmp, idct_pos, component.width_stride);
                            }
                        }
                    }
                }
            }

            self.todo = self.todo.wrapping_sub(1);

            if self.todo == 0 {
                self.handle_rst_main(stream)?;
                continue;
            }

            if stream.marker.is_some() && stream.bits_left == 0 {
                break;
            }
        }

        self.check_stream_marker_after_mcu_width(stream)
    }

    fn check_stream_marker_after_mcu_width(
        &mut self, stream: &mut BitStream
    ) -> Result<McuContinuation, DecodeErrors> {
        // After all interleaved components, that's an MCU
        // handle stream markers
        //
        // In some corrupt images, it may occur that header markers occur in the stream.
        // The spec EXPLICITLY FORBIDS this, specifically, in
        // routine F.2.2.5  it says
        // `The only valid marker which may occur within the Huffman coded data is the RSTm marker.`
        //
        // But libjpeg-turbo allows it because of some weird reason. so I'll also
        // allow it because of some weird reason.
        if let Some(m) = stream.marker {
            if m == Marker::EOI {
                // acknowledge and ignore EOI marker.
                stream.marker.take();
                trace!("Found EOI marker");
                // Google Introduced the Ultra-HD image format which is basically
                // stitching two images into one container.
                // They basically separate two images via a EOI and SOI marker
                // so let's just ensure if we ever see EOI, we never read past that
                // ever.
                // https://github.com/google/libultrahdr
                stream.seen_eoi = true;
            } else if let Marker::RST(_) = m {
                //debug_assert_eq!(self.todo, 0);
                if self.todo == 0 {
                    self.handle_rst(stream)?;
                }
            } else if let Marker::SOS = m {
                self.parse_marker_inner(m)?;
                stream.marker.take();
                stream.reset();
                trace!("Found SOS marker");
                return Ok(McuContinuation::AnotherSos);
            } else if matches!(m, Marker::DHT | Marker::DQT | Marker::DRI | Marker::COM)
                || matches!(m, Marker::APP(_))
            {
                // For non-interleaved images, setup markers can appear between scans.
                // Signal the caller to handle this marker and find the next SOS.
                // This keeps all marker parsing in the caller's loop.
                stream.marker.take();
                trace!("Found inter-scan marker {:?}", m);
                return Ok(McuContinuation::InterScanMarker(m));
            } else {
                if self.options.strict_mode() {
                    return Err(DecodeErrors::Format(format!(
                        "Marker {m:?} found where not expected"
                    )));
                }
                error!(
                    "Marker `{:?}` Found within Huffman Stream, possibly corrupt jpeg",
                    m
                );

                self.parse_marker_inner(m)?;
                stream.marker.take();
                stream.reset();
                return Ok(McuContinuation::Terminate);
            }
        }

        Ok(McuContinuation::Ok)
    }

    /// Scan for the next SOS marker, parsing setup markers along the way.
    ///
    /// This is the unified marker scanning function used after encountering an
    /// inter-scan marker. It handles DHT, DQT, DRI, COM, and APP markers that
    /// can appear between scans in non-interleaved images.
    ///
    /// # Arguments
    /// * `first_marker` - The first marker that was already detected (not yet parsed)
    /// * `stream` - The bitstream state
    ///
    /// # Returns
    /// * `Ok(true)` - Found SOS, ready to continue decoding
    /// * `Ok(false)` - Found EOI, decoding complete
    /// * `Err(_)` - Error (too many markers, unexpected marker in strict mode, etc.)
    fn advance_to_next_sos(
        &mut self,
        first_marker: Marker,
        stream: &mut BitStream
    ) -> Result<bool, DecodeErrors> {
        // Limit iterations to prevent DoS from malicious files.
        const MAX_INTER_SCAN_MARKERS: usize = 64;

        // Parse the first marker that triggered this call
        self.parse_marker_inner(first_marker)?;
        stream.reset();

        for _ in 0..MAX_INTER_SCAN_MARKERS {
            let marker = get_marker(&mut self.stream, stream)?;

            match marker {
                Marker::SOS => {
                    self.parse_marker_inner(Marker::SOS)?;
                    stream.reset();
                    trace!("Found SOS marker, continuing decode");
                    return Ok(true);
                }
                Marker::EOI => {
                    stream.seen_eoi = true;
                    trace!("Found EOI marker");
                    return Ok(false);
                }
                Marker::DHT | Marker::DQT | Marker::DRI | Marker::COM => {
                    trace!("Parsing inter-scan marker {:?}", marker);
                    self.parse_marker_inner(marker)?;
                }
                Marker::APP(_) => {
                    trace!("Parsing inter-scan APP marker {:?}", marker);
                    self.parse_marker_inner(marker)?;
                }
                other => {
                    if self.options.strict_mode() {
                        return Err(DecodeErrors::Format(format!(
                            "Unexpected marker {:?} while scanning for SOS between scans",
                            other
                        )));
                    }
                    // Non-strict: skip unknown marker
                    warn!("Skipping unexpected marker {:?} between scans", other);
                    let length = self.stream.get_u16_be_err()?;
                    if length >= 2 {
                        self.stream.skip((length - 2) as usize)?;
                    }
                }
            }
        }

        Err(DecodeErrors::FormatStatic(
            "Too many markers between scans (exceeded limit of 64)"
        ))
    }

    // handle RST markers.
    // No-op if not using restarts
    // this routine is shared with mcu_prog
    #[cold]
    pub(crate) fn handle_rst(&mut self, stream: &mut BitStream) -> Result<(), DecodeErrors> {
        self.todo = self.restart_interval;

        if let Some(marker) = stream.marker {
            // Found a marker
            // Read stream and see what marker is stored there
            match marker {
                Marker::RST(_) => {
                    // reset stream
                    stream.reset();
                    // Initialize dc predictions to zero for all components
                    self.components.iter_mut().for_each(|x| x.dc_pred = 0);
                    // Start iterating again. from position.
                }
                Marker::EOI => {
                    // silent pass
                }
                // Valid markers that can appear between scans at a restart boundary
                // (restart interval aligns with end of scan). Leave for caller.
                Marker::SOS | Marker::DHT | Marker::DQT | Marker::DRI | Marker::COM
                | Marker::APP(_) => {}
                _ => {
                    if self.options.strict_mode() {
                        return Err(DecodeErrors::MCUError(format!(
                            "Unexpected marker {marker:?} at restart boundary"
                        )));
                    }
                    warn!("Unexpected marker {:?} at restart boundary", marker);
                }
            }
        }
        Ok(())
    }
    #[allow(clippy::too_many_lines, clippy::too_many_arguments)]
    pub(crate) fn post_process(
        &mut self, pixels: &mut [u8], i: usize, mcu_height: usize, width: usize,
        padded_width: usize, pixels_written: &mut usize, upsampler_scratch_space: &mut [i16]
    ) -> Result<(), DecodeErrors> {
        let out_colorspace_components = self.options.jpeg_get_out_colorspace().num_components();

        let mut px = *pixels_written;
        // indicates whether image is vertically up-sampled
        let is_vertically_sampled = self
            .components
            .iter()
            .any(|c| c.sample_ratio == SampleRatios::HV || c.sample_ratio == SampleRatios::V);

        let mut comp_len = self.components.len();

        // If we are moving from YCbCr -> Luma, we do not allocate storage for other components, so we
        // will panic when we are trying to read samples, so for that case,
        // hardcode it so that we  don't panic when doing
        //   *samp = &samples[j][pos * padded_width..(pos + 1) * padded_width]
        if out_colorspace_components < comp_len && self.options.jpeg_get_out_colorspace() == Luma {
            comp_len = out_colorspace_components;
        }
        let mut color_conv_function =
            |num_iters: usize, samples: [&[i16]; 4]| -> Result<(), DecodeErrors> {
                for (pos, output) in pixels[px..]
                    .chunks_exact_mut(width * out_colorspace_components)
                    .take(num_iters)
                    .enumerate()
                {
                    let mut raw_samples: [&[i16]; 4] = [&[], &[], &[], &[]];

                    // iterate over each line, since color-convert needs only
                    // one line
                    for (j, samp) in raw_samples.iter_mut().enumerate().take(comp_len) {
                        let temp = &samples[j].get(pos * padded_width..(pos + 1) * padded_width);
                        if temp.is_none() {
                            return Err(DecodeErrors::FormatStatic("Missing samples"));
                        }
                        *samp = temp.unwrap();
                    }
                    color_convert(
                        &raw_samples,
                        self.color_convert_16,
                        self.input_colorspace,
                        self.options.jpeg_get_out_colorspace(),
                        output,
                        width,
                        padded_width
                    )?;
                    px += width * out_colorspace_components;
                }
                Ok(())
            };

        let comps = &mut self.components[..];

        if self.is_interleaved && self.options.jpeg_get_out_colorspace() != ColorSpace::Luma {
            for comp in comps.iter_mut() {
                upsample(
                    comp,
                    mcu_height,
                    i,
                    upsampler_scratch_space,
                    is_vertically_sampled
                )?;
            }

            if is_vertically_sampled {
                if i > 0 {
                    // write the last line, it wasn't  up-sampled as we didn't have row_down
                    // yet
                    let mut samples: [&[i16]; 4] = [&[], &[], &[], &[]];

                    for (samp, component) in samples.iter_mut().zip(comps.iter()) {
                        *samp = &component.first_row_upsample_dest;
                    }

                    // ensure length matches for all samples
                    let _first_len = samples[0].len();

                    // This was a good check, but can be caused to panic, esp on invalid/corrupt images.
                    // See one in issue https://github.com/etemesi254/zune-image/issues/262, so for now
                    // we just ignore and generate invalid images at the end.

                    //
                    //
                    // for samp in samples.iter().take(comp_len) {
                    //     assert_eq!(first_len, samp.len());
                    // }
                    let num_iters = self.coeff * self.v_max;

                    color_conv_function(num_iters, samples)?;
                }

                // After up-sampling the last row, save  any row that can be used for
                // a later up-sampling,
                //
                // E.g the Y sample is not sampled but we haven't finished upsampling the last row of
                // the previous mcu, since we don't have the down row, so save it
                for component in comps.iter_mut() {
                    if component.sample_ratio != SampleRatios::H {
                        // We don't care about H sampling factors, since it's copied in the workers function

                        // copy last row to be used for the  next color conversion
                        let size = component.vertical_sample
                            * component.width_stride
                            * component.sample_ratio.sample();

                        let last_bytes =
                            component.raw_coeff.rchunks_exact_mut(size).next().unwrap();

                        component
                            .first_row_upsample_dest
                            .copy_from_slice(last_bytes);
                    }
                }
            }

            let mut samples: [&[i16]; 4] = [&[], &[], &[], &[]];

            for (samp, component) in samples.iter_mut().zip(comps.iter()) {
                *samp = if component.sample_ratio == SampleRatios::None {
                    &component.raw_coeff
                } else {
                    &component.upsample_dest
                };
            }

            // we either do 7 or 8 MCU's depending on the state, this only applies to
            // vertically sampled images
            //
            // for rows up until the last MCU, we do not upsample the last stride of the MCU
            // which means that the number of iterations should take that into account is one less the
            // up-sampled size
            //
            // For the last MCU, we upsample the last stride, meaning that if we hit the last MCU, we
            // should sample full raw coeffs
            let is_last_considered = is_vertically_sampled && (i != mcu_height.saturating_sub(1));

            let num_iters = (8 - usize::from(is_last_considered)) * self.coeff * self.v_max;

            color_conv_function(num_iters, samples)?;
        } else {
            let mut channels_ref: [&[i16]; MAX_COMPONENTS] = [&[]; MAX_COMPONENTS];

            self.components
                .iter()
                .enumerate()
                .for_each(|(pos, x)| channels_ref[pos] = &x.raw_coeff);

            if let SampleRatios::Generic(_, v) = self.info.sample_ratio {
                color_conv_function(8 * v * self.coeff, channels_ref)?;
            } else {
                color_conv_function(8 * self.coeff, channels_ref)?;
            }
        }

        *pixels_written = px;
        Ok(())
    }
}

enum McuContinuation {
    Ok,
    AnotherSos,
    /// Found an inter-scan marker (DHT/DQT/DRI/COM/APP) that needs handling.
    /// The caller should parse it and scan for the next SOS.
    InterScanMarker(Marker),
    Terminate
}
