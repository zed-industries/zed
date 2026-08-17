# AVIF image serializer (muxer)

Minimal writer for AVIF header structure. This is lean, safe-Rust alternative to [libavif](https://lib.rs/libavif).
It creates the jungle of MPEG/HEIF/MIAF/ISO-BMFF "boxes" as appropriate for AVIF files. Supports alpha channel embedding.

Compatible with decoders in Chrome 85+, libavif v0.8.1, and Firefox 92. It's used in [cavif](https://lib.rs/cavif) and other encoders.

Together with [rav1e](https://lib.rs/rav1e), it allows pure-Rust AVIF image encoding.

## Requirements

* [Latest stable](https://rustup.rs) version of Rust.

## Usage

1. Compress pixels using an AV1 encoder, such as [rav1e](https://lib.rs/rav1e). [libaom](https://lib.rs/libaom-sys) works too.

2. Call `avif_serialize::serialize_to_vec(av1_data, None, width, height, 8)`

See [`ravif` crate sources](https://github.com/kornelski/cavif-rs) for example usage.

