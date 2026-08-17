/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Instruments are used to record values for metrics.

use std::{borrow::Cow, fmt::Debug, marker::PhantomData, sync::Arc};

use crate::{meter::Meter, Attributes, Context};

/// Configuration for building a sync instrument.
#[non_exhaustive]
pub struct InstrumentBuilder<'a, T> {
    instrument_provider: &'a dyn ProvideInstrument,
    name: Cow<'static, str>,
    description: Option<Cow<'static, str>>,
    units: Option<Cow<'static, str>>,
    _phantom: PhantomData<T>,
}

impl<'a, T> InstrumentBuilder<'a, T> {
    /// Create a new instrument builder
    pub(crate) fn new(meter: &'a Meter, name: Cow<'static, str>) -> Self {
        InstrumentBuilder {
            instrument_provider: meter.instrument_provider.as_ref(),
            name,
            description: None,
            units: None,
            _phantom: PhantomData::<T>,
        }
    }

    /// Get the name.
    pub fn get_name(&self) -> &Cow<'static, str> {
        &self.name
    }

    /// Set the description.
    pub fn set_description(mut self, description: impl Into<Cow<'static, str>>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Get the description.
    pub fn get_description(&self) -> &Option<Cow<'static, str>> {
        &self.description
    }

    /// Set the units.
    pub fn set_units(mut self, units: impl Into<Cow<'static, str>>) -> Self {
        self.units = Some(units.into());
        self
    }

    /// Get the units.
    pub fn get_units(&self) -> &Option<Cow<'static, str>> {
        &self.units
    }
}

/// Takes in the name of function from [ProvideInstrument] and the type of instrument being created
/// (ex: [Histogram]) and adds a `build` function for it.
macro_rules! build_instrument {
    ($name:ident, $instrument:ty) => {
        impl<'a> InstrumentBuilder<'a, $instrument> {
            #[doc = concat!("Create a new `",  stringify!($instrument), "`.")]
            pub fn build(self) -> $instrument {
                self.instrument_provider.$name(self)
            }
        }
    };
}

build_instrument!(create_histogram, Arc<dyn Histogram>);
build_instrument!(create_monotonic_counter, Arc<dyn MonotonicCounter>);
build_instrument!(create_up_down_counter, Arc<dyn UpDownCounter>);

/// Configuration for building an async instrument.
#[non_exhaustive]
pub struct AsyncInstrumentBuilder<'a, T, M> {
    instrument_provider: &'a dyn ProvideInstrument,
    name: Cow<'static, str>,
    // Implementation note: I could not make the lifetimes work out in the impl ProvideInstrument
    // in aws-smithy-observability-otel without making this field pub
    /// The callback function for this AsyncInstrumentBuilder.
    #[allow(clippy::type_complexity)]
    pub callback: Arc<dyn Fn(&dyn AsyncMeasure<Value = M>) + Send + Sync>,
    description: Option<Cow<'static, str>>,
    units: Option<Cow<'static, str>>,
    _phantom: PhantomData<T>,
}

#[allow(clippy::type_complexity)]
impl<'a, T, M> AsyncInstrumentBuilder<'a, T, M> {
    /// Create a new async instrument builder
    pub(crate) fn new(
        meter: &'a Meter,
        name: Cow<'static, str>,
        callback: Arc<dyn Fn(&dyn AsyncMeasure<Value = M>) + Send + Sync>,
    ) -> Self {
        AsyncInstrumentBuilder {
            instrument_provider: meter.instrument_provider.as_ref(),
            name,
            callback,
            description: None,
            units: None,
            _phantom: PhantomData::<T>,
        }
    }

    /// Get the name.
    pub fn get_name(&self) -> &Cow<'static, str> {
        &self.name
    }

    /// Get the callback function.
    pub fn get_callback(&self) -> Arc<dyn Fn(&dyn AsyncMeasure<Value = M>) + Send + Sync> {
        self.callback.clone()
    }

    /// Set the description.
    pub fn set_description(mut self, description: impl Into<Cow<'static, str>>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Get the description.
    pub fn get_description(&self) -> &Option<Cow<'static, str>> {
        &self.description
    }

    /// Set the units.
    pub fn set_units(mut self, units: impl Into<Cow<'static, str>>) -> Self {
        self.units = Some(units.into());
        self
    }

    /// Get the units.
    pub fn get_units(&self) -> &Option<Cow<'static, str>> {
        &self.units
    }
}

