/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Metrics are used to gain insight into the operational performance and health of a system in
//! real time.

use crate::instruments::{
    AsyncInstrumentBuilder, AsyncMeasure, Histogram, InstrumentBuilder, MonotonicCounter,
    UpDownCounter,
};
use crate::{attributes::Attributes, instruments::ProvideInstrument};
use std::{borrow::Cow, fmt::Debug, sync::Arc};

/// Provides named instances of [Meter].
pub trait ProvideMeter: Send + Sync + Debug {
    /// Get or create a named [Meter].
    fn get_meter(&self, scope: &'static str, attributes: Option<&Attributes>) -> Meter;

    /// Returns the name of this provider implementation.
    /// This is used for feature tracking without requiring type imports.
    fn provider_name(&self) -> &'static str {
        "unknown"
    }
}

/// The entry point to creating instruments. A grouping of related metrics.
#[derive(Clone)]
pub struct Meter {
    pub(crate) instrument_provider: Arc<dyn ProvideInstrument + Send + Sync>,
}

impl Meter {
    /// Create a new [Meter] from an [ProvideInstrument]
    pub fn new(instrument_provider: Arc<dyn ProvideInstrument + Send + Sync>) -> Self {
        Meter {
            instrument_provider,
        }
    }

    /// Create a new Gauge.
    #[allow(clippy::type_complexity)]
    pub fn create_gauge<F>(
        &self,
        name: impl Into<Cow<'static, str>>,
        callback: F,
    ) -> AsyncInstrumentBuilder<'_, Arc<dyn AsyncMeasure<Value = f64>>, f64>
    where
        F: Fn(&dyn AsyncMeasure<Value = f64>) + Send + Sync + 'static,
    {
        AsyncInstrumentBuilder::new(self, name.into(), Arc::new(callback))
    }

    /// Create a new [UpDownCounter].
    pub fn create_up_down_counter(
        &self,
        name: impl Into<Cow<'static, str>>,
    ) -> InstrumentBuilder<'_, Arc<dyn UpDownCounter>> {
        InstrumentBuilder::new(self, name.into())
    }

    /// Create a new AsyncUpDownCounter.
    #[allow(clippy::type_complexity)]
    pub fn create_async_up_down_counter<F>(
        &self,
        name: impl Into<Cow<'static, str>>,
        callback: F,
    ) -> AsyncInstrumentBuilder<'_, Arc<dyn AsyncMeasure<Value = i64>>, i64>
    where
        F: Fn(&dyn AsyncMeasure<Value = i64>) + Send + Sync + 'static,
    {
        AsyncInstrumentBuilder::new(self, name.into(), Arc::new(callback))
    }

    /// Create a new [MonotonicCounter].
    pub fn create_monotonic_counter(
        &self,
        name: impl Into<Cow<'static, str>>,
    ) -> InstrumentBuilder<'_, Arc<dyn MonotonicCounter>> {
        InstrumentBuilder::new(self, name.into())
    }

    /// Create a new AsyncMonotonicCounter.
    #[allow(clippy::type_complexity)]
    pub fn create_async_monotonic_counter<F>(
        &self,
        name: impl Into<Cow<'static, str>>,
        callback: F,
    ) -> AsyncInstrumentBuilder<'_, Arc<dyn AsyncMeasure<Value = u64>>, u64>
    where
        F: Fn(&dyn AsyncMeasure<Value = u64>) + Send + Sync + 'static,
    {
        AsyncInstrumentBuilder::new(self, name.into(), Arc::new(callback))
    }

    /// Create a new [Histogram].
    pub fn create_histogram(
        &self,
        name: impl Into<Cow<'static, str>>,
    ) -> InstrumentBuilder<'_, Arc<dyn Histogram>> {
        InstrumentBuilder::new(self, name.into())
    }
}
