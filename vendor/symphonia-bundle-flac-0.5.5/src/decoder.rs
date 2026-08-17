// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::cmp;
use std::convert::TryInto;
use std::num::Wrapping;

use symphonia_core::audio::{AsAudioBufferRef, AudioBuffer, AudioBufferRef};
use symphonia_core::audio::{Signal, SignalSpec};
use symphonia_core::codecs::{
    CodecDescriptor, CodecParameters, VerificationCheck, CODEC_TYPE_FLAC,
};
use symphonia_core::codecs::{Decoder, DecoderOptions, FinalizeResult};
use symphonia_core::errors::{decode_error, unsupported_error, Result};
use symphonia_core::formats::Packet;
use symphonia_core::io::{BitReaderLtr, BufReader, ReadBitsLtr};
use symphonia_core::support_codec;
use symphonia_core::units::TimeBase;
use symphonia_core::util::bits::sign_extend_leq32_to_i32;
use symphonia_utils_xiph::flac::metadata::StreamInfo;

use log::{debug, log_enabled, warn};

use super::frame::*;
use super::validate::Validator;

fn decorrelate_left_side(left: &[i32], side: &mut [i32]) {
    for (s, l) in side.iter_mut().zip(left) {
        *s = *l - *s;
    }
}

fn decorrelate_mid_side(mid: &mut [i32], side: &mut [i32]) {
    for (m, s) in mid.iter_mut().zip(side) {
        // Mid (M) is given as M = L/2 + R/2, while Side (S) is given as S = L - R.
        //
        // To calculate the individual channels, the following equations can be used:
        //      - L = S/2 + M
        //      - R = M - S/2
        //
        // Ideally, this would work, but since samples are represented as integers, division yields
        // the floor of the divided value. Therefore, the channel restoration equations actually
        // yield:
        //      - L = floor(S/2) + M
        //      - R = M - floor(S/2)
        //
        // This will produce incorrect samples whenever the sample S is odd. For example:
        //      - 2/2 = 1
        //      - 3/2 = 1 (should be 2 if rounded!)
        //
        // To get the proper rounding behaviour, the solution is to add one to the result if S is
        // odd:
        //      - L = floor(S/2) + M + (S%2) = M + (S%2) + floor(S/2)
        //      - R = M - floor(S/2) + (S%2) = M + (S%2) - floor(S/2)
        //
        // Further, to prevent loss of accuracy, instead of dividing S/2 and adding or subtracting
        // it from M, multiply M*2, then add or subtract S, and then divide the whole result by 2.
        // This gives one extra bit of precision for the intermediate computations.
        //
        // Conveniently, since M should be doubled, the LSB will always be 0. This allows S%2 to
        // be added simply by bitwise ORing S&1 to M<<1.
        //
        // Therefore the final equations yield:
        //      - L = (2*M + (S%2) + S) / 2
        //      - R = (2*M + (S%2) - S) / 2
        let mid = (*m << 1) | (*s & 1);
        let side = *s;
        *m = (mid + side) >> 1;
        *s = (mid - side) >> 1;
    }
}

fn decorrelate_right_side(right: &[i32], side: &mut [i32]) {
    for (s, r) in side.iter_mut().zip(right) {
        *s += *r;
    }
}

/// Free Lossless Audio Codec (FLAC) decoder.
pub struct FlacDecoder {
    params: CodecParameters,
    is_validating: bool,
    validator: Validator,
    buf: AudioBuffer<i32>,
}

