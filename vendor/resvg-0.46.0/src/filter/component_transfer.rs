// Copyright 2020 the Resvg Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::{ImageRefMut, f32_bound};
use usvg::filter::{ComponentTransfer, TransferFunction};

/// Applies component transfer functions for each `src` image channel.
///
/// Input image pixels should have an **unpremultiplied alpha**.
pub fn apply(fe: &ComponentTransfer, src: ImageRefMut) {
    for pixel in src.data {
        if !is_dummy(fe.func_r()) {
            pixel.r = transfer(fe.func_r(), pixel.r);
        }

        if !is_dummy(fe.func_b()) {
            pixel.b = transfer(fe.func_b(), pixel.b);
        }

        if !is_dummy(fe.func_g()) {
            pixel.g = transfer(fe.func_g(), pixel.g);
        }

        if !is_dummy(fe.func_a()) {
            pixel.a = transfer(fe.func_a(), pixel.a);
        }
    }
}

fn is_dummy(func: &TransferFunction) -> bool {
    match func {
        TransferFunction::Identity => true,
        TransferFunction::Table(values) => values.is_empty(),
        TransferFunction::Discrete(values) => values.is_empty(),
        TransferFunction::Linear { .. } => false,
        TransferFunction::Gamma { .. } => false,
    }
}

fn transfer(func: &TransferFunction, c: u8) -> u8 {
    let c = c as f32 / 255.0;
    let c = match func {
        TransferFunction::Identity => c,
        TransferFunction::Table(values) => {
            let n = values.len() - 1;
            let k = (c * (n as f32)).floor() as usize;
            let k = std::cmp::min(k, n);
            if k == n {
                values[k]
            } else {
                let vk = values[k];
                let vk1 = values[k + 1];
                let k = k as f32;
                let n = n as f32;
                vk + (c - k / n) * n * (vk1 - vk)
            }
        }
        TransferFunction::Discrete(values) => {
            let n = values.len();
            let k = (c * (n as f32)).floor() as usize;
            values[std::cmp::min(k, n - 1)]
        }
        TransferFunction::Linear { slope, intercept } => slope * c + intercept,
        TransferFunction::Gamma {
            amplitude,
            exponent,
            offset,
        } => amplitude * c.powf(*exponent) + offset,
    };

    (f32_bound(0.0, c, 1.0) * 255.0) as u8
}
