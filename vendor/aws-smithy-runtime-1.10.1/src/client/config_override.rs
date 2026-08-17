/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_async::rt::sleep::SharedAsyncSleep;
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder;
use aws_smithy_types::config_bag::{
    CloneableLayer, FrozenLayer, Layer, Storable, Store, StoreReplace,
};

macro_rules! component {
    ($typ:ty, $accessor:ident, $latest_accessor:ident, $doc:tt) => {
        #[doc = $doc]
        pub fn $accessor(&self) -> Option<$typ> {
            fallback_component!(self, $typ, $accessor)
        }

        #[doc = $doc]
        pub fn $latest_accessor(&self) -> Option<$typ> {
            latest_component!(self, $typ, $accessor)
        }
    };
}
macro_rules! fallback_component {
    ($self:ident, $typ:ty, $accessor:ident) => {
        match &$self.inner {
            Inner::Initial(initial) => initial.components.$accessor(),
            Inner::Override(overrid) => overrid
                .components
                .$accessor()
                .or_else(|| overrid.initial_components.$accessor()),
        }
    };
}
macro_rules! latest_component {
    ($self:ident, $typ:ty, $accessor:ident) => {
        match &$self.inner {
            Inner::Initial(initial) => initial.components.$accessor(),
            Inner::Override(overrid) => overrid.components.$accessor(),
        }
    };
}

struct Initial<'a> {
    config: &'a mut CloneableLayer,
    components: &'a mut RuntimeComponentsBuilder,
}

struct Override<'a> {
    initial_config: FrozenLayer,
    initial_components: &'a RuntimeComponentsBuilder,
    config: &'a mut CloneableLayer,
    components: &'a mut RuntimeComponentsBuilder,
}

enum Inner<'a> {
    Initial(Initial<'a>),
    Override(Override<'a>),
}

/// Utility to simplify config building and config overrides.
///
/// The resolver allows the same initialization logic to be reused
/// for both initial config and override config.
///
/// This resolver can be initialized to one of two modes:
/// 1. _Initial mode_: The resolver is being used in a service `Config` builder's `build()` method, and thus,
///    there is no config override at this point.
/// 2. _Override mode_: The resolver is being used by the `ConfigOverrideRuntimePlugin`'s constructor and needs
///    to incorporate both the original config and the given config override for this operation.
///
/// In all the methods on [`Resolver`], the term "latest" refers to the initial config when in _Initial mode_,
/// and to config override when in _Override mode_.
pub struct Resolver<'a> {
    inner: Inner<'a>,
}

impl<'a> Resolver<'a> {
    /// Construct a new [`Resolver`] in _initial mode_.
    pub fn initial(
        config: &'a mut CloneableLayer,
        components: &'a mut RuntimeComponentsBuilder,
    ) -> Self {
        Self {
            inner: Inner::Initial(Initial { config, components }),
        }
    }

    /// Construct a new [`Resolver`] in _override mode_.
    pub fn overrid(
        initial_config: FrozenLayer,
        initial_components: &'a RuntimeComponentsBuilder,
        config: &'a mut CloneableLayer,
        components: &'a mut RuntimeComponentsBuilder,
    ) -> Self {
        Self {
            inner: Inner::Override(Override {
                initial_config,
                initial_components,
                config,
                components,
            }),
        }
    }

    /// Returns true if in _initial mode_.
    pub fn is_initial(&self) -> bool {
        matches!(self.inner, Inner::Initial(_))
    }

    /// Returns a mutable reference to the latest config.
    pub fn config_mut(&mut self) -> &mut CloneableLayer {
        match &mut self.inner {
            Inner::Initial(initial) => initial.config,
            Inner::Override(overrid) => overrid.config,
        }
    }

    /// Returns a mutable reference to the latest runtime components.
    pub fn runtime_components_mut(&mut self) -> &mut RuntimeComponentsBuilder {
        match &mut self.inner {
            Inner::Initial(initial) => initial.components,
            Inner::Override(overrid) => overrid.components,
        }
    }

    /// Returns true if the latest config has `T` set.
    ///
    /// The "latest" is initial for `Resolver::Initial`, and override for `Resolver::Override`.
    pub fn is_latest_set<T>(&self) -> bool
    where
        T: Storable<Storer = StoreReplace<T>>,
    {
        self.config().load::<T>().is_some()
    }