impl FlacDecoder {
    fn decode_inner(&mut self, packet: &Packet) -> Result<()> {
        let mut reader = packet.as_buf_reader();

        // Synchronize to a frame and get the synchronization code.
        let sync = sync_frame(&mut reader)?;

        let header = read_frame_header(&mut reader, sync)?;

        // Use the bits per sample and sample rate as stated in the frame header, falling back to
        // the stream information if provided. If neither are available, return an error.
        let bits_per_sample = if let Some(bps) = header.bits_per_sample {
            bps
        }
        else if let Some(bps) = self.params.bits_per_sample {
            bps
        }
        else {
            return decode_error("flac: bits per sample not provided");
        };

        // trace!("frame: [{:?}] strategy={:?}, n_samples={}, bps={}, channels={:?}",
        //     header.block_sequence,
        //     header.blocking_strategy,
        //     header.block_num_samples,
        //     bits_per_sample,
        //     &header.channel_assignment);

        // Reserve a writeable chunk in the buffer equal to the number of samples in the block.
        self.buf.clear();
        self.buf.render_reserved(Some(header.block_num_samples as usize));

        // Only Bitstream reading for subframes.
        {
            // Sub-frames don't have any byte-aligned content, so use a BitReader.
            let mut bs = BitReaderLtr::new(reader.read_buf_bytes_available_ref());

            // Read each subframe based on the channel assignment into a planar buffer.
            match header.channel_assignment {
                ChannelAssignment::Independant(channels) => {
                    for i in 0..channels as usize {
                        read_subframe(&mut bs, bits_per_sample, self.buf.chan_mut(i))?;
                    }
                }
                // For Left/Side, Mid/Side, and Right/Side channel configurations, the Side
                // (Difference) channel requires an extra bit per sample.
                ChannelAssignment::LeftSide => {
                    let (left, side) = self.buf.chan_pair_mut(0, 1);

                    read_subframe(&mut bs, bits_per_sample, left)?;
                    read_subframe(&mut bs, bits_per_sample + 1, side)?;

                    decorrelate_left_side(left, side);
                }
                ChannelAssignment::MidSide => {
                    let (mid, side) = self.buf.chan_pair_mut(0, 1);

                    read_subframe(&mut bs, bits_per_sample, mid)?;
                    read_subframe(&mut bs, bits_per_sample + 1, side)?;

                    decorrelate_mid_side(mid, side);
                }
                ChannelAssignment::RightSide => {
                    let (side, right) = self.buf.chan_pair_mut(0, 1);

                    read_subframe(&mut bs, bits_per_sample + 1, side)?;
                    read_subframe(&mut bs, bits_per_sample, right)?;

                    decorrelate_right_side(right, side);
                }
            }
        }

        // Feed the validator if validation is enabled.
        if self.is_validating {
            self.validator.update(&self.buf, bits_per_sample);
        }

        // The decoder uses a 32bit sample format as a common denominator, but that doesn't mean
        // the encoded audio samples are actually 32bit. Shift all samples in the output buffer
        // so that regardless the encoded bits/sample, the output is always 32bits/sample.
        if bits_per_sample < 32 {
            let shift = 32 - bits_per_sample;
            self.buf.transform(|sample| sample << shift);
        }

        Ok(())
    }
}

impl Decoder for FlacDecoder {
    fn try_new(params: &CodecParameters, options: &DecoderOptions) -> Result<Self> {
        // This decoder only supports FLAC.
        if params.codec != CODEC_TYPE_FLAC {
            return unsupported_error("flac: invalid codec type");
        }

        // Obtain the extra data.
        let extra_data = match params.extra_data.as_ref() {
            Some(buf) => buf,
            _ => return unsupported_error("flac: missing extra data"),
        };

        // Read the stream information block.
        let info = StreamInfo::read(&mut BufReader::new(extra_data))?;

        // Clone the codec parameters so that the parameters can be supplemented and/or amended.
        let mut params = params.clone();

        // Amend the provided codec parameters with information from the stream information block.
        params
            .with_sample_rate(info.sample_rate)
            .with_time_base(TimeBase::new(1, info.sample_rate))
            .with_bits_per_sample(info.bits_per_sample)
            .with_max_frames_per_packet(u64::from(info.block_len_max))
            .with_channels(info.channels);

        if let Some(md5) = info.md5 {
            params.with_verification_code(VerificationCheck::Md5(md5));
        }

        if let Some(n_frames) = info.n_samples {
            params.with_n_frames(n_frames);
        }

        let spec = SignalSpec::new(info.sample_rate, info.channels);
        let buf = AudioBuffer::new(u64::from(info.block_len_max), spec);

        // TODO: Verify packet integrity if the demuxer is not.
        // if !params.packet_data_integrity {
        //     return unsupported_error("flac: packet integrity is required");
        // }

        Ok(FlacDecoder {
            params,
            is_validating: options.verify,
            validator: Default::default(),
            buf,
        })
    }

