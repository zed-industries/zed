<h1 align="center">async-tar</h1>
<div align="center">
 <strong>
   A tar archive reading/writing library for async Rust.
 </strong>
</div>

<br />

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/async-tar">
    <img src="https://img.shields.io/crates/v/async-tar.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/async-tar">
    <img src="https://img.shields.io/crates/d/async-tar.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/async-tar">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>

<div align="center">
  <h3>
    <a href="https://docs.rs/async-tar">
      API Docs
    </a>
    <span> | </span>
    <a href="https://github.com/dignifiedquire/async-tar/releases">
      Releases
    </a>
  </h3>
</div>
<br/>

> Based on the great [tar-rs](https://github.com/alexcrichton/tar-rs).

## Features

- `runtime-async-std`: enabled by default, makes this crate compatible with `async-std`
- `runtime-tokio`: makes this crate compatible with `tokio`

> **Note**: These features are mutually exclusive. Enable only one runtime feature.

## Reading an archive

```rust,no_run
use async_std::io::stdin;
use async_std::prelude::*;
use async_tar::Archive;

fn main() {
    async_std::task::block_on(async {
        let mut ar = Archive::new(stdin());
        let mut entries = ar.entries().unwrap();
        while let Some(file) = entries.next().await {
            let f = file.unwrap();
            println!("{}", f.path().unwrap().display());
        }
    });
}
```

## Writing an archive

```rust,no_run
use async_std::fs::File;
use async_tar::Builder;

fn main() {
    async_std::task::block_on(async {
        let file = File::create("foo.tar").await.unwrap();
        let mut a = Builder::new(file);

        a.append_path("README.md").await.unwrap();
        a.append_file("lib.rs", &mut File::open("src/lib.rs").await.unwrap())
            .await
            .unwrap();
        a.into_inner().await.unwrap();
    });
}
```

# MSRV

Minimal stable rust version: 1.85

*An increase to the MSRV is accompanied by a minor version bump*

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
