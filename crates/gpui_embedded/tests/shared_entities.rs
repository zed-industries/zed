//! End-to-end tests for shared entities: a real wasm32-wasip2 guest (see `test_plugin/`)
//! loaded into a wasmtime store, driven from GPUI's deterministic test executor.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use gpui::{AppContext as _, Entity, TestAppContext};
use gpui_embedded::{PluginHost, PluginInstance};
use gpui_embedded_shared::test_schema::{
    Bump, ChameleonSpec, CreateItem, CreateReadonlyItem, FactorySpec, TestCounterSpec,
    TestIncrement,
};
use gpui_embedded_shared::encode;
use rand::prelude::*;

/// Builds the test plugin once per process and returns the component path.
fn test_plugin_path() -> PathBuf {
    use std::sync::Once;
    static BUILD: Once = Once::new();
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let plugin_dir = manifest_dir.join("test_plugin");
    BUILD.call_once(|| {
        // Blocking is fine here: tests build their fixture once, up front.
        #[allow(clippy::disallowed_methods)]
        let output = std::process::Command::new("cargo")
            .args(["build", "--target", "wasm32-wasip2"])
            .current_dir(&plugin_dir)
            .output()
            .expect("failed to spawn cargo to build test_plugin");
        assert!(
            output.status.success(),
            "building test_plugin failed:
{}",
            String::from_utf8_lossy(&output.stderr)
        );
    });
    plugin_dir.join("target/wasm32-wasip2/debug/test_plugin.wasm")
}

fn setup(cx: &mut TestAppContext) -> Entity<PluginHost> {
    let path = test_plugin_path();
    let instance = cx.update(|_| {
        PluginInstance::new(&path, Arc::new(gpui::NoopTextSystem::new()))
            .expect("failed to instantiate test plugin")
    });
    let host = cx.new(|_| PluginHost::new(instance));
    host.update(cx, |host, cx| host.init(cx));
    host
}

/// Flush deferred effects and host-scheduled ticks deterministically.
fn settle(cx: &mut TestAppContext) {
    for _ in 0..5 {
        cx.executor().run_until_parked();
        cx.executor().advance_clock(Duration::from_millis(100));
    }
    cx.executor().run_until_parked();
}

#[gpui::test]
async fn test_send_gives_read_your_writes(cx: &mut TestAppContext) {
    let host = setup(cx);
    let counter = host.update(cx, |host, cx| host.remote::<TestCounterSpec>("guest-counter", cx));

    let receipt = cx.update(|cx| counter.send(TestIncrement { by: 3 }, cx));
    settle(cx);
    receipt.await.expect("send should be acked");

    // Read-your-writes: at receipt resolution the replica already reflects the write.
    let observed = counter
        .replica()
        .read_with(cx, |replica, _| replica.state.as_ref().map(|s| s.count));
    assert_eq!(observed, Some(3));
}

#[gpui::test]
async fn test_call_returns_response_after_snapshot(cx: &mut TestAppContext) {
    let host = setup(cx);
    let counter = host.update(cx, |host, cx| host.remote::<TestCounterSpec>("guest-counter", cx));

    let first = cx.update(|cx| counter.call(TestIncrement { by: 2 }, cx));
    let second = cx.update(|cx| counter.call(TestIncrement { by: 5 }, cx));
    settle(cx);

    // FIFO ordering makes responses deterministic prefix sums.
    assert_eq!(first.await.expect("first call"), 2);
    assert_eq!(second.await.expect("second call"), 7);

    let observed = counter
        .replica()
        .read_with(cx, |replica, _| replica.state.as_ref().map(|s| s.count));
    assert_eq!(observed, Some(7));
}

