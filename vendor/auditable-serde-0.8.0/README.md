Parses and serializes the JSON dependency tree embedded in executables by the
[`cargo auditable`](https://github.com/rust-secure-code/cargo-auditable).

This crate defines the data structures that a serialized to/from JSON
and implements the serialization/deserialization routines via `serde`.

The [`VersionInfo`] struct is where all the magic happens, see the docs on it for more info.

## Basic usage

**Note:** this is a low-level crate that only implements JSON parsing. It rarely should be used directly.
You probably want the higher-level [`auditable-info`](https://docs.rs/auditable-info) crate instead.

The following snippet demonstrates full extraction pipeline using this crate,
including platform-specific executable handling via
[`auditable-extract`](http://docs.rs/auditable-serde/) and decompression
using the safe-Rust [`miniz_oxide`](http://docs.rs/miniz_oxide/):

```rust,ignore
use std::io::{Read, BufReader};
use std::{error::Error, fs::File, str::FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    // Read the input
    let f = File::open("target/release/hello-world")?;
    let mut f = BufReader::new(f);
    let mut input_binary = Vec::new();
    f.read_to_end(&mut input_binary)?;
    // Extract the compressed audit data
    let compressed_audit_data = auditable_extract::raw_auditable_data(&input_binary)?;
    // Decompress it with your Zlib implementation of choice. We recommend miniz_oxide
    use miniz_oxide::inflate::decompress_to_vec_zlib;
    let decompressed_data = decompress_to_vec_zlib(&compressed_audit_data)
        .map_err(|_| "Failed to decompress audit data")?;
    let decompressed_data = String::from_utf8(decompressed_data)?;
    println!("{}", decompressed_data);
    // Parse the audit data to Rust data structures
    let dependency_tree = auditable_serde::VersionInfo::from_str(&decompressed_data);
    Ok(())
}
```
