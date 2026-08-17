# rust-embed

Rust macro which loads files into the rust binary at compile time during release and loads the file from the fs during dev.

You can use this to embed your css, js and images into a single executable which can be deployed to your servers. Also it makes it easy to build a very small docker image for you to deploy.

## Installation

```toml
[dependencies]
rust-embed="8.11.0"
```

## Documentation

You need to add the custom derive macro RustEmbed to your struct with an attribute `folder` which is the path to your static folder.

The path resolution works as follows:

- In `debug` and when `debug-embed` feature is not enabled, the folder path is resolved relative to where the binary is run from.
- In `release` or when `debug-embed` feature is enabled, the folder path is resolved relative to where `Cargo.toml` is.

```rust
#[derive(Embed)]
#[folder = "examples/public/"]
struct Asset;
```

The macro will generate the following code:

```rust
impl Asset {
  pub fn get(file_path: &str) -> Option<rust_embed::EmbeddedFile> {
    ...
  }

  pub fn iter() -> impl Iterator<Item = Cow<'static, str>> {
    ...
  }
}
impl RustEmbed for Asset {
  fn get(file_path: &str) -> Option<rust_embed::EmbeddedFile> {
    ...
  }
  fn iter() -> impl Iterator<Item = Cow<'static, str>> {
    ...
  }
}

// Where EmbeddedFile contains these fields,
pub struct EmbeddedFile {
  pub data: Cow<'static, [u8]>,
  pub metadata: Metadata,
}
pub struct Metadata {
  hash: [u8; 32],
  last_modified: Option<u64>,
  created: Option<u64>,
}
```

## Methods
* `get(file_path: &str) -> Option<rust_embed::EmbeddedFile>`

Given a relative path from the assets folder returns the `EmbeddedFile` if found.
If the feature `debug-embed` is enabled or the binary compiled in release mode the bytes have been embeded in the binary and a `Option<rust_embed::EmbeddedFile>` is returned.
Otherwise the bytes are read from the file system on each call and a `Option<rust_embed::EmbeddedFile>` is returned.

* `iter()`

Iterates the files in this assets folder.
If the feature `debug-embed` is enabled or the binary compiled in release mode a static array to the list of relative paths to the files is returned.
Otherwise the files are listed from the file system on each call.

## Attributes
* `prefix`

You can add `#[prefix = "my_prefix/"]` to the `RustEmbed` struct to add a prefix
to all of the file paths. This prefix will be required on `get` calls, and will
be included in the file paths returned by `iter`.

* `metadata_only`

You can add `#[metadata_only = true]` to the `RustEmbed` struct to exclude file contents from the
binary. Only file paths and metadata will be embedded.

* `allow_missing`

You can add `#[allow_missing = true]` to the `RustEmbed` struct to allow the embedded folder to be missing.
In that case, RustEmbed will be empty.

## Features

* `debug-embed`: Always embed the files in the binary, even in debug mode.
* `compression`: Compress each file when embedding into the binary. Compression is done via [include-flate](https://crates.io/crates/include-flate).
* `deterministic-timestamps`: Overwrite embedded files' timestamps with `0` to preserve deterministic builds with `debug-embed` or release mode.
* `interpolate-folder-path`: Allow environment variables to be used in the `folder` path. This will pull the `foo` directory relative to your `Cargo.toml` file.
```rust
#[derive(Embed)]
#[folder = "$CARGO_MANIFEST_DIR/foo"]
struct Asset;
```
* `include-exclude`: Filter files to be embedded with multiple `#[include = "*.txt"]` and `#[exclude = "*.jpg"]` attributes. 
Matching is done on relative file paths, via [globset](https://crates.io/crates/globset). `exclude` attributes have higher priority than `include` attributes.
```rust
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "examples/public/"]
#[include = "*.html"]
#[include = "images/*"]
#[exclude = "*.txt"]
struct Asset;
```

## Usage

```rust
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "examples/public/"]
#[prefix = "prefix/"]
struct Asset;

fn main() {
  let index_html = Asset::get("prefix/index.html").unwrap();
  println!("{:?}", std::str::from_utf8(index_html.data.as_ref()));

  for file in Asset::iter() {
      println!("{}", file.as_ref());
  }
}
```

## Integrations

1. [Poem](https://github.com/poem-web/poem) for poem framework under feature flag "embed"
2. [warp_embed](https://docs.rs/warp-embed/latest/warp_embed/) for warp framework

## Examples

```sh
cargo run --example basic #  dev mode where it reads from the fs
cargo run --example basic --release # release mode where it reads from binary
cargo run --example actix --features actix # https://github.com/actix/actix-web
cargo run --example rocket --features rocket # https://github.com/SergioBenitez/Rocket
cargo run --example warp --features warp-ex # https://github.com/seanmonstar/warp
cargo run --example axum --features axum-ex # https://github.com/tokio-rs/axum
cargo run --example poem --features poem-ex # https://github.com/poem-web/poem
cargo run --example salvo --features salvo-ex # https://github.com/salvo-rs/salvo
```

## Testing

```sh
cargo test --test lib
cargo test --test lib --features "debug-embed"
cargo test --test lib --features "compression" --release
cargo test --test mime_guess --features "mime-guess"
cargo test --test mime_guess --features "mime-guess" --release
cargo test --test interpolated_path --features "interpolate-folder-path"
cargo test --test interpolated_path --features "interpolate-folder-path" --release
cargo test --test custom_crate_path
cargo test --test custom_crate_path --release
cargo build --example basic
cargo build --example rocket --features rocket
cargo build --example actix --features actix
cargo build --example axum --features axum-ex
cargo build --example warp --features warp-ex
cargo test --test lib --release
cargo build --example basic --release
cargo build --example rocket --features rocket --release
cargo build --example actix --features actix --release
cargo build --example axum --features axum-ex --release
cargo build --example warp --features warp-ex --release
```