/// Takes in the name of function from [ProvideInstrument] and the type of instrument being created
/// (ex: [AsyncMeasure]) and adds a `build` function for it.
//TODO(observability): Can I derive the measurement from the Value of the instrument type or vice versa?
macro_rules! build_async_instrument {
    ($name:ident, $instrument:ty, $measurement:ty) => {
        impl<'a> AsyncInstrumentBuilder<'a, $instrument, $measurement> {
            #[doc = concat!("Create a new `",  stringify!($instrument), "`.")]
            pub fn build(self) -> $instrument {
                self.instrument_provider.$name(self)
            }
        }
    };
}

build_async_instrument!(create_gauge, Arc<dyn AsyncMeasure<Value = f64>>, f64);
build_async_instrument!(
    create_async_up_down_counter,
    Arc<dyn AsyncMeasure<Value = i64>>,
    i64
);
build_async_instrument!(
    create_async_monotonic_counter,
    Arc<dyn AsyncMeasure<Value = u64>>,
    u64
);

/// The entry point to creating instruments. A grouping of related metrics.
pub trait ProvideInstrument: Send + Sync + Debug {
    /// Create a new Gauge.
    #[allow(clippy::type_complexity)]
    fn create_gauge(
        &self,
        builder: AsyncInstrumentBuilder<'_, Arc<dyn AsyncMeasure<Value = f64>>, f64>,
    ) -> Arc<dyn AsyncMeasure<Value = f64>>;

    /// Create a new [UpDownCounter].
    fn create_up_down_counter(
        &self,
        builder: InstrumentBuilder<'_, Arc<dyn UpDownCounter>>,
    ) -> Arc<dyn UpDownCounter>;

    /// Create a new AsyncUpDownCounter.
    #[allow(clippy::type_complexity)]
    fn create_async_up_down_counter(
        &self,
        builder: AsyncInstrumentBuilder<'_, Arc<dyn AsyncMeasure<Value = i64>>, i64>,
    ) -> Arc<dyn AsyncMeasure<Value = i64>>;

    /// Create a new [MonotonicCounter].
    fn create_monotonic_counter(
        &self,
        builder: InstrumentBuilder<'_, Arc<dyn MonotonicCounter>>,
    ) -> Arc<dyn MonotonicCounter>;

    /// Create a new AsyncMonotonicCounter.
    #[allow(clippy::type_complexity)]
    fn create_async_monotonic_counter(
        &self,
        builder: AsyncInstrumentBuilder<'_, Arc<dyn AsyncMeasure<Value = u64>>, u64>,
    ) -> Arc<dyn AsyncMeasure<Value = u64>>;

    /// Create a new [Histogram].
    fn create_histogram(
        &self,
        builder: InstrumentBuilder<'_, Arc<dyn Histogram>>,
    ) -> Arc<dyn Histogram>;
}

/// Collects a set of events with an event count and sum for all events.
pub trait Histogram: Send + Sync + Debug {
    /// Record a value.
    fn record(&self, value: f64, attributes: Option<&Attributes>, context: Option<&dyn Context>);
}

/// A counter that monotonically increases.
pub trait MonotonicCounter: Send + Sync + Debug {
    /// Increment a counter by a fixed amount.
    fn add(&self, value: u64, attributes: Option<&Attributes>, context: Option<&dyn Context>);
}

/// A counter that can increase or decrease.
pub trait UpDownCounter: Send + Sync + Debug {
    /// Increment or decrement a counter by a fixed amount.
    fn add(&self, value: i64, attributes: Option<&Attributes>, context: Option<&dyn Context>);
}

/// A measurement that can be taken asynchronously.
pub trait AsyncMeasure: Send + Sync + Debug {
    /// The type recorded by the measurement.
    type Value;

    /// Record a value
    fn record(
        &self,
        value: Self::Value,
        attributes: Option<&Attributes>,
        context: Option<&dyn Context>,
    );

    /// Stop recording, unregister callback.
    fn stop(&self);
}
