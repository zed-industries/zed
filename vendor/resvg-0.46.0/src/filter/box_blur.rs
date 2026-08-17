// Copyright 2020 the Resvg Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Based on https://github.com/fschutt/fastblur

#![allow(clippy::needless_range_loop)]

use super::ImageRefMut;
use rgb::RGBA8;
use std::cmp;

const STEPS: usize = 5;

/// Applies a box blur.
///
/// Input image pixels should have a **premultiplied alpha**.
///
/// A negative or zero `sigma_x`/`sigma_y` will disable the blur along that axis.
///
/// # Allocations
///
/// This method will allocate a copy of the `src` image as a back buffer.
pub fn apply(sigma_x: f64, sigma_y: f64, mut src: ImageRefMut) {
    let boxes_horz = create_box_gauss(sigma_x as f32);
    let boxes_vert = create_box_gauss(sigma_y as f32);
    let mut backbuf = src.data.to_vec();
    let mut backbuf = ImageRefMut::new(src.width, src.height, &mut backbuf);

    for (box_size_horz, box_size_vert) in boxes_horz.iter().zip(boxes_vert.iter()) {
        let radius_horz = ((box_size_horz - 1) / 2) as usize;
        let radius_vert = ((box_size_vert - 1) / 2) as usize;
        box_blur_impl(radius_horz, radius_vert, &mut backbuf, &mut src);
    }
}

#[inline(never)]
fn create_box_gauss(sigma: f32) -> [i32; STEPS] {
    if sigma > 0.0 {
        let n_float = STEPS as f32;

        // Ideal averaging filter width
        let w_ideal = (12.0 * sigma * sigma / n_float).sqrt() + 1.0;
        let mut wl = w_ideal.floor() as i32;
        if wl % 2 == 0 {
            wl -= 1;
        }

        let wu = wl + 2;

        let wl_float = wl as f32;
        let m_ideal = (12.0 * sigma * sigma
            - n_float * wl_float * wl_float
            - 4.0 * n_float * wl_float
            - 3.0 * n_float)
            / (-4.0 * wl_float - 4.0);
        let m = m_ideal.round() as usize;

        let mut sizes = [0; STEPS];
        for i in 0..STEPS {
            if i < m {
                sizes[i] = wl;
            } else {
                sizes[i] = wu;
            }
        }

        sizes
    } else {
        [1; STEPS]
    }
}

#[inline]
fn box_blur_impl(
    blur_radius_horz: usize,
    blur_radius_vert: usize,
    backbuf: &mut ImageRefMut,
    frontbuf: &mut ImageRefMut,
) {
    box_blur_vert(blur_radius_vert, frontbuf, backbuf);
    box_blur_horz(blur_radius_horz, backbuf, frontbuf);
}

