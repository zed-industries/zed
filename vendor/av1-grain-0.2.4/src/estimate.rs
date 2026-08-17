use std::{f64::consts::PI, mem::size_of};

use v_frame::{
    plane::Plane,
    prelude::{CastFromPrimitive, Pixel},
};

/// Estimates the amount of noise within a plane.
/// Returns `None` if a reliable estimate cannot be obtained
/// due to too few smooth pixels.
///
/// Ranges seem to be approximately:
///
/// - `0.0..0.5` = no noticeable noise
/// - `0.5..1.0` = light noise, probably photon-noise-esque
///
/// # Panics
/// - If called with a `bit_depth` not between `8..=16`
#[must_use]
pub fn estimate_plane_noise<T: Pixel>(plane: &Plane<T>, bit_depth: usize) -> Option<f64> {
    const EDGE_THRESHOLD: u16 = 50;

    if size_of::<T>() == 1 {
        assert_eq!(bit_depth, 8);
    } else if size_of::<T>() == 2 {
        assert!(bit_depth > 8 && bit_depth <= 16);
    } else {
        unimplemented!("Bit depths greater than 16 are not currently supported");
    }

    let width = plane.cfg.width;
    let height = plane.cfg.height;
    let stride = plane.cfg.stride;

    let mut accum = 0u64;
    let mut count = 0u64;
    for i in 1..(height - 1) {
        for j in 1..(width - 1) {
            // Setup a small 3x3 matrix.
            let center_idx = (i * stride + j) as isize;
            let mut mat = [[0i16; 3]; 3];
            for ii in -1isize..=1isize {
                for jj in -1isize..=1isize {
                    let idx = (center_idx + ii * stride as isize + jj) as usize;
                    mat[(ii + 1) as usize][(jj + 1) as usize] = if size_of::<T>() == 1 {
                        i16::cast_from(plane.data_origin()[idx])
                    } else {
                        (u16::cast_from(plane.data_origin()[idx]) >> (bit_depth - 8usize)) as i16
                    };
                }
            }

            // Compute sobel gradients
            let g_x =
                (mat[0][0] - mat[0][2]) + (mat[2][0] - mat[2][2]) + 2 * (mat[1][0] - mat[1][2]);
            let g_y =
                (mat[0][0] - mat[2][0]) + (mat[0][2] - mat[2][2]) + 2 * (mat[0][1] - mat[2][1]);
            let g_a = (g_x.abs() + g_y.abs()) as u16;
            // Accumulate Laplacian
            if g_a < EDGE_THRESHOLD {
                // Only count smooth pixels
                let v = 4 * mat[1][1] - 2 * (mat[0][1] + mat[2][1] + mat[1][0] + mat[1][2])
                    + (mat[0][0] + mat[0][2] + mat[2][0] + mat[2][2]);
                accum += u64::from(v.unsigned_abs());
                count += 1;
            }
        }
    }

    (count >= 16).then(|| accum as f64 / (6u64 * count) as f64 * (PI / 2f64).sqrt())
}
