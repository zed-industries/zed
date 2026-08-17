use std::ptr;

use v_frame::plane::Plane;

use crate::diff::BLOCK_SIZE;

/// Solves Ax = b, where x and b are column vectors of size nx1 and A is nxn
#[allow(clippy::many_single_char_names)]
pub(super) fn linsolve(
    n: usize,
    a: &mut [f64],
    stride: usize,
    b: &mut [f64],
    x: &mut [f64],
) -> bool {
    // SAFETY: We need to ensure that `n` doesn't exceed the bounds of these arrays.
    // But this is a crate-private function, so we control all input.
    unsafe {
        // Forward elimination
        for k in 0..(n - 1) {
            // Bring the largest magnitude to the diagonal position
            ((k + 1)..n).rev().for_each(|i| {
                if a.get_unchecked((i - 1) * stride + k).abs()
                    < a.get_unchecked(i * stride + k).abs()
                {
                    (0..n).for_each(|j| {
                        swap_unchecked(a, i * stride + j, (i - 1) * stride + j);
                    });
                    swap_unchecked(b, i, i - 1);
                }
            });

            for i in k..(n - 1) {
                if a.get_unchecked(k * stride + k).abs() < f64::EPSILON {
                    return false;
                }
                let c = *a.get_unchecked((i + 1) * stride + k) / *a.get_unchecked(k * stride + k);
                (0..n).for_each(|j| {
                    let a2_val = *a.get_unchecked(k * stride + j);
                    let a_val = a.get_unchecked_mut((i + 1) * stride + j);
                    *a_val = c.mul_add(-a2_val, *a_val);
                });
                let b2_val = *b.get_unchecked(k);
                let b_val = b.get_unchecked_mut(i + 1);
                *b_val = c.mul_add(-b2_val, *b_val);
            }
        }

        // Backward substitution
        for i in (0..n).rev() {
            if a.get_unchecked(i * stride + i).abs() < f64::EPSILON {
                return false;
            }
            let mut c = 0.0f64;
            for j in (i + 1)..n {
                c = a
                    .get_unchecked(i * stride + j)
                    .mul_add(*x.get_unchecked(j), c);
            }
            *x.get_unchecked_mut(i) = (*b.get_unchecked(i) - c) / *a.get_unchecked(i * stride + i);
        }
    }

    true
}

// TODO: This is unstable upstream. Once it's stable upstream, use that.
unsafe fn swap_unchecked<T>(slice: &mut [T], a: usize, b: usize) {
    let ptr = slice.as_mut_ptr();
    // SAFETY: caller has to guarantee that `a < self.len()` and `b < self.len()`
    unsafe {
        ptr::swap(ptr.add(a), ptr.add(b));
    }
}

pub(super) fn multiply_mat(
    m1: &[f64],
    m2: &[f64],
    res: &mut [f64],
    m1_rows: usize,
    inner_dim: usize,
    m2_cols: usize,
) {
    assert!(res.len() >= m1_rows * m2_cols);
    assert!(m1.len() >= m1_rows * inner_dim);
    assert!(m2.len() >= m2_cols * inner_dim);
    let mut idx = 0;
    for row in 0..m1_rows {
        for col in 0..m2_cols {
            let mut sum = 0f64;
            for inner in 0..inner_dim {
                // SAFETY: We do the bounds checks once at the top to improve performance.
                unsafe {
                    sum += m1.get_unchecked(row * inner_dim + inner)
                        * m2.get_unchecked(inner * m2_cols + col);
                }
            }
            // SAFETY: We do the bounds checks once at the top to improve performance.
            unsafe {
                *res.get_unchecked_mut(idx) = sum;
            }
            idx += 1;
        }
    }
}

#[must_use]
pub(super) fn normalized_cross_correlation(a: &[f64], b: &[f64], n: usize) -> f64 {
    let mut c = 0f64;
    let mut a_len = 0f64;
    let mut b_len = 0f64;
    for (a, b) in a.iter().zip(b.iter()).take(n) {
        a_len = (*a).mul_add(*a, a_len);
        b_len = (*b).mul_add(*b, b_len);
        c = (*a).mul_add(*b, c);
    }
    c / (a_len.sqrt() * b_len.sqrt())
}

