<div align="center">
  <h1><code>system-interface</code></h1>

  <p>
    <strong>Extensions to the Rust standard library</strong>
  </p>

  <strong>A <a href="https://bytecodealliance.org/">Bytecode Alliance</a> project</strong>

  <p>
    <a href="https://github.com/bytecodealliance/system-interface/actions?query=workflow%3ACI"><img src="https://github.com/bytecodealliance/system-interface/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://bytecodealliance.zulipchat.com/#narrow/stream/217126-wasmtime"><img src="https://img.shields.io/badge/zulip-join_chat-brightgreen.svg" alt="zulip chat" /></a>
    <a href="https://crates.io/crates/system-interface"><img src="https://img.shields.io/crates/v/system-interface.svg" alt="crates.io page" /></a>
    <a href="https://docs.rs/system-interface"><img src="https://docs.rs/system-interface/badge.svg" alt="docs.rs docs" /></a>
  </p>
</div>

`system-interface` adds extensions to the Rust standard library, seeking to
stay within the style of [`std`], while exposing additional functionality:

  - [`fs::FileIoExt`] - Extra support for working with files, including
    all the features of [`std::io::Read`], [`std::io::Write`],
    [`std::io::Seek`], and [`std::os::unix::fs::FileExt`], but with both
    POSIX-ish and Windows support, and with additional features, including
    `read` and `write` with all combinations of `_vectored`, `_at`, and
    `_exact`/`_all`. If you've ever wanted something like
    [`read_exact_vectored_at`], [`write_all_vectored_at`], or any other
    combination, or even [`read_to_end_at`] or [`read_to_string_at`],
    they're all here, *and* they work on Windows too!
  - [`io::IsTerminal`] - Test whether a given I/O handle refers to a terminal
    (aka a tty).
  - [`io::ReadReady`] - Query the number of bytes ready to be read immediately
    from an I/O handle.
  - [`io::Peek`] - Read from an I/O handle without consuming the data.

Everything in this crate is portable across popular POSIX-ish platforms and
Windows.

Many of `system-interface`'s features correspond to features in [WASI], and are
designed to work with [`cap-std`], however it's not specific to WASI and can be
used with regular [`std`] too. To separate concerns, all sandboxing and
capability-oriented APIs are left to `cap-std`, so this crate's features are
usable independently.

Support for async-std and socket2 is temporarily disabled until those crates
contain the needed implementations of the I/O safety traits.

[`std`]: https://doc.rust-lang.org/std/
[`cap-std`]: https://crates.io/crates/cap-std
[WASI]: https://github.com/WebAssembly/WASI/
[`fs::FileIoExt`]: https://docs.rs/system-interface/latest/system_interface/fs/trait.FileIoExt.html
[`io::IsTerminal`]: https://docs.rs/system-interface/latest/system_interface/io/trait.IsTerminal.html
[`io::ReadReady`]: https://docs.rs/system-interface/latest/system_interface/io/trait.ReadReady.html
[`io::Peek`]: https://docs.rs/system-interface/latest/system_interface/io/trait.Peek.html
[`std::io::Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
[`std::io::Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
[`std::io::Seek`]: https://doc.rust-lang.org/std/io/trait.Seek.html
[`std::os::unix::fs::FileExt`]: https://doc.rust-lang.org/std/os/unix/fs/trait.FileExt.html
[`read_exact_vectored_at`]: https://docs.rs/system-interface/latest/system_interface/fs/trait.FileIoExt.html#method.read_exact_vectored_at
[`write_all_vectored_at`]: https://docs.rs/system-interface/latest/system_interface/fs/trait.FileIoExt.html#method.write_all_vectored_at
[`read_to_end_at`]: https://docs.rs/system-interface/latest/system_interface/fs/trait.FileIoExt.html#method.read_to_end_at
[`read_to_string_at`]: https://docs.rs/system-interface/latest/system_interface/fs/trait.FileIoExt.html#method.read_to_string_at
