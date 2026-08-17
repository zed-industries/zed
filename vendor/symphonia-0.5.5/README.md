<div align="center">
<h1>Symphonia</h1>

<!--
<p>
    <img src="https://raw.githubusercontent.com/pdeljanov/symphonia/master/assets/logo.png" width="200px" />
</p>
-->

<p>
    <a href="https://crates.io/crates/symphonia">
        <img alt="Crate Info" src="https://img.shields.io/crates/v/symphonia.svg"/>
    </a>
    <a href="https://docs.rs/symphonia/">
        <img alt="API Docs" src="https://img.shields.io/badge/docs.rs-symphonia-brightgreen"/>
    </a>
    <a href="https://github.com/pdeljanov/Symphonia/actions/workflows/ci.yml">
        <img src="https://github.com/pdeljanov/Symphonia/actions/workflows/ci.yml/badge.svg" />
    </a>
    <a href="https://deps.rs/repo/github/pdeljanov/symphonia">
        <img src="https://deps.rs/repo/github/pdeljanov/symphonia/status.svg" />
    </a>
    <a href="https://blog.rust-lang.org/2021/06/17/Rust-1.53.0.html">
        <img alt="Rustc Version 1.53.0+" src="https://img.shields.io/badge/rustc-1.53%2B-lightgrey.svg"/>
    </a>
</p>

<p>
    <strong>
        Symphonia is a pure Rust audio decoding and media demuxing library supporting AAC, ADPCM, AIFF, ALAC, CAF, FLAC, MKV, MP1, MP2, MP3, MP4, OGG, Vorbis, WAV, and WebM.
    </strong>
</p>

<p>
    <h3>
        <a href="https://github.com/pdeljanov/Symphonia/blob/master/GETTING_STARTED.md">Getting Started</a>
        <span> · </span>
        <a href="https://docs.rs/symphonia">Documentation</a>
        <span> · </span>
        <a href="https://github.com/pdeljanov/Symphonia/tree/master/symphonia/examples">Examples</a>
        <span> · </span>
        <a href="https://github.com/pdeljanov/Symphonia/blob/master/BENCHMARKS.md">Benchmarks</a>
    </h3>
</p>
</div>

---

## Features

* Decode support for the most popular audio codecs with support for gapless playback
* Demux the most common media container formats
* Read most metadata and tagging formats
* Automatic format and decoder detection
* Basic audio primitives for manipulating audio data efficiently
* 100% safe Rust
* Minimal dependencies
* Fast with no compromises in performance!

Additionally, planned features include:

* Providing a C API for integration into other languages
* Providing a WASM API for web usage

## Current Support

Support for individual audio codecs and media formats are provided by separate crates. By default, Symphonia only enables support royalty-free open standard codecs and formats, but others may be enabled using feature flags.

> **Tip:** All formats and codecs can be enabled with the `all` feature flag.

### Status

The following status classifications are used to determine the state of development for each format or codec.

| Status    | Meaning                                                                                                                  |
|-----------|--------------------------------------------------------------------------------------------------------------------------|
| Good      | Many media streams play. Some streams may panic, error, or produce audible glitches. Some features may not be supported. |
| Great     | Most media streams play. Inaudible glitches may be present. Most common features are supported.                          |
| Excellent | All media streams play.  No audible or inaudible glitches. All required features are supported.                          |

A status of *Great* indicates that major development is complete and that the feature is in a state that would be acceptable for most applications to use.

A status of *Excellent* is only assigned after the feature passes all compliance tests. If no compliance tests are readily available, then a status of *Excellent* will be assigned if Symphonia's output matches that of a reference implementation, or `ffmpeg`, over a large test corpus.

### Formats (Demuxers)

| Format   | Status    | Gapless* | Feature Flag | Default | Crate                       |
|----------|-----------|----------|--------------|---------|-----------------------------|
| AIFF     | Great     | Yes      | `aiff`       | No      | [`symphonia-format-riff`]   |
| CAF      | Good      | No       | `caf`        | No      | [`symphonia-format-caf`]    |
| ISO/MP4  | Great     | No       | `isomp4`     | No      | [`symphonia-format-isomp4`] |
| MKV/WebM | Good      | No       | `mkv`        | Yes     | [`symphonia-format-mkv`]    |
| OGG      | Great     | Yes      | `ogg`        | Yes     | [`symphonia-format-ogg`]    |
| Wave     | Excellent | Yes      | `wav`        | Yes     | [`symphonia-format-riff`]   |

\* Gapless playback requires support from both the demuxer and decoder.

[`symphonia-format-caf`]: https://docs.rs/symphonia-format-caf
[`symphonia-format-isomp4`]: https://docs.rs/symphonia-format-isomp4
[`symphonia-format-mkv`]: https://docs.rs/symphonia-format-mkv
[`symphonia-format-ogg`]: https://docs.rs/symphonia-format-ogg
[`symphonia-format-riff`]: https://docs.rs/symphonia-format-riff

