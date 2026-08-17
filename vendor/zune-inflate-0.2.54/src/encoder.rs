/*
 * Copyright (c) 2023.
 *
 * This software is free software; You can redistribute it or modify it under terms of the MIT, Apache License or Zlib license
 */

use alloc::vec;
use alloc::vec::Vec;

use crate::constants::DEFLATE_BLOCKTYPE_UNCOMPRESSED;

mod fast_match_finder;

const _SEQ_LENGTH_SHIFT: u32 = 23;

const _SEQ_LITRUNLEN_MASK: u32 = (1_u32 << _SEQ_LENGTH_SHIFT) - 1;

pub(crate) struct _Sequence
{
    /*
     * Bits 0..22: the number of literals in this run.  This may be 0 and
     * can be at most MAX_BLOCK_LENGTH.  The literals are not stored
     * explicitly in this structure; instead, they are read directly from
     * the uncompressed data.
     *
     * Bits 23..31: the length of the match which follows the literals, or 0
     * if this literal run was the last in the block, so there is no match
     * which follows it.
     */
    litrunlen_and_length: u32
}

#[derive(Debug, Copy, Clone)]
pub enum DeflateEncodingStrategy
{
    NoCompression
}

impl DeflateEncodingStrategy
{
    #[allow(dead_code)]
    fn to_level(self) -> u8
    {
        match self
        {
            Self::NoCompression => 0
        }
    }
}

pub struct DeflateEncodingOptions
{
    strategy: DeflateEncodingStrategy
}

impl Default for DeflateEncodingOptions
{
    fn default() -> Self
    {
        DeflateEncodingOptions {
            strategy: DeflateEncodingStrategy::NoCompression
        }
    }
}

/// A simple Deflate Encoder.
///
/// Not yet complete
pub struct DeflateEncoder<'a>
{
    data:            &'a [u8],
    options:         DeflateEncodingOptions,
    output_position: usize,
    input_position:  usize,
    output:          Vec<u8>
}

impl<'a> DeflateEncoder<'a>
{
    /// Create a new deflate encoder.
    ///
    /// The
    pub fn new(data: &'a [u8]) -> DeflateEncoder<'a>
    {
        DeflateEncoder::new_with_options(data, DeflateEncodingOptions::default())
    }
    pub fn new_with_options(data: &'a [u8], options: DeflateEncodingOptions) -> DeflateEncoder<'a>
    {
        let length = data.len() + 1024;
        let out_array = vec![0; length];

        DeflateEncoder {
            data,
            options,
            output_position: 0,
            input_position: 0,
            output: out_array
        }
    }

    #[cfg(feature = "zlib")]
    fn write_zlib_header(&mut self)
    {
        const ZLIB_CM_DEFLATE: u16 = 8;
        const ZLIB_CINFO_32K_WINDOW: u16 = 7;

        let level_hint = self.options.strategy.to_level();

        let mut hdr = (ZLIB_CM_DEFLATE << 8) | (ZLIB_CINFO_32K_WINDOW << 12);

        hdr |= u16::from(level_hint) << 6;
        hdr |= 31 - (hdr % 31);

        self.output[self.output_position..self.output_position + 2]
            .copy_from_slice(&hdr.to_be_bytes());
    }
    /// Encode a deflate data block with no compression
    ///
    /// # Argument
    /// - `bytes`: number of bytes to compress from input as non-compressed
    /// bytes
    fn encode_no_compression(&mut self, bytes: usize)
    {
        let final_position = self.input_position + bytes;

        /*
         * If the input is zero-length, we still must output a block in order
         * for the output to be a valid DEFLATE stream.  Handle this case
         * specially to avoid potentially passing NULL to memcpy() below.
         */
        if self.data.is_empty()
        {
            /* BFINAL and BTYPE */
            self.output[self.output_position] = (1 | (DEFLATE_BLOCKTYPE_UNCOMPRESSED << 1)) as u8;
            self.output_position += 1;
            /* LEN and NLEN */
            let num: u32 = 0xFFFF0000;
            self.output[self.output_position..self.output_position + 4]
                .copy_from_slice(&num.to_le_bytes());
            self.output_position += 4;
            return;
        }
        loop
        {
            let mut bfinal = 0;
            let mut len = usize::from(u16::MAX);

            if final_position - self.input_position <= usize::from(u16::MAX)
            {
                bfinal = 1;
                len = final_position - self.input_position;
            }
            /*
             * Output BFINAL and BTYPE.  The stream is already byte-aligned
             * here, so this step always requires outputting exactly 1 byte.
             */
            self.output[self.output_position] =
                (bfinal | (DEFLATE_BLOCKTYPE_UNCOMPRESSED << 1)) as u8;

            self.output_position += 1;
            // output len and nlen
            let len_u16 = len as u16;

            self.output[self.output_position..self.output_position + 2]
                .copy_from_slice(&len_u16.to_le_bytes());
            self.output_position += 2;

            self.output[self.output_position..self.output_position + 2]
                .copy_from_slice(&(!len_u16).to_le_bytes());
            self.output_position += 2;

            // copy from input to output
            self.output[self.output_position..self.output_position + len]
                .copy_from_slice(&self.data[self.input_position..self.input_position + len]);
            self.output_position += len;
            self.input_position += len;

            if self.input_position == final_position
            {
                break;
            }
        }
    }

    /// Encode a deflate stream
    pub fn encode_deflate(&mut self)
    {
        match self.options.strategy
        {
            DeflateEncodingStrategy::NoCompression =>
            {
                self.encode_no_compression(self.data.len());
            }
        }
    }

    #[cfg(feature = "zlib")]
    pub fn encode_zlib(&mut self) -> Vec<u8>
    {
        let extra = 40 * ((self.data.len() + 41) / 40);
        self.output = vec![0_u8; self.data.len() + extra];
        self.write_zlib_header();
        self.output_position = 2;

        self.encode_deflate();

        // add adler hash
        let hash = crate::utils::calc_adler_hash(self.data);
        self.output[self.output_position..self.output_position + 4]
            .copy_from_slice(&hash.to_be_bytes());
        self.output_position += 4;

        self.output.truncate(self.output_position);

        core::mem::take(&mut self.output)
    }
}
