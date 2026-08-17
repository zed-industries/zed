// Copyright (c) 2001-2016, Alliance for Open Media. All rights reserved
// Copyright (c) 2017-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use super::Muxer;
use crate::error::*;
use ivf::*;
use rav1e::prelude::*;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

pub struct IvfMuxer {
  output: Box<dyn Write + Send>,
}

impl Muxer for IvfMuxer {
  fn write_header(
    &mut self, width: usize, height: usize, framerate_num: usize,
    framerate_den: usize,
  ) {
    write_ivf_header(
      &mut self.output,
      width,
      height,
      framerate_num,
      framerate_den,
    );
  }

  #[profiling::function]
  fn write_frame(&mut self, pts: u64, data: &[u8], _frame_type: FrameType) {
    write_ivf_frame(&mut self.output, pts, data);
  }

  fn flush(&mut self) -> io::Result<()> {
    self.output.flush()
  }
}

cfg_if::cfg_if! {
  if #[cfg(unix)] {
    use std::os::unix::fs::*;
    fn is_file<P: AsRef<Path>>(path: P) -> bool {
      fs::metadata(path).map(|meta| {
        !meta.file_type().is_char_device() && !meta.file_type().is_socket()
      }).unwrap_or(false)
    }
  } else {
    fn is_file<P: AsRef<Path>>(path: P) -> bool {
      fs::metadata(path).is_ok()
    }
  }
}

impl IvfMuxer {
  pub fn check_file<P: AsRef<Path>>(path: P) -> Result<(), CliError> {
    if is_file(path.as_ref()) {
      eprint!(
        "File '{}' already exists. Overwrite ? [y/N] ",
        path.as_ref().display()
      );
      io::stdout().flush().unwrap();

      let mut option_input = String::new();
      io::stdin()
        .read_line(&mut option_input)
        .expect("Failed to read option.");

      match option_input.as_str().trim() {
        "y" | "Y" => return Ok(()),
        _ => return Err(CliError::new("Not overwriting, exiting.")),
      };
    }
    Ok(())
  }

  pub fn open<P: AsRef<Path>>(
    path: P,
  ) -> Result<Box<dyn Muxer + Send>, CliError> {
    let ivf = IvfMuxer {
      output: match path.as_ref().to_str() {
        Some("-") => Box::new(std::io::stdout()),
        _ => Box::new(
          File::create(path)
            .map_err(|e| e.context("Cannot open output file"))?,
        ),
      },
    };
    Ok(Box::new(ivf))
  }
}
