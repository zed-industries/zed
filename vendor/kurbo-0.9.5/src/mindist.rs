// Copyright 2021 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Minimum distance between two Bézier curves
//!
//! This implements the algorithm in "Computing the minimum distance between
//! two Bézier curves", Chen et al., *Journal of Computational and Applied
//! Mathematics* 229(2009), 294-301

use crate::Vec2;
use core::cmp::Ordering;

#[cfg(not(feature = "std"))]
use crate::common::FloatFuncs;

pub(crate) fn min_dist_param(
    bez1: &[Vec2],
    bez2: &[Vec2],
    u: (f64, f64),
    v: (f64, f64),
    epsilon: f64,
    best_alpha: Option<f64>,
) -> (f64, f64, f64) {
    assert!(!bez1.is_empty() && !bez2.is_empty());
    let n = bez1.len() - 1;
    let m = bez2.len() - 1;
    let (umin, umax) = u;
    let (vmin, vmax) = v;
    let umid = (umin + umax) / 2.0;
    let vmid = (vmin + vmax) / 2.0;
    let svalues: [(f64, f64, f64); 4] = [
        (S(umin, vmin, bez1, bez2), umin, vmin),
        (S(umin, vmax, bez1, bez2), umin, vmax),
        (S(umax, vmin, bez1, bez2), umax, vmin),
        (S(umax, vmax, bez1, bez2), umax, vmax),
    ];
    let alpha: f64 = svalues.iter().map(|(a, _, _)| *a).reduce(f64::min).unwrap();
    if let Some(best) = best_alpha {
        if alpha > best {
            return (alpha, umid, vmid);
        }
    }

    if (umax - umin).abs() < epsilon || (vmax - vmin).abs() < epsilon {
        return (alpha, umid, vmid);
    }

    // Property one: D(r>k) > alpha
    let mut is_outside = true;
    let mut min_drk = None;
    let mut min_ij = None;
    for r in 0..(2 * n) {
        for k in 0..(2 * m) {
            let d_rk = D_rk(r, k, bez1, bez2);
            if d_rk < alpha {
                is_outside = false;
            }
            if min_drk.is_none() || Some(d_rk) < min_drk {
                min_drk = Some(d_rk);
                min_ij = Some((r, k));
            }
        }
    }
    if is_outside {
        return (alpha, umid, vmid);
    }

    // Property two: boundary check
    let mut at_boundary0on_bez1 = true;
    let mut at_boundary1on_bez1 = true;
    let mut at_boundary0on_bez2 = true;
    let mut at_boundary1on_bez2 = true;
    for i in 0..2 * n {
        for j in 0..2 * m {
            let dij = D_rk(i, j, bez1, bez2);
            let dkj = D_rk(0, j, bez1, bez2);
            if dij < dkj {
                at_boundary0on_bez1 = false
            }
            let dkj = D_rk(2 * n, j, bez1, bez2);
            if dij < dkj {
                at_boundary1on_bez1 = false
            }
            let dkj = D_rk(i, 0, bez1, bez2);
            if dij < dkj {
                at_boundary0on_bez2 = false
            }
            let dkj = D_rk(i, 2 * n, bez1, bez2);
            if dij < dkj {
                at_boundary1on_bez2 = false
            }
        }
    }
    if at_boundary0on_bez1 && at_boundary0on_bez2 {
        return svalues[0];
    }
    if at_boundary0on_bez1 && at_boundary1on_bez2 {
        return svalues[1];
    }
    if at_boundary1on_bez1 && at_boundary0on_bez2 {
        return svalues[2];
    }
    if at_boundary1on_bez1 && at_boundary1on_bez2 {
        return svalues[3];
    }

    let (min_i, min_j) = min_ij.unwrap();
    let new_umid = umin + (umax - umin) * (min_i as f64 / (2 * n) as f64);
    let new_vmid = vmin + (vmax - vmin) * (min_j as f64 / (2 * m) as f64);

    // Subdivide
    let results = [
        min_dist_param(
            bez1,
            bez2,
            (umin, new_umid),
            (vmin, new_vmid),
            epsilon,
            Some(alpha),
        ),
        min_dist_param(
            bez1,
            bez2,
            (umin, new_umid),
            (new_vmid, vmax),
            epsilon,
            Some(alpha),
        ),
        min_dist_param(
            bez1,
            bez2,
            (new_umid, umax),
            (vmin, new_vmid),
            epsilon,
            Some(alpha),
        ),
        min_dist_param(
            bez1,
            bez2,
            (new_umid, umax),
            (new_vmid, vmax),
            epsilon,
            Some(alpha),
        ),
    ];

    *results
        .iter()
        .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Less))
        .unwrap()
}

// $ S(u,v)=\sum_{r=0}^{2n} \sum _{k=0}^{2m} D_{r,k} B_{2n}^r(u) B_{2m}^k(v) $

#[allow(non_snake_case)]
fn S(u: f64, v: f64, bez1: &[Vec2], bez2: &[Vec2]) -> f64 {
    let n = bez1.len() - 1;
    let m = bez2.len() - 1;
    let mut summand = 0.0;
    for r in 0..=2 * n {
        for k in 0..=2 * m {
            summand +=
                D_rk(r, k, bez1, bez2) * basis_function(2 * n, r, u) * basis_function(2 * m, k, v)
        }
    }
    summand
}

