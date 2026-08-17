# `ravif` — Pure Rust library for AVIF image encoding

Encoder for AVIF images. Based on [`rav1e`](https://lib.rs/crates/rav1e) and [`avif-serialize`](https://lib.rs/crates/avif-serialize).

The API is just a single `encode_rgba()` function call that spits an AVIF image.

This library powers the [`cavif`](https://lib.rs/crates/cavif) encoder. It has an encoding configuration specifically tuned for still images, and gives better quality/performance than stock `rav1e`.