    fn supported_codecs() -> &'static [CodecDescriptor] {
        &[support_codec!(CODEC_TYPE_FLAC, "flac", "Free Lossless Audio Codec")]
    }

    fn reset(&mut self) {
        // No state is stored between packets, therefore do nothing.
    }

    fn codec_params(&self) -> &CodecParameters {
        &self.params
    }

    fn decode(&mut self, packet: &Packet) -> Result<AudioBufferRef<'_>> {
        if let Err(e) = self.decode_inner(packet) {
            self.buf.clear();
            Err(e)
        }
        else {
            Ok(self.buf.as_audio_buffer_ref())
        }
    }

    fn finalize(&mut self) -> FinalizeResult {
        let mut result: FinalizeResult = Default::default();

        // If verifying...
        if self.is_validating {
            // Try to get the expected MD5 checksum and compare it against the decoded checksum.
            if let Some(VerificationCheck::Md5(expected)) = self.params.verification_check {
                let decoded = self.validator.md5();

                // Only generate the expected and decoded MD5 checksum strings if logging is
                // enabled at the debug level.
                if log_enabled!(log::Level::Debug) {
                    use std::fmt::Write;

                    let mut expected_s = String::with_capacity(32);
                    let mut decoded_s = String::with_capacity(32);

                    expected.iter().for_each(|b| write!(expected_s, "{:02x}", b).unwrap());
                    decoded.iter().for_each(|b| write!(decoded_s, "{:02x}", b).unwrap());

                    debug!("verification: expected md5 = {}", expected_s);
                    debug!("verification: decoded md5  = {}", decoded_s);
                }

                result.verify_ok = Some(decoded == expected)
            }
            else {
                warn!("verification requested but the expected md5 checksum was not provided");
            }
        }

        result
    }

    fn last_decoded(&self) -> AudioBufferRef<'_> {
        self.buf.as_audio_buffer_ref()
    }
}

// Subframe business

#[derive(Debug)]
enum SubFrameType {
    Constant,
    Verbatim,
    FixedLinear(u32),
    Linear(u32),
}

