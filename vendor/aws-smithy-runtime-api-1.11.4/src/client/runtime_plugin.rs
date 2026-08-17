/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Runtime plugin type definitions.
//!
//! Runtime plugins are used to extend the runtime with custom behavior.
//! This can include:
//! - Registering interceptors
//! - Registering auth schemes
//! - Adding entries to the [`ConfigBag`] for orchestration
//! - Setting runtime components
//!
//! Runtime plugins are divided into service/operation "levels", with service runtime plugins
//! executing before operation runtime plugins. Runtime plugins configured in a service
//! config will always be at the service level, while runtime plugins added during
//! operation customization will be at the operation level. Custom runtime plugins will
//! always run after the default runtime plugins within their level.

use crate::box_error::BoxError;
use crate::client::runtime_components::{
    RuntimeComponentsBuilder, EMPTY_RUNTIME_COMPONENTS_BUILDER,
};
use crate::impl_shared_conversions;
use crate::shared::IntoShared;
use aws_smithy_types::config_bag::{ConfigBag, FrozenLayer};
use std::borrow::Cow;
use std::fmt::Debug;
use std::sync::Arc;

const DEFAULT_ORDER: Order = Order::Overrides;

/// Runtime plugin ordering.
///
/// There are two runtime plugin "levels" that run in the following order:
/// 1. Service runtime plugins - runtime plugins that pertain to the entire service.
/// 2. Operation runtime plugins - runtime plugins relevant only to a single operation.
///
/// This enum is used to determine runtime plugin order within those levels.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Order {
    /// Runtime plugins with `Defaults` order are executed first within their level.
    ///
    /// Runtime plugins with this order should only be used for registering default components and config.
    Defaults,

    /// Runtime plugins with `Overrides` order are executed after `Defaults` within their level.
    ///
    /// This is the default order.
    Overrides,

    /// Runtime plugins with `NestedComponents` order are executed after `Overrides` within their level.
    ///
    /// This level is intended to be used for wrapping components configured in previous runtime plugins.
    NestedComponents,
}

/// Runtime plugin trait
///
/// A `RuntimePlugin` is the unit of configuration for augmenting the client with new behavior.
///
/// Runtime plugins can register interceptors, set runtime components, and modify configuration.
pub trait RuntimePlugin: Debug + Send + Sync {
    /// Runtime plugin ordering.
    ///
    /// There are two runtime plugin "levels" that run in the following order:
    /// 1. Service runtime plugins - runtime plugins that pertain to the entire service.
    /// 2. Operation runtime plugins - runtime plugins relevant only to a single operation.
    ///
    /// This function is used to determine runtime plugin order within those levels. So
    /// regardless of this `Order` value, service runtime plugins will still always execute before
    /// operation runtime plugins. However, [`Defaults`](Order::Defaults)
    /// service runtime plugins will run before [`Overrides`](Order::Overrides)
    /// service runtime plugins.
    fn order(&self) -> Order {
        DEFAULT_ORDER
    }

    /// Optionally returns additional config that should be added to the [`ConfigBag`].
    ///
    /// As a best practice, a frozen layer should be stored on the runtime plugin instance as
    /// a member, and then cloned upon return since that clone is cheap. Constructing a new
    /// [`Layer`](aws_smithy_types::config_bag::Layer) and freezing it will require a lot of allocations.
    fn config(&self) -> Option<FrozenLayer> {
        None
    }

    /// Returns a [`RuntimeComponentsBuilder`] to incorporate into the final runtime components.
    ///
    /// The order of runtime plugins determines which runtime components "win". Components set by later runtime plugins will
    /// override those set by earlier runtime plugins.
    ///
    /// If no runtime component changes are desired, just return an empty builder.
    ///
    /// This method returns a [`Cow`] for flexibility. Some implementers may want to store the components builder
    /// as a member and return a reference to it, while others may need to create the builder every call. If possible,
    /// returning a reference is preferred for performance.
    ///
    /// Components configured by previous runtime plugins are in the `current_components` argument, and can be used
    /// to create nested/wrapped components, such as a connector calling into an inner (customer provided) connector.
    fn runtime_components(
        &self,
        current_components: &RuntimeComponentsBuilder,
    ) -> Cow<'_, RuntimeComponentsBuilder> {
        let _ = current_components;
        Cow::Borrowed(&EMPTY_RUNTIME_COMPONENTS_BUILDER)
    }
}

