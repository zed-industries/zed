# async_zip
[![Crates.io](https://img.shields.io/crates/v/async_zip?style=flat-square)](https://crates.io/crates/async_zip)
[![Crates.io](https://img.shields.io/crates/d/async_zip?style=flat-square)](https://crates.io/crates/async_zip)
[![docs.rs](https://img.shields.io/docsrs/async_zip?style=flat-square)](https://docs.rs/async_zip/)
[![GitHub Workflow Status (branch)](https://img.shields.io/github/actions/workflow/status/Majored/rs-async-zip/ci-linux.yml?branch=main&style=flat-square)](https://github.com/Majored/rs-async-zip/actions?query=branch%3Amain)
[![GitHub](https://img.shields.io/github/license/Majored/rs-async-zip?style=flat-square)](https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

An asynchronous ZIP archive reading/writing crate.

## Features
- A base implementation atop `futures`'s IO traits.
- An extended implementation atop `tokio`'s IO traits.
- Support for Stored, Deflate, bzip2, LZMA, zstd, and xz compression methods.
- Various different reading approaches (seek, stream, filesystem, in-memory buffer, etc).
- Support for writing complete data (u8 slices) or streams using data descriptors.
- Initial support for ZIP64 reading and writing.
- Aims for reasonable [specification](https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md) compliance.

## Installation & Basic Usage

```toml
[dependencies]
async_zip = { version = "0.0.17", features = ["full"] }
```

A (soon to be) extensive list of [examples](https://github.com/Majored/rs-async-zip/tree/main/examples) can be found under the `/examples` directory.

### Feature Flags
- `full` - Enables all below features.
- `full-wasm` - Enables all below features that are compatible with WASM.
- `chrono` - Enables support for parsing dates via `chrono`.
- `tokio` - Enables support for the `tokio` implementation module.
- `tokio-fs` - Enables support for the `tokio::fs` reading module.
- `deflate` - Enables support for the Deflate compression method.
- `bzip2` - Enables support for the bzip2 compression method.
- `lzma` - Enables support for the LZMA compression method.
- `zstd` - Enables support for the zstd compression method.
- `xz` - Enables support for the xz compression method.

### Reading
```rust
use tokio::{io::BufReader, fs::File};
use async_zip::tokio::read::seek::ZipFileReader;
...

let mut file = BufReader::new(File::open("./Archive.zip").await?);
let mut zip = ZipFileReader::with_tokio(&mut file).await?;

let mut string = String::new();
let mut reader = zip.reader_with_entry(0).await?;
reader.read_to_string_checked(&mut string).await?;

println!("{}", string);
```

### Writing
```rust
use async_zip::tokio::write::ZipFileWriter;
use async_zip::{Compression, ZipEntryBuilder};
use tokio::fs::File;
...

let mut file = File::create("foo.zip").await?;
let mut writer = ZipFileWriter::with_tokio(&mut file);

let data = b"This is an example file.";
let builder = ZipEntryBuilder::new("bar.txt".into(), Compression::Deflate);

writer.write_entry_whole(builder, data).await?;
writer.close().await?;
```

## Contributions
Whilst I will be continuing to maintain this crate myself, reasonable specification compliance is a huge undertaking for a single individual. As such, contributions will always be encouraged and appreciated.

No contribution guidelines exist but additions should be developed with readability in mind, with appropriate comments, and make use of `rustfmt`.

## Issues & Support
Whether you're wanting to report a bug you've come across during use of this crate or are seeking general help/assistance, please utilise the [issues tracker](https://github.com/Majored/rs-async-zip/issues) and provide as much detail as possible (eg. recreation steps).

I try to respond to issues within a reasonable timeframe.