#[gpui::test]
async fn test_shared_refs_build_object_graphs(cx: &mut TestAppContext) {
    let host = setup(cx);
    let factory = host.update(cx, |host, cx| host.remote::<FactorySpec>("factory", cx));

    // A call whose response is a capability reference to a freshly shared child.
    let created = cx.update(|cx| {
        factory.call(
            CreateItem {
                label: "alpha".to_string(),
            },
            cx,
        )
    });
    settle(cx);
    let item_ref = created.await.expect("create should respond with a ref");

    // Materialize the ref: no names involved, subscribe delivers the initial snapshot.
    let item = host.update(cx, |host, cx| host.remote_from_ref(item_ref, cx));
    settle(cx);
    let snapshot = item
        .replica()
        .read_with(cx, |replica, _| replica.state.clone());
    let snapshot = snapshot.expect("subscribe should deliver a snapshot");
    assert_eq!(snapshot.label, "alpha");
    assert_eq!(snapshot.bumps, 0);

    // The ref is a live capability: calls dispatch to the child entity.
    let bumped = cx.update(|cx| item.call(Bump {}, cx));
    settle(cx);
    assert_eq!(bumped.await.expect("bump"), 1);
    let bumps = item
        .replica()
        .read_with(cx, |replica, _| replica.state.as_ref().map(|s| s.bumps));
    assert_eq!(bumps, Some(1));

    // Distinct creations yield distinct capabilities.
    let created_again = cx.update(|cx| {
        factory.call(
            CreateItem {
                label: "beta".to_string(),
            },
            cx,
        )
    });
    settle(cx);
    let second_ref = created_again.await.expect("second create");
    assert_ne!(second_ref.entity_id(), item_ref.entity_id());
}

#[gpui::test]
async fn test_attenuated_refs_read_but_do_not_write(cx: &mut TestAppContext) {
    let host = setup(cx);
    let factory = host.update(cx, |host, cx| host.remote::<FactorySpec>("factory", cx));

    let created = cx.update(|cx| {
        factory.call(
            CreateReadonlyItem {
                label: "sealed".to_string(),
            },
            cx,
        )
    });
    settle(cx);
    let readonly_ref = created.await.expect("create-readonly");

    let item = host.update(cx, |host, cx| host.remote_from_ref(readonly_ref, cx));
    settle(cx);

    // Reading works: subscription snapshots flow.
    let label = item
        .replica()
        .read_with(cx, |replica, _| replica.state.as_ref().map(|s| s.label.clone()));
    assert_eq!(label.as_deref(), Some("sealed"));

    // Writing fails: this capability was shared with an empty method table.
    let bump = cx.update(|cx| item.call(Bump {}, cx));
    settle(cx);
    let error = bump.await.expect_err("bump must be rejected");
    assert!(
        error.to_string().contains("no handler for shared method"),
        "unexpected error: {error:#}"
    );
}

#[gpui::test]
async fn test_attenuation_composes_from_held_refs(cx: &mut TestAppContext) {
    let host = setup(cx);
    let factory = host.update(cx, |host, cx| host.remote::<FactorySpec>("factory", cx));

    // Start from a FULL capability...
    let created = cx.update(|cx| {
        factory.call(
            CreateItem {
                label: "gamma".to_string(),
            },
            cx,
        )
    });
    settle(cx);
    let full_ref = created.await.expect("create");
    let full = host.update(cx, |host, cx| host.remote_from_ref(full_ref, cx));
    settle(cx);

    // ...and derive a weaker one from it, OCAP-style: no factory method involved, just the
    // generic $attenuate control on a ref the holder already possesses.
    let attenuated = cx.update(|cx| full.attenuate(&[], cx));
    settle(cx);
    let readonly_ref = attenuated.await.expect("attenuate");
    assert_ne!(readonly_ref.entity_id(), full_ref.entity_id());

    let readonly = host.update(cx, |host, cx| host.remote_from_ref(readonly_ref, cx));
    settle(cx);

    // The facet reads the same state...
    let label = readonly
        .replica()
        .read_with(cx, |replica, _| replica.state.as_ref().map(|s| s.label.clone()));
    assert_eq!(label.as_deref(), Some("gamma"));

    // ...but rejects writes...
    let bump = cx.update(|cx| readonly.call(Bump {}, cx));
    settle(cx);
    bump.await.expect_err("attenuated ref must reject writes");

    // ...while the original still writes, and the change fans out to the facet's replica.
    let bump = cx.update(|cx| full.call(Bump {}, cx));
    settle(cx);
    assert_eq!(bump.await.expect("bump via full ref"), 1);
    let facet_view = readonly
        .replica()
        .read_with(cx, |replica, _| replica.state.as_ref().map(|s| s.bumps));
    assert_eq!(facet_view, Some(1), "facet replicas follow the shared state");
}

