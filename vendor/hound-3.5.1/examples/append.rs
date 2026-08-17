// Hound -- A wav encoding and decoding library in Rust
// Copyright 2018 Ruud van Asseldonk
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// This example appends one second of a 440 Hz sine wave to the file "sine.wav".
// If the file does not exist, it is created instead.

use std::f32::consts::PI;
use std::i16;
use std::path::Path;

extern crate hound;

fn main() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let path: &Path = "sine.wav".as_ref();

    let mut writer = match path.is_file() {
        true => hound::WavWriter::append(path).unwrap(),
        false => hound::WavWriter::create(path, spec).unwrap(),
    };

    // We should not append blindly, we should make sure that the existing file
    // has the right spec, because that is what we assume when writing.
    assert_eq!(spec, writer.spec());

    println!("Old duration is {} seconds.", writer.duration() / spec.sample_rate);

    for t in (0 .. 44100).map(|x| x as f32 / 44100.0) {
        let sample = (t * 440.0 * 2.0 * PI).sin();
        let amplitude = i16::MAX as f32;
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }

    println!("New duration is {} seconds.", writer.duration() / spec.sample_rate);

    writer.finalize().unwrap();
}
