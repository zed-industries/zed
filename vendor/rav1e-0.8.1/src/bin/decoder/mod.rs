// Copyright (c) 2001-2016, Alliance for Open Media. All rights reserved
// Copyright (c) 2017-2021, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use rav1e::prelude::*;

pub mod y4m;

pub trait FrameBuilder<T: Pixel> {
  fn new_frame(&self) -> Frame<T>;
}

pub trait Decoder: Send {
  fn get_video_details(&self) -> VideoDetails;
  fn read_frame<T: Pixel, F: FrameBuilder<T>>(
    &mut self, ctx: &F, cfg: &VideoDetails,
  ) -> Result<Frame<T>, DecodeError>;
}

#[derive(Debug)]
pub enum DecodeError {
  EOF,
  BadInput,
  UnknownColorspace,
  ParseError,
  IoError,
  MemoryLimitExceeded,
}

#[derive(Debug, Clone, Copy)]
pub struct VideoDetails {
  pub width: usize,
  pub height: usize,
  pub sample_aspect_ratio: Rational,
  pub bit_depth: usize,
  pub chroma_sampling: ChromaSampling,
  pub chroma_sample_position: ChromaSamplePosition,
  pub time_base: Rational,
}

impl Default for VideoDetails {
  fn default() -> Self {
    VideoDetails {
      width: 640,
      height: 480,
      sample_aspect_ratio: Rational { num: 1, den: 1 },
      bit_depth: 8,
      chroma_sampling: ChromaSampling::Cs420,
      chroma_sample_position: ChromaSamplePosition::Unknown,
      time_base: Rational { num: 30, den: 1 },
    }
  }
}
