//! Bitmap scaling.

#![allow(dead_code)]

use alloc::vec::Vec;
#[cfg(feature = "libm")]
use core_maths::CoreFloat;

mod png;

/// Decodes a PNG image.
pub fn decode_png(data: &[u8], scratch: &mut Vec<u8>, target: &mut [u8]) -> Option<(u32, u32)> {
    png::decode(data, scratch, target)
        .map(|(w, h, _)| (w, h))
        .ok()
}

pub fn blit(
    mask: &[u8],
    mask_width: u32,
    mask_height: u32,
    x: i32,
    y: i32,
    color: [u8; 4],
    target: &mut [u8],
    target_width: u32,
    target_height: u32,
) {
    if mask_width == 0 || mask_height == 0 || target_width == 0 || target_height == 0 {
        return;
    }
    let source_width = mask_width as usize;
    let source_height = mask_height as usize;
    let dest_width = target_width as usize;
    let dest_height = target_height as usize;
    let source_x = if x < 0 { -x as usize } else { 0 };
    let source_y = if y < 0 { -y as usize } else { 0 };
    if source_x >= source_width || source_y >= source_height {
        return;
    }
    let dest_x = if x < 0 { 0 } else { x as usize };
    let dest_y = if y < 0 { 0 } else { y as usize };
    if dest_x >= dest_width || dest_y >= dest_height {
        return;
    }
    let source_end_x = (source_width).min(dest_width - dest_x + source_x);
    let source_end_y = (source_height).min(dest_height - dest_y + source_y);
    let source_columns = source_y..source_end_y;
    let source_rows = source_x..source_end_x;
    let dest_pitch = target_width as usize * 4;
    let color_a = color[3] as u32;
    let mut dy = dest_y;
    for sy in source_columns {
        let src_row = &mask[sy * mask_width as usize..];
        let dst_row = &mut target[dy * dest_pitch..];
        dy += 1;
        let mut dx = dest_x * 4;
        for sx in source_rows.clone() {
            let a = (src_row[sx] as u32 * color_a) >> 8;
            if a >= 255 {
                dst_row[dx + 3] = 255;
                dst_row[dx..(dx + 3)].copy_from_slice(&color[..3]);
                dst_row[dx + 3] = 255;
            } else if a != 0 {
                let inverse_a = 255 - a;
                for i in 0..3 {
                    let d = dst_row[dx + i] as u32;
                    let c = ((inverse_a * d) + (a * color[i] as u32)) >> 8;
                    dst_row[dx + i] = c as u8;
                }
                let d = dst_row[dx + 3] as u32;
                let c = ((inverse_a * d) + a * 255) >> 8;
                dst_row[dx + 3] = c as u8;
            }
            dx += 4;
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Filter {
    Nearest,
    Bilinear,
    Bicubic,
    Mitchell,
    Lanczos3,
    Gaussian,
}

pub fn resize(
    image: &[u8],
    width: u32,
    height: u32,
    channels: u32,
    target: &mut [u8],
    target_width: u32,
    target_height: u32,
    filter: Filter,
    scratch: Option<&mut Vec<u8>>,
) -> bool {
    if target_width == 0 || target_height == 0 {
        return true;
    }
    let mut tmp = Vec::new();
    let scratch = if let Some(scratch) = scratch {
        scratch
    } else {
        &mut tmp
    };
    let image_size = (width * height * channels) as usize;
    if image.len() < image_size {
        return false;
    }
    let target_size = (target_width * target_height * channels) as usize;
    if target.len() < target_size {
        return false;
    }
    let scratch_size = (target_width * height * channels) as usize;
    scratch.resize(scratch_size, 0);
    use Filter::*;
    match filter {
        Nearest => resample(
            image,
            width,
            height,
            channels,
            target,
            target_width,
            target_height,
            scratch,
            0.,
            &nearest,
        ),
        Bilinear => resample(
            image,
            width,
            height,
            channels,
            target,
            target_width,
            target_height,
            scratch,
            1.,
            &bilinear,
        ),
        Bicubic => resample(
            image,
            width,
            height,
            channels,
            target,
            target_width,
            target_height,
            scratch,
            2.,
            &bicubic,
        ),
        Mitchell => resample(
            image,
            width,
            height,
            channels,
            target,
            target_width,
            target_height,
            scratch,
            2.,
            &mitchell,
        ),
        Lanczos3 => resample(
            image,
            width,
            height,
            channels,
            target,
            target_width,
            target_height,
            scratch,
            3.,
            &lanczos3,
        ),
        Gaussian => resample(
            image,
            width,
            height,
            channels,
            target,
            target_width,
            target_height,
            scratch,
            3.,
            &|x| gaussian(x, 0.5),
        ),
    }
}

fn resample<Filter>(
    image: &[u8],
    width: u32,
    height: u32,
    channels: u32,
    target: &mut [u8],
    target_width: u32,
    target_height: u32,
    scratch: &mut [u8],
    support: f32,
    filter: &Filter,
) -> bool
where
    Filter: Fn(f32) -> f32,
{
    let tmp_width = target_width;
    let tmp_height = height;
    let s = 1. / 255.;
    if channels == 1 {
        sample_dir(
            &|x, y| [0., 0., 0., image[(y * width + x) as usize] as f32 * s],
            width,
            height,
            target_width,
            filter,
            support,
            &mut |x, y, p| scratch[(y * tmp_width + x) as usize] = (p[3] * 255.) as u8,
        );
        sample_dir(
            &|y, x| [0., 0., 0., scratch[(y * tmp_width + x) as usize] as f32 * s],
            tmp_height,
            tmp_width,
            target_height,
            filter,
            support,
            &mut |y, x, p| target[(y * target_width + x) as usize] = (p[3] * 255.) as u8,
        );
        true
    } else if channels == 4 {
        sample_dir(
            &|x, y| {
                let row = (y * width * channels + x * channels) as usize;
                [
                    image[row] as f32 * s,
                    image[row + 1] as f32 * s,
                    image[row + 2] as f32 * s,
                    image[row + 3] as f32 * s,
                ]
            },
            width,
            height,
            target_width,
            filter,
            support,
            &mut |x, y, p| {
                let row = (y * target_width * channels + x * channels) as usize;
                scratch[row] = (p[0] * 255.) as u8;
                scratch[row + 1] = (p[1] * 255.) as u8;
                scratch[row + 2] = (p[2] * 255.) as u8;
                scratch[row + 3] = (p[3] * 255.) as u8;
            },
        );
        sample_dir(
            &|y, x| {
                let row = (y * tmp_width * channels + x * channels) as usize;
                [
                    scratch[row] as f32 * s,
                    scratch[row + 1] as f32 * s,
                    scratch[row + 2] as f32 * s,
                    scratch[row + 3] as f32 * s,
                ]
            },
            tmp_height,
            tmp_width,
            target_height,
            filter,
            support,
            &mut |y, x, p| {
                let row = (y * target_width * channels + x * channels) as usize;
                target[row] = (p[0] * 255.) as u8;
                target[row + 1] = (p[1] * 255.) as u8;
                target[row + 2] = (p[2] * 255.) as u8;
                target[row + 3] = (p[3] * 255.) as u8;
            },
        );
        true
    } else {
        false
    }
}

fn sample_dir<Input, Output, Filter>(
    input: &Input,
    width: u32,
    height: u32,
    new_width: u32,
    filter: &Filter,
    support: f32,
    output: &mut Output,
) where
    Input: Fn(u32, u32) -> [f32; 4],
    Output: FnMut(u32, u32, &[f32; 4]),
    Filter: Fn(f32) -> f32,
{
    const MAX_WEIGHTS: usize = 64;
    let mut weights = [0f32; MAX_WEIGHTS];
    let mut num_weights;
    let ratio = width as f32 / new_width as f32;
    let sratio = ratio.max(1.);
    let src_support = support * sratio;
    let isratio = 1. / sratio;
    for outx in 0..new_width {
        let inx = (outx as f32 + 0.5) * ratio;
        let left = (inx - src_support).floor() as i32;
        let mut left = left.max(0).min(width as i32 - 1) as usize;
        let right = (inx + src_support).ceil() as i32;
        let mut right = right.max(left as i32 + 1).min(width as i32) as usize;
        let inx = inx - 0.5;
        while right - left > MAX_WEIGHTS {
            right -= 1;
            left += 1;
        }
        num_weights = 0;
        let mut sum = 0.;
        for i in left..right {
            let w = filter((i as f32 - inx) * isratio);
            weights[num_weights] = w;
            num_weights += 1;
            sum += w;
        }
        let isum = 1. / sum;
        let weights = &weights[..num_weights];
        for y in 0..height {
            let mut accum = [0f32; 4];
            for (i, w) in weights.iter().enumerate() {
                let p = input((left + i) as u32, y);
                let a = p[3];
                accum[0] += p[0] * w * a;
                accum[1] += p[1] * w * a;
                accum[2] += p[2] * w * a;
                accum[3] += p[3] * w;
            }
            if accum[3] != 0. {
                let a = 1. / accum[3];
                accum[0] *= a;
                accum[1] *= a;
                accum[2] *= a;
                accum[3] *= isum;
            }
            output(outx, y, &accum);
        }
    }
}

fn sinc(t: f32) -> f32 {
    let a = t * core::f32::consts::PI;
    if t == 0. {
        1.
    } else {
        a.sin() / a
    }
}

fn lanczos3(x: f32) -> f32 {
    if x.abs() < 3. {
        (sinc(x) * sinc(x / 3.)).abs()
    } else {
        0.
    }
}

fn bilinear(x: f32) -> f32 {
    let x = x.abs();
    if x < 1. {
        1. - x
    } else {
        0.
    }
}

fn bicubic(x: f32) -> f32 {
    let a = x.abs();
    let b = 0.;
    let c = 0.5;
    let k = if a < 1. {
        (12. - 9. * b - 6. * c) * a.powi(3) + (-18. + 12. * b + 6. * c) * a.powi(2) + (6. - 2. * b)
    } else if a < 2. {
        (-b - 6. * c) * a.powi(3)
            + (6. * b + 30. * c) * a.powi(2)
            + (-12. * b - 48. * c) * a
            + (8. * b + 24. * c)
    } else {
        0.
    };
    (k / 6.).abs()
}

fn mitchell(x: f32) -> f32 {
    let x = x.abs();
    if x < 1. {
        ((16. + x * x * (21. * x - 36.)) / 18.).abs()
    } else if x < 2. {
        ((32. + x * (-60. + x * (36. - 7. * x))) / 18.).abs()
    } else {
        0.
    }
}

fn nearest(_x: f32) -> f32 {
    1.
}

fn gaussian(x: f32, r: f32) -> f32 {
    ((2. * core::f32::consts::PI).sqrt() * r).recip() * (-x.powi(2) / (2. * r.powi(2))).exp()
}
