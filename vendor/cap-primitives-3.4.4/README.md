<div align="center">
  <h1><code>cap-primitives</code></h1>

  <p>
    <strong>Capability-based primitives</strong>
  </p>

  <p>
    <a href="https://github.com/bytecodealliance/cap-std/actions?query=workflow%3ACI"><img src="https://github.com/bytecodealliance/cap-std/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://crates.io/crates/cap-primitives"><img src="https://img.shields.io/crates/v/cap-primitives.svg" alt="crates.io page" /></a>
    <a href="https://docs.rs/cap-primitives"><img src="https://docs.rs/cap-primitives/badge.svg" alt="docs.rs docs" /></a>
  </p>
</div>

The `cap-primitives` crate provides primitive sandboxing operations that
[`cap-std`] and [`cap-async-std`] are built on.

The filesystem module [`cap_primitives::fs`], the networking module
[`cap_primitives::net`], and time module [`cap_primitives::time`] currently
support Linux, macOS, FreeBSD, and Windows. WASI support is in development,
though not yet usable.

[`cap-std`]: https://github.com/bytecodealliance/cap-std/blob/main/cap-std/README.md
[`cap-async-std`]: https://github.com/bytecodealliance/cap-std/blob/main/cap-async-std/README.md
[`cap_primitives::fs`]: https://docs.rs/cap-primitives/latest/cap_primitives/fs/index.html
[`cap_primitives::net`]: https://docs.rs/cap-primitives/latest/cap_primitives/net/index.html
[`cap_primitives::time`]: https://docs.rs/cap-primitives/latest/cap_primitives/time/index.html