// $ C_{r,k} = ( \sum_{i=\theta}^\upsilon P_i C_n^i C_n^{r-i} / C_{2n}^r ) \cdot (\sum_{j=\sigma}^\varsigma Q_j C_m^j C_m^{k-j} / C_{2m}^k ) $
#[allow(non_snake_case)]
fn C_rk(r: usize, k: usize, bez1: &[Vec2], bez2: &[Vec2]) -> f64 {
    let n = bez1.len() - 1;
    let upsilon = r.min(n);
    let theta = r - n.min(r);
    let mut left: Vec2 = Vec2::ZERO;
    for (i, &item) in bez1.iter().enumerate().take(upsilon + 1).skip(theta) {
        left += item * (choose(n, i) * choose(n, r - i)) as f64 / (choose(2 * n, r) as f64);
    }

    let m = bez2.len() - 1;
    let varsigma = k.min(m);
    let sigma = k - m.min(k);
    let mut right: Vec2 = Vec2::ZERO;
    for (j, &item) in bez2.iter().enumerate().take(varsigma + 1).skip(sigma) {
        right += item * (choose(m, j) * choose(m, k - j)) as f64 / (choose(2 * m, k) as f64);
    }
    left.dot(right)
}

// $ A_r=\sum _{i=\theta} ^\upsilon (P_i \cdot P_{r-i}) C_n^i C_n^{r-i} / C_{2n}^r $
// ($ B_k $ is just the same as $ A_r $ but for the other curve.)

#[allow(non_snake_case)]
fn A_r(r: usize, p: &[Vec2]) -> f64 {
    let n = p.len() - 1;
    let upsilon = r.min(n);
    let theta = r - n.min(r);
    (theta..=upsilon)
        .map(|i| {
            let dot = p[i].dot(p[r - i]); // These are bounds checked by the sum limits
            let factor = (choose(n, i) * choose(n, r - i)) as f64 / (choose(2 * n, r) as f64);
            dot * factor
        })
        .sum()
}

#[allow(non_snake_case)]
fn D_rk(r: usize, k: usize, bez1: &[Vec2], bez2: &[Vec2]) -> f64 {
    // In the paper, B_k is used for the second factor, but it's the same thing
    A_r(r, bez1) + A_r(k, bez2) - 2.0 * C_rk(r, k, bez1, bez2)
}

// Bezier basis function
fn basis_function(n: usize, i: usize, u: f64) -> f64 {
    choose(n, i) as f64 * (1.0 - u).powi((n - i) as i32) * u.powi(i as i32)
}

// Binomial co-efficient, but returning zeros for values outside of domain
fn choose(n: usize, k: usize) -> u32 {
    let mut n = n;
    if k > n {
        return 0;
    }
    let mut p = 1;
    for i in 1..=(n - k) {
        p *= n;
        p /= i;
        n -= 1;
    }
    p as u32
}

#[cfg(test)]
mod tests {
    use crate::mindist::A_r;
    use crate::mindist::{choose, D_rk};
    use crate::{CubicBez, Line, PathSeg, Vec2};

    #[test]
    fn test_choose() {
        assert_eq!(choose(6, 0), 1);
        assert_eq!(choose(6, 1), 6);
        assert_eq!(choose(6, 2), 15);
    }

    #[test]
    fn test_d_rk() {
        let bez1 = vec![
            Vec2::new(129.0, 139.0),
            Vec2::new(190.0, 139.0),
            Vec2::new(201.0, 364.0),
            Vec2::new(90.0, 364.0),
        ];
        let bez2 = vec![
            Vec2::new(309.0, 159.0),
            Vec2::new(178.0, 159.0),
            Vec2::new(215.0, 408.0),
            Vec2::new(309.0, 408.0),
        ];
        let b = A_r(1, &bez2);
        assert!((b - 80283.0).abs() < 0.005, "B_1(Q)={b:?}");
        let d = D_rk(0, 1, &bez1, &bez2);
        assert!((d - 9220.0).abs() < 0.005, "D={d:?}");
    }

    #[test]
    fn test_mindist() {
        let bez1 = PathSeg::Cubic(CubicBez::new(
            (129.0, 139.0),
            (190.0, 139.0),
            (201.0, 364.0),
            (90.0, 364.0),
        ));
        let bez2 = PathSeg::Cubic(CubicBez::new(
            (309.0, 159.0),
            (178.0, 159.0),
            (215.0, 408.0),
            (309.0, 408.0),
        ));
        let mindist = bez1.min_dist(bez2, 0.001);
        assert!((mindist.distance - 50.9966).abs() < 0.5);
    }

    #[test]
    fn test_overflow() {
        let bez1 = PathSeg::Cubic(CubicBez::new(
            (232.0, 126.0),
            (134.0, 126.0),
            (139.0, 232.0),
            (141.0, 301.0),
        ));
        let bez2 = PathSeg::Line(Line::new((359.0, 416.0), (367.0, 755.0)));
        let mindist = bez1.min_dist(bez2, 0.001);
        assert!((mindist.distance - 246.4731222669117).abs() < 0.5);
    }

    #[test]
    fn test_out_of_order() {
        let bez1 = PathSeg::Cubic(CubicBez::new(
            (287.0, 182.0),
            (346.0, 277.0),
            (356.0, 299.0),
            (359.0, 416.0),
        ));
        let bez2 = PathSeg::Line(Line::new((141.0, 301.0), (152.0, 709.0)));
        let mindist1 = bez1.min_dist(bez2, 0.5);
        let mindist2 = bez2.min_dist(bez1, 0.5);
        assert!((mindist1.distance - mindist2.distance).abs() < 0.5);
    }
}