fn read_subframe<B: ReadBitsLtr>(bs: &mut B, frame_bps: u32, buf: &mut [i32]) -> Result<()> {
    // First sub-frame bit must always 0.
    if bs.read_bool()? {
        return decode_error("flac: subframe padding is not 0");
    }

    // Next 6 bits designate the sub-frame type.
    let subframe_type_enc = bs.read_bits_leq32(6)?;

    let subframe_type = match subframe_type_enc {
        0x00 => SubFrameType::Constant,
        0x01 => SubFrameType::Verbatim,
        0x08..=0x0f => {
            let order = subframe_type_enc & 0x07;
            // The Fixed Predictor only supports orders between 0 and 4.
            if order > 4 {
                return decode_error("flac: fixed predictor orders of greater than 4 are invalid");
            }
            SubFrameType::FixedLinear(order)
        }
        0x20..=0x3f => SubFrameType::Linear((subframe_type_enc & 0x1f) + 1),
        _ => {
            return decode_error("flac: subframe type set to reserved value");
        }
    };

    // Bit 7 of the sub-frame header designates if there are any dropped (wasted in FLAC terms)
    // bits per sample in the audio sub-block. If the bit is set, unary decode the number of
    // dropped bits per sample.
    let dropped_bps = if bs.read_bool()? { bs.read_unary_zeros()? + 1 } else { 0 };

    // The bits per sample stated in the frame header is for the decoded audio sub-block samples.
    // However, it is likely that the lower order bits of all the samples are simply 0. Therefore,
    // the encoder will truncate `dropped_bps` of lower order bits for every sample in a sub-block.
    // The decoder simply needs to shift left all samples by `dropped_bps` after decoding the
    // sub-frame and obtaining the truncated audio sub-block samples.
    let bps = frame_bps - dropped_bps;

    // trace!("\tsubframe: type={:?}, bps={}, dropped_bps={}",
    //     &subframe_type,
    //     bps,
    //     dropped_bps);

    match subframe_type {
        SubFrameType::Constant => decode_constant(bs, bps, buf)?,
        SubFrameType::Verbatim => decode_verbatim(bs, bps, buf)?,
        SubFrameType::FixedLinear(order) => decode_fixed_linear(bs, bps, order, buf)?,
        SubFrameType::Linear(order) => decode_linear(bs, bps, order, buf)?,
    };

    // Shift the samples to account for the dropped bits.
    samples_shl(dropped_bps, buf);

    Ok(())
}

#[inline(always)]
fn samples_shl(shift: u32, buf: &mut [i32]) {
    if shift > 0 {
        for sample in buf.iter_mut() {
            *sample = sample.wrapping_shl(shift);
        }
    }
}

fn decode_constant<B: ReadBitsLtr>(bs: &mut B, bps: u32, buf: &mut [i32]) -> Result<()> {
    let const_sample = sign_extend_leq32_to_i32(bs.read_bits_leq32(bps)?, bps);

    for sample in buf.iter_mut() {
        *sample = const_sample;
    }

    Ok(())
}

fn decode_verbatim<B: ReadBitsLtr>(bs: &mut B, bps: u32, buf: &mut [i32]) -> Result<()> {
    for sample in buf.iter_mut() {
        *sample = sign_extend_leq32_to_i32(bs.read_bits_leq32(bps)?, bps);
    }

    Ok(())
}

fn decode_fixed_linear<B: ReadBitsLtr>(
    bs: &mut B,
    bps: u32,
    order: u32,
    buf: &mut [i32],
) -> Result<()> {
    // The first `order` samples are encoded verbatim to warm-up the LPC decoder.
    decode_verbatim(bs, bps, &mut buf[..order as usize])?;

    // Decode the residuals for the predicted samples.
    decode_residual(bs, order, buf)?;

    // Run the Fixed predictor (appends to residuals).
    //
    // TODO: The fixed predictor uses 64-bit accumulators by default to support bps > 26. On 64-bit
    // machines, this is preferable, but on 32-bit machines if bps <= 26, run a 32-bit predictor,
    // and fallback to the 64-bit predictor if necessary (which is basically never).
    fixed_predict(order, buf);

    Ok(())
}

