mod util;

use std::ops::{Add, AddAssign};

use anyhow::anyhow;
use arrayvec::ArrayVec;
use v_frame::{frame::Frame, math::clamp, plane::Plane};

use self::util::{extract_ar_row, get_block_mean, get_noise_var, linsolve, multiply_mat};
use super::{NoiseStatus, BLOCK_SIZE, BLOCK_SIZE_SQUARED};
use crate::{
    diff::solver::util::normalized_cross_correlation, GrainTableSegment, DEFAULT_GRAIN_SEED,
    NUM_UV_COEFFS, NUM_UV_POINTS, NUM_Y_COEFFS, NUM_Y_POINTS,
};

const LOW_POLY_NUM_PARAMS: usize = 3;
const NOISE_MODEL_LAG: usize = 3;
const BLOCK_NORMALIZATION: f64 = 255.0f64;

#[derive(Debug, Clone)]
pub(super) struct FlatBlockFinder {
    a: Box<[f64]>,
    a_t_a_inv: [f64; LOW_POLY_NUM_PARAMS * LOW_POLY_NUM_PARAMS],
}

impl FlatBlockFinder {
    #[must_use]
    pub fn new() -> Self {
        let mut eqns = EquationSystem::new(LOW_POLY_NUM_PARAMS);
        let mut a_t_a_inv = [0.0f64; LOW_POLY_NUM_PARAMS * LOW_POLY_NUM_PARAMS];
        let mut a = vec![0.0f64; LOW_POLY_NUM_PARAMS * BLOCK_SIZE_SQUARED];

        let bs_half = (BLOCK_SIZE / 2) as f64;
        (0..BLOCK_SIZE).for_each(|y| {
            let yd = (y as f64 - bs_half) / bs_half;
            (0..BLOCK_SIZE).for_each(|x| {
                let xd = (x as f64 - bs_half) / bs_half;
                let coords = [yd, xd, 1.0f64];
                let row = y * BLOCK_SIZE + x;
                a[LOW_POLY_NUM_PARAMS * row] = yd;
                a[LOW_POLY_NUM_PARAMS * row + 1] = xd;
                a[LOW_POLY_NUM_PARAMS * row + 2] = 1.0f64;

                (0..LOW_POLY_NUM_PARAMS).for_each(|i| {
                    (0..LOW_POLY_NUM_PARAMS).for_each(|j| {
                        eqns.a[LOW_POLY_NUM_PARAMS * i + j] += coords[i] * coords[j];
                    });
                });
            });
        });

        // Lazy inverse using existing equation solver.
        (0..LOW_POLY_NUM_PARAMS).for_each(|i| {
            eqns.b.fill(0.0f64);
            eqns.b[i] = 1.0f64;
            eqns.solve();

            (0..LOW_POLY_NUM_PARAMS).for_each(|j| {
                a_t_a_inv[j * LOW_POLY_NUM_PARAMS + i] = eqns.x[j];
            });
        });

        FlatBlockFinder {
            a: a.into_boxed_slice(),
            a_t_a_inv,
        }
    }