#[inline]
fn box_blur_vert(blur_radius: usize, backbuf: &ImageRefMut, frontbuf: &mut ImageRefMut) {
    if blur_radius == 0 {
        frontbuf.data.copy_from_slice(backbuf.data);
        return;
    }

    let width = backbuf.width as usize;
    let height = backbuf.height as usize;

    let iarr = 1.0 / (blur_radius + blur_radius + 1) as f32;
    let blur_radius_prev = blur_radius as isize - height as isize;
    let blur_radius_next = blur_radius as isize + 1;

    for i in 0..width {
        let col_start = i; //inclusive
        let col_end = i + width * (height - 1); //inclusive
        let mut ti = i;
        let mut li = ti;
        let mut ri = ti + blur_radius * width;

        let fv = RGBA8::default();
        let lv = RGBA8::default();

        let mut val_r = blur_radius_next * (fv.r as isize);
        let mut val_g = blur_radius_next * (fv.g as isize);
        let mut val_b = blur_radius_next * (fv.b as isize);
        let mut val_a = blur_radius_next * (fv.a as isize);

        // Get the pixel at the specified index, or the first pixel of the column
        // if the index is beyond the top edge of the image
        let get_top = |i| {
            if i < col_start { fv } else { backbuf.data[i] }
        };

        // Get the pixel at the specified index, or the last pixel of the column
        // if the index is beyond the bottom edge of the image
        let get_bottom = |i| {
            if i > col_end { lv } else { backbuf.data[i] }
        };

        for j in 0..cmp::min(blur_radius, height) {
            let bb = backbuf.data[ti + j * width];
            val_r += bb.r as isize;
            val_g += bb.g as isize;
            val_b += bb.b as isize;
            val_a += bb.a as isize;
        }
        if blur_radius > height {
            val_r += blur_radius_prev * (lv.r as isize);
            val_g += blur_radius_prev * (lv.g as isize);
            val_b += blur_radius_prev * (lv.b as isize);
            val_a += blur_radius_prev * (lv.a as isize);
        }

        for _ in 0..cmp::min(height, blur_radius + 1) {
            let bb = get_bottom(ri);
            ri += width;
            val_r += sub(bb.r, fv.r);
            val_g += sub(bb.g, fv.g);
            val_b += sub(bb.b, fv.b);
            val_a += sub(bb.a, fv.a);

            frontbuf.data[ti] = RGBA8 {
                r: round(val_r as f32 * iarr) as u8,
                g: round(val_g as f32 * iarr) as u8,
                b: round(val_b as f32 * iarr) as u8,
                a: round(val_a as f32 * iarr) as u8,
            };
            ti += width;
        }

        if height <= blur_radius {
            // otherwise `(height - blur_radius)` will underflow
            continue;
        }

        for _ in (blur_radius + 1)..(height - blur_radius) {
            let bb1 = backbuf.data[ri];
            ri += width;
            let bb2 = backbuf.data[li];
            li += width;

            val_r += sub(bb1.r, bb2.r);
            val_g += sub(bb1.g, bb2.g);
            val_b += sub(bb1.b, bb2.b);
            val_a += sub(bb1.a, bb2.a);

            frontbuf.data[ti] = RGBA8 {
                r: round(val_r as f32 * iarr) as u8,
                g: round(val_g as f32 * iarr) as u8,
                b: round(val_b as f32 * iarr) as u8,
                a: round(val_a as f32 * iarr) as u8,
            };
            ti += width;
        }

        for _ in 0..cmp::min(height - blur_radius - 1, blur_radius) {
            let bb = get_top(li);
            li += width;

            val_r += sub(lv.r, bb.r);
            val_g += sub(lv.g, bb.g);
            val_b += sub(lv.b, bb.b);
            val_a += sub(lv.a, bb.a);

            frontbuf.data[ti] = RGBA8 {
                r: round(val_r as f32 * iarr) as u8,
                g: round(val_g as f32 * iarr) as u8,
                b: round(val_b as f32 * iarr) as u8,
                a: round(val_a as f32 * iarr) as u8,
            };
            ti += width;
        }
    }
}