fn decode_linear<B: ReadBitsLtr>(bs: &mut B, bps: u32, order: u32, buf: &mut [i32]) -> Result<()> {
    // The order of the Linear Predictor should be between 1 and 32.
    debug_assert!(order > 0 && order <= 32);

    // The first `order` samples are encoded verbatim to warm-up the LPC decoder.
    decode_verbatim(bs, bps, &mut buf[0..order as usize])?;

    // Quantized linear predictor (QLP) coefficients precision in bits (1-16).
    let qlp_precision = bs.read_bits_leq32(4)? + 1;

    if qlp_precision > 15 {
        return decode_error("flac: qlp precision set to reserved value");
    }

    // QLP coefficients bit shift [-16, 15].
    let qlp_coeff_shift = sign_extend_leq32_to_i32(bs.read_bits_leq32(5)?, 5);

    if qlp_coeff_shift >= 0 {
        let mut qlp_coeffs = [0i32; 32];

        for c in qlp_coeffs.iter_mut().rev().take(order as usize) {
            *c = sign_extend_leq32_to_i32(bs.read_bits_leq32(qlp_precision)?, qlp_precision);
        }

        decode_residual(bs, order, buf)?;

        // Helper function to dispatch to a predictor with a maximum order of N.
        #[inline(always)]
        fn lpc<const N: usize>(order: u32, coeffs: &[i32; 32], coeff_shift: i32, buf: &mut [i32]) {
            let coeffs_n = (&coeffs[32 - N..32]).try_into().unwrap();
            lpc_predict::<N>(order as usize, coeffs_n, coeff_shift as u32, buf);
        }

        // Pick the best length linear predictor to use based on the order. Most FLAC streams use
        // the subset format and have an order <= 12. Therefore, for orders <= 12, dispatch to
        // predictors that roughly match the order. If a predictor is too long for a given order,
        // then there will be wasted computations. On the other hand, it is not worth the code bloat
        // to specialize for every order <= 12.
        match order {
            0..=4 => lpc::<4>(order, &qlp_coeffs, qlp_coeff_shift, buf),
            5..=6 => lpc::<6>(order, &qlp_coeffs, qlp_coeff_shift, buf),
            7..=8 => lpc::<8>(order, &qlp_coeffs, qlp_coeff_shift, buf),
            9..=10 => lpc::<10>(order, &qlp_coeffs, qlp_coeff_shift, buf),
            11..=12 => lpc::<12>(order, &qlp_coeffs, qlp_coeff_shift, buf),
            _ => lpc::<32>(order, &qlp_coeffs, qlp_coeff_shift, buf),
        };
    }
    else {
        return unsupported_error("flac: lpc shifts less than 0 are not supported");
    }

    Ok(())
}

fn decode_residual<B: ReadBitsLtr>(
    bs: &mut B,
    n_prelude_samples: u32,
    buf: &mut [i32],
) -> Result<()> {
    let method_enc = bs.read_bits_leq32(2)?;

    // The FLAC specification defines two residual coding methods: Rice and Rice2. The
    // only difference between the two is the bit width of the Rice parameter. Note the
    // bit width based on the residual encoding method and use the same code path for
    // both cases.
    let param_bit_width = match method_enc {
        0x0 => 4,
        0x1 => 5,
        _ => {
            return decode_error("flac: residual method set to reserved value");
        }
    };

    // Read the partition order.
    let order = bs.read_bits_leq32(4)?;

    // The number of partitions is equal to 2^order.
    let n_partitions = 1usize << order;

    // In general, all partitions have the same number of samples such that the sum of all partition
    // lengths equal the block length. The number of samples in a partition can therefore be
    // calculated with block_size / 2^order *in general*. However, since there are warm-up samples
    // stored verbatim, the first partition has n_prelude_samples less samples. Likewise, if there
    // is only one partition, then it too has n_prelude_samples less samples.
    let n_partition_samples = buf.len() >> order;

    // The size of the first (and/or only) partition as per the specification is n_partition_samples
    // minus the number of warm-up samples (which is the predictor order). Ensure the number of
    // samples in these types of partitions cannot be negative.
    if n_prelude_samples as usize > n_partition_samples {
        return decode_error("flac: residual partition too small for given predictor order");
    }

    // Ensure that the sum of all partition lengths equal the block size.
    if n_partitions * n_partition_samples != buf.len() {
        return decode_error("flac: block size is not same as encoded residual");
    }

    // trace!("\t\tresidual: n_partitions={}, n_partition_samples={}, n_prelude_samples={}",
    //     n_partitions,
    //     n_partition_samples,
    //     n_prelude_samples);

    // Decode the first partition as it may have less than n_partition_samples samples.
    decode_rice_partition(
        bs,
        param_bit_width,
        &mut buf[n_prelude_samples as usize..n_partition_samples],
    )?;

    // Decode the remaining partitions.
    for buf_chunk in buf[n_partition_samples..].chunks_mut(n_partition_samples) {
        decode_rice_partition(bs, param_bit_width, buf_chunk)?;
    }

    Ok(())
}