    // The gradient-based features used in this code are based on:
    //  A. Kokaram, D. Kelly, H. Denman and A. Crawford, "Measuring noise
    //  correlation for improved video denoising," 2012 19th, ICIP.
    // The thresholds are more lenient to allow for correct grain modeling
    // in extreme cases.
    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn run(&self, plane: &Plane<u8>) -> (Vec<u8>, usize) {
        const TRACE_THRESHOLD: f64 = 0.15f64 / BLOCK_SIZE_SQUARED as f64;
        const RATIO_THRESHOLD: f64 = 1.25f64;
        const NORM_THRESHOLD: f64 = 0.08f64 / BLOCK_SIZE_SQUARED as f64;
        const VAR_THRESHOLD: f64 = 0.005f64 / BLOCK_SIZE_SQUARED as f64;

        // The following weights are used to combine the above features to give
        // a sigmoid score for flatness. If the input was normalized to [0,100]
        // the magnitude of these values would be close to 1 (e.g., weights
        // corresponding to variance would be a factor of 10000x smaller).
        const VAR_WEIGHT: f64 = -6682f64;
        const RATIO_WEIGHT: f64 = -0.2056f64;
        const TRACE_WEIGHT: f64 = 13087f64;
        const NORM_WEIGHT: f64 = -12434f64;
        const OFFSET: f64 = 2.5694f64;

        let num_blocks_w = (plane.cfg.width + BLOCK_SIZE - 1) / BLOCK_SIZE;
        let num_blocks_h = (plane.cfg.height + BLOCK_SIZE - 1) / BLOCK_SIZE;
        let num_blocks = num_blocks_w * num_blocks_h;
        let mut flat_blocks = vec![0u8; num_blocks];
        let mut num_flat = 0;
        let mut plane_result = [0.0f64; BLOCK_SIZE_SQUARED];
        let mut block_result = [0.0f64; BLOCK_SIZE_SQUARED];
        let mut scores = vec![IndexAndScore::default(); num_blocks];

        for by in 0..num_blocks_h {
            for bx in 0..num_blocks_w {
                // Compute gradient covariance matrix.
                let mut gxx = 0f64;
                let mut gxy = 0f64;
                let mut gyy = 0f64;
                let mut var = 0f64;
                let mut mean = 0f64;

                self.extract_block(
                    plane,
                    bx * BLOCK_SIZE,
                    by * BLOCK_SIZE,
                    &mut plane_result,
                    &mut block_result,
                );
                for yi in 1..(BLOCK_SIZE - 1) {
                    for xi in 1..(BLOCK_SIZE - 1) {
                        // SAFETY: We know the size of `block_result` and that we cannot exceed the bounds of it
                        unsafe {
                            let result_ptr = block_result.as_ptr().add(yi * BLOCK_SIZE + xi);

                            let gx = (*result_ptr.add(1) - *result_ptr.sub(1)) / 2f64;
                            let gy =
                                (*result_ptr.add(BLOCK_SIZE) - *result_ptr.sub(BLOCK_SIZE)) / 2f64;
                            gxx += gx * gx;
                            gxy += gx * gy;
                            gyy += gy * gy;

                            let block_val = *result_ptr;
                            mean += block_val;
                            var += block_val * block_val;
                        }
                    }
                }
                let block_size_norm_factor = (BLOCK_SIZE - 2).pow(2) as f64;
                mean /= block_size_norm_factor;

                // Normalize gradients by block_size.
                gxx /= block_size_norm_factor;
                gxy /= block_size_norm_factor;
                gyy /= block_size_norm_factor;
                var = mean.mul_add(-mean, var / block_size_norm_factor);

                let trace = gxx + gyy;
                let det = gxx.mul_add(gyy, -gxy.powi(2));
                let e_sub = (trace.mul_add(trace, -4f64 * det)).max(0.).sqrt();
                let e1 = (trace + e_sub) / 2.0f64;
                let e2 = (trace - e_sub) / 2.0f64;
                // Spectral norm
                let norm = e1;
                let ratio = e1 / e2.max(1.0e-6_f64);
                let is_flat = trace < TRACE_THRESHOLD
                    && ratio < RATIO_THRESHOLD
                    && norm < NORM_THRESHOLD
                    && var > VAR_THRESHOLD;

                let sum_weights = NORM_WEIGHT.mul_add(
                    norm,
                    TRACE_WEIGHT.mul_add(
                        trace,
                        VAR_WEIGHT.mul_add(var, RATIO_WEIGHT.mul_add(ratio, OFFSET)),
                    ),
                );
                // clamp the value to [-25.0, 100.0] to prevent overflow
                let sum_weights = clamp(sum_weights, -25.0f64, 100.0f64);
                let score = (1.0f64 / (1.0f64 + (-sum_weights).exp())) as f32;
                // SAFETY: We know the size of `flat_blocks` and `scores` and that we cannot exceed the bounds of it
                unsafe {
                    let index = by * num_blocks_w + bx;
                    *flat_blocks.get_unchecked_mut(index) = if is_flat { 255 } else { 0 };
                    *scores.get_unchecked_mut(index) = IndexAndScore {
                        score: if var > VAR_THRESHOLD { score } else { 0f32 },
                        index,
                    };
                }
                if is_flat {
                    num_flat += 1;
                }
            }
        }

        scores.sort_unstable_by(|a, b| a.score.partial_cmp(&b.score).expect("Shouldn't be NaN"));
        // SAFETY: We know the size of `flat_blocks` and `scores` and that we cannot exceed the bounds of it
        unsafe {
            let top_nth_percentile = num_blocks * 90 / 100;
            let score_threshold = scores.get_unchecked(top_nth_percentile).score;
            for score in &scores {
                if score.score >= score_threshold {
                    let block_ref = flat_blocks.get_unchecked_mut(score.index);
                    if *block_ref == 0 {
                        num_flat += 1;
                    }
                    *block_ref |= 1;
                }
            }
        }

        (flat_blocks, num_flat)
    }

