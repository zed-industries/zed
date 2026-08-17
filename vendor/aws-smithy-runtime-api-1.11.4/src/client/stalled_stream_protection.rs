/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

#![allow(missing_docs)]

//! Stalled stream protection.
//!
//! When enabled, upload and download streams that stall (stream no data) for
//! longer than a configured grace period will return an error.

use aws_smithy_types::config_bag::{Storable, StoreReplace};
use std::time::Duration;

/// The default grace period for stalled stream protection.
///
/// When a stream stalls for longer than this grace period, the stream will
/// return an error.
pub const DEFAULT_GRACE_PERIOD: Duration = Duration::from_secs(20);

/// Configuration for stalled stream protection.
///
/// When enabled, download streams that stall out will be cancelled.
#[derive(Clone, Debug)]
pub struct StalledStreamProtectionConfig {
    upload_enabled: bool,
    download_enabled: bool,
    grace_period: Duration,
}

impl StalledStreamProtectionConfig {
    /// Create a new config that enables stalled stream protection for both uploads and downloads.
    pub fn enabled() -> Builder {
        Builder {
            upload_enabled: Some(true),
            download_enabled: Some(true),
            grace_period: None,
        }
    }

    /// Create a new config that disables stalled stream protection.
    pub fn disabled() -> Self {
        Self {
            upload_enabled: false,
            download_enabled: false,
            grace_period: DEFAULT_GRACE_PERIOD,
        }
    }

    /// Return whether stalled stream protection is enabled for either uploads or downloads.
    pub fn is_enabled(&self) -> bool {
        self.upload_enabled || self.download_enabled
    }

    /// True if stalled stream protection is enabled for upload streams.
    pub fn upload_enabled(&self) -> bool {
        self.upload_enabled
    }

    /// True if stalled stream protection is enabled for download streams.
    pub fn download_enabled(&self) -> bool {
        self.download_enabled
    }

    /// Return the grace period for stalled stream protection.
    ///
    /// When a stream stalls for longer than this grace period, the stream will
    /// return an error.
    pub fn grace_period(&self) -> Duration {
        self.grace_period
    }
}

#[derive(Clone, Debug)]
pub struct Builder {
    upload_enabled: Option<bool>,
    download_enabled: Option<bool>,
    grace_period: Option<Duration>,
}

impl Builder {
    /// Set the grace period for stalled stream protection.
    pub fn grace_period(mut self, grace_period: Duration) -> Self {
        self.grace_period = Some(grace_period);
        self
    }

    /// Set the grace period for stalled stream protection.
    pub fn set_grace_period(&mut self, grace_period: Option<Duration>) -> &mut Self {
        self.grace_period = grace_period;
        self
    }

    /// Set whether stalled stream protection is enabled for both uploads and downloads.
    pub fn is_enabled(mut self, enabled: bool) -> Self {
        self.set_is_enabled(Some(enabled));
        self
    }

    /// Set whether stalled stream protection is enabled for both uploads and downloads.
    pub fn set_is_enabled(&mut self, enabled: Option<bool>) -> &mut Self {
        self.set_upload_enabled(enabled);
        self.set_download_enabled(enabled);
        self
    }

    /// Set whether stalled stream protection is enabled for upload streams.
    pub fn upload_enabled(mut self, enabled: bool) -> Self {
        self.set_upload_enabled(Some(enabled));
        self
    }

    /// Set whether stalled stream protection is enabled for upload streams.
    pub fn set_upload_enabled(&mut self, enabled: Option<bool>) -> &mut Self {
        self.upload_enabled = enabled;
        self
    }

    /// Set whether stalled stream protection is enabled for download streams.
    pub fn download_enabled(mut self, enabled: bool) -> Self {
        self.set_download_enabled(Some(enabled));
        self
    }

    /// Set whether stalled stream protection is enabled for download streams.
    pub fn set_download_enabled(&mut self, enabled: Option<bool>) -> &mut Self {
        self.download_enabled = enabled;
        self
    }

    /// Build the config.
    pub fn build(self) -> StalledStreamProtectionConfig {
        StalledStreamProtectionConfig {
            upload_enabled: self.upload_enabled.unwrap_or_default(),
            download_enabled: self.download_enabled.unwrap_or_default(),
            grace_period: self.grace_period.unwrap_or(DEFAULT_GRACE_PERIOD),
        }
    }
}

impl From<StalledStreamProtectionConfig> for Builder {
    fn from(config: StalledStreamProtectionConfig) -> Self {
        Builder {
            upload_enabled: Some(config.upload_enabled),
            download_enabled: Some(config.download_enabled),
            grace_period: Some(config.grace_period),
        }
    }
}

impl Storable for StalledStreamProtectionConfig {
    type Storer = StoreReplace<Self>;
}