#[gpui::test]
async fn test_chameleon_handles_methods_dynamically(cx: &mut TestAppContext) {
    let host = setup(cx);
    let chameleon = host.update(cx, |host, cx| host.remote::<ChameleonSpec>("chameleon", cx));

    // Default mode echoes.
    let poke = cx.update(|cx| {
        chameleon.call_raw::<String>("poke", encode(&"hello").unwrap(), cx)
    });
    settle(cx);
    assert_eq!(poke.await.expect("poke"), "hello");

    // The entity reinterprets its own dispatch at runtime.
    let become_shout =
        cx.update(|cx| chameleon.send_raw("become", encode(&"shout").unwrap(), cx));
    settle(cx);
    become_shout.await.expect("become");

    let poke = cx.update(|cx| {
        chameleon.call_raw::<String>("poke", encode(&"hello").unwrap(), cx)
    });
    settle(cx);
    assert_eq!(poke.await.expect("poke"), "HELLO");

    // Unknown methods surface the entity's own error, not a protocol failure.
    let nonsense = cx.update(|cx| {
        chameleon.call_raw::<String>("transmogrify", encode(&"x").unwrap(), cx)
    });
    settle(cx);
    let error = nonsense.await.expect_err("must be rejected");
    assert!(error.to_string().contains("does not understand"));

    // Snapshots observed the dynamic writes: two pokes, shout mode.
    let snapshot = chameleon
        .replica()
        .read_with(cx, |replica, _| replica.state.clone())
        .expect("snapshot");
    assert_eq!(snapshot.pokes, 2);
    assert_eq!(snapshot.mode, "shout");
}

#[gpui::test(iterations = 10)]
async fn test_random_interleavings_stay_consistent(cx: &mut TestAppContext, mut rng: StdRng) {
    let host = setup(cx);
    let counter = host.update(cx, |host, cx| host.remote::<TestCounterSpec>("guest-counter", cx));

    let mut expected_total = 0u32;
    let mut pending_calls = Vec::new();
    let mut pending_sends = Vec::new();

    for _ in 0..rng.random_range(5..25) {
        match rng.random_range(0..3) {
            0 => {
                let by = rng.random_range(1..10);
                expected_total += by;
                let receipt = cx.update(|cx| counter.call(TestIncrement { by }, cx));
                // FIFO + single writer: each response must equal the running prefix sum.
                pending_calls.push((receipt, expected_total));
            }
            1 => {
                let by = rng.random_range(1..10);
                expected_total += by;
                let receipt = cx.update(|cx| counter.send(TestIncrement { by }, cx));
                pending_sends.push((receipt, expected_total));
            }
            _ => settle(cx),
        }
    }
    settle(cx);

    for (receipt, prefix_sum) in pending_calls {
        assert_eq!(receipt.await.expect("call"), prefix_sum);
    }
    for (receipt, prefix_sum) in pending_sends {
        receipt.await.expect("send");
        // At ack time the replica must reflect at least this write.
        let observed = counter
            .replica()
            .read_with(cx, |replica, _| replica.state.as_ref().map_or(0, |s| s.count));
        assert!(observed >= prefix_sum, "replica {observed} < acked write {prefix_sum}");
    }

    let final_count = counter
        .replica()
        .read_with(cx, |replica, _| replica.state.as_ref().map(|s| s.count));
    assert_eq!(final_count, Some(expected_total));
}