fn decode_rice_partition<B: ReadBitsLtr>(
    bs: &mut B,
    param_bit_width: u32,
    buf: &mut [i32],
) -> Result<()> {
    // Read the encoding parameter, generally the Rice parameter.
    let rice_param = bs.read_bits_leq32(param_bit_width)?;

    // If the Rice parameter is all 1s (e.g., 0xf for a 4bit parameter, 0x1f for a 5bit parameter),
    // then it indicates that residuals in this partition are not Rice encoded, rather they are
    // binary encoded. Conversely, if the parameter is less than this value, the residuals are Rice
    // encoded.
    if rice_param < (1 << param_bit_width) - 1 {
        // println!("\t\t\tPartition (Rice): n_residuals={}, rice_param={}", buf.len(), rice_param);

        // Read each rice encoded residual and store in buffer.
        for sample in buf.iter_mut() {
            let q = bs.read_unary_zeros()?;
            let r = bs.read_bits_leq32(rice_param)?;
            *sample = rice_signed_to_i32((q << rice_param) | r);
        }
    }
    else {
        let residual_bits = bs.read_bits_leq32(5)?;

        // trace!(
        //     "\t\t\tpartition (Binary): n_residuals={}, residual_bits={}",
        //     buf.len(),
        //     residual_bits
        // );

        // Read each binary encoded residual and store in buffer.
        for sample in buf.iter_mut() {
            *sample = sign_extend_leq32_to_i32(bs.read_bits_leq32(residual_bits)?, residual_bits);
        }
    }

    Ok(())
}

#[inline(always)]
fn rice_signed_to_i32(word: u32) -> i32 {
    // Input  => 0  1  2  3  4  5  6  7  8  9  10
    // Output => 0 -1  1 -2  2 -3  3 -4  4 -5   5
    //
    //  - If even: output = input / 2
    //  - If odd:  output = -(input + 1) / 2
    //                    =  (input / 2) - 1

    // Divide the input by 2 and convert to signed.
    let div2 = (word >> 1) as i32;

    // Using the LSB of the input, create a new signed integer that's either
    // -1 (0b1111_11110) or 0 (0b0000_0000). For odd inputs, this will be -1, for even
    // inputs it'll be 0.
    let sign = -((word & 0x1) as i32);

    // XOR the div2 result with the sign. If sign is 0, the XOR produces div2. If sign is -1, then
    // -div2 - 1 is returned.
    //
    // Example:  input = 9 => div2 = 0b0000_0100, sign = 0b1111_11110
    //
    //           div2 ^ sign =   0b0000_0100
    //                         ^ 0b1111_1110
    //                           -----------
    //                           0b1111_1011  (-5)
    div2 ^ sign
}

#[test]
fn verify_rice_signed_to_i32() {
    assert_eq!(rice_signed_to_i32(0), 0);
    assert_eq!(rice_signed_to_i32(1), -1);
    assert_eq!(rice_signed_to_i32(2), 1);
    assert_eq!(rice_signed_to_i32(3), -2);
    assert_eq!(rice_signed_to_i32(4), 2);
    assert_eq!(rice_signed_to_i32(5), -3);
    assert_eq!(rice_signed_to_i32(6), 3);
    assert_eq!(rice_signed_to_i32(7), -4);
    assert_eq!(rice_signed_to_i32(8), 4);
    assert_eq!(rice_signed_to_i32(9), -5);
    assert_eq!(rice_signed_to_i32(10), 5);

    assert_eq!(rice_signed_to_i32(u32::max_value()), -2_147_483_648);
}

