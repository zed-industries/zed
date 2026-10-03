//! A corgi-compatible `scratch` implementation.
//!
//! Upstream bakes its own OUT_DIR in at compile time (`env!`), making one
//! global directory that every crate's build script writes into --
//! cross-action shared mutable state, which a content-addressed build
//! cannot allow. This patch reads OUT_DIR at *runtime*, so each build
//! script gets a private dir inside its own (published, deterministic)
//! entry. Self-contained cxx crates are unaffected; genuine cross-crate
//! header sharing would fail loudly with a missing include, which is the
//! signal to build the union-of-exports view.

use std::fs;
use std::path::PathBuf;

pub fn path(suffix: &str) -> PathBuf {
    let p = PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join(suffix);
    let _ = fs::create_dir_all(&p);
    p
}