    /// Returns true if `T` is set anywhere.
    pub fn is_set<T>(&self) -> bool
    where
        T: Storable<Storer = StoreReplace<T>>,
    {
        match &self.inner {
            Inner::Initial(initial) => initial.config.load::<T>().is_some(),
            Inner::Override(overrid) => {
                overrid.initial_config.load::<T>().is_some() || overrid.config.load::<T>().is_some()
            }
        }
    }

    /// Resolves the value `T` with fallback
    pub fn resolve_config<T>(&self) -> <T::Storer as Store>::ReturnedType<'_>
    where
        T: Storable<Storer = StoreReplace<T>>,
    {
        let mut maybe_value = self.config().load::<T>();
        if maybe_value.is_none() {
            // Try to fallback
            if let Inner::Override(overrid) = &self.inner {
                maybe_value = overrid.initial_config.load::<T>()
            }
        }
        maybe_value
    }

    // Add additional component methods as needed
    component!(
        SharedAsyncSleep,
        sleep_impl,
        latest_sleep_impl,
        "The async sleep implementation."
    );

    fn config(&self) -> &Layer {
        match &self.inner {
            Inner::Initial(initial) => initial.config,
            Inner::Override(overrid) => overrid.config,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aws_smithy_types::config_bag::CloneableLayer;

    #[derive(Clone, Debug)]
    struct TestStorable(String);
    impl Storable for TestStorable {
        type Storer = StoreReplace<Self>;
    }

    #[test]
    fn initial_mode_config() {
        let mut config = CloneableLayer::new("test");
        let mut components = RuntimeComponentsBuilder::new("test");

        let mut resolver = Resolver::initial(&mut config, &mut components);
        assert!(resolver.is_initial());
        assert!(!resolver.is_latest_set::<TestStorable>());
        assert!(!resolver.is_set::<TestStorable>());
        assert!(resolver.resolve_config::<TestStorable>().is_none());

        resolver.config_mut().store_put(TestStorable("test".into()));
        assert!(resolver.is_latest_set::<TestStorable>());
        assert!(resolver.is_set::<TestStorable>());
        assert_eq!("test", resolver.resolve_config::<TestStorable>().unwrap().0);
    }

    #[test]
    fn override_mode_config() {
        let mut initial_config = CloneableLayer::new("initial");
        let initial_components = RuntimeComponentsBuilder::new("initial");
        let mut config = CloneableLayer::new("override");
        let mut components = RuntimeComponentsBuilder::new("override");

        let resolver = Resolver::overrid(
            initial_config.clone().freeze(),
            &initial_components,
            &mut config,
            &mut components,
        );
        assert!(!resolver.is_initial());
        assert!(!resolver.is_latest_set::<TestStorable>());
        assert!(!resolver.is_set::<TestStorable>());
        assert!(resolver.resolve_config::<TestStorable>().is_none());

        initial_config.store_put(TestStorable("test".into()));
        let resolver = Resolver::overrid(
            initial_config.clone().freeze(),
            &initial_components,
            &mut config,
            &mut components,
        );
        assert!(!resolver.is_latest_set::<TestStorable>());
        assert!(resolver.is_set::<TestStorable>());
        assert_eq!("test", resolver.resolve_config::<TestStorable>().unwrap().0);

        initial_config.unset::<TestStorable>();
        config.store_put(TestStorable("test".into()));
        let resolver = Resolver::overrid(
            initial_config.clone().freeze(),
            &initial_components,
            &mut config,
            &mut components,
        );
        assert!(resolver.is_latest_set::<TestStorable>());
        assert!(resolver.is_set::<TestStorable>());
        assert_eq!("test", resolver.resolve_config::<TestStorable>().unwrap().0);

        initial_config.store_put(TestStorable("override me".into()));
        config.store_put(TestStorable("override".into()));
        let resolver = Resolver::overrid(
            initial_config.freeze(),
            &initial_components,
            &mut config,
            &mut components,
        );
        assert!(resolver.is_latest_set::<TestStorable>());
        assert!(resolver.is_set::<TestStorable>());
        assert_eq!(
            "override",
            resolver.resolve_config::<TestStorable>().unwrap().0
        );
    }
}
