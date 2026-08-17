// Copyright (c) 2017-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

mod ivf;
use self::ivf::IvfMuxer;

mod y4m;
pub use self::y4m::write_y4m_frame;

use rav1e::prelude::*;

use std::ffi::OsStr;
use std::io;
use std::path::Path;

use crate::error::*;

pub trait Muxer: Send {
  fn write_header(
    &mut self, width: usize, height: usize, framerate_num: usize,
    framerate_den: usize,
  );

  fn write_frame(&mut self, pts: u64, data: &[u8], frame_type: FrameType);

  fn flush(&mut self) -> io::Result<()>;
}

pub fn create_muxer<P: AsRef<Path>>(
  path: P, overwrite: bool,
) -> Result<Box<dyn Muxer + Send>, CliError> {
  if !overwrite {
    IvfMuxer::check_file(path.as_ref())?;
  }

  if let Some(path) = path.as_ref().to_str() {
    if path == "-" {
      return IvfMuxer::open(path);
    }
  }

  let ext = path
    .as_ref()
    .extension()
    .and_then(OsStr::to_str)
    .map(str::to_lowercase)
    .unwrap_or_else(|| "ivf".into());

  match &ext[..] {
    "ivf" => IvfMuxer::open(path),
    _e => {
      panic!("{ext} is not a supported extension, please change to .ivf");
    }
  }
}
