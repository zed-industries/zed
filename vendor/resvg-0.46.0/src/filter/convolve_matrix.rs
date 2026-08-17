// Copyright 2020 the Resvg Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::{ImageRefMut, f32_bound};
use rgb::RGBA8;
use usvg::filter::{ConvolveMatrix, EdgeMode};

/// Applies a convolve matrix.
///
/// Input image pixels should have a **premultiplied alpha** when `preserve_alpha=false`.
///
/// # Allocations
///
/// This method will allocate a copy of the `src` image as a back buffer.
pub fn apply(matrix: &ConvolveMatrix, src: ImageRefMut) {
    fn bound(min: i32, val: i32, max: i32) -> i32 {
        core::cmp::max(min, core::cmp::min(max, val))
    }

    let width_max = src.width as i32 - 1;
    let height_max = src.height as i32 - 1;

    let mut buf = vec![RGBA8::default(); src.data.len()];
    let mut buf = ImageRefMut::new(src.width, src.height, &mut buf);
    let mut x = 0;
    let mut y = 0;
    for in_p in src.data.iter() {
        let mut new_r = 0.0;
        let mut new_g = 0.0;
        let mut new_b = 0.0;
        let mut new_a = 0.0;
        for oy in 0..matrix.matrix().rows() {
            for ox in 0..matrix.matrix().columns() {
                let mut tx = x as i32 - matrix.matrix().target_x() as i32 + ox as i32;
                let mut ty = y as i32 - matrix.matrix().target_y() as i32 + oy as i32;

                match matrix.edge_mode() {
                    EdgeMode::None => {
                        if tx < 0 || tx > width_max || ty < 0 || ty > height_max {
                            continue;
                        }
                    }
                    EdgeMode::Duplicate => {
                        tx = bound(0, tx, width_max);
                        ty = bound(0, ty, height_max);
                    }
                    EdgeMode::Wrap => {
                        while tx < 0 {
                            tx += src.width as i32;
                        }
                        tx %= src.width as i32;

                        while ty < 0 {
                            ty += src.height as i32;
                        }
                        ty %= src.height as i32;
                    }
                }

                let k = matrix.matrix().get(
                    matrix.matrix().columns() - ox - 1,
                    matrix.matrix().rows() - oy - 1,
                );

                let p = src.pixel_at(tx as u32, ty as u32);
                new_r += (p.r as f32) / 255.0 * k;
                new_g += (p.g as f32) / 255.0 * k;
                new_b += (p.b as f32) / 255.0 * k;

                if !matrix.preserve_alpha() {
                    new_a += (p.a as f32) / 255.0 * k;
                }
            }
        }

        if matrix.preserve_alpha() {
            new_a = in_p.a as f32 / 255.0;
        } else {
            new_a = new_a / matrix.divisor().get() + matrix.bias();
        }

        let bounded_new_a = f32_bound(0.0, new_a, 1.0);

        let calc = |x| {
            let x = x / matrix.divisor().get() + matrix.bias() * new_a;

            let x = if matrix.preserve_alpha() {
                f32_bound(0.0, x, 1.0) * bounded_new_a
            } else {
                f32_bound(0.0, x, bounded_new_a)
            };

            (x * 255.0 + 0.5) as u8
        };

        let out_p = buf.pixel_at_mut(x, y);
        out_p.r = calc(new_r);
        out_p.g = calc(new_g);
        out_p.b = calc(new_b);
        out_p.a = (bounded_new_a * 255.0 + 0.5) as u8;

        x += 1;
        if x == src.width {
            x = 0;
            y += 1;
        }
    }

    // Do not use `mem::swap` because `data` referenced via FFI.
    src.data.copy_from_slice(buf.data);
}
