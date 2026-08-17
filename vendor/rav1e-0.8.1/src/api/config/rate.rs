// Copyright (c) 2020-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use thiserror::Error;

use crate::rate::*;

/// Rate control errors
#[derive(Debug, Error)]
pub enum Error {
  /// The summary provided is not compatible with the current encoder version
  #[error("Incompatible version {0}")]
  InvalidVersion(i64),
  /// The summary provided is possibly corrupted
  #[error("The summary content is invalid: {0}")]
  CorruptedSummary(String),
}

/// Rate control configuration
#[derive(Clone, Debug, Default)]
pub struct RateControlConfig {
  pub(crate) emit_pass_data: bool,
  pub(crate) summary: Option<RateControlSummary>,
}

pub use crate::rate::RCSummary as RateControlSummary;

impl RateControlSummary {
  /// Deserializes a byte slice into a `RateControlSummary`
  pub(crate) fn from_slice(bytes: &[u8]) -> Result<Self, Error> {
    let mut de = RCDeserialize::default();
    let _ = de.buffer_fill(bytes, 0, TWOPASS_HEADER_SZ);

    de.parse_summary().map_err(Error::CorruptedSummary)
  }
}

impl RateControlConfig {
  /// Create a rate control configuration from a serialized summary
  ///
  /// # Errors
  ///
  /// Returns an error if the serialized data is invalid.
  pub fn from_summary_slice(bytes: &[u8]) -> Result<Self, Error> {
    Ok(Self {
      summary: Some(RateControlSummary::from_slice(bytes)?),
      ..Default::default()
    })
  }
  /// Create a default rate control configuration
  ///
  /// By default the encoder is in single pass mode.
  pub fn new() -> Self {
    Default::default()
  }

  /// Set a rate control summary
  ///
  /// Enable the second pass encoding mode
  pub const fn with_summary(mut self, summary: RateControlSummary) -> Self {
    self.summary = Some(summary);
    self
  }

  /// Emit the current pass data
  ///
  /// The pass data will be used in a second pass encoding session
  pub const fn with_emit_data(mut self, emit: bool) -> Self {
    self.emit_pass_data = emit;
    self
  }
}
