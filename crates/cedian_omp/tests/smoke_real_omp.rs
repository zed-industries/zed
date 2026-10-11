//! Phase 1 smoke: `OmpRuntime` against the real `omp --mode rpc-ui`.
//!
//!mirrors the Phase 0.5 spike acceptance (`spike-pong`, host-tool + host-URI
//! roundtrips, abort) through the production `OmpRuntime` surface instead of
//! the throwaway decoder.
//!
//! Requires ambient OMP auth (same as the spike). Ignored by default:
//! `cargo test -p cedian_omp -- --ignored --nocapture smoke_against_real_omp`

use cedian_omp::{OmpBinary, OmpRuntime, RuntimeConfig, RuntimeState, image_content, last_text};
use omp_rpc::{HostTool, HostUri};
use serde_json::json;
use std::time::Duration;

fn dev_config(tag: &str) -> RuntimeConfig {
    RuntimeConfig {
        binary: OmpBinary::Path("omp".to_string()),
        session_dir: std::env::temp_dir()
            .join(format!("cedian-phase1-{tag}-{}", std::process::id())),
        sessions: cedian_omp::Sessions::InSessionDir,
        cwd: std::env::temp_dir(),
        ask_dialog: true,
        prompt_timeout: Duration::from_secs(240),
        policy: cedian_omp::SpawnPolicy {
            host_tools: ["echo_host".to_string()].into(),
            ..Default::default()
        },
    }
}

#[test]
#[ignore]
fn smoke_against_real_omp() {
    let mut rt = OmpRuntime::spawn(dev_config("smoke")).expect("spawn");
    assert_eq!(rt.state(), RuntimeState::Ready);

    // Host tool + host URI registered BEFORE the prompts that use them.
    let params: serde_json::Map<String, serde_json::Value> =
        json!({"type":"object","properties":{"message":{"type":"string"}},"required":["message"],"additionalProperties":false})
            .as_object()
            .unwrap()
            .clone();
    rt.set_host_tools(vec![HostTool::new(
        "echo_host",
        "Echo a value back from the host.",
        params,
        |args, _ctx| {
            let msg = args
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            Ok(format!("ECHO:{msg}").into())
        },
    )])
    .expect("register host tool");
    rt.set_host_uris(vec![
        HostUri::new("smoke", |url, _ctx| Ok(format!("URI-ECHO:{url}").into()))
            .expect("uri scheme"),
    ])
    .expect("register host uri");

    // P1: plain prompt → stream → completed, verbatim text.
    let turn = rt
        .prompt(
            "Reply with exactly this word and nothing else: phase1-pong",
            vec![],
        )
        .expect("p1");
    assert_eq!(last_text(&turn), Some("phase1-pong"));
    assert!(
        turn.result
            .as_ref()
            .map(|r| r.session_settled)
            .unwrap_or(false)
    );

    // P2: host-tool roundtrip served by the vendored client inline.
    let turn = rt
        .prompt(
            "Call the echo_host tool with message 'rt-ping' and reply with only the tool result text.",
            vec![],
        )
        .expect("p2");
    assert_eq!(last_text(&turn), Some("ECHO:rt-ping"));

    // P3: host-URI roundtrip through `read smoke://...`.
    let turn = rt
        .prompt("Read the file at smoke://notes/7 with the read tool and reply with only its exact content.", vec![])
        .expect("p3");
    assert_eq!(last_text(&turn), Some("URI-ECHO:smoke://notes/7"));

    // P4: image prompt completes.
    let px = [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
    let turn = rt
        .prompt(
            "Reply with exactly: img1-ok",
            vec![image_content("image/png", &px)],
        )
        .expect("p4");
    assert!(turn.result.is_some());

    // Router saw the session activity (agent starts, prompt results, settle).
    let log = rt.event_log();
    assert!(log.len() >= 4, "router log has {} entries", log.len());

    rt.shutdown().expect("shutdown");
}

/// The pin's `read` over a host URI comes back verbatim, with no `1:` line
/// numbering (the 18.7.0 regression in ROADMAP's Follow-ups, due at the
/// ADR-0059 pin bump). One model call, no host tool.
/// `cargo test -p cedian_omp -- --ignored read_`
#[test]
#[ignore]
fn read_of_a_host_uri_is_verbatim() {
    let mut rt = OmpRuntime::spawn(dev_config("host-uri")).expect("spawn");
    assert_eq!(rt.state(), RuntimeState::Ready);
    rt.set_host_uris(vec![
        HostUri::new("smoke", |url, _ctx| Ok(format!("URI-ECHO:{url}").into()))
            .expect("uri scheme"),
    ])
    .expect("register host uri");
    let turn = rt
        .prompt("Read the file at smoke://notes/7 with the read tool and reply with only its exact content.", vec![])
        .expect("read the host uri");
    let text = last_text(&turn).unwrap_or_default();
    assert!(
        !text.contains("1:"),
        "the read of a host URI was numbered: {text:?}"
    );
    assert!(text.contains("URI-ECHO:smoke://notes/7"), "{text:?}");
    rt.shutdown().expect("shutdown");
}
