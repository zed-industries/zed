// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::cmp::min;
use std::convert::TryInto;
use std::io;

use symphonia_core::errors::{decode_error, Error, Result};
use symphonia_core::io::{BitReaderRtl, ReadBitsRtl};

use super::codebook::VorbisCodebook;
use super::common::*;
use super::DspChannel;

#[derive(Debug, Default)]
struct ResidueVqClass {
    books: [u8; 8],
    is_used: u8,
}

impl ResidueVqClass {
    #[inline(always)]
    fn is_used(&self, pass: usize) -> bool {
        debug_assert!(pass < 8);
        self.is_used & (1 << pass) != 0
    }
}

#[derive(Debug)]
struct ResidueSetup {
    /// The residue format.
    residue_type: u16,
    /// The residue's starting offset.
    residue_begin: u32,
    /// The residue's ending offset.
    residue_end: u32,
    /// Residue partition size (max. value 2^24).
    residue_partition_size: u32,
    /// Residue classifications (max. value 64).
    residue_classifications: u8,
    /// Codebook for reading partition classifications.
    residue_classbook: u8,
    /// Codebooks for each partition classification.
    residue_vq_class: Vec<ResidueVqClass>,
    /// The maximum pass.
    residue_max_pass: usize,
}

#[derive(Debug)]
pub struct Residue {
    setup: ResidueSetup,
    /// Classifications vector.
    part_classes: Vec<u8>,
    /// Vector to read interleaved format 2 residuals.
    type2_buf: Vec<f32>,
}

impl Residue {
    pub fn try_read(
        bs: &mut BitReaderRtl<'_>,
        residue_type: u16,
        max_codebook: u8,
    ) -> Result<Self> {
        let setup = Self::read_setup(bs, residue_type, max_codebook)?;

        Ok(Residue { setup, part_classes: Default::default(), type2_buf: Default::default() })
    }

    fn read_setup(
        bs: &mut BitReaderRtl<'_>,
        residue_type: u16,
        max_codebook: u8,
    ) -> Result<ResidueSetup> {
        let residue_begin = bs.read_bits_leq32(24)?;
        let residue_end = bs.read_bits_leq32(24)?;
        let residue_partition_size = bs.read_bits_leq32(24)? + 1;
        let residue_classifications = bs.read_bits_leq32(6)? as u8 + 1;
        let residue_classbook = bs.read_bits_leq32(8)? as u8;

        if residue_end < residue_begin {
            return decode_error("vorbis: invalid residue begin and end");
        }

        let mut residue_vq_books = Vec::<ResidueVqClass>::new();

        for _ in 0..residue_classifications {
            let low_bits = bs.read_bits_leq32(3)? as u8;

            let high_bits = if bs.read_bool()? { bs.read_bits_leq32(5)? as u8 } else { 0 };

            let is_used = (high_bits << 3) | low_bits;

            residue_vq_books.push(ResidueVqClass { is_used, books: [0; 8] });
        }

        let mut residue_max_pass = 0;

        for vq_books in &mut residue_vq_books {
            // For each set of residue codebooks, if the codebook is used, read the codebook
            // number.
            for (j, book) in vq_books.books.iter_mut().enumerate() {
                // Is a codebook used?
                let is_codebook_used = vq_books.is_used & (1 << j) != 0;

                if is_codebook_used {
                    // Read the codebook number.
                    *book = bs.read_bits_leq32(8)? as u8;

                    // The codebook number cannot be 0 or exceed the number of codebooks in this
                    // stream.
                    if *book == 0 || *book >= max_codebook {
                        return decode_error("vorbis: invalid codebook for residue");
                    }

                    residue_max_pass = residue_max_pass.max(j);
                }
            }
        }

        let residue = ResidueSetup {
            residue_type,
            residue_begin,
            residue_end,
            residue_partition_size,
            residue_classifications,
            residue_classbook,
            residue_vq_class: residue_vq_books,
            residue_max_pass,
        };

        Ok(residue)
    }