> **Tip:** All formats can be enabled with the `all-formats` feature flag.

### Codecs (Decoders)

| Codec                        | Status    | Gapless | Feature Flag | Default | Crate                      |
|------------------------------|-----------|---------|--------------|---------|----------------------------|
| AAC-LC                       | Great     | No      | `aac`        | No      | [`symphonia-codec-aac`]    |
| ADPCM                        | Good      | Yes     | `adpcm`      | Yes     | [`symphonia-codec-adpcm`]  |
| ALAC                         | Great     | Yes     | `alac`       | No      | [`symphonia-codec-alac`]   |
| FLAC                         | Excellent | Yes     | `flac`       | Yes     | [`symphonia-bundle-flac`]  |
| MP1                          | Great     | No      | `mp1`, `mpa` | No      | [`symphonia-bundle-mp3`]   |
| MP2                          | Great     | No      | `mp2`, `mpa` | No      | [`symphonia-bundle-mp3`]   |
| MP3                          | Excellent | Yes     | `mp3`, `mpa` | No      | [`symphonia-bundle-mp3`]   |
| PCM                          | Excellent | Yes     | `pcm`        | Yes     | [`symphonia-codec-pcm`]    |
| Vorbis                       | Excellent | Yes     | `vorbis`     | Yes     | [`symphonia-codec-vorbis`] |

A `symphonia-bundle-*` package is a combination of a decoder and a native demuxer.

[`symphonia-codec-aac`]: https://docs.rs/symphonia-codec-aac
[`symphonia-codec-adpcm`]: https://docs.rs/symphonia-codec-adpcm
[`symphonia-codec-alac`]: https://docs.rs/symphonia-codec-alac
[`symphonia-bundle-flac`]: https://docs.rs/symphonia-bundle-flac
[`symphonia-bundle-mp3`]: https://docs.rs/symphonia-bundle-mp3
[`symphonia-codec-pcm`]: https://docs.rs/symphonia-codec-pcm
[`symphonia-codec-vorbis`]: https://docs.rs/symphonia-codec-vorbis

> **Tip:** All codecs can be enabled with the `all-codecs` feature flag. Similarly, all MPEG audio codecs can be enabled with the `mpa` feature flag.

### Tags (Readers)

All metadata readers are provided by the `symphonia-metadata` crate.

| Format                | Status    |
|-----------------------|-----------|
| ID3v1                 | Great     |
| ID3v2                 | Great     |
| ISO/MP4               | Great     |
| RIFF                  | Great     |
| Vorbis comment (FLAC) | Perfect   |
| Vorbis comment (OGG)  | Perfect   |

## Quality

In addition to the safety guarantees afforded by Rust, Symphonia aims to:

* Decode media as correctly as the leading free-and-open-source software decoders
* Prevent denial-of-service attacks
* Be fuzz-tested
* Provide a powerful, consistent, and easy to use API

## Performance

Symphonia aims to be comparable to, or faster than, popular open-source C-based implementations. Currently, Symphonia's decoders are generally +/-15% the performance of FFMpeg. However, the exact range will depend strongly on the codec, which features of the codec are being leveraged in the encoding, the Rust compiler version, and the CPU architecture being compiled for.

See the [benchmarks](https://github.com/pdeljanov/Symphonia/blob/master/BENCHMARKS.md) for more information.

### Optimizations

At this time, SIMD optimizations are **not** enabled by default. Enabling any SIMD support feature flags will pull in the `rustfft` dependency.

| Instruction Set | Feature Flag    | Default |
|-----------------|-----------------|---------|
| SSE             | `opt-simd-sse`  | No      |
| AVX             | `opt-simd-avx`  | No      |
| Neon            | `opt-simd-neon` | No      |

> **Tip:** All SIMD optimizations can be enabled with the `opt-simd` feature flag.

## Examples

Basic usage examples may be found [`here`](https://github.com/pdeljanov/Symphonia/tree/master/symphonia/examples).

For a more complete application, see [`symphonia-play`](https://github.com/pdeljanov/Symphonia/tree/master/symphonia-play), a simple music player.

## Tools

Symphonia provides the following tools for debugging purposes:

* [`symphonia-play`](https://github.com/pdeljanov/Symphonia/tree/master/symphonia-play) for probing, decoding, validating, and playing back media streams.
* [`symphonia-check`](https://github.com/pdeljanov/Symphonia/tree/master/symphonia-check) for validating Symphonia's decoded output against various decoders.

## Author

The primary author is Philip Deljanov.

## Special Thanks

* Kostya Shishkov (AAC-LC decoder contribution, see `symphonia-codec-aac`)

## License

Symphonia is provided under the MPL v2.0 license. Please refer to the LICENSE file for more details.

## Contributing

Symphonia is a free and open-source project that welcomes contributions! To get started, please read our [Contribution Guidelines](https://github.com/pdeljanov/Symphonia/tree/master/CONTRIBUTING.md).
