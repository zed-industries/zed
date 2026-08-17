// Copyright 2020 the Resvg Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(clippy::needless_range_loop)]

use super::{ImageRefMut, f32_bound};
use usvg::ApproxZeroUlps;

const RAND_M: i32 = 2147483647; // 2**31 - 1
const RAND_A: i32 = 16807; // 7**5; primitive root of m
const RAND_Q: i32 = 127773; // m / a
const RAND_R: i32 = 2836; // m % a
const B_SIZE: usize = 0x100;
const B_SIZE_32: i32 = 0x100;
const B_LEN: usize = B_SIZE + B_SIZE + 2;
const BM: i32 = 0xff;
const PERLIN_N: i32 = 0x1000;

#[derive(Clone, Copy)]
struct StitchInfo {
    width: i32, // How much to subtract to wrap for stitching.
    height: i32,
    wrap_x: i32, // Minimum value to wrap.
    wrap_y: i32,
}

/// Applies a turbulence filter.
///
/// `dest` image pixels will have an **unpremultiplied alpha**.
///
/// - `offset_x` and `offset_y` indicate filter region offset.
/// - `sx` and `sy` indicate canvas scale.
pub fn apply(
    offset_x: f64,
    offset_y: f64,
    sx: f64,
    sy: f64,
    base_frequency_x: f64,
    base_frequency_y: f64,
    num_octaves: u32,
    seed: i32,
    stitch_tiles: bool,
    fractal_noise: bool,
    dest: ImageRefMut,
) {
    let (lattice_selector, gradient) = init(seed);
    let width = dest.width;
    let height = dest.height;
    let mut x = 0;
    let mut y = 0;
    for pixel in dest.data.iter_mut() {
        let turb = |channel| {
            let (tx, ty) = ((x as f64 + offset_x) / sx, (y as f64 + offset_y) / sy);
            let n = turbulence(
                channel,
                tx,
                ty,
                x as f64,
                y as f64,
                width as f64,
                height as f64,
                base_frequency_x,
                base_frequency_y,
                num_octaves,
                fractal_noise,
                stitch_tiles,
                &lattice_selector,
                &gradient,
            );

            let n = if fractal_noise {
                (n * 255.0 + 255.0) / 2.0
            } else {
                n * 255.0
            };

            (f32_bound(0.0, n as f32, 255.0) + 0.5) as u8
        };

        pixel.r = turb(0);
        pixel.g = turb(1);
        pixel.b = turb(2);
        pixel.a = turb(3);

        x += 1;
        if x == dest.width {
            x = 0;
            y += 1;
        }
    }
}

fn init(mut seed: i32) -> (Vec<usize>, Vec<Vec<Vec<f64>>>) {
    let mut lattice_selector = vec![0; B_LEN];
    let mut gradient = vec![vec![vec![0.0; 2]; B_LEN]; 4];

    if seed <= 0 {
        seed = -seed % (RAND_M - 1) + 1;
    }

    if seed > RAND_M - 1 {
        seed = RAND_M - 1;
    }

    for k in 0..4 {
        for i in 0..B_SIZE {
            lattice_selector[i] = i;
            for j in 0..2 {
                seed = random(seed);
                gradient[k][i][j] =
                    ((seed % (B_SIZE_32 + B_SIZE_32)) - B_SIZE_32) as f64 / B_SIZE_32 as f64;
            }

            let s = (gradient[k][i][0] * gradient[k][i][0] + gradient[k][i][1] * gradient[k][i][1])
                .sqrt();

            gradient[k][i][0] /= s;
            gradient[k][i][1] /= s;
        }
    }

    for i in (1..B_SIZE).rev() {
        let k = lattice_selector[i];
        seed = random(seed);
        let j = (seed % B_SIZE_32) as usize;
        lattice_selector[i] = lattice_selector[j];
        lattice_selector[j] = k;
    }

    for i in 0..B_SIZE + 2 {
        lattice_selector[B_SIZE + i] = lattice_selector[i];
        for g in gradient.iter_mut().take(4) {
            for j in 0..2 {
                g[B_SIZE + i][j] = g[i][j];
            }
        }
    }

    (lattice_selector, gradient)
}

