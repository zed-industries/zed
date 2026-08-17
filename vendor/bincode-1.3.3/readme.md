# Bincode

<img align="right" src="./logo.png" />

![CI](https://github.com/servo/bincode/workflows/CI/badge.svg)
[![](https://meritbadge.herokuapp.com/bincode)](https://crates.io/crates/bincode)
[![](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![](https://img.shields.io/badge/bincode-rustc_1.18+-lightgray.svg)](https://blog.rust-lang.org/2017/06/08/Rust-1.18.html)

A compact encoder / decoder pair that uses a binary zero-fluff encoding scheme.
The size of the encoded object will be the same or smaller than the size that
the object takes up in memory in a running Rust program.

In addition to exposing two simple functions
(one that encodes to `Vec<u8>`, and one that decodes from `&[u8]`),
binary-encode exposes a Reader/Writer API that makes it work
perfectly with other stream-based APIs such as Rust files, network streams,
and the [flate2-rs](https://github.com/alexcrichton/flate2-rs) compression
library.

## [API Documentation](https://docs.rs/bincode/)

## Bincode in the wild

* [google/tarpc](https://github.com/google/tarpc): Bincode is used to serialize and deserialize networked RPC messages.
* [servo/webrender](https://github.com/servo/webrender): Bincode records webrender API calls for record/replay-style graphics debugging.
* [servo/ipc-channel](https://github.com/servo/ipc-channel): IPC-Channel uses Bincode to send structs between processes using a channel-like API.

## Example

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Entity {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct World(Vec<Entity>);

fn main() {
    let world = World(vec![Entity { x: 0.0, y: 4.0 }, Entity { x: 10.0, y: 20.5 }]);

    let encoded: Vec<u8> = bincode::serialize(&world).unwrap();

    // 8 bytes for the length of the vector, 4 bytes per float.
    assert_eq!(encoded.len(), 8 + 4 * 4);

    let decoded: World = bincode::deserialize(&encoded[..]).unwrap();

    assert_eq!(world, decoded);
}
```

## Details

The encoding (and thus decoding) proceeds unsurprisingly -- primitive
types are encoded according to the underlying `Writer`, tuples and
structs are encoded by encoding their fields one-by-one, and enums are
encoded by first writing out the tag representing the variant and
then the contents.

However, there are some implementation details to be aware of:

* `isize`/`usize` are encoded as `i64`/`u64`, for portability.
* enums variants are encoded as a `u32` instead of a `usize`.
  `u32` is enough for all practical uses.
* `str` is encoded as `(u64, &[u8])`, where the `u64` is the number of
  bytes contained in the encoded string.

## Specification

Bincode's format will eventually be codified into a specification, along with
its configuration options and default configuration. In the meantime, here are
some frequently asked questions regarding use of the crate:

### Is Bincode suitable for storage?

The encoding format is stable across minor revisions, provided the same
configuration is used. This should ensure that later versions can still read
data produced by a previous versions of the library if no major version change
has occured.

Bincode is invariant over byte-order in the default configuration
(`bincode::options::DefaultOptions`), making an exchange between different
architectures possible. It is also rather space efficient, as it stores no
metadata like struct field names in the output format and writes long streams of
binary data without needing any potentially size-increasing encoding.

As a result, Bincode is suitable for storing data. Be aware that it does not
implement any sort of data versioning scheme or file headers, as these
features are outside the scope of this crate.

### Is Bincode suitable for untrusted inputs?

Bincode attempts to protect against hostile data. There is a maximum size
configuration available (`bincode::config::Bounded`), but not enabled in the
default configuration. Enabling it causes pre-allocation size to be limited to
prevent against memory exhaustion attacks.

Deserializing any incoming data will not cause undefined behavior or memory
issues, assuming that the deserialization code for the struct is safe itself.

Bincode can be used for untrusted inputs in the sense that it will not create a
security issues in your application, provided the configuration is changed to enable a
maximum size limit. Malicious inputs will fail upon deserialization.

### What is Bincode's MSRV (minimum supported Rust version)?

Bincode 1.0 maintains support for rust 1.18.0. Any changes to this are considered a breaking change for semver purposes.