    pub fn read_residue(
        &mut self,
        bs: &mut BitReaderRtl<'_>,
        bs_exp: u8,
        codebooks: &[VorbisCodebook],
        residue_channels: &BitSet256,
        channels: &mut [DspChannel],
    ) -> Result<()> {
        let result = if self.setup.residue_type == 2 {
            self.read_residue_inner_type_2(bs, bs_exp, codebooks, residue_channels, channels)
        }
        else {
            self.read_residue_inner_type_0_1(bs, bs_exp, codebooks, residue_channels, channels)
        };

        // Read the residue, and ignore end-of-bitstream errors which are legal.
        match result {
            Ok(_) => (),
            // An end-of-bitstream error is classified under ErrorKind::Other. This condition
            // should not be treated as an error.
            Err(Error::IoError(ref e)) if e.kind() == io::ErrorKind::Other => (),
            Err(e) => return Err(e),
        };

        // For format 2, the residue vectors for all channels are interleaved together into one
        // large vector. This vector is in the scratch-pad buffer and can now be de-interleaved
        // into the channel buffers.
        if self.setup.residue_type == 2 {
            self.deinterleave_2(residue_channels, channels);
        }

        Ok(())
    }

    /// Deinterleave samples from the type 2 format buffer into channel buffers.
    fn deinterleave_2(&mut self, residue_channels: &BitSet256, channels: &mut [DspChannel]) {
        match residue_channels.count() {
            2 => {
                // Two channel deinterleave.
                let (ch0, ch1) = {
                    let mut iter = residue_channels.iter();

                    // Get indicies of first two channels in the residue.
                    let ch0 = iter.next().unwrap();
                    let ch1 = iter.next().unwrap();

                    // Get references to the channels.
                    let (a, b) = channels.split_at_mut(ch0).1.split_at_mut(ch1);

                    (&mut a[0], &mut b[0])
                };

                // Deinterleave.
                for ((buf, chan0), chan1) in self
                    .type2_buf
                    .chunks_exact(2)
                    .zip(ch0.residue.iter_mut())
                    .zip(ch1.residue.iter_mut())
                {
                    *chan0 = buf[0];
                    *chan1 = buf[1];
                }
            }
            stride => {
                // Generic deinterleave.
                for (i, ch) in residue_channels.iter().enumerate() {
                    let channel = &mut channels[ch];

                    let iter = self.type2_buf.chunks_exact(stride).map(|c| c[i]);

                    for (o, i) in channel.residue.iter_mut().zip(iter) {
                        *o = i;
                    }
                }
            }
        }
    }

    fn read_residue_inner_type_2(
        &mut self,
        bs: &mut BitReaderRtl<'_>,
        bs_exp: u8,
        codebooks: &[VorbisCodebook],
        residue_channels: &BitSet256,
        channels: &mut [DspChannel],
    ) -> Result<()> {
        let class_book = &codebooks[self.setup.residue_classbook as usize];

        // The actual length of the entire residue vector for a channel (formats 0 and 1), or all
        // interleaved channels (format 2).
        let full_residue_len = ((1 << bs_exp) >> 1) * residue_channels.count();

        // The range of the residue vector being decoded.
        let limit_residue_begin = min(self.setup.residue_begin as usize, full_residue_len);
        let limit_residue_end = min(self.setup.residue_end as usize, full_residue_len);

        // Length of the decoded part of the residue vector.
        let residue_len = limit_residue_end - limit_residue_begin;

        // Number of partitions per classword.
        let parts_per_classword = class_book.dimensions();

        // Number of partitions to read.
        let parts_to_read = residue_len / self.setup.residue_partition_size as usize;

        // Reserve partition classification space.
        self.prepare_partition_classes(parts_to_read);

        // Reserve the type 2 interleave buffer storage, and zero all samples.
        self.prepare_type_2_format_buffer(full_residue_len);

        // If all channels are marked do-not-decode then exit immediately.
        let has_channel_to_decode =
            residue_channels.iter().fold(false, |val, ch| val | !channels[ch].do_not_decode);

        if !has_channel_to_decode {
            return Ok(());
        }

        let part_size = self.setup.residue_partition_size as usize;

        // Residues may be encoded in up-to 8 passes. Fewer passes may be encoded by prematurely
        // "ending" the packet. This means that an end-of-bitstream error is actually NOT an error.
        for pass in 0..self.setup.residue_max_pass + 1 {
            // Iterate over the partitions in batches grouped by classword.
            for part_first in (0..parts_to_read).step_by(parts_per_classword as usize) {
                // The class assignments for each partition in the classword group are only
                // encoded in the first pass.
                if pass == 0 {
                    let code = class_book.read_scalar(bs)?;

                    decode_classes(
                        code,
                        parts_per_classword,
                        self.setup.residue_classifications as u32,
                        &mut self.part_classes[part_first..],
                    );
                }

                // The last partition in this batch of partitions, being careful not to exceed the
                // total number of partitions.
                let part_last = min(parts_to_read, part_first + parts_per_classword as usize);

                // Iterate over all partitions belonging to the current classword group.
                for part in part_first..part_last {
                    let vq_class = &self.setup.residue_vq_class[self.part_classes[part] as usize];

                    if vq_class.is_used(pass) {
                        let vq_book = &codebooks[vq_class.books[pass] as usize];

                        let part_start = limit_residue_begin + part_size * part;

                        // Residue type 2 is implemented in term of type 1.
                        read_residue_partition_format1(
                            bs,
                            vq_book,
                            &mut self.type2_buf[part_start..part_start + part_size],
                        )?;
                    }
                }
                // End of partition batch iteration.
            }
            // End of pass iteration.
        }

        Ok(())
    }

