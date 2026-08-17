/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Conversion traits for converting an unshared type into a shared type.
//!
//! The standard [`From`]/[`Into`] traits can't be
//! used for this purpose due to the blanket implementation of `Into`.
//!
//! This implementation also adds a [`maybe_shared`] method and [`impl_shared_conversions`](crate::impl_shared_conversions)
//! macro to trivially avoid nesting shared types with other shared types.
//!
//! # What is a shared type?
//!
//! A shared type is a new-type around a `Send + Sync` reference counting smart pointer
//! (i.e., an [`Arc`](std::sync::Arc)) around an object-safe trait. Shared types are
//! used to share a trait object among multiple threads/clients/requests.
#![cfg_attr(
    feature = "client",
    doc = "
For example, [`SharedHttpConnector`](crate::client::http::SharedHttpConnector), is
a shared type for the [`HttpConnector`](crate::client::http::HttpConnector) trait,
which allows for sharing a single HTTP connector instance (and its connection pool) among multiple clients.
"
)]
//!
//! A shared type implements the [`FromUnshared`] trait, which allows any implementation
//! of the trait it wraps to easily be converted into it.
//!
#![cfg_attr(
    feature = "client",
    doc = "
To illustrate, let's examine the
[`RuntimePlugin`](crate::client::runtime_plugin::RuntimePlugin)/[`SharedRuntimePlugin`](crate::client::runtime_plugin::SharedRuntimePlugin)
duo.
The following instantiates a concrete implementation of the `RuntimePlugin` trait.
We can do `RuntimePlugin` things on this instance.

```rust,no_run
use aws_smithy_runtime_api::client::runtime_plugin::StaticRuntimePlugin;

let some_plugin = StaticRuntimePlugin::new();
```

We can convert this instance into a shared type in two different ways.

```rust,no_run
# use aws_smithy_runtime_api::client::runtime_plugin::StaticRuntimePlugin;
# let some_plugin = StaticRuntimePlugin::new();
use aws_smithy_runtime_api::client::runtime_plugin::SharedRuntimePlugin;
use aws_smithy_runtime_api::shared::{IntoShared, FromUnshared};

// Using the `IntoShared` trait
let shared: SharedRuntimePlugin = some_plugin.into_shared();

// Using the `FromUnshared` trait:
# let some_plugin = StaticRuntimePlugin::new();
let shared = SharedRuntimePlugin::from_unshared(some_plugin);
```

The `IntoShared` trait is useful for making functions that take any `RuntimePlugin` impl and convert it to a shared type.
For example, this function will convert the given `plugin` argument into a `SharedRuntimePlugin`.

```rust,no_run
# use aws_smithy_runtime_api::client::runtime_plugin::{RuntimePlugin, SharedRuntimePlugin};
use aws_smithy_runtime_api::shared::IntoShared;

fn take_shared(plugin: impl RuntimePlugin + 'static) {
    let _plugin: SharedRuntimePlugin = plugin.into_shared();
}
```

This can be called with different types, and even if a `SharedRuntimePlugin` is passed in, it won't nest that
`SharedRuntimePlugin` inside of another `SharedRuntimePlugin`.

```rust,no_run
# use aws_smithy_runtime_api::client::runtime_plugin::{RuntimePlugin, SharedRuntimePlugin, StaticRuntimePlugin};
# use aws_smithy_runtime_api::shared::{IntoShared, FromUnshared};
# fn take_shared(plugin: impl RuntimePlugin + 'static) {
#     let _plugin: SharedRuntimePlugin = plugin.into_shared();
# }
// Automatically converts it to `SharedRuntimePlugin(StaticRuntimePlugin)`
take_shared(StaticRuntimePlugin::new());

// This is OK.
// It create a `SharedRuntimePlugin(StaticRuntimePlugin))`
// instead of a nested `SharedRuntimePlugin(SharedRuntimePlugin(StaticRuntimePlugin)))`
take_shared(SharedRuntimePlugin::new(StaticRuntimePlugin::new()));
```
"
)]

use std::any::{Any, TypeId};

/// Like the `From` trait, but for converting to a shared type.
///
/// See the [module docs](crate::shared) for information about shared types.
pub trait FromUnshared<Unshared> {
    /// Creates a shared type from an unshared type.
    fn from_unshared(value: Unshared) -> Self;
}