#[inline]
fn box_blur_horz(blur_radius: usize, backbuf: &ImageRefMut, frontbuf: &mut ImageRefMut) {
    if blur_radius == 0 {
        frontbuf.data.copy_from_slice(backbuf.data);
        return;
    }

    let width = backbuf.width as usize;
    let height = backbuf.height as usize;

    let iarr = 1.0 / (blur_radius + blur_radius + 1) as f32;
    let blur_radius_prev = blur_radius as isize - width as isize;
    let blur_radius_next = blur_radius as isize + 1;

    for i in 0..height {
        let row_start = i * width; // inclusive
        let row_end = (i + 1) * width - 1; // inclusive
        let mut ti = i * width; // VERTICAL: $i;
        let mut li = ti;
        let mut ri = ti + blur_radius;

        let fv = RGBA8::default();
        let lv = RGBA8::default();

        let mut val_r = blur_radius_next * (fv.r as isize);
        let mut val_g = blur_radius_next * (fv.g as isize);
        let mut val_b = blur_radius_next * (fv.b as isize);
        let mut val_a = blur_radius_next * (fv.a as isize);

        // Get the pixel at the specified index, or the first pixel of the row
        // if the index is beyond the left edge of the image
        let get_left = |i| {
            if i < row_start { fv } else { backbuf.data[i] }
        };

        // Get the pixel at the specified index, or the last pixel of the row
        // if the index is beyond the right edge of the image
        let get_right = |i| {
            if i > row_end { lv } else { backbuf.data[i] }
        };

        for j in 0..cmp::min(blur_radius, width) {
            let bb = backbuf.data[ti + j]; // VERTICAL: ti + j * width
            val_r += bb.r as isize;
            val_g += bb.g as isize;
            val_b += bb.b as isize;
            val_a += bb.a as isize;
        }
        if blur_radius > width {
            val_r += blur_radius_prev * (lv.r as isize);
            val_g += blur_radius_prev * (lv.g as isize);
            val_b += blur_radius_prev * (lv.b as isize);
            val_a += blur_radius_prev * (lv.a as isize);
        }

        // Process the left side where we need pixels from beyond the left edge
        for _ in 0..cmp::min(width, blur_radius + 1) {
            let bb = get_right(ri);
            ri += 1;
            val_r += sub(bb.r, fv.r);
            val_g += sub(bb.g, fv.g);
            val_b += sub(bb.b, fv.b);
            val_a += sub(bb.a, fv.a);

            frontbuf.data[ti] = RGBA8 {
                r: round(val_r as f32 * iarr) as u8,
                g: round(val_g as f32 * iarr) as u8,
                b: round(val_b as f32 * iarr) as u8,
                a: round(val_a as f32 * iarr) as u8,
            };
            ti += 1; // VERTICAL : ti += width, same with the other areas
        }

        if width <= blur_radius {
            // otherwise `(width - blur_radius)` will underflow
            continue;
        }

        // Process the middle where we know we won't bump into borders
        // without the extra indirection of get_left/get_right. This is faster.
        for _ in (blur_radius + 1)..(width - blur_radius) {
            let bb1 = backbuf.data[ri];
            ri += 1;
            let bb2 = backbuf.data[li];
            li += 1;

            val_r += sub(bb1.r, bb2.r);
            val_g += sub(bb1.g, bb2.g);
            val_b += sub(bb1.b, bb2.b);
            val_a += sub(bb1.a, bb2.a);

            frontbuf.data[ti] = RGBA8 {
                r: round(val_r as f32 * iarr) as u8,
                g: round(val_g as f32 * iarr) as u8,
                b: round(val_b as f32 * iarr) as u8,
                a: round(val_a as f32 * iarr) as u8,
            };
            ti += 1;
        }

        // Process the right side where we need pixels from beyond the right edge
        for _ in 0..cmp::min(width - blur_radius - 1, blur_radius) {
            let bb = get_left(li);
            li += 1;

            val_r += sub(lv.r, bb.r);
            val_g += sub(lv.g, bb.g);
            val_b += sub(lv.b, bb.b);
            val_a += sub(lv.a, bb.a);

            frontbuf.data[ti] = RGBA8 {
                r: round(val_r as f32 * iarr) as u8,
                g: round(val_g as f32 * iarr) as u8,
                b: round(val_b as f32 * iarr) as u8,
                a: round(val_a as f32 * iarr) as u8,
            };
            ti += 1;
        }
    }
}

/// Fast rounding for x <= 2^23.
/// This is orders of magnitude faster than built-in rounding intrinsic.
///
/// Source: https://stackoverflow.com/a/42386149/585725
#[inline]
fn round(mut x: f32) -> f32 {
    x += 12582912.0;
    x -= 12582912.0;
    x
}

#[inline]
fn sub(c1: u8, c2: u8) -> isize {
    c1 as isize - c2 as isize
}