    fn read_residue_inner_type_0_1(
        &mut self,
        bs: &mut BitReaderRtl<'_>,
        bs_exp: u8,
        codebooks: &[VorbisCodebook],
        residue_channels: &BitSet256,
        channels: &mut [DspChannel],
    ) -> Result<()> {
        let class_book = &codebooks[self.setup.residue_classbook as usize];

        // The actual length of the entire residue vector for a channel (formats 0 and 1), or all
        // interleaved channels (format 2).
        let full_residue_len = (1 << bs_exp) >> 1;

        // The range of the residue vector being decoded.
        let limit_residue_begin = min(self.setup.residue_begin as usize, full_residue_len);
        let limit_residue_end = min(self.setup.residue_end as usize, full_residue_len);

        // Length of the decoded part of the residue vector.
        let residue_len = limit_residue_end - limit_residue_begin;

        // Number of partitions per classword.
        let parts_per_classword = class_book.dimensions();

        // Number of partitions to read.
        let parts_to_read = residue_len / self.setup.residue_partition_size as usize;

        // Reserve partition classification space.
        self.prepare_partition_classes(parts_to_read * residue_channels.count());

        let mut has_channel_to_decode = false;

        for ch in residue_channels.iter() {
            // Record if the channel needs to be decoded.
            has_channel_to_decode |= !channels[ch].do_not_decode;

            // Zero the channel residue buffer.
            channels[ch].residue[..full_residue_len].fill(0.0);
        }

        // If all channels are marked do-not-decode then exit immediately.
        if !has_channel_to_decode {
            return Ok(());
        }

        let part_size = self.setup.residue_partition_size as usize;

        // Residues may be encoded in up-to 8 passes. Fewer passes may be encoded by prematurely
        // "ending" the packet. This means that an end-of-bitstream error is actually NOT an error.
        for pass in 0..self.setup.residue_max_pass + 1 {
            // Iterate over the partitions in batches grouped by classword.
            for part_first in (0..parts_to_read).step_by(parts_per_classword as usize) {
                // The class assignments for each partition in the classword group are only
                // encoded in the first pass.
                if pass == 0 {
                    for (i, ch) in residue_channels.iter().enumerate() {
                        let channel = &channels[ch];

                        // If the channel is marked do-not-decode then advance to the next
                        // channel.
                        if channel.do_not_decode {
                            continue;
                        }

                        let code = class_book.read_scalar(bs)?;

                        decode_classes(
                            code,
                            parts_per_classword,
                            self.setup.residue_classifications as u32,
                            &mut self.part_classes[part_first + i * parts_to_read..],
                        );
                    }
                }

                // The last partition in this batch of partitions, being careful not to exceed the
                // total number of partitions.
                let part_last = min(parts_to_read, part_first + parts_per_classword as usize);

                // Iterate over all partitions belonging to the current classword group.
                for part in part_first..part_last {
                    // Iterate over each channel vector in the partition.
                    for (i, ch) in residue_channels.iter().enumerate() {
                        let channel = &mut channels[ch];

                        // If the channel is marked do-not-decode, then advance to the next channel.
                        if channel.do_not_decode {
                            continue;
                        }

                        let class_idx = self.part_classes[part + parts_to_read * i] as usize;
                        let vq_class = &self.setup.residue_vq_class[class_idx];

                        if vq_class.is_used(pass) {
                            let vq_book = &codebooks[vq_class.books[pass] as usize];

                            let part_start = limit_residue_begin + part_size * part;

                            match self.setup.residue_type {
                                0 => read_residue_partition_format0(
                                    bs,
                                    vq_book,
                                    &mut channel.residue[part_start..part_start + part_size],
                                ),
                                1 => read_residue_partition_format1(
                                    bs,
                                    vq_book,
                                    &mut channel.residue[part_start..part_start + part_size],
                                ),
                                _ => unreachable!(),
                            }?;
                        }
                    }
                }
                // End of partition batch iteration.
            }
            // End of pass iteration.
        }

        Ok(())
    }

