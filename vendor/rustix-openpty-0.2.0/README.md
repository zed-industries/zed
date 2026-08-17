<div align="center">
  <h1><code>rustix-openpty</code></h1>

  <p>
    <strong>Safe Rust bindings to `openpty` and related functions</strong>
  </p>

  <p>
    <a href="https://github.com/sunfishcode/rustix-openpty/actions?query=workflow%3ACI"><img src="https://github.com/sunfishcode/rustix-openpty/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://bytecodealliance.zulipchat.com/#narrow/stream/206238-general"><img src="https://img.shields.io/badge/zulip-join_chat-brightgreen.svg" alt="zulip chat" /></a>
    <a href="https://crates.io/crates/rustix-openpty"><img src="https://img.shields.io/crates/v/rustix-openpty.svg" alt="crates.io page" /></a>
    <a href="https://docs.rs/rustix-openpty"><img src="https://docs.rs/rustix-openpty/badge.svg" alt="docs.rs docs" /></a>
  </p>
</div>

rustix-openpty is a wrapper around [`rustix::pty`] on Linux and
[`libc::openpty`] on other platforms.

## Minimum Supported Rust Version (MSRV)

This crate currently works on the version of [Rust on Debian stable], which is
currently Rust 1.63. This policy may change in the future, in minor version
releases, so users using a fixed version of Rust should pin to a specific
version of this crate.

[Rust on Debian stable]: https://packages.debian.org/stable/rust/rustc
[`rustix::pty`]: https://docs.rs/rustix/latest/rustix/pty
[`libc::openpty`]: https://docs.rs/libc/latest/libc/fn.openpty.html
