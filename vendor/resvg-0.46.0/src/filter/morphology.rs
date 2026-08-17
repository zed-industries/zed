// Copyright 2020 the Resvg Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::ImageRefMut;
use rgb::RGBA8;
use usvg::filter::MorphologyOperator;

/// Applies a morphology filter.
///
/// `src` pixels should have a **premultiplied alpha**.
///
/// # Allocations
///
/// This method will allocate a copy of the `src` image as a back buffer.
pub fn apply(operator: MorphologyOperator, rx: f32, ry: f32, src: ImageRefMut) {
    // No point in making matrix larger than image.
    let columns = std::cmp::min(rx.ceil() as u32 * 2, src.width);
    let rows = std::cmp::min(ry.ceil() as u32 * 2, src.height);
    let target_x = (columns as f32 / 2.0).floor() as u32;
    let target_y = (rows as f32 / 2.0).floor() as u32;

    let width_max = src.width as i32 - 1;
    let height_max = src.height as i32 - 1;

    let mut buf = vec![RGBA8::default(); src.data.len()];
    let mut buf = ImageRefMut::new(src.width, src.height, &mut buf);
    let mut x = 0;
    let mut y = 0;
    for _ in src.data.iter() {
        let mut new_p = RGBA8::default();
        if operator == MorphologyOperator::Erode {
            new_p.r = 255;
            new_p.g = 255;
            new_p.b = 255;
            new_p.a = 255;
        }

        for oy in 0..rows {
            for ox in 0..columns {
                let tx = x as i32 - target_x as i32 + ox as i32;
                let ty = y as i32 - target_y as i32 + oy as i32;

                if tx < 0 || tx > width_max || ty < 0 || ty > height_max {
                    continue;
                }

                let p = src.pixel_at(tx as u32, ty as u32);
                if operator == MorphologyOperator::Erode {
                    new_p.r = std::cmp::min(p.r, new_p.r);
                    new_p.g = std::cmp::min(p.g, new_p.g);
                    new_p.b = std::cmp::min(p.b, new_p.b);
                    new_p.a = std::cmp::min(p.a, new_p.a);
                } else {
                    new_p.r = std::cmp::max(p.r, new_p.r);
                    new_p.g = std::cmp::max(p.g, new_p.g);
                    new_p.b = std::cmp::max(p.b, new_p.b);
                    new_p.a = std::cmp::max(p.a, new_p.a);
                }
            }
        }

        *buf.pixel_at_mut(x, y) = new_p;

        x += 1;
        if x == src.width {
            x = 0;
            y += 1;
        }
    }

    // Do not use `mem::swap` because `data` referenced via FFI.
    src.data.copy_from_slice(buf.data);
}