/// Shared runtime plugin
///
/// Allows for multiple places to share ownership of one runtime plugin.
#[derive(Debug, Clone)]
pub struct SharedRuntimePlugin(Arc<dyn RuntimePlugin>);

impl SharedRuntimePlugin {
    /// Creates a new [`SharedRuntimePlugin`].
    pub fn new(plugin: impl RuntimePlugin + 'static) -> Self {
        Self(Arc::new(plugin))
    }
}

impl RuntimePlugin for SharedRuntimePlugin {
    fn order(&self) -> Order {
        self.0.order()
    }

    fn config(&self) -> Option<FrozenLayer> {
        self.0.config()
    }

    fn runtime_components(
        &self,
        current_components: &RuntimeComponentsBuilder,
    ) -> Cow<'_, RuntimeComponentsBuilder> {
        self.0.runtime_components(current_components)
    }
}

impl_shared_conversions!(convert SharedRuntimePlugin from RuntimePlugin using SharedRuntimePlugin::new);

/// Runtime plugin that simply returns the config and components given at construction time.
#[derive(Default, Debug)]
pub struct StaticRuntimePlugin {
    config: Option<FrozenLayer>,
    runtime_components: Option<RuntimeComponentsBuilder>,
    order: Option<Order>,
}

impl StaticRuntimePlugin {
    /// Returns a new [`StaticRuntimePlugin`].
    pub fn new() -> Self {
        Default::default()
    }

    /// Changes the config.
    pub fn with_config(mut self, config: FrozenLayer) -> Self {
        self.config = Some(config);
        self
    }

    /// Changes the runtime components.
    pub fn with_runtime_components(mut self, runtime_components: RuntimeComponentsBuilder) -> Self {
        self.runtime_components = Some(runtime_components);
        self
    }

    /// Changes the order of this runtime plugin.
    pub fn with_order(mut self, order: Order) -> Self {
        self.order = Some(order);
        self
    }
}

impl RuntimePlugin for StaticRuntimePlugin {
    fn order(&self) -> Order {
        self.order.unwrap_or(DEFAULT_ORDER)
    }

    fn config(&self) -> Option<FrozenLayer> {
        self.config.clone()
    }

    fn runtime_components(
        &self,
        _current_components: &RuntimeComponentsBuilder,
    ) -> Cow<'_, RuntimeComponentsBuilder> {
        self.runtime_components
            .as_ref()
            .map(Cow::Borrowed)
            .unwrap_or_else(|| Cow::Borrowed(&EMPTY_RUNTIME_COMPONENTS_BUILDER))
    }
}

macro_rules! insert_plugin {
    ($vec:expr, $plugin:expr) => {{
        // Insert the plugin in the correct order
        let plugin = $plugin;
        let mut insert_index = 0;
        let order = plugin.order();
        for (index, other_plugin) in $vec.iter().enumerate() {
            let other_order = other_plugin.order();
            if other_order <= order {
                insert_index = index + 1;
            } else if other_order > order {
                break;
            }
        }
        $vec.insert(insert_index, plugin);
    }};
}

macro_rules! apply_plugins {
    ($name:ident, $plugins:expr, $cfg:ident) => {{
        tracing::trace!(concat!("applying ", stringify!($name), " runtime plugins"));
        let mut merged =
            RuntimeComponentsBuilder::new(concat!("apply_", stringify!($name), "_configuration"));
        for plugin in &$plugins {
            if let Some(layer) = plugin.config() {
                $cfg.push_shared_layer(layer);
            }
            let next = plugin.runtime_components(&merged);
            merged = merged.merge_from(&next);
        }
        Ok(merged)
    }};
}

/// Used internally in the orchestrator implementation and in the generated code. Not intended to be used elsewhere.
#[derive(Default, Clone, Debug)]
pub struct RuntimePlugins {
    client_plugins: Vec<SharedRuntimePlugin>,
    operation_plugins: Vec<SharedRuntimePlugin>,
}

impl RuntimePlugins {
    /// Create a new empty set of runtime plugins.
    pub fn new() -> Self {
        Default::default()
    }

    /// Add several client-level runtime plugins from an iterator.
    pub fn with_client_plugins(
        mut self,
        plugins: impl IntoIterator<Item = SharedRuntimePlugin>,
    ) -> Self {
        for plugin in plugins.into_iter() {
            self = self.with_client_plugin(plugin);
        }
        self
    }

