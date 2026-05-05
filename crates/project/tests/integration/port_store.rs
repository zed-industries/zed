//! Layer 1 property tests for the `PortResourceSet` pure reducer.
//!
//! These tests have no GPUI dependency — they exercise the state machine
//! directly with deterministic and property-based inputs.

use project::port_store::{ApplyError, PortResource, PortResourceSet};
use proptest::prelude::*;
use std::{collections::HashSet, sync::Arc};

// ─── Strategies ────────────────────────────────────────────────────────────────

/// proptest strategy that produces an arbitrary `PortResource`.
///
/// Defined as a function rather than an `Arbitrary` impl to avoid the orphan
/// rule (neither `Arbitrary` nor `PortResource` are local to this crate).
fn arb_resource() -> impl Strategy<Value = PortResource> {
    (
        any::<String>(),
        any::<u64>(),
        proptest::bool::ANY,
        any::<u32>(),
        any::<u32>(),
        any::<u64>(),
    )
        .prop_map(|(id_suffix, version, is_v6, port, uid, inode)| {
            let proto_str = if is_v6 { "tcp6" } else { "tcp4" };
            let addr = if is_v6 { "::1" } else { "127.0.0.1" };
            let id = format!("{proto_str}:{addr}:{port}");
            PortResource {
                id: id.into(),
                version,
                proto: proto_str.into(),
                bind_addr: format!("{addr}:{id_suffix}").into(),
                port,
                uid,
                inode,
                process: "".into(),
                exposure: "loopback".into(),
            }
        })
}

// ─── Helpers ───────────────────────────────────────────────────────────────────

fn resource(id: &str, version: u64) -> PortResource {
    PortResource {
        id: id.into(),
        version,
        proto: "tcp4".into(),
        bind_addr: "127.0.0.1".into(),
        port: 3000,
        uid: 0,
        inode: 0,
        process: "".into(),
        exposure: "loopback".into(),
    }
}

// ─── Deterministic tests ───────────────────────────────────────────────────────

#[test]
fn apply_initial_sets_exactly_given_resources() {
    let mut set = PortResourceSet::default();
    let resources = vec![resource("tcp4:127.0.0.1:3000", 1)];
    set.apply_initial(resources, 1).unwrap();
    assert_eq!(set.resources().len(), 1);
    assert!(set.resources().contains_key("tcp4:127.0.0.1:3000" as &str));
    assert_eq!(set.version(), 1);
}

#[test]
fn apply_initial_nothing_extra() {
    let mut set = PortResourceSet::default();
    set.apply_initial(vec![resource("tcp4:127.0.0.1:3000", 1)], 1)
        .unwrap();
    assert_eq!(set.resources().len(), 1);
}

#[test]
fn apply_delta_upsert_and_remove() {
    let mut set = PortResourceSet::default();
    set.apply_initial(vec![resource("tcp4:0.0.0.0:8080", 1)], 1)
        .unwrap();
    set.apply_delta(
        vec![resource("tcp4:0.0.0.0:9000", 2)],
        vec![Arc::from("tcp4:0.0.0.0:8080")],
        2,
    )
    .unwrap();
    assert!(!set.resources().contains_key("tcp4:0.0.0.0:8080" as &str));
    assert!(set.resources().contains_key("tcp4:0.0.0.0:9000" as &str));
}

#[test]
fn out_of_order_delta_rejected() {
    let mut set = PortResourceSet::default();
    set.apply_initial(vec![], 5).unwrap();
    let err = set.apply_delta(vec![], vec![], 3).unwrap_err();
    assert!(matches!(err, ApplyError::OutOfOrder { .. }));
    assert_eq!(set.version(), 5);
}

#[test]
fn duplicate_version_rejected() {
    let mut set = PortResourceSet::default();
    set.apply_initial(vec![], 5).unwrap();
    let err = set.apply_delta(vec![], vec![], 5).unwrap_err();
    assert!(matches!(err, ApplyError::Duplicate(5)));
    assert_eq!(set.version(), 5);
}

#[test]
fn apply_bookmark_advances_version_without_changing_resources() {
    let mut set = PortResourceSet::default();
    set.apply_initial(vec![resource("tcp4:127.0.0.1:1234", 1)], 1)
        .unwrap();
    set.apply_bookmark(10).unwrap();
    assert_eq!(set.resources().len(), 1);
    assert_eq!(set.version(), 10);
}

#[test]
fn apply_resync_clears_all_state() {
    let mut set = PortResourceSet::default();
    set.apply_initial(vec![resource("tcp4:127.0.0.1:1234", 5)], 5)
        .unwrap();
    set.apply_resync_required();
    assert!(set.resources().is_empty());
    assert_eq!(set.version(), 0);
}

#[test]
fn duplicate_initial_after_populate_rejected() {
    let mut set = PortResourceSet::default();
    set.apply_initial(vec![resource("tcp4:127.0.0.1:1", 3)], 3)
        .unwrap();
    let err = set
        .apply_initial(vec![resource("tcp4:127.0.0.1:2", 3)], 3)
        .unwrap_err();
    assert!(matches!(err, ApplyError::Duplicate(3)));
}

// ─── Property tests ────────────────────────────────────────────────────────────

proptest! {
    #[test]
    fn proptest_initial_exactly_given_resources(
        resources in proptest::collection::vec(arb_resource(), 0..20),
        version in 1u64..1000,
    ) {
        let mut set = PortResourceSet::default();
        let expected_count = resources
            .iter()
            .map(|r| r.id.clone())
            .collect::<HashSet<_>>()
            .len();
        set.apply_initial(resources, version).unwrap();
        prop_assert_eq!(set.resources().len(), expected_count);
        prop_assert_eq!(set.version(), version);
    }

    #[test]
    fn proptest_versions_monotonic(
        v1 in 1u64..500,
        v2 in 501u64..1000,
    ) {
        let mut set = PortResourceSet::default();
        set.apply_initial(vec![], v1).unwrap();
        set.apply_bookmark(v2).unwrap();
        prop_assert_eq!(set.version(), v2);
    }

    #[test]
    fn proptest_out_of_order_rejected(
        v_high in 10u64..1000,
        v_low in 1u64..10,
    ) {
        let mut set = PortResourceSet::default();
        set.apply_initial(vec![], v_high).unwrap();
        let result = set.apply_delta(vec![], vec![], v_low);
        prop_assert!(result.is_err());
        prop_assert_eq!(set.version(), v_high);
    }
}
