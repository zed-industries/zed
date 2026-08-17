//! Graph container APIs used by `dugong`.
//!
//! Baseline: `@dagrejs/graphlib` (see `docs/adr/0044-dugong-parity-and-testing.md`).
//!
//! This module contains the core `Graph` container plus a small set of helper algorithms
//! re-exported as `dugong_graphlib::alg` for Dagre compatibility.

mod adj_cache;
pub mod alg;
mod core;
mod edge_key;
mod entries;
mod options;

pub use core::{Graph, GraphError};
pub use edge_key::EdgeKey;
pub use options::GraphOptions;
