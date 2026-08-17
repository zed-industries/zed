// Copyright (c) 2017-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use crate::decoder::VideoDetails;
use rav1e::prelude::*;
use std::io::Write;
use std::slice;

#[profiling::function]
pub fn write_y4m_frame<T: Pixel>(
  y4m_enc: &mut y4m::Encoder<Box<dyn Write + Send>>, rec: &Frame<T>,
  y4m_details: VideoDetails,
) {
  let planes =
    if y4m_details.chroma_sampling == ChromaSampling::Cs400 { 1 } else { 3 };
  let bytes_per_sample = if y4m_details.bit_depth > 8 { 2 } else { 1 };
  let (chroma_width, chroma_height) = y4m_details
    .chroma_sampling
    .get_chroma_dimensions(y4m_details.width, y4m_details.height);
  let pitch_y = y4m_details.width * bytes_per_sample;
  let pitch_uv = chroma_width * bytes_per_sample;

  let (mut rec_y, mut rec_u, mut rec_v) = (
    vec![128u8; pitch_y * y4m_details.height],
    vec![128u8; pitch_uv * chroma_height],
    vec![128u8; pitch_uv * chroma_height],
  );

  let (stride_y, stride_u, stride_v) = (
    rec.planes[0].cfg.stride,
    rec.planes[1].cfg.stride,
    rec.planes[2].cfg.stride,
  );

  for (line, line_out) in
    rec.planes[0].data_origin().chunks(stride_y).zip(rec_y.chunks_mut(pitch_y))
  {
    if y4m_details.bit_depth > 8 {
      // SAFETY: This is essentially doing a transmute to u16, but safer.
      unsafe {
        line_out.copy_from_slice(slice::from_raw_parts::<u8>(
          line.as_ptr() as *const u8,
          pitch_y,
        ));
      }
    } else {
      line_out.copy_from_slice(
        &line.iter().map(|&v| u8::cast_from(v)).collect::<Vec<u8>>()
          [..pitch_y],
      );
    }
  }

  if planes > 1 {
    for (line, line_out) in rec.planes[1]
      .data_origin()
      .chunks(stride_u)
      .zip(rec_u.chunks_mut(pitch_uv))
    {
      if y4m_details.bit_depth > 8 {
        // SAFETY: This is essentially doing a transmute to u16, but safer.
        unsafe {
          line_out.copy_from_slice(slice::from_raw_parts::<u8>(
            line.as_ptr() as *const u8,
            pitch_uv,
          ));
        }
      } else {
        line_out.copy_from_slice(
          &line.iter().map(|&v| u8::cast_from(v)).collect::<Vec<u8>>()
            [..pitch_uv],
        );
      }
    }
    for (line, line_out) in rec.planes[2]
      .data_origin()
      .chunks(stride_v)
      .zip(rec_v.chunks_mut(pitch_uv))
    {
      if y4m_details.bit_depth > 8 {
        // SAFETY: This is essentially doing a transmute to u16, but safer.
        unsafe {
          line_out.copy_from_slice(slice::from_raw_parts::<u8>(
            line.as_ptr() as *const u8,
            pitch_uv,
          ));
        }
      } else {
        line_out.copy_from_slice(
          &line.iter().map(|&v| u8::cast_from(v)).collect::<Vec<u8>>()
            [..pitch_uv],
        );
      }
    }
  }

  let rec_frame = y4m::Frame::new([&rec_y, &rec_u, &rec_v], None);
  y4m_enc.write_frame(&rec_frame).unwrap();
}