fn turbulence(
    color_channel: usize,
    mut x: f64,
    mut y: f64,
    tile_x: f64,
    tile_y: f64,
    tile_width: f64,
    tile_height: f64,
    mut base_freq_x: f64,
    mut base_freq_y: f64,
    num_octaves: u32,
    fractal_sum: bool,
    do_stitching: bool,
    lattice_selector: &[usize],
    gradient: &[Vec<Vec<f64>>],
) -> f64 {
    // Adjust the base frequencies if necessary for stitching.
    let mut stitch = if do_stitching {
        // When stitching tiled turbulence, the frequencies must be adjusted
        // so that the tile borders will be continuous.
        if !base_freq_x.approx_zero_ulps(4) {
            let lo_freq = (tile_width * base_freq_x).floor() / tile_width;
            let hi_freq = (tile_width * base_freq_x).ceil() / tile_width;
            if base_freq_x / lo_freq < hi_freq / base_freq_x {
                base_freq_x = lo_freq;
            } else {
                base_freq_x = hi_freq;
            }
        }

        if !base_freq_y.approx_zero_ulps(4) {
            let lo_freq = (tile_height * base_freq_y).floor() / tile_height;
            let hi_freq = (tile_height * base_freq_y).ceil() / tile_height;
            if base_freq_y / lo_freq < hi_freq / base_freq_y {
                base_freq_y = lo_freq;
            } else {
                base_freq_y = hi_freq;
            }
        }

        // Set up initial stitch values.
        let width = (tile_width * base_freq_x + 0.5) as i32;
        let height = (tile_height * base_freq_y + 0.5) as i32;
        let wrap_x = (tile_x * base_freq_x + PERLIN_N as f64 + width as f64) as i32;
        let wrap_y = (tile_y * base_freq_y + PERLIN_N as f64 + height as f64) as i32;
        Some(StitchInfo {
            width,
            height,
            wrap_x,
            wrap_y,
        })
    } else {
        None
    };

    let mut sum = 0.0;
    x *= base_freq_x;
    y *= base_freq_y;
    let mut ratio = 1.0;
    for _ in 0..num_octaves {
        if fractal_sum {
            sum += noise2(color_channel, x, y, lattice_selector, gradient, stitch) / ratio;
        } else {
            sum += noise2(color_channel, x, y, lattice_selector, gradient, stitch).abs() / ratio;
        }
        x *= 2.0;
        y *= 2.0;
        ratio *= 2.0;

        if let Some(ref mut stitch) = stitch {
            // Update stitch values. Subtracting PerlinN before the multiplication and
            // adding it afterward simplifies to subtracting it once.
            stitch.width *= 2;
            stitch.wrap_x = 2 * stitch.wrap_x - PERLIN_N;
            stitch.height *= 2;
            stitch.wrap_y = 2 * stitch.wrap_y - PERLIN_N;
        }
    }

    sum
}

fn noise2(
    color_channel: usize,
    x: f64,
    y: f64,
    lattice_selector: &[usize],
    gradient: &[Vec<Vec<f64>>],
    stitch_info: Option<StitchInfo>,
) -> f64 {
    let t = x + PERLIN_N as f64;
    let mut bx0 = t as i32;
    let mut bx1 = bx0 + 1;
    let rx0 = t - t as i64 as f64;
    let rx1 = rx0 - 1.0;
    let t = y + PERLIN_N as f64;
    let mut by0 = t as i32;
    let mut by1 = by0 + 1;
    let ry0 = t - t as i64 as f64;
    let ry1 = ry0 - 1.0;

    // If stitching, adjust lattice points accordingly.
    if let Some(info) = stitch_info {
        if bx0 >= info.wrap_x {
            bx0 -= info.width;
        }

        if bx1 >= info.wrap_x {
            bx1 -= info.width;
        }

        if by0 >= info.wrap_y {
            by0 -= info.height;
        }

        if by1 >= info.wrap_y {
            by1 -= info.height;
        }
    }

    bx0 &= BM;
    bx1 &= BM;
    by0 &= BM;
    by1 &= BM;
    let i = lattice_selector[bx0 as usize];
    let j = lattice_selector[bx1 as usize];
    let b00 = lattice_selector[i + by0 as usize];
    let b10 = lattice_selector[j + by0 as usize];
    let b01 = lattice_selector[i + by1 as usize];
    let b11 = lattice_selector[j + by1 as usize];
    let sx = s_curve(rx0);
    let sy = s_curve(ry0);
    let q = &gradient[color_channel][b00];
    let u = rx0 * q[0] + ry0 * q[1];
    let q = &gradient[color_channel][b10];
    let v = rx1 * q[0] + ry0 * q[1];
    let a = lerp(sx, u, v);
    let q = &gradient[color_channel][b01];
    let u = rx0 * q[0] + ry1 * q[1];
    let q = &gradient[color_channel][b11];
    let v = rx1 * q[0] + ry1 * q[1];
    let b = lerp(sx, u, v);
    lerp(sy, a, b)
}

fn random(seed: i32) -> i32 {
    let mut result = RAND_A * (seed % RAND_Q) - RAND_R * (seed / RAND_Q);
    if result <= 0 {
        result += RAND_M;
    }

    result
}

#[inline]
fn s_curve(t: f64) -> f64 {
    t * t * (3.0 - 2.0 * t)
}

#[inline]
fn lerp(t: f64, a: f64, b: f64) -> f64 {
    a + t * (b - a)
}
