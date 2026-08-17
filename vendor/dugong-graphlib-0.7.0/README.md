# dugong-graphlib

[![Crates.io](https://img.shields.io/crates/v/dugong-graphlib.svg)](https://crates.io/crates/dugong-graphlib)
[![Documentation](https://docs.rs/dugong-graphlib/badge.svg)](https://docs.rs/dugong-graphlib)
[![Crates.io Downloads](https://img.shields.io/crates/d/dugong-graphlib.svg)](https://crates.io/crates/dugong-graphlib)
[![Made with Rust](https://img.shields.io/badge/made%20with-Rust-orange.svg)](https://www.rust-lang.org)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Graph container APIs used by `dugong` (port of `@dagrejs/graphlib`).

This crate is intentionally small and focused:

- directed/undirected graph storage
- compound graphs (parent/child)
- multigraph edge keys
- helper algorithms (`dugong_graphlib::alg`)

Baseline revisions are tracked in `tools/upstreams/REPOS.lock.json` in the repository.
