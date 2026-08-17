<div align="center">
  <h1><code>rustix-linux-procfs</code></h1>

  <p>
    <strong>Utilities for opening Linux procfs files and directories</strong>
  </p>

  <p>
    <a href="https://github.com/sunfishcode/rustix-linux-procfs/actions?query=workflow%3ACI"><img src="https://github.com/sunfishcode/rustix-linux-procfs/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://bytecodealliance.zulipchat.com/#narrow/stream/206238-general"><img src="https://img.shields.io/badge/zulip-join_chat-brightgreen.svg" alt="zulip chat" /></a>
    <a href="https://crates.io/crates/rustix-linux-procfs"><img src="https://img.shields.io/crates/v/rustix-linux-procfs.svg" alt="crates.io page" /></a>
    <a href="https://docs.rs/rustix-linux-procfs"><img src="https://docs.rs/rustix-linux-procfs/badge.svg" alt="docs.rs docs" /></a>
  </p>
</div>

This crate contains functions for obtaining file descriptors for files and
directories under "/proc" on Linux.

This crate does a considerable amount of work to determine whether `/proc` is
mounted, with actual `procfs`, and without any additional mount points on top
of the paths we open.

Why all the effort to detect bind mount points? People are doing all kinds of
things with Linux containers these days, with many different privilege schemes,
and we want to avoid making any unnecessary assumptions. Libraries
will sometimes use procfs *implicitly* (when Linux gives them no better
options), in ways that aren't obvious from their public APIs. These filesystem
accesses might not be visible to someone auditing the main code of an
application for places which may be influenced by the filesystem namespace. So
with the checking here, they may fail, but they won't be able to succeed with
bogus results.

As a caveat, QEMU intercepts selected paths in `open` to emulate their
contents, however this crate's extra checks bypass QEMU's interceptions, so
using this crate instead of just opening the paths directly can cause problems
when running under QEMU.

As a historical note, the functions used to be part of the [`rustix`] crate,
but were factored out into a separate crate to simplify the `rustix` crate, as
they can be implemented on top of rustix's public API.

[`rustix`]: https://crates.io/crates/rustix
