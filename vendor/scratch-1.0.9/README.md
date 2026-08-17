Shared scratch for build scripts
================================

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/scratch-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/scratch)
[<img alt="crates.io" src="https://img.shields.io/crates/v/scratch.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/scratch)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-scratch-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/scratch)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/dtolnay/scratch/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/dtolnay/scratch/actions?query=branch%3Amaster)

This crate exposes a compile-time temporary directory shareable by multiple
crates in a build graph and erased by `cargo clean`.

The intended usage is from a build.rs Cargo build script, or more likely from a
library which is called by other crates' build scripts.

```toml
# Cargo.toml

[build-dependencies]
scratch = "1.0"
```

```rust
// build.rs

fn main() {
    let dir = scratch::path("mycrate");
    // ... write or read inside of that path
}
```

<br>

## Comparisons

Comparison to **`std::env::var_os("OUT_DIR")`**:

- This functionality is different from OUT\_DIR in that the same directory path
  will be seen by *all* crates whose build passes a matching `suffix` argument,
  and each crate can see content placed into the directory by those other
  crates' build scripts that have already run.

- This functionality is similar to OUT\_DIR in that both are erased when a
  `cargo clean` is executed.

Comparison to **`std::env::temp_dir()`**:

- This functionality is similar to temp\_dir() in that stuff that goes in is
  visible to subsequently running build scripts.

- This functionality is different from temp\_dir() in that `cargo clean` cleans
  up the contents.

<br>

## Tips

You'll want to consider what happens when Cargo runs multiple build scripts
concurrently that access the same scratch dir. In some use cases you likely want
some synchronization over the contents of the scratch directory, such as by an
advisory [file lock]. On Unix-like and Windows host systems the simplest way to
sequence the build scripts such that each one gets exclusive access one after
the other is something like:

[file lock]: https://man7.org/linux/man-pages/man2/flock.2.html

```rust
use std::fs::File;
use std::io;

fn main() -> io::Result<()> {
    let dir = scratch::path("demo");
    let flock = File::create(dir.join(".lock"))?;
    flock.lock()?;

    // ... now do work
}
```

This simplest approach is fine for a cache which is slow to fill (maybe a large
download) but fast/almost immediate to use. On the other hand if the build
scripts using your cache will take a while to complete even if they only read
from the scratch directory, a different approach which allows readers to make
progress in parallel would perform better.

```rust
use std::fs::{self, File};
use std::io;

fn main() -> io::Result<()> {
    let dir = scratch::path("demo");
    let flock = File::create(dir.join(".lock"))?;
    let sdk = dir.join("thing.sdk");

    if !sdk.exists() {
        flock.lock()?;
        if !sdk.exists() {
            let download_location = sdk.with_file_name("thing.sdk.partial");
            download_sdk_to(&download_location)?;
            fs::rename(&download_location, &sdk)?;
        }
        flock.unlock()?;
    }

    // ... now use the SDK
}
```

For use cases that are not just a matter of the first build script writing to
the directory and the rest reading, more elaborate schemes involving
`lock_shared` might be something to consider.

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
