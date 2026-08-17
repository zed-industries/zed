/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Utilities for interacting with the currently set `GlobalTelemetryProvider`

use std::{
    mem,
    sync::{Arc, LazyLock, RwLock},
};

use crate::{
    error::{ErrorKind, GlobalTelemetryProviderError},
    provider::{GlobalTelemetryProvider, TelemetryProvider},
    ObservabilityError,
};

// Statically store the global provider
static GLOBAL_TELEMETRY_PROVIDER: LazyLock<RwLock<GlobalTelemetryProvider>> =
    LazyLock::new(|| RwLock::new(GlobalTelemetryProvider::new(TelemetryProvider::default())));

/// Set the current global [TelemetryProvider].
///
/// This is meant to be run once at the beginning of an application. Will return an [Err] if the
/// [RwLock] holding the global [TelemetryProvider] is locked or poisoned.
pub fn set_telemetry_provider(new_provider: TelemetryProvider) -> Result<(), ObservabilityError> {
    if let Ok(mut old_provider) = GLOBAL_TELEMETRY_PROVIDER.try_write() {
        let new_global_provider = GlobalTelemetryProvider::new(new_provider);

        let _ = mem::replace(&mut *old_provider, new_global_provider);

        Ok(())
    } else {
        Err(ObservabilityError::new(
            ErrorKind::Other,
            GlobalTelemetryProviderError::new("Failed to set global TelemetryProvider."),
        ))
    }
}

/// Get an [Arc] reference to the current global [TelemetryProvider]. Will return an [Err] if the
/// [RwLock] holding the global [TelemetryProvider] is locked or poisoned.
pub fn get_telemetry_provider() -> Result<Arc<TelemetryProvider>, ObservabilityError> {
    if let Ok(tp) = GLOBAL_TELEMETRY_PROVIDER.try_read() {
        Ok(tp.telemetry_provider().clone())
    } else {
        Err(ObservabilityError::new(
            ErrorKind::Other,
            GlobalTelemetryProviderError::new("Failed to get global TelemetryProvider"),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::TelemetryProvider;
    use serial_test::serial;

    // Note: the tests in this module are run serially to prevent them from stepping on each other and poisoning the
    // RwLock holding the GlobalTelemetryProvider
    #[test]
    #[serial]
    fn can_set_global_telemetry_provider() {
        let my_provider = TelemetryProvider::default();

        // Set the new counter and get a reference to the old one
        set_telemetry_provider(my_provider).unwrap();
    }

    #[test]
    #[serial]
    fn can_get_global_telemetry_provider() {
        let curr_provider = get_telemetry_provider().unwrap();

        // Use the global provider to create an instrument and record a value with it
        let curr_mp = curr_provider.meter_provider();
        let curr_meter = curr_mp.get_meter("TestMeter", None);
        let instrument = curr_meter
            .create_monotonic_counter("TestMonoCounter")
            .build();
        instrument.add(4, None, None);
    }
}
