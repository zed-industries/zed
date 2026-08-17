// Hound -- A wav encoding and decoding library in Rust
// Copyright (C) 2015 Ruud van Asseldonk
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// This example computes the mean value and rms of a file, where samples are
// first interpreted as 16-bit signed integer, and then as a 16-bit unsigned
// integer. This should allow us to determine whether the samples stored are
// signed or unsigned: for signed the average value is expected to be 0, and
// for unsigned the value is expected to be 2^16 - 1.
//   Note that this example is not a particularly good example of proper coding
// style; it does not handle failure properly, and it assumes that the provided
// file has 16 bits per sample.

// TODO: This example should probably be removed, it is just here for verifying
// and assumption at this point.

extern crate hound;

use std::env;

fn main() {
    let fname = env::args().nth(1).expect("no file given");
    let mut reader = hound::WavReader::open(&fname).unwrap();
    let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();

    let (ts, tu, n) = samples.iter().fold((0.0, 0.0, 0.0), |(ts, tu, n), &s| {
        let signed = s as f64;
        let unsigned = (s as u16) as f64;
        (ts + signed, tu + unsigned, n + 1.0)
    });
    let ms = ts / n;
    let mu = tu / n;
    println!("mean signed:   {} (should be 0, deviation is {})", ms, ms.abs());
    println!("mean unsigned: {} (should be 2^16 - 1, deviation is {})", mu, (mu - 32767.0).abs());

    let (ts, tu) = samples.iter().fold((0.0, 0.0), |(ts, tu), &s| {
        let ds = s as f64 - ms;
        let du = (s as u16) as f64 - mu;
        (ts + ds * ds, tu + du * du)
    });
    let rmss = (ts / n).sqrt();
    let rmsu = (tu / n).sqrt();
    println!("rms signed:    {}", rmss);
    println!("rms unsigned:  {}", rmsu);
}