    /// Ensures enough storage for `len` partition classes.
    #[inline]
    fn prepare_partition_classes(&mut self, len: usize) {
        if self.part_classes.len() < len {
            self.part_classes.resize(len, Default::default());
        }
    }

    /// Ensures the interleave buffer for type 2 residues can accomodate `len` samples, and that the
    /// buffer is zeroed.
    #[inline]
    fn prepare_type_2_format_buffer(&mut self, len: usize) {
        if self.type2_buf.len() < len {
            self.type2_buf.resize(len, Default::default());
        }
        self.type2_buf[..len].fill(0.0);
    }
}

fn decode_classes(mut val: u32, parts_per_classword: u16, classifications: u32, out: &mut [u8]) {
    let parts_per_classword = usize::from(parts_per_classword);

    // The number of partitions that need a class assignment.
    let num_parts = out.len();

    // If the number of partitions per classword is greater than the number of partitions that need
    // a class assignment, then the excess classes must be dropped because class assignments are
    // assigned in reverse order.
    let skip = if parts_per_classword > num_parts {
        let skip = parts_per_classword - num_parts;

        for _ in 0..skip {
            val /= classifications;
        }

        skip
    }
    else {
        0
    };

    for out in out[..parts_per_classword - skip].iter_mut().rev() {
        *out = (val % classifications) as u8;
        val /= classifications;
    }
}

fn read_residue_partition_format0(
    bs: &mut BitReaderRtl<'_>,
    codebook: &VorbisCodebook,
    out: &mut [f32],
) -> Result<()> {
    let step = out.len() / codebook.dimensions() as usize;

    for i in 0..step {
        let vq = codebook.read_vq(bs)?;

        for (o, &v) in out[i..].iter_mut().step_by(step).zip(vq) {
            *o += v;
        }
    }

    Ok(())
}

#[inline(always)]
fn read_residue_partition_format1(
    bs: &mut BitReaderRtl<'_>,
    codebook: &VorbisCodebook,
    out: &mut [f32],
) -> Result<()> {
    let dim = codebook.dimensions() as usize;

    // For small dimensions it is too expensive to use iterator loops. Special case small sizes
    // to improve performance.
    match dim {
        2 => {
            for out in out.chunks_exact_mut(2) {
                let vq = codebook.read_vq(bs)?;

                // Amortize the cost of the bounds check.
                let v: [f32; 2] = vq.try_into().unwrap();

                out[0] += v[0];
                out[1] += v[1];
            }
        }
        4 => {
            for out in out.chunks_exact_mut(4) {
                let vq = codebook.read_vq(bs)?;

                let v: [f32; 4] = vq.try_into().unwrap();

                out[0] += v[0];
                out[1] += v[1];
                out[2] += v[2];
                out[3] += v[3];
            }
        }
        _ => {
            for out in out.chunks_exact_mut(dim) {
                let vq = codebook.read_vq(bs)?;

                for (o, &v) in out.iter_mut().zip(vq) {
                    *o += v;
                }
            }
        }
    }

    Ok(())
}
