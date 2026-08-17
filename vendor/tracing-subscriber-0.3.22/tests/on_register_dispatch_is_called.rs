#![cfg(all(feature = "registry", feature = "std"))]
//! Test that `on_register_dispatch` is called on both layers when a layered
//! subscriber is set as the default.

use tracing_mock::layer;
use tracing_subscriber::layer::SubscriberExt;

#[test]
fn on_register_dispatch_is_called() {
    let (inner_layer, inner_handle) = layer::named("inner")
        .on_register_dispatch()
        .run_with_handle();

    let (outer_layer, outer_handle) = layer::named("outer")
        .on_register_dispatch()
        .run_with_handle();

    let subscriber = tracing_subscriber::registry()
        .with(inner_layer)
        .with(outer_layer);

    tracing::subscriber::with_default(subscriber, || {});

    // Verify that on_register_dispatch was called on both layers
    inner_handle.assert_finished();
    outer_handle.assert_finished();
}