#[allow(clippy::too_many_arguments)]
pub(super) fn extract_ar_row(
    coords: &[[isize; 2]],
    num_coords: usize,
    source_origin: &[u8],
    denoised_origin: &[u8],
    stride: usize,
    dec: (usize, usize),
    alt_source_origin: Option<&[u8]>,
    alt_denoised_origin: Option<&[u8]>,
    alt_stride: usize,
    x: usize,
    y: usize,
    buffer: &mut [f64],
) -> f64 {
    debug_assert!(buffer.len() > num_coords);
    debug_assert!(coords.len() >= num_coords);

    // SAFETY: We know the indexes we provide do not overflow the data bounds
    unsafe {
        for i in 0..num_coords {
            let x_i = x as isize + coords.get_unchecked(i)[0];
            let y_i = y as isize + coords.get_unchecked(i)[1];
            debug_assert!(x_i >= 0);
            debug_assert!(y_i >= 0);
            let index = y_i as usize * stride + x_i as usize;
            *buffer.get_unchecked_mut(i) = f64::from(*source_origin.get_unchecked(index))
                - f64::from(*denoised_origin.get_unchecked(index));
        }
        let val = f64::from(*source_origin.get_unchecked(y * stride + x))
            - f64::from(*denoised_origin.get_unchecked(y * stride + x));

        if let Some(alt_source_origin) = alt_source_origin {
            if let Some(alt_denoised_origin) = alt_denoised_origin {
                let mut source_sum = 0u64;
                let mut denoised_sum = 0u64;
                let mut num_samples = 0usize;

                for dy_i in 0..(1 << dec.1) {
                    let y_up = (y << dec.1) + dy_i;
                    for dx_i in 0..(1 << dec.0) {
                        let x_up = (x << dec.0) + dx_i;
                        let index = y_up * alt_stride + x_up;
                        source_sum += u64::from(*alt_source_origin.get_unchecked(index));
                        denoised_sum += u64::from(*alt_denoised_origin.get_unchecked(index));
                        num_samples += 1;
                    }
                }
                *buffer.get_unchecked_mut(num_coords) =
                    (source_sum as f64 - denoised_sum as f64) / num_samples as f64;
            }
        }

        val
    }
}

#[must_use]
pub(super) fn get_block_mean(
    source: &Plane<u8>,
    frame_dims: (usize, usize),
    x_o: usize,
    y_o: usize,
) -> f64 {
    let max_h = (frame_dims.1 - y_o).min(BLOCK_SIZE);
    let max_w = (frame_dims.0 - x_o).min(BLOCK_SIZE);

    let data_origin = source.data_origin();
    let mut block_sum = 0u64;
    for y in 0..max_h {
        for x in 0..max_w {
            let index = (y_o + y) * source.cfg.stride + x_o + x;
            // SAFETY: We know the index cannot exceed the dimensions of the plane data
            unsafe {
                block_sum += u64::from(*data_origin.get_unchecked(index));
            }
        }
    }

    block_sum as f64 / (max_w * max_h) as f64
}

#[must_use]
pub(super) fn get_noise_var(
    source: &Plane<u8>,
    denoised: &Plane<u8>,
    frame_dims: (usize, usize),
    x_o: usize,
    y_o: usize,
    block_w: usize,
    block_h: usize,
) -> f64 {
    let max_h = (frame_dims.1 - y_o).min(block_h);
    let max_w = (frame_dims.0 - x_o).min(block_w);

    let source_origin = source.data_origin();
    let denoised_origin = denoised.data_origin();
    let mut noise_var_sum = 0u64;
    let mut noise_sum = 0i64;
    for y in 0..max_h {
        for x in 0..max_w {
            let index = (y_o + y) * source.cfg.stride + x_o + x;
            // SAFETY: We know the index cannot exceed the dimensions of the plane data
            unsafe {
                let noise = i64::from(*source_origin.get_unchecked(index))
                    - i64::from(*denoised_origin.get_unchecked(index));
                noise_sum += noise;
                noise_var_sum += noise.pow(2) as u64;
            }
        }
    }

    let noise_mean = noise_sum as f64 / (max_w * max_h) as f64;
    noise_mean.mul_add(-noise_mean, noise_var_sum as f64 / (max_w * max_h) as f64)
}
