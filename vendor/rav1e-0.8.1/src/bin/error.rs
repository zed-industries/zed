// Copyright (c) 2001-2016, Alliance for Open Media. All rights reserved
// Copyright (c) 2017-2020, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

#[derive(Debug, thiserror::Error)]
pub enum CliError {
  #[error("Cannot parse option `{opt}`: {err}")]
  ParseInt { opt: String, err: std::num::ParseIntError },
  #[error("{msg}")]
  Message { msg: String },
  #[error("{msg}: {e}")]
  Generic { msg: String, e: String },
}

impl CliError {
  pub fn new(msg: &str) -> CliError {
    CliError::Message { msg: msg.to_owned() }
  }
}

pub trait ToError: std::error::Error + Sized {
  fn context(self, msg: &str) -> CliError {
    CliError::Generic { msg: msg.to_owned(), e: self.to_string() }
  }
}

impl ToError for std::num::ParseIntError {
  fn context(self, opt: &str) -> CliError {
    CliError::ParseInt { opt: opt.to_lowercase(), err: self }
  }
}

impl ToError for std::io::Error {}
impl ToError for rav1e::InvalidConfig {}
impl ToError for rav1e::EncoderStatus {}
impl ToError for rav1e::config::RateControlError {}

pub fn print_error(e: &dyn std::error::Error) {
  error!("{}", e);
  let mut cause = e.source();
  while let Some(e) = cause {
    error!("Caused by: {}", e);
    cause = e.source();
  }
}
