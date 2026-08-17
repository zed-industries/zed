// Hound -- A wav encoding and decoding library in Rust
// Copyright (C) 2017 Ruud van Asseldonk
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// This example computes the root mean square (rms) of an audio file with
// integer or float samples, of at most 32 bits per sample. It is a slightly
// more elaborate version of the example found in the readme, mostly useful for
// checking whether Hound can read a specific file.

extern crate hound;

use std::env;
use std::io;

/// Compute the RMS of either integers or float samples.
fn compute_rms<S, R>(reader: &mut hound::WavReader<R>) -> f64
where
    f64: From<S>,
    S: hound::Sample,
    R: io::Read,
{
    let sqr_sum = reader.samples::<S>().fold(0.0, |sqr_sum, s| {
        let sample = f64::from(s.unwrap());
        sqr_sum + sample * sample
    });
    (sqr_sum / reader.len() as f64).sqrt()
}

fn main() {
    // Compute the RMS for all files given on the command line.
    for fname in env::args().skip(1) {
        let mut reader = hound::WavReader::open(&fname).unwrap();
        let rms = match reader.spec().sample_format {
            hound::SampleFormat::Float => compute_rms::<f32, _>(&mut reader),
            hound::SampleFormat::Int => compute_rms::<i32, _>(&mut reader),
        };
        println!("{}: {:0.1} ({} samples)", fname, rms, reader.len());
    }
}
