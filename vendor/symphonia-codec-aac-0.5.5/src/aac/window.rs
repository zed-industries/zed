// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// Previous Author: Kostya Shishkov <kostya.shiskov@gmail.com>
//
// This source file includes code originally written for the NihAV
// project. With the author's permission, it has been relicensed for,
// and ported to the Symphonia project.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::f32::consts;

/// Window types.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowType {
    /// Simple sine window.
    Sine,
    /// Kaiser-Bessel derived window.
    KaiserBessel(f32),
}

/// Calculates window coefficients for the requested window type and size.
///
/// Set `half` flag to calculate only the first half of the window.
pub fn generate_window(mode: WindowType, scale: f32, size: usize, half: bool, dst: &mut [f32]) {
    match mode {
        WindowType::Sine => {
            let param =
                if half { consts::PI / ((2 * size) as f32) } else { consts::PI / (size as f32) };
            for n in 0..size {
                dst[n] = (((n as f32) + 0.5) * param).sin() * scale;
            }
        }
        WindowType::KaiserBessel(alpha) => {
            let dlen = if half { size as f32 } else { (size as f32) * 0.5 };
            let alpha2 = f64::from((alpha * consts::PI / dlen) * (alpha * consts::PI / dlen));

            let mut kb: Vec<f64> = Vec::with_capacity(size);
            let mut sum = 0.0;
            for n in 0..size {
                let b = bessel_i0(((n * (size - n)) as f64) * alpha2);
                sum += b;
                kb.push(sum);
            }
            sum += 1.0;
            for n in 0..size {
                dst[n] = (kb[n] / sum).sqrt() as f32;
            }
        }
    };
}

fn bessel_i0(inval: f64) -> f64 {
    let mut val: f64 = 1.0;
    for n in (1..64).rev() {
        val *= inval / f64::from(n * n);
        val += 1.0;
    }
    val
}
