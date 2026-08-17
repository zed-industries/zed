Hound
=====
A wav encoding and decoding library in Rust.

[![Crates.io version][crate-img]][crate]
[![Changelog][changelog-img]](changelog.md)
[![Documentation][docs-img]][docs]

Hound can read and write the WAVE audio format, an ubiquitous format for raw,
uncompressed audio. The main motivation to write it was to test
[Claxon][claxon], a FLAC decoding library written in Rust.

Examples
--------
The following example renders a 440 Hz sine wave, and stores it as a mono wav
file with a sample rate of 44.1 kHz and 16 bits per sample.

```rust
use std::f32::consts::PI;
use std::i16;
use hound;

let spec = hound::WavSpec {
    channels: 1,
    sample_rate: 44100,
    bits_per_sample: 16,
    sample_format: hound::SampleFormat::Int,
};
let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
for t in (0 .. 44100).map(|x| x as f32 / 44100.0) {
    let sample = (t * 440.0 * 2.0 * PI).sin();
    let amplitude = i16::MAX as f32;
    writer.write_sample((sample * amplitude) as i16).unwrap();
}
```

The file is finalized implicitly when the writer is dropped, call
`writer.finalize()` to observe errors.

The following example computes the root mean square (RMS) of an audio file with
at most 16 bits per sample.

```rust
use hound;

let mut reader = hound::WavReader::open("testsamples/pop.wav").unwrap();
let sqr_sum = reader.samples::<i16>()
                    .fold(0.0, |sqr_sum, s| {
    let sample = s.unwrap() as f64;
    sqr_sum + sample * sample
});
println!("RMS is {}", (sqr_sum / reader.len() as f64).sqrt());
```

Features
--------

|                 | Read                                                    | Write                                   |
|-----------------|---------------------------------------------------------|-----------------------------------------|
| Format          | `PCMWAVEFORMAT`, `WAVEFORMATEX`, `WAVEFORMATEXTENSIBLE` | `PCMWAVEFORMAT`, `WAVEFORMATEXTENSIBLE` |
| Encoding        | Integer PCM, IEEE Float                                 | Integer PCM, IEEE Float                 |
| Bits per sample | 8, 16, 24, 32 (integer), 32 (float)                     | 8, 16, 24, 32 (integer), 32 (float)     |

Contributing
------------
Contributions in the form of bug reports, feature requests, or pull requests are
welcome. See [contributing.md](contributing.md).

License
-------
Hound is licensed under the [Apache 2.0][apache2] license. It may be used in
free software as well as closed-source applications, both for commercial and
non-commercial use under the conditions given in the license. If you want to
use Hound in your GPLv2-licensed software, you can add an [exception][exception]
to your copyright notice. Please do not open an issue if you disagree with the
choice of license.

[crate-img]:     https://img.shields.io/crates/v/hound.svg
[crate]:         https://crates.io/crates/hound
[changelog-img]: https://img.shields.io/badge/changelog-online-blue.svg
[docs-img]:      https://img.shields.io/badge/docs-online-blue.svg
[docs]:          https://docs.rs/hound
[claxon]:        https://github.com/ruuda/claxon
[apache2]:       https://www.apache.org/licenses/LICENSE-2.0
[exception]:     https://www.gnu.org/licenses/gpl-faq.html#GPLIncompatibleLibs