    /// Adds a client-level runtime plugin.
    pub fn with_client_plugin(mut self, plugin: impl RuntimePlugin + 'static) -> Self {
        insert_plugin!(
            self.client_plugins,
            IntoShared::<SharedRuntimePlugin>::into_shared(plugin)
        );
        self
    }

    /// Add several operation-level runtime plugins from an iterator.
    pub fn with_operation_plugins(
        mut self,
        plugins: impl IntoIterator<Item = SharedRuntimePlugin>,
    ) -> Self {
        for plugin in plugins.into_iter() {
            self = self.with_operation_plugin(plugin);
        }
        self
    }

    /// Adds an operation-level runtime plugin.
    pub fn with_operation_plugin(mut self, plugin: impl RuntimePlugin + 'static) -> Self {
        insert_plugin!(
            self.operation_plugins,
            IntoShared::<SharedRuntimePlugin>::into_shared(plugin)
        );
        self
    }

    /// Apply the client-level runtime plugins' config to the given config bag.
    pub fn apply_client_configuration(
        &self,
        cfg: &mut ConfigBag,
    ) -> Result<RuntimeComponentsBuilder, BoxError> {
        apply_plugins!(client, self.client_plugins, cfg)
    }

    /// Apply the operation-level runtime plugins' config to the given config bag.
    pub fn apply_operation_configuration(
        &self,
        cfg: &mut ConfigBag,
    ) -> Result<RuntimeComponentsBuilder, BoxError> {
        apply_plugins!(operation, self.operation_plugins, cfg)
    }
}

#[cfg(all(test, feature = "test-util", feature = "http-02x"))]
mod tests {
    use super::{RuntimePlugin, RuntimePlugins};
    use crate::client::http::{
        http_client_fn, HttpClient, HttpConnector, HttpConnectorFuture, SharedHttpConnector,
    };
    use crate::client::orchestrator::HttpRequest;
    use crate::client::runtime_components::RuntimeComponentsBuilder;
    use crate::client::runtime_plugin::{Order, SharedRuntimePlugin};
    use crate::shared::IntoShared;
    use aws_smithy_types::body::SdkBody;
    use aws_smithy_types::config_bag::ConfigBag;
    use http_02x::HeaderValue;
    use std::borrow::Cow;

    #[derive(Debug)]
    struct SomeStruct;

    impl RuntimePlugin for SomeStruct {}

    #[test]
    fn can_add_runtime_plugin_implementors_to_runtime_plugins() {
        RuntimePlugins::new().with_client_plugin(SomeStruct);
    }

