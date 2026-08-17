<div align="center">
  <h1><code>cap-std</code></h1>

  <p>
    <strong>Capability-based version of the Rust standard library</strong>
  </p>

  <p>
    <a href="https://github.com/bytecodealliance/cap-std/actions?query=workflow%3ACI"><img src="https://github.com/bytecodealliance/cap-std/workflows/CI/badge.svg" alt="Github Actions CI Status" /></a>
    <a href="https://crates.io/crates/cap-std"><img src="https://img.shields.io/crates/v/cap-std.svg" alt="crates.io page" /></a>
    <a href="https://docs.rs/cap-std"><img src="https://docs.rs/cap-std/badge.svg" alt="docs.rs docs" /></a>
  </p>
</div>

This crate provides a capability-based version of [`std`], providing
sandboxed filesystem, networking, and clock APIs. See the [toplevel README.md]
for more information about sandboxing using capability-based security.

The filesystem module [`cap_std::fs`], the networking module [`cap_std::net`],
and the time module [`cap_std::time`] currently support Linux, macOS, FreeBSD,
and Windows. WASI support is in development, though not yet usable.

Example usage of [`Dir`] for filesystem access:

```rust
use std::io;
use cap_std::fs::Dir;

/// Open files relative to `dir`.
fn dir_example(dir: &Dir) -> io::Result<()> {
    // This works (assuming symlinks don't lead outside of `dir`).
    let file = dir.open("the/thing.txt")?;

    // This fails, since `..` leads outside of `dir`.
    let hidden = dir.open("../hidden.txt")?;

    // This fails, as creating symlinks to absolute paths is not permitted.
    dir.symlink("/hidden.txt", "misdirection.txt")?;

    // However, even if the symlink had succeeded, or, if there is a
    // pre-existing symlink to an absolute directory, following a
    // symlink which would lead outside the sandbox also fails.
    let secret = dir.open("misdirection.txt")?;

    Ok(())
}
```

Example usage of [`Pool`] for network access:

```rust
use std::io;
use cap_std::net::Pool;

/// Open network addresses within `pool`.
fn pool_example(pool: &Pool) -> io::Result<()> {
    // Connect to an address. This succeeds only if the given address and
    // port are present in `pool`.
    let stream = pool.connect_tcp_stream("localhost:3333")?;

    Ok(())
}
```

[`std`]: https://doc.rust-lang.org/std/
[toplevel README.md]: https://github.com/bytecodealliance/cap-std/blob/main/README.md
[`cap_std::fs`]: https://docs.rs/cap-std/latest/cap_std/fs/index.html
[`cap_std::net`]: https://docs.rs/cap-std/latest/cap_std/net/index.html
[`cap_std::time`]: https://docs.rs/cap-std/latest/cap_std/time/index.html
[`Pool`]: https://docs.rs/cap-std/latest/cap_std/net/struct.Pool.html
[`Dir`]: https://docs.rs/cap-std/latest/cap_std/fs/struct.Dir.html