fn fixed_predict(order: u32, buf: &mut [i32]) {
    debug_assert!(order <= 4);

    // The Fixed Predictor is just a hard-coded version of the Linear Predictor up to order 4 and
    // with fixed coefficients. Some cases may be simplified such as orders 0 and 1. For orders 2
    // through 4, use the same IIR-style algorithm as the Linear Predictor.
    match order {
        // A 0th order predictor always predicts 0, and therefore adds nothing to any of the samples
        // in buf. Do nothing.
        0 => (),
        // A 1st order predictor always returns the previous sample since the polynomial is:
        // s(i) = 1*s(i),
        1 => {
            for i in 1..buf.len() {
                buf[i] += buf[i - 1];
            }
        }
        // A 2nd order predictor uses the polynomial: s(i) = 2*s(i-1) - 1*s(i-2).
        2 => {
            for i in 2..buf.len() {
                let a = Wrapping(-1) * Wrapping(i64::from(buf[i - 2]));
                let b = Wrapping(2) * Wrapping(i64::from(buf[i - 1]));
                buf[i] += (a + b).0 as i32;
            }
        }
        // A 3rd order predictor uses the polynomial: s(i) = 3*s(i-1) - 3*s(i-2) + 1*s(i-3).
        3 => {
            for i in 3..buf.len() {
                let a = Wrapping(1) * Wrapping(i64::from(buf[i - 3]));
                let b = Wrapping(-3) * Wrapping(i64::from(buf[i - 2]));
                let c = Wrapping(3) * Wrapping(i64::from(buf[i - 1]));
                buf[i] += (a + b + c).0 as i32;
            }
        }
        // A 4th order predictor uses the polynomial:
        // s(i) = 4*s(i-1) - 6*s(i-2) + 4*s(i-3) - 1*s(i-4).
        4 => {
            for i in 4..buf.len() {
                let a = Wrapping(-1) * Wrapping(i64::from(buf[i - 4]));
                let b = Wrapping(4) * Wrapping(i64::from(buf[i - 3]));
                let c = Wrapping(-6) * Wrapping(i64::from(buf[i - 2]));
                let d = Wrapping(4) * Wrapping(i64::from(buf[i - 1]));
                buf[i] += (a + b + c + d).0 as i32;
            }
        }
        _ => unreachable!(),
    };
}

/// Generalized Linear Predictive Coding (LPC) decoder. The exact number of coefficients given is
/// specified by `order`. Coefficients must be stored in reverse order in `coeffs` with the first
/// coefficient at index 31. Coefficients at indices less than 31 - `order` must be 0.
/// It is expected that the first `order` samples in `buf` are warm-up samples.
fn lpc_predict<const N: usize>(order: usize, coeffs: &[i32; N], coeff_shift: u32, buf: &mut [i32]) {
    // Order must be less than or equal to the number of coefficients.
    debug_assert!(order <= coeffs.len());

    // Order must be less than to equal to the number of samples the buffer can hold.
    debug_assert!(order <= buf.len());

    // The main, efficient, predictor loop needs N previous samples to run. Since order <= N,
    // calculate enough samples to reach N.
    let n_prefill = cmp::min(N, buf.len()) - order;

    for i in order..order + n_prefill {
        let predicted = coeffs[N - order..N]
            .iter()
            .zip(&buf[i - order..i])
            .map(|(&c, &sample)| c as i64 * sample as i64)
            .sum::<i64>();

        buf[i] += (predicted >> coeff_shift) as i32;
    }

    // If the pre-fill operation filled the entire sample buffer, return immediately.
    if buf.len() <= N {
        return;
    }

    // Main predictor loop. Calculate each sample by applying what is essentially an IIR filter.
    for i in N..buf.len() {
        let predicted = coeffs
            .iter()
            .zip(&buf[i - N..i])
            .map(|(&c, &s)| i64::from(c) * i64::from(s))
            .sum::<i64>();

        buf[i] += (predicted >> coeff_shift) as i32;
    }
}