    #[test]
    fn runtime_plugins_are_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<RuntimePlugins>();
    }

    #[test]
    fn insert_plugin() {
        #[derive(Debug)]
        struct RP(isize, Order);
        impl RuntimePlugin for RP {
            fn order(&self) -> Order {
                self.1
            }
        }

        fn insert_plugin(vec: &mut Vec<RP>, plugin: RP) {
            insert_plugin!(vec, plugin);
        }

        let mut vec = Vec::new();
        insert_plugin(&mut vec, RP(5, Order::NestedComponents));
        insert_plugin(&mut vec, RP(3, Order::Overrides));
        insert_plugin(&mut vec, RP(1, Order::Defaults));
        insert_plugin(&mut vec, RP(6, Order::NestedComponents));
        insert_plugin(&mut vec, RP(2, Order::Defaults));
        insert_plugin(&mut vec, RP(4, Order::Overrides));
        insert_plugin(&mut vec, RP(7, Order::NestedComponents));
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6, 7],
            vec.iter().map(|rp| rp.0).collect::<Vec<isize>>()
        );

        let mut vec = Vec::new();
        insert_plugin(&mut vec, RP(3, Order::Overrides));
        insert_plugin(&mut vec, RP(4, Order::Overrides));
        insert_plugin(&mut vec, RP(5, Order::NestedComponents));
        insert_plugin(&mut vec, RP(6, Order::NestedComponents));
        insert_plugin(&mut vec, RP(7, Order::NestedComponents));
        insert_plugin(&mut vec, RP(1, Order::Defaults));
        insert_plugin(&mut vec, RP(2, Order::Defaults));
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6, 7],
            vec.iter().map(|rp| rp.0).collect::<Vec<isize>>()
        );

        let mut vec = Vec::new();
        insert_plugin(&mut vec, RP(1, Order::Defaults));
        insert_plugin(&mut vec, RP(2, Order::Defaults));
        insert_plugin(&mut vec, RP(3, Order::Overrides));
        insert_plugin(&mut vec, RP(4, Order::Overrides));
        insert_plugin(&mut vec, RP(5, Order::NestedComponents));
        insert_plugin(&mut vec, RP(6, Order::NestedComponents));
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6],
            vec.iter().map(|rp| rp.0).collect::<Vec<isize>>()
        );
    }

    #[tokio::test]
    async fn components_can_wrap_components() {
        // Connector1, the inner connector, creates a response with a `rp1` header
        #[derive(Debug)]
        struct Connector1;
        impl HttpConnector for Connector1 {
            fn call(&self, _: HttpRequest) -> HttpConnectorFuture {
                HttpConnectorFuture::new(async {
                    Ok(http_02x::Response::builder()
                        .status(200)
                        .header("rp1", "1")
                        .body(SdkBody::empty())
                        .unwrap()
                        .try_into()
                        .unwrap())
                })
            }
        }

        // Connector2, the outer connector, calls the inner connector and adds the `rp2` header to the response
        #[derive(Debug)]
        struct Connector2(SharedHttpConnector);
        impl HttpConnector for Connector2 {
            fn call(&self, request: HttpRequest) -> HttpConnectorFuture {
                let inner = self.0.clone();
                HttpConnectorFuture::new(async move {
                    let mut resp = inner.call(request).await.unwrap();
                    resp.headers_mut()
                        .append("rp2", HeaderValue::from_static("1"));
                    Ok(resp)
                })
            }
        }

        // Plugin1 registers Connector1
        #[derive(Debug)]
        struct Plugin1;
        impl RuntimePlugin for Plugin1 {
            fn order(&self) -> Order {
                Order::Overrides
            }

            fn runtime_components(
                &self,
                _: &RuntimeComponentsBuilder,
            ) -> Cow<'_, RuntimeComponentsBuilder> {
                Cow::Owned(
                    RuntimeComponentsBuilder::new("Plugin1")
                        .with_http_client(Some(http_client_fn(|_, _| Connector1.into_shared()))),
                )
            }
        }

        // Plugin2 registers Connector2
        #[derive(Debug)]
        struct Plugin2;
        impl RuntimePlugin for Plugin2 {
            fn order(&self) -> Order {
                Order::NestedComponents
            }

            fn runtime_components(
                &self,
                current_components: &RuntimeComponentsBuilder,
            ) -> Cow<'_, RuntimeComponentsBuilder> {
                let current = current_components.http_client().unwrap();
                Cow::Owned(
                    RuntimeComponentsBuilder::new("Plugin2").with_http_client(Some(
                        http_client_fn(move |settings, components| {
                            let connector = current.http_connector(settings, components);
                            SharedHttpConnector::new(Connector2(connector))
                        }),
                    )),
                )
            }
        }

        // Emulate assembling a full runtime plugins list and using it to apply configuration
        let plugins = RuntimePlugins::new()
            // intentionally configure the plugins in the reverse order
            .with_client_plugin(Plugin2)
            .with_client_plugin(Plugin1);
        let mut cfg = ConfigBag::base();
        let components = plugins.apply_client_configuration(&mut cfg).unwrap();
        let fake_components = RuntimeComponentsBuilder::for_tests().build().unwrap();

        // Use the resulting HTTP connector to make a response
        let resp = components
            .http_client()
            .unwrap()
            .http_connector(&Default::default(), &fake_components)
            .call(HttpRequest::empty())
            .await
            .unwrap();
        dbg!(&resp);

        // Verify headers from both connectors are present,
        // which will only be possible if they were run in the correct order
        assert_eq!("1", resp.headers().get("rp1").unwrap());
        assert_eq!("1", resp.headers().get("rp2").unwrap());
    }

    #[test]
    fn shared_runtime_plugin_new_specialization() {
        #[derive(Debug)]
        struct RP;
        impl RuntimePlugin for RP {}

        use crate::shared::IntoShared;
        let shared1 = SharedRuntimePlugin::new(RP);
        let shared2: SharedRuntimePlugin = shared1.clone().into_shared();
        assert_eq!(
            "SharedRuntimePlugin(RP)",
            format!("{shared1:?}"),
            "precondition: RP shows up in the debug format"
        );
        assert_eq!(
            format!("{shared1:?}"),
            format!("{shared2:?}"),
            "it should not nest the shared runtime plugins"
        );
    }
}