/// Like the `Into` trait, but for (efficiently) converting into a shared type.
///
/// If the type is already a shared type, it won't be nested in another shared type.
///
/// See the [module docs](crate::shared) for information about shared types.
pub trait IntoShared<Shared> {
    /// Creates a shared type from an unshared type.
    fn into_shared(self) -> Shared;
}

impl<Unshared, Shared> IntoShared<Shared> for Unshared
where
    Shared: FromUnshared<Unshared>,
{
    fn into_shared(self) -> Shared {
        FromUnshared::from_unshared(self)
    }
}

/// Given a `value`, determine if that value is already shared. If it is, return it. Otherwise, wrap it in a shared type.
///
/// See the [module docs](crate::shared) for information about shared types.
pub fn maybe_shared<Shared, MaybeShared, F>(value: MaybeShared, ctor: F) -> Shared
where
    Shared: 'static,
    MaybeShared: IntoShared<Shared> + 'static,
    F: FnOnce(MaybeShared) -> Shared,
{
    // Check if the type is already a shared type
    if TypeId::of::<MaybeShared>() == TypeId::of::<Shared>() {
        // Convince the compiler it is already a shared type and return it
        let mut placeholder = Some(value);
        let value: Shared = (&mut placeholder as &mut dyn Any)
            .downcast_mut::<Option<Shared>>()
            .expect("type checked above")
            .take()
            .expect("set to Some above");
        value
    } else {
        (ctor)(value)
    }
}

/// Implements `FromUnshared` for a shared type.
///
/// See the [`shared` module docs](crate::shared) for information about shared types.
///
/// # Example
/// ```rust,no_run
/// use aws_smithy_runtime_api::impl_shared_conversions;
/// use std::sync::Arc;
///
/// trait Thing {}
///
/// struct Thingamajig;
/// impl Thing for Thingamajig {}
///
/// struct SharedThing(Arc<dyn Thing>);
/// impl Thing for SharedThing {}
/// impl SharedThing {
///     fn new(thing: impl Thing + 'static) -> Self {
///         Self(Arc::new(thing))
///     }
/// }
/// impl_shared_conversions!(convert SharedThing from Thing using SharedThing::new);
/// ```
#[macro_export]
macro_rules! impl_shared_conversions {
    (convert $shared_type:ident from $unshared_trait:ident using $ctor:expr) => {
        impl<T> $crate::shared::FromUnshared<T> for $shared_type
        where
            T: $unshared_trait + 'static,
        {
            fn from_unshared(value: T) -> Self {
                $crate::shared::maybe_shared(value, $ctor)
            }
        }
    };
}

// TODO(https://github.com/smithy-lang/smithy-rs/issues/3016): Move these impls once aws-smithy-async is merged into aws-smithy-runtime-api
mod async_impls {
    use aws_smithy_async::rt::sleep::{AsyncSleep, SharedAsyncSleep};
    use aws_smithy_async::time::{SharedTimeSource, TimeSource};
    impl_shared_conversions!(convert SharedAsyncSleep from AsyncSleep using SharedAsyncSleep::new);
    impl_shared_conversions!(convert SharedTimeSource from TimeSource using SharedTimeSource::new);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt;
    use std::sync::Arc;

    trait Thing: fmt::Debug {}

    #[derive(Debug)]
    struct Thingamajig;
    impl Thing for Thingamajig {}

    #[derive(Debug)]
    struct SharedThing(#[allow(dead_code)] Arc<dyn Thing>);
    impl Thing for SharedThing {}
    impl SharedThing {
        fn new(thing: impl Thing + 'static) -> Self {
            Self(Arc::new(thing))
        }
    }
    impl_shared_conversions!(convert SharedThing from Thing using SharedThing::new);

    #[test]
    fn test() {
        let thing = Thingamajig;
        assert_eq!("Thingamajig", format!("{thing:?}"), "precondition");

        let shared_thing: SharedThing = thing.into_shared();
        assert_eq!(
            "SharedThing(Thingamajig)",
            format!("{shared_thing:?}"),
            "precondition"
        );

        let very_shared_thing: SharedThing = shared_thing.into_shared();
        assert_eq!(
            "SharedThing(Thingamajig)",
            format!("{very_shared_thing:?}"),
            "it should not nest the shared thing in another shared thing"
        );
    }
}