    fn extract_block(
        &self,
        plane: &Plane<u8>,
        offset_x: usize,
        offset_y: usize,
        plane_result: &mut [f64; BLOCK_SIZE_SQUARED],
        block_result: &mut [f64; BLOCK_SIZE_SQUARED],
    ) {
        let mut plane_coords = [0f64; LOW_POLY_NUM_PARAMS];
        let mut a_t_a_inv_b = [0f64; LOW_POLY_NUM_PARAMS];
        let plane_origin = plane.data_origin();

        for yi in 0..BLOCK_SIZE {
            let y = clamp(offset_y + yi, 0, plane.cfg.height - 1);
            for xi in 0..BLOCK_SIZE {
                let x = clamp(offset_x + xi, 0, plane.cfg.width - 1);
                // SAFETY: We know the bounds of the plane data and `block_result`
                // and do not exceed them.
                unsafe {
                    *block_result.get_unchecked_mut(yi * BLOCK_SIZE + xi) =
                        f64::from(*plane_origin.get_unchecked(y * plane.cfg.stride + x))
                            / BLOCK_NORMALIZATION;
                }
            }
        }

        multiply_mat(
            block_result,
            &self.a,
            &mut a_t_a_inv_b,
            1,
            BLOCK_SIZE_SQUARED,
            LOW_POLY_NUM_PARAMS,
        );
        multiply_mat(
            &self.a_t_a_inv,
            &a_t_a_inv_b,
            &mut plane_coords,
            LOW_POLY_NUM_PARAMS,
            LOW_POLY_NUM_PARAMS,
            1,
        );
        multiply_mat(
            &self.a,
            &plane_coords,
            plane_result,
            BLOCK_SIZE_SQUARED,
            LOW_POLY_NUM_PARAMS,
            1,
        );

        for (block_res, plane_res) in block_result.iter_mut().zip(plane_result.iter()) {
            *block_res -= *plane_res;
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct IndexAndScore {
    pub index: usize,
    pub score: f32,
}

/// Wrapper of data required to represent linear system of eqns and soln.
#[derive(Debug, Clone)]
struct EquationSystem {
    a: Vec<f64>,
    b: Vec<f64>,
    x: Vec<f64>,
    n: usize,
}

impl EquationSystem {
    #[must_use]
    pub fn new(n: usize) -> Self {
        Self {
            a: vec![0.0f64; n * n],
            b: vec![0.0f64; n],
            x: vec![0.0f64; n],
            n,
        }
    }

    pub fn solve(&mut self) -> bool {
        let n = self.n;
        let mut a = self.a.clone();
        let mut b = self.b.clone();

        linsolve(n, &mut a, self.n, &mut b, &mut self.x)
    }

    pub fn set_chroma_coefficient_fallback_solution(&mut self) {
        const TOLERANCE: f64 = 1.0e-6f64;
        let last = self.n - 1;
        // Set all of the AR coefficients to zero, but try to solve for correlation
        // with the luma channel
        self.x.fill(0f64);
        if self.a[last * self.n + last].abs() > TOLERANCE {
            self.x[last] = self.b[last] / self.a[last * self.n + last];
        }
    }

    pub fn copy_from(&mut self, other: &Self) {
        assert_eq!(self.n, other.n);
        self.a.copy_from_slice(&other.a);
        self.x.copy_from_slice(&other.x);
        self.b.copy_from_slice(&other.b);
    }

    pub fn clear(&mut self) {
        self.a.fill(0f64);
        self.b.fill(0f64);
        self.x.fill(0f64);
    }
}

impl Add<&EquationSystem> for EquationSystem {
    type Output = EquationSystem;

    fn add(self, addend: &EquationSystem) -> Self::Output {
        let mut dest = self.clone();
        let n = self.n;
        for i in 0..n {
            for j in 0..n {
                dest.a[i * n + j] += addend.a[i * n + j];
            }
            dest.b[i] += addend.b[i];
        }
        dest
    }
}

impl AddAssign<&EquationSystem> for EquationSystem {
    fn add_assign(&mut self, rhs: &EquationSystem) {
        *self = self.clone() + rhs;
    }
}

/// Representation of a piecewise linear curve
///
/// Holds n points as (x, y) pairs, that store the curve.
struct NoiseStrengthLut {
    points: Vec<[f64; 2]>,
}

impl NoiseStrengthLut {
    #[must_use]
    pub fn new(num_bins: usize) -> Self {
        assert!(num_bins > 0);
        Self {
            points: vec![[0f64; 2]; num_bins],
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct NoiseModel {
    combined_state: [NoiseModelState; 3],
    latest_state: [NoiseModelState; 3],
    n: usize,
    coords: Vec<[isize; 2]>,
}

impl NoiseModel {
    #[must_use]
    pub fn new() -> Self {
        let n = Self::num_coeffs();
        let combined_state = [
            NoiseModelState::new(n),
            NoiseModelState::new(n + 1),
            NoiseModelState::new(n + 1),
        ];
        let latest_state = [
            NoiseModelState::new(n),
            NoiseModelState::new(n + 1),
            NoiseModelState::new(n + 1),
        ];
        let mut coords = Vec::new();

        let neg_lag = -(NOISE_MODEL_LAG as isize);
        for y in neg_lag..=0 {
            let max_x = if y == 0 {
                -1isize
            } else {
                NOISE_MODEL_LAG as isize
            };
            for x in neg_lag..=max_x {
                coords.push([x, y]);
            }
        }
        assert!(n == coords.len());

        Self {
            combined_state,
            latest_state,
            n,
            coords,
        }
    }

    pub fn update(
        &mut self,
        source: &Frame<u8>,
        denoised: &Frame<u8>,
        flat_blocks: &[u8],
    ) -> NoiseStatus {
        let num_blocks_w = (source.planes[0].cfg.width + BLOCK_SIZE - 1) / BLOCK_SIZE;
        let num_blocks_h = (source.planes[0].cfg.height + BLOCK_SIZE - 1) / BLOCK_SIZE;
        let mut y_model_different = false;

        // Clear the latest equation system
        for i in 0..3 {
            self.latest_state[i].eqns.clear();
            self.latest_state[i].num_observations = 0;
            self.latest_state[i].strength_solver.clear();
        }

        // Check that we have enough flat blocks
        let num_blocks = flat_blocks.iter().filter(|b| **b > 0).count();
        if num_blocks <= 1 {
            return NoiseStatus::Error(anyhow!("Not enough flat blocks to update noise estimate"));
        }

        let frame_dims = (source.planes[0].cfg.width, source.planes[0].cfg.height);
        for channel in 0..3 {
            if source.planes[channel].data.is_empty() {
                // Monochrome source
                break;
            }
            let is_chroma = channel > 0;
            let alt_source = (channel > 0).then(|| &source.planes[0]);
            let alt_denoised = (channel > 0).then(|| &denoised.planes[0]);
            self.add_block_observations(
                channel,
                &source.planes[channel],
                &denoised.planes[channel],
                alt_source,
                alt_denoised,
                frame_dims,
                flat_blocks,
                num_blocks_w,
                num_blocks_h,
            );
            if !self.latest_state[channel].ar_equation_system_solve(is_chroma) {
                if is_chroma {
                    self.latest_state[channel]
                        .eqns
                        .set_chroma_coefficient_fallback_solution();
                } else {
                    return NoiseStatus::Error(anyhow!(
                        "Solving latest noise equation system failed on plane {}",
                        channel
                    ));
                }
            }
            self.add_noise_std_observations(
                channel,
                &source.planes[channel],
                &denoised.planes[channel],
                alt_source,
                frame_dims,
                flat_blocks,
                num_blocks_w,
                num_blocks_h,
            );
            if !self.latest_state[channel].strength_solver.solve() {
                return NoiseStatus::Error(anyhow!(
                    "Failed to solve strength solver for latest state"
                ));
            }

            // Check noise characteristics and return if error
            if channel == 0
                && self.combined_state[channel].strength_solver.num_equations > 0
                && self.is_different()
            {
                y_model_different = true;
            }

            if y_model_different {
                continue;
            }

            self.combined_state[channel].num_observations +=
                self.latest_state[channel].num_observations;
            self.combined_state[channel].eqns += &self.latest_state[channel].eqns;
            if !self.combined_state[channel].ar_equation_system_solve(is_chroma) {
                if is_chroma {
                    self.combined_state[channel]
                        .eqns
                        .set_chroma_coefficient_fallback_solution();
                } else {
                    return NoiseStatus::Error(anyhow!(
                        "Solving combined noise equation system failed on plane {}",
                        channel
                    ));
                }
            }

            self.combined_state[channel].strength_solver +=
                &self.latest_state[channel].strength_solver;

            if !self.combined_state[channel].strength_solver.solve() {
                return NoiseStatus::Error(anyhow!(
                    "Failed to solve strength solver for combined state"
                ));
            };
        }

        if y_model_different {
            return NoiseStatus::DifferentType;
        }

        NoiseStatus::Ok
    }

    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub fn get_grain_parameters(&self, start_ts: u64, end_ts: u64) -> GrainTableSegment {
        // Both the domain and the range of the scaling functions in the film_grain
        // are normalized to 8-bit (e.g., they are implicitly scaled during grain
        // synthesis).
        let scaling_points_y = self.combined_state[0]
            .strength_solver
            .fit_piecewise(NUM_Y_POINTS)
            .points;
        let scaling_points_cb = self.combined_state[1]
            .strength_solver
            .fit_piecewise(NUM_UV_POINTS)
            .points;
        let scaling_points_cr = self.combined_state[2]
            .strength_solver
            .fit_piecewise(NUM_UV_POINTS)
            .points;

        let mut max_scaling_value: f64 = 1.0e-4f64;
        for p in scaling_points_y
            .iter()
            .chain(scaling_points_cb.iter())
            .chain(scaling_points_cr.iter())
            .map(|p| p[1])
        {
            if p > max_scaling_value {
                max_scaling_value = p;
            }
        }

        // Scaling_shift values are in the range [8,11]
        let max_scaling_value_log2 =
            clamp((max_scaling_value.log2() + 1f64).floor() as u8, 2u8, 5u8);
        let scale_factor = f64::from(1u32 << (8u8 - max_scaling_value_log2));
        let map_scaling_point = |p: [f64; 2]| {
            [
                (p[0] + 0.5f64) as u8,
                clamp(scale_factor.mul_add(p[1], 0.5f64) as i32, 0i32, 255i32) as u8,
            ]
        };

        let scaling_points_y: ArrayVec<_, NUM_Y_POINTS> = scaling_points_y
            .into_iter()
            .map(map_scaling_point)
            .collect();
        let scaling_points_cb: ArrayVec<_, NUM_UV_POINTS> = scaling_points_cb
            .into_iter()
            .map(map_scaling_point)
            .collect();
        let scaling_points_cr: ArrayVec<_, NUM_UV_POINTS> = scaling_points_cr
            .into_iter()
            .map(map_scaling_point)
            .collect();

        // Convert the ar_coeffs into 8-bit values
        let n_coeff = self.combined_state[0].eqns.n;
        let mut max_coeff = 1.0e-4f64;
        let mut min_coeff = 1.0e-4f64;
        let mut y_corr = [0f64; 2];
        let mut avg_luma_strength = 0f64;
        for c in 0..3 {
            let eqns = &self.combined_state[c].eqns;
            for i in 0..n_coeff {
                if eqns.x[i] > max_coeff {
                    max_coeff = eqns.x[i];
                }
                if eqns.x[i] < min_coeff {
                    min_coeff = eqns.x[i];
                }
            }

            // Since the correlation between luma/chroma was computed in an already
            // scaled space, we adjust it in the un-scaled space.
            let solver = &self.combined_state[c].strength_solver;
            // Compute a weighted average of the strength for the channel.
            let mut average_strength = 0f64;
            let mut total_weight = 0f64;
            for i in 0..solver.eqns.n {
                let mut w = 0f64;
                for j in 0..solver.eqns.n {
                    w += solver.eqns.a[i * solver.eqns.n + j];
                }
                w = w.sqrt();
                average_strength += solver.eqns.x[i] * w;
                total_weight += w;
            }
            if total_weight.abs() < f64::EPSILON {
                average_strength = 1f64;
            } else {
                average_strength /= total_weight;
            }
            if c == 0 {
                avg_luma_strength = average_strength;
            } else {
                y_corr[c - 1] = avg_luma_strength * eqns.x[n_coeff] / average_strength;
                max_coeff = max_coeff.max(y_corr[c - 1]);
                min_coeff = min_coeff.min(y_corr[c - 1]);
            }
        }

        // Shift value: AR coeffs range (values 6-9)
        // 6: [-2, 2),  7: [-1, 1), 8: [-0.5, 0.5), 9: [-0.25, 0.25)
        let ar_coeff_shift = clamp(
            7i32 - (1.0f64 + max_coeff.log2().floor()).max((-min_coeff).log2().ceil()) as i32,
            6i32,
            9i32,
        ) as u8;
        let scale_ar_coeff = f64::from(1u16 << ar_coeff_shift);
        let ar_coeffs_y = self.get_ar_coeffs_y(n_coeff, scale_ar_coeff);
        let ar_coeffs_cb = self.get_ar_coeffs_uv(1, n_coeff, scale_ar_coeff, y_corr);
        let ar_coeffs_cr = self.get_ar_coeffs_uv(2, n_coeff, scale_ar_coeff, y_corr);

        GrainTableSegment {
            random_seed: if start_ts == 0 { DEFAULT_GRAIN_SEED } else { 0 },
            start_time: start_ts,
            end_time: end_ts,
            ar_coeff_lag: NOISE_MODEL_LAG as u8,
            scaling_points_y,
            scaling_points_cb,
            scaling_points_cr,
            scaling_shift: 5 + (8 - max_scaling_value_log2),
            ar_coeff_shift,
            ar_coeffs_y,
            ar_coeffs_cb,
            ar_coeffs_cr,
            // At the moment, the noise modeling code assumes that the chroma scaling
            // functions are a function of luma.
            cb_mult: 128,
            cb_luma_mult: 192,
            cb_offset: 256,
            cr_mult: 128,
            cr_luma_mult: 192,
            cr_offset: 256,
            chroma_scaling_from_luma: false,
            grain_scale_shift: 0,
            overlap_flag: true,
        }
    }

    pub fn save_latest(&mut self) {
        for c in 0..3 {
            let latest_state = &self.latest_state[c];
            let combined_state = &mut self.combined_state[c];
            combined_state.eqns.copy_from(&latest_state.eqns);
            combined_state
                .strength_solver
                .eqns
                .copy_from(&latest_state.strength_solver.eqns);
            combined_state.strength_solver.num_equations =
                latest_state.strength_solver.num_equations;
            combined_state.num_observations = latest_state.num_observations;
            combined_state.ar_gain = latest_state.ar_gain;
        }
    }

    #[must_use]
    const fn num_coeffs() -> usize {
        let n = 2 * NOISE_MODEL_LAG + 1;
        (n * n) / 2
    }

    #[must_use]
    fn get_ar_coeffs_y(&self, n_coeff: usize, scale_ar_coeff: f64) -> ArrayVec<i8, NUM_Y_COEFFS> {
        assert!(n_coeff <= NUM_Y_COEFFS);
        let mut coeffs = ArrayVec::new();
        let eqns = &self.combined_state[0].eqns;
        for i in 0..n_coeff {
            coeffs.push(clamp((scale_ar_coeff * eqns.x[i]).round() as i32, -128i32, 127i32) as i8);
        }
        coeffs
    }

    #[must_use]
    fn get_ar_coeffs_uv(
        &self,
        channel: usize,
        n_coeff: usize,
        scale_ar_coeff: f64,
        y_corr: [f64; 2],
    ) -> ArrayVec<i8, NUM_UV_COEFFS> {
        assert!(n_coeff <= NUM_Y_COEFFS);
        let mut coeffs = ArrayVec::new();
        let eqns = &self.combined_state[channel].eqns;
        for i in 0..n_coeff {
            coeffs.push(clamp((scale_ar_coeff * eqns.x[i]).round() as i32, -128i32, 127i32) as i8);
        }
        coeffs.push(clamp(
            (scale_ar_coeff * y_corr[channel - 1]).round() as i32,
            -128i32,
            127i32,
        ) as i8);
        coeffs
    }

    // Return true if the noise estimate appears to be different from the combined
    // (multi-frame) estimate. The difference is measured by checking whether the
    // AR coefficients have diverged (using a threshold on normalized cross
    // correlation), or whether the noise strength has changed.
    #[must_use]
    fn is_different(&self) -> bool {
        const COEFF_THRESHOLD: f64 = 0.9f64;
        const STRENGTH_THRESHOLD: f64 = 0.005f64;

        let latest = &self.latest_state[0];
        let combined = &self.combined_state[0];
        let corr = normalized_cross_correlation(&latest.eqns.x, &combined.eqns.x, combined.eqns.n);
        if corr < COEFF_THRESHOLD {
            return true;
        }

        let dx = 1.0f64 / latest.strength_solver.num_bins as f64;
        let latest_eqns = &latest.strength_solver.eqns;
        let combined_eqns = &combined.strength_solver.eqns;
        let mut diff = 0.0f64;
        let mut total_weight = 0.0f64;
        for j in 0..latest_eqns.n {
            let mut weight = 0.0f64;
            for i in 0..latest_eqns.n {
                weight += latest_eqns.a[i * latest_eqns.n + j];
            }
            weight = weight.sqrt();
            diff += weight * (latest_eqns.x[j] - combined_eqns.x[j]).abs();
            total_weight += weight;
        }

        diff * dx / total_weight > STRENGTH_THRESHOLD
    }

    #[allow(clippy::too_many_arguments)]
    fn add_block_observations(
        &mut self,
        channel: usize,
        source: &Plane<u8>,
        denoised: &Plane<u8>,
        alt_source: Option<&Plane<u8>>,
        alt_denoised: Option<&Plane<u8>>,
        frame_dims: (usize, usize),
        flat_blocks: &[u8],
        num_blocks_w: usize,
        num_blocks_h: usize,
    ) {
        let num_coords = self.n;
        let state = &mut self.latest_state[channel];
        let a = &mut state.eqns.a;
        let b = &mut state.eqns.b;
        let mut buffer = vec![0f64; num_coords + 1].into_boxed_slice();
        let n = state.eqns.n;
        let block_w = BLOCK_SIZE >> source.cfg.xdec;
        let block_h = BLOCK_SIZE >> source.cfg.ydec;

        let dec = (source.cfg.xdec, source.cfg.ydec);
        let stride = source.cfg.stride;
        let source_origin = source.data_origin();
        let denoised_origin = denoised.data_origin();
        let alt_stride = alt_source.map_or(0, |s| s.cfg.stride);
        let alt_source_origin = alt_source.map(|s| s.data_origin());
        let alt_denoised_origin = alt_denoised.map(|s| s.data_origin());

        for by in 0..num_blocks_h {
            let y_o = by * block_h;
            for bx in 0..num_blocks_w {
                // SAFETY: We know the indexes we provide do not overflow the data bounds
                unsafe {
                    let flat_block_ptr = flat_blocks.as_ptr().add(by * num_blocks_w + bx);
                    let x_o = bx * block_w;
                    if *flat_block_ptr == 0 {
                        continue;
                    }
                    let y_start = if by > 0 && *flat_block_ptr.sub(num_blocks_w) > 0 {
                        0
                    } else {
                        NOISE_MODEL_LAG
                    };
                    let x_start = if bx > 0 && *flat_block_ptr.sub(1) > 0 {
                        0
                    } else {
                        NOISE_MODEL_LAG
                    };
                    let y_end = ((frame_dims.1 >> dec.1) - by * block_h).min(block_h);
                    let x_end = ((frame_dims.0 >> dec.0) - bx * block_w - NOISE_MODEL_LAG).min(
                        if bx + 1 < num_blocks_w && *flat_block_ptr.add(1) > 0 {
                            block_w
                        } else {
                            block_w - NOISE_MODEL_LAG
                        },
                    );
                    for y in y_start..y_end {
                        for x in x_start..x_end {
                            let val = extract_ar_row(
                                &self.coords,
                                num_coords,
                                source_origin,
                                denoised_origin,
                                stride,
                                dec,
                                alt_source_origin,
                                alt_denoised_origin,
                                alt_stride,
                                x + x_o,
                                y + y_o,
                                &mut buffer,
                            );
                            for i in 0..n {
                                for j in 0..n {
                                    *a.get_unchecked_mut(i * n + j) += (*buffer.get_unchecked(i)
                                        * *buffer.get_unchecked(j))
                                        / BLOCK_NORMALIZATION.powi(2);
                                }
                                *b.get_unchecked_mut(i) +=
                                    (*buffer.get_unchecked(i) * val) / BLOCK_NORMALIZATION.powi(2);
                            }
                            state.num_observations += 1;
                        }
                    }
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn add_noise_std_observations(
        &mut self,
        channel: usize,
        source: &Plane<u8>,
        denoised: &Plane<u8>,
        alt_source: Option<&Plane<u8>>,
        frame_dims: (usize, usize),
        flat_blocks: &[u8],
        num_blocks_w: usize,
        num_blocks_h: usize,
    ) {
        let coeffs = &self.latest_state[channel].eqns.x;
        let num_coords = self.n;
        let luma_gain = self.latest_state[0].ar_gain;
        let noise_gain = self.latest_state[channel].ar_gain;
        let block_w = BLOCK_SIZE >> source.cfg.xdec;
        let block_h = BLOCK_SIZE >> source.cfg.ydec;

        for by in 0..num_blocks_h {
            let y_o = by * block_h;
            for bx in 0..num_blocks_w {
                let x_o = bx * block_w;
                if flat_blocks[by * num_blocks_w + bx] == 0 {
                    continue;
                }
                let num_samples_h = ((frame_dims.1 >> source.cfg.ydec) - by * block_h).min(block_h);
                let num_samples_w = ((frame_dims.0 >> source.cfg.xdec) - bx * block_w).min(block_w);
                // Make sure that we have a reasonable amount of samples to consider the
                // block
                if num_samples_w * num_samples_h > BLOCK_SIZE {
                    let block_mean = get_block_mean(
                        alt_source.unwrap_or(source),
                        frame_dims,
                        x_o << source.cfg.xdec,
                        y_o << source.cfg.ydec,
                    );
                    let noise_var = get_noise_var(
                        source,
                        denoised,
                        (
                            frame_dims.0 >> source.cfg.xdec,
                            frame_dims.1 >> source.cfg.ydec,
                        ),
                        x_o,
                        y_o,
                        block_w,
                        block_h,
                    );
                    // We want to remove the part of the noise that came from being
                    // correlated with luma. Note that the noise solver for luma must
                    // have already been run.
                    let luma_strength = if channel > 0 {
                        luma_gain * self.latest_state[0].strength_solver.get_value(block_mean)
                    } else {
                        0f64
                    };
                    let corr = if channel > 0 {
                        coeffs[num_coords]
                    } else {
                        0f64
                    };
                    // Chroma noise:
                    //    N(0, noise_var) = N(0, uncorr_var) + corr * N(0, luma_strength^2)
                    // The uncorrelated component:
                    //   uncorr_var = noise_var - (corr * luma_strength)^2
                    // But don't allow fully correlated noise (hence the max), since the
                    // synthesis cannot model it.
                    let uncorr_std = (noise_var / 16f64)
                        .max((corr * luma_strength).mul_add(-(corr * luma_strength), noise_var))
                        .sqrt();
                    let adjusted_strength = uncorr_std / noise_gain;
                    self.latest_state[channel]
                        .strength_solver
                        .add_measurement(block_mean, adjusted_strength);
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct NoiseModelState {
    eqns: EquationSystem,
    ar_gain: f64,
    num_observations: usize,
    strength_solver: StrengthSolver,
}

impl NoiseModelState {
    #[must_use]
    pub fn new(n: usize) -> Self {
        const NUM_BINS: usize = 20;

        Self {
            eqns: EquationSystem::new(n),
            ar_gain: 1.0f64,
            num_observations: 0usize,
            strength_solver: StrengthSolver::new(NUM_BINS),
        }
    }

    pub fn ar_equation_system_solve(&mut self, is_chroma: bool) -> bool {
        let ret = self.eqns.solve();
        self.ar_gain = 1.0f64;
        if !ret {
            return ret;
        }

        // Update the AR gain from the equation system as it will be used to fit
        // the noise strength as a function of intensity.  In the Yule-Walker
        // equations, the diagonal should be the variance of the correlated noise.
        // In the case of the least squares estimate, there will be some variability
        // in the diagonal. So use the mean of the diagonal as the estimate of
        // overall variance (this works for least squares or Yule-Walker formulation).
        let mut var = 0f64;
        let n_adjusted = self.eqns.n - usize::from(is_chroma);
        for i in 0..n_adjusted {
            var += self.eqns.a[i * self.eqns.n + i] / self.num_observations as f64;
        }
        var /= n_adjusted as f64;

        // Keep track of E(Y^2) = <b, x> + E(X^2)
        // In the case that we are using chroma and have an estimate of correlation
        // with luma we adjust that estimate slightly to remove the correlated bits by
        // subtracting out the last column of a scaled by our correlation estimate
        // from b. E(y^2) = <b - A(:, end)*x(end), x>
        let mut sum_covar = 0f64;
        for i in 0..n_adjusted {
            let mut bi = self.eqns.b[i];
            if is_chroma {
                bi -= self.eqns.a[i * self.eqns.n + n_adjusted] * self.eqns.x[n_adjusted];
            }
            sum_covar += (bi * self.eqns.x[i]) / self.num_observations as f64;
        }

        // Now, get an estimate of the variance of uncorrelated noise signal and use
        // it to determine the gain of the AR filter.
        let noise_var = (var - sum_covar).max(1e-6f64);
        self.ar_gain = 1f64.max((var / noise_var).max(1e-6f64).sqrt());
        ret
    }
}

#[derive(Debug, Clone)]
struct StrengthSolver {
    eqns: EquationSystem,
    num_bins: usize,
    num_equations: usize,
    total: f64,
}

impl StrengthSolver {
    #[must_use]
    pub fn new(num_bins: usize) -> Self {
        Self {
            eqns: EquationSystem::new(num_bins),
            num_bins,
            num_equations: 0usize,
            total: 0f64,
        }
    }

    pub fn add_measurement(&mut self, block_mean: f64, noise_std: f64) {
        let bin = self.get_bin_index(block_mean);
        let bin_i0 = bin.floor() as usize;
        let bin_i1 = (self.num_bins - 1).min(bin_i0 + 1);
        let a = bin - bin_i0 as f64;
        let n = self.num_bins;
        let eqns = &mut self.eqns;
        eqns.a[bin_i0 * n + bin_i0] += (1f64 - a).powi(2);
        eqns.a[bin_i1 * n + bin_i0] += a * (1f64 - a);
        eqns.a[bin_i1 * n + bin_i1] += a.powi(2);
        eqns.a[bin_i0 * n + bin_i1] += (1f64 - a) * a;
        eqns.b[bin_i0] += (1f64 - a) * noise_std;
        eqns.b[bin_i1] += a * noise_std;
        self.total += noise_std;
        self.num_equations += 1;
    }

    pub fn solve(&mut self) -> bool {
        // Add regularization proportional to the number of constraints
        let n = self.num_bins;
        let alpha = 2f64 * self.num_equations as f64 / n as f64;

        // Do this in a non-destructive manner so it is not confusing to the caller
        let old_a = self.eqns.a.clone();
        for i in 0..n {
            let i_lo = i.saturating_sub(1);
            let i_hi = (n - 1).min(i + 1);
            self.eqns.a[i * n + i_lo] -= alpha;
            self.eqns.a[i * n + i] += 2f64 * alpha;
            self.eqns.a[i * n + i_hi] -= alpha;
        }

        // Small regularization to give average noise strength
        let mean = self.total / self.num_equations as f64;
        for i in 0..n {
            self.eqns.a[i * n + i] += 1f64 / 8192f64;
            self.eqns.b[i] += mean / 8192f64;
        }
        let result = self.eqns.solve();
        self.eqns.a = old_a;
        result
    }

    #[must_use]
    pub fn fit_piecewise(&self, max_output_points: usize) -> NoiseStrengthLut {
        const TOLERANCE: f64 = 0.00625f64;

        let mut lut = NoiseStrengthLut::new(self.num_bins);
        for i in 0..self.num_bins {
            lut.points[i][0] = self.get_center(i);
            lut.points[i][1] = self.eqns.x[i];
        }

        let mut residual = vec![0.0f64; self.num_bins];
        self.update_piecewise_linear_residual(&lut, &mut residual, 0, self.num_bins);

        // Greedily remove points if there are too many or if it doesn't hurt local
        // approximation (never remove the end points)
        while lut.points.len() > 2 {
            let mut min_index = 1usize;
            for j in 1..(lut.points.len() - 1) {
                if residual[j] < residual[min_index] {
                    min_index = j;
                }
            }
            let dx = lut.points[min_index + 1][0] - lut.points[min_index - 1][0];
            let avg_residual = residual[min_index] / dx;
            if lut.points.len() <= max_output_points && avg_residual > TOLERANCE {
                break;
            }

            lut.points.remove(min_index);
            self.update_piecewise_linear_residual(
                &lut,
                &mut residual,
                min_index - 1,
                min_index + 1,
            );
        }

        lut
    }

    #[must_use]
    pub fn get_value(&self, x: f64) -> f64 {
        let bin = self.get_bin_index(x);
        let bin_i0 = bin.floor() as usize;
        let bin_i1 = (self.num_bins - 1).min(bin_i0 + 1);
        let a = bin - bin_i0 as f64;
        (1f64 - a).mul_add(self.eqns.x[bin_i0], a * self.eqns.x[bin_i1])
    }

    pub fn clear(&mut self) {
        self.eqns.clear();
        self.num_equations = 0;
        self.total = 0f64;
    }

    #[must_use]
    fn get_bin_index(&self, value: f64) -> f64 {
        let max = 255f64;
        let val = clamp(value, 0f64, max);
        (self.num_bins - 1) as f64 * val / max
    }

    fn update_piecewise_linear_residual(
        &self,
        lut: &NoiseStrengthLut,
        residual: &mut [f64],
        start: usize,
        end: usize,
    ) {
        let dx = 255f64 / self.num_bins as f64;
        #[allow(clippy::needless_range_loop)]
        for i in start.max(1)..end.min(lut.points.len() - 1) {
            let lower = 0usize.max(self.get_bin_index(lut.points[i - 1][0]).floor() as usize);
            let upper =
                (self.num_bins - 1).min(self.get_bin_index(lut.points[i + 1][0]).ceil() as usize);
            let mut r = 0f64;
            for j in lower..=upper {
                let x = self.get_center(j);
                if x < lut.points[i - 1][0] || x >= lut.points[i + 1][0] {
                    continue;
                }

                let y = self.eqns.x[j];
                let a = (x - lut.points[i - 1][0]) / (lut.points[i + 1][0] - lut.points[i - 1][0]);
                let estimate_y = lut.points[i - 1][1].mul_add(1f64 - a, lut.points[i + 1][1] * a);
                r += (y - estimate_y).abs();
            }
            residual[i] = r * dx;
        }
    }

    #[must_use]
    fn get_center(&self, i: usize) -> f64 {
        let range = 255f64;
        let n = self.num_bins;
        i as f64 / (n - 1) as f64 * range
    }
}

impl Add<&StrengthSolver> for StrengthSolver {
    type Output = StrengthSolver;

    fn add(self, addend: &StrengthSolver) -> Self::Output {
        let mut dest = self;
        dest.eqns += &addend.eqns;
        dest.num_equations += addend.num_equations;
        dest.total += addend.total;
        dest
    }
}

impl AddAssign<&StrengthSolver> for StrengthSolver {
    fn add_assign(&mut self, rhs: &StrengthSolver) {
        *self = self.clone() + rhs;
    }
}
