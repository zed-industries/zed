// Symphonia
// Copyright (c) 2019-2024 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Readers for FLAC comment and picture metadata blocks.

use std::num::NonZeroU32;

use symphonia_core::errors::{decode_error, Result};
use symphonia_core::io::ReadBytes;
use symphonia_core::meta::{ColorMode, MetadataBuilder, Size, StandardTagKey, Tag, Value, Visual};

use crate::{id3v2, vorbis};

/// Converts a string of bytes to an ASCII string if all characters are within the printable ASCII
/// range. If a null byte is encounted, the string terminates at that point.
fn printable_ascii_to_string(bytes: &[u8]) -> Option<String> {
    let mut result = String::with_capacity(bytes.len());

    for c in bytes {
        match c {
            0x00 => break,
            0x20..=0x7e => result.push(char::from(*c)),
            _ => return None,
        }
    }

    Some(result)
}

/// Read a comment metadata block.
pub fn read_comment_block<B: ReadBytes>(
    reader: &mut B,
    metadata: &mut MetadataBuilder,
) -> Result<()> {
    vorbis::read_comment_no_framing(reader, metadata)
}

/// Read a picture metadata block.
pub fn read_picture_block<B: ReadBytes>(
    reader: &mut B,
    metadata: &mut MetadataBuilder,
) -> Result<()> {
    let type_enc = reader.read_be_u32()?;

    // Read the Media Type length in bytes.
    let media_type_len = reader.read_be_u32()? as usize;

    // Read the Media Type bytes
    let mut media_type_buf = vec![0u8; media_type_len];
    reader.read_buf_exact(&mut media_type_buf)?;

    // Convert Media Type bytes to an ASCII string. Non-printable ASCII characters are invalid.
    let media_type = match printable_ascii_to_string(&media_type_buf) {
        Some(s) => s,
        None => return decode_error("meta (flac): picture mime-type contains invalid characters"),
    };

    // Read the description length in bytes.
    let desc_len = reader.read_be_u32()? as usize;

    // Read the description bytes.
    let mut desc_buf = vec![0u8; desc_len];
    reader.read_buf_exact(&mut desc_buf)?;

    let desc = String::from_utf8_lossy(&desc_buf);

    // Convert description bytes into a standard Vorbis DESCRIPTION tag.
    let tags = vec![Tag::new(Some(StandardTagKey::Description), "DESCRIPTION", Value::from(desc))];

    // Read the width, and height of the visual.
    let width = reader.read_be_u32()?;
    let height = reader.read_be_u32()?;

    // If either the width or height is 0, then the size is invalid.
    let dimensions = if width > 0 && height > 0 { Some(Size { width, height }) } else { None };

    // Read bits-per-pixel of the visual.
    let bits_per_pixel = NonZeroU32::new(reader.read_be_u32()?);

    // Indexed colours is only valid for image formats that use an indexed colour palette. If it is
    // 0, the image does not used indexed colours.
    let indexed_colours_enc = reader.read_be_u32()?;

    let color_mode = match indexed_colours_enc {
        0 => Some(ColorMode::Discrete),
        _ => Some(ColorMode::Indexed(NonZeroU32::new(indexed_colours_enc).unwrap())),
    };

    // Read the image data
    let data_len = reader.read_be_u32()? as usize;
    let data = reader.read_boxed_slice_exact(data_len)?;

    metadata.add_visual(Visual {
        media_type,
        dimensions,
        bits_per_pixel,
        color_mode,
        usage: id3v2::util::apic_picture_type_to_visual_key(type_enc),
        tags,
        data,
    });

    Ok(())
}
