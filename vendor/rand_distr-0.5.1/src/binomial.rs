// Copyright 2018 Developers of the Rand project.
// Copyright 2016-2017 The Rust Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The binomial distribution `Binomial(n, p)`.

use crate::{Distribution, Uniform};
use core::cmp::Ordering;
use core::fmt;
#[allow(unused_imports)]
use num_traits::Float;
use rand::Rng;

/// The [binomial distribution](https://en.wikipedia.org/wiki/Binomial_distribution) `Binomial(n, p)`.
///
/// The binomial distribution is a discrete probability distribution
/// which describes the probability of seeing `k` successes in `n`
/// independent trials, each of which has success probability `p`.
///
/// # Density function
///
/// `f(k) = n!/(k! (n-k)!) p^k (1-p)^(n-k)` for `k >= 0`.
///
/// # Plot
///
/// The following plot of the binomial distribution illustrates the
/// probability of `k` successes out of `n = 10` trials with `p = 0.2`
/// and `p = 0.6` for `0 <= k <= n`.
///
/// ![Binomial distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/binomial.svg)
///
/// # Example
///
/// ```
/// use rand_distr::{Binomial, Distribution};
///
/// let bin = Binomial::new(20, 0.3).unwrap();
/// let v = bin.sample(&mut rand::rng());
/// println!("{} is from a binomial distribution", v);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Binomial {
    method: Method,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum Method {
    Binv(Binv, bool),
    Btpe(Btpe, bool),
    Poisson(crate::poisson::KnuthMethod<f64>),
    Constant(u64),
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct Binv {
    r: f64,
    s: f64,
    a: f64,
    n: u64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct Btpe {
    n: u64,
    p: f64,
    m: i64,
    p1: f64,
}

/// Error type returned from [`Binomial::new`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
// Marked non_exhaustive to allow a new error code in the solution to #1378.
#[non_exhaustive]
pub enum Error {
    /// `p < 0` or `nan`.
    ProbabilityTooSmall,
    /// `p > 1`.
    ProbabilityTooLarge,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Error::ProbabilityTooSmall => "p < 0 or is NaN in binomial distribution",
            Error::ProbabilityTooLarge => "p > 1 in binomial distribution",
        })
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl Binomial {
    /// Construct a new `Binomial` with the given shape parameters `n` (number
    /// of trials) and `p` (probability of success).
    pub fn new(n: u64, p: f64) -> Result<Binomial, Error> {
        if !(p >= 0.0) {
            return Err(Error::ProbabilityTooSmall);
        }
        if !(p <= 1.0) {
            return Err(Error::ProbabilityTooLarge);
        }

        if p == 0.0 {
            return Ok(Binomial {
                method: Method::Constant(0),
            });
        }

        if p == 1.0 {
            return Ok(Binomial {
                method: Method::Constant(n),
            });
        }

        // The binomial distribution is symmetrical with respect to p -> 1-p
        let flipped = p > 0.5;
        let p = if flipped { 1.0 - p } else { p };

        // For small n * min(p, 1 - p), the BINV algorithm based on the inverse
        // transformation of the binomial distribution is efficient. Otherwise,
        // the BTPE algorithm is used.
        //
        // Voratas Kachitvichyanukul and Bruce W. Schmeiser. 1988. Binomial
        // random variate generation. Commun. ACM 31, 2 (February 1988),
        // 216-222. http://dx.doi.org/10.1145/42372.42381

        // Threshold for preferring the BINV algorithm. The paper suggests 10,
        // Ranlib uses 30, and GSL uses 14.
        const BINV_THRESHOLD: f64 = 10.;

        let np = n as f64 * p;
        let method = if np < BINV_THRESHOLD {
            let q = 1.0 - p;
            if q == 1.0 {
                // p is so small that this is extremely close to a Poisson distribution.
                // The flipped case cannot occur here.
                Method::Poisson(crate::poisson::KnuthMethod::new(np))
            } else {
                let s = p / q;
                Method::Binv(
                    Binv {
                        r: q.powf(n as f64),
                        s,
                        a: (n as f64 + 1.0) * s,
                        n,
                    },
                    flipped,
                )
            }
        } else {
            let q = 1.0 - p;
            let npq = np * q;
            let p1 = (2.195 * npq.sqrt() - 4.6 * q).floor() + 0.5;
            let f_m = np + p;
            let m = f64_to_i64(f_m);
            Method::Btpe(Btpe { n, p, m, p1 }, flipped)
        };
        Ok(Binomial { method })
    }
}

/// Convert a `f64` to an `i64`, panicking on overflow.
fn f64_to_i64(x: f64) -> i64 {
    assert!(x < (i64::MAX as f64));
    x as i64
}

fn binv<R: Rng + ?Sized>(binv: Binv, flipped: bool, rng: &mut R) -> u64 {
    // Same value as in GSL.
    // It is possible for BINV to get stuck, so we break if x > BINV_MAX_X and try again.
    // It would be safer to set BINV_MAX_X to self.n, but it is extremely unlikely to be relevant.
    // When n*p < 10, so is n*p*q which is the variance, so a result > 110 would be 100 / sqrt(10) = 31 standard deviations away.
    const BINV_MAX_X: u64 = 110;

    let sample = 'outer: loop {
        let mut r = binv.r;
        let mut u: f64 = rng.random();
        let mut x = 0;

        while u > r {
            u -= r;
            x += 1;
            if x > BINV_MAX_X {
                continue 'outer;
            }
            r *= binv.a / (x as f64) - binv.s;
        }
        break x;
    };

    if flipped {
        binv.n - sample
    } else {
        sample
    }
}

#[allow(clippy::many_single_char_names)] // Same names as in the reference.
fn btpe<R: Rng + ?Sized>(btpe: Btpe, flipped: bool, rng: &mut R) -> u64 {
    // Threshold for using the squeeze algorithm. This can be freely
    // chosen based on performance. Ranlib and GSL use 20.
    const SQUEEZE_THRESHOLD: i64 = 20;

    // Step 0: Calculate constants as functions of `n` and `p`.
    let n = btpe.n as f64;
    let np = n * btpe.p;
    let q = 1. - btpe.p;
    let npq = np * q;
    let f_m = np + btpe.p;
    let m = btpe.m;
    // radius of triangle region, since height=1 also area of region
    let p1 = btpe.p1;
    // tip of triangle
    let x_m = (m as f64) + 0.5;
    // left edge of triangle
    let x_l = x_m - p1;
    // right edge of triangle
    let x_r = x_m + p1;
    let c = 0.134 + 20.5 / (15.3 + (m as f64));
    // p1 + area of parallelogram region
    let p2 = p1 * (1. + 2. * c);

    fn lambda(a: f64) -> f64 {
        a * (1. + 0.5 * a)
    }

    let lambda_l = lambda((f_m - x_l) / (f_m - x_l * btpe.p));
    let lambda_r = lambda((x_r - f_m) / (x_r * q));

    let p3 = p2 + c / lambda_l;

    let p4 = p3 + c / lambda_r;

    // return value
    let mut y: i64;

    let gen_u = Uniform::new(0., p4).unwrap();
    let gen_v = Uniform::new(0., 1.).unwrap();

    loop {
        // Step 1: Generate `u` for selecting the region. If region 1 is
        // selected, generate a triangularly distributed variate.
        let u = gen_u.sample(rng);
        let mut v = gen_v.sample(rng);
        if !(u > p1) {
            y = f64_to_i64(x_m - p1 * v + u);
            break;
        }

        if !(u > p2) {
            // Step 2: Region 2, parallelograms. Check if region 2 is
            // used. If so, generate `y`.
            let x = x_l + (u - p1) / c;
            v = v * c + 1.0 - (x - x_m).abs() / p1;
            if v > 1. {
                continue;
            } else {
                y = f64_to_i64(x);
            }
        } else if !(u > p3) {
            // Step 3: Region 3, left exponential tail.
            y = f64_to_i64(x_l + v.ln() / lambda_l);
            if y < 0 {
                continue;
            } else {
                v *= (u - p2) * lambda_l;
            }
        } else {
            // Step 4: Region 4, right exponential tail.
            y = f64_to_i64(x_r - v.ln() / lambda_r);
            if y > 0 && (y as u64) > btpe.n {
                continue;
            } else {
                v *= (u - p3) * lambda_r;
            }
        }

        // Step 5: Acceptance/rejection comparison.

        // Step 5.0: Test for appropriate method of evaluating f(y).
        let k = (y - m).abs();
        if !(k > SQUEEZE_THRESHOLD && (k as f64) < 0.5 * npq - 1.) {
            // Step 5.1: Evaluate f(y) via the recursive relationship. Start the
            // search from the mode.
            let s = btpe.p / q;
            let a = s * (n + 1.);
            let mut f = 1.0;
            match m.cmp(&y) {
                Ordering::Less => {
                    let mut i = m;
                    loop {
                        i += 1;
                        f *= a / (i as f64) - s;
                        if i == y {
                            break;
                        }
                    }
                }
                Ordering::Greater => {
                    let mut i = y;
                    loop {
                        i += 1;
                        f /= a / (i as f64) - s;
                        if i == m {
                            break;
                        }
                    }
                }
                Ordering::Equal => {}
            }
            if v > f {
                continue;
            } else {
                break;
            }
        }

        // Step 5.2: Squeezing. Check the value of ln(v) against upper and
        // lower bound of ln(f(y)).
        let k = k as f64;
        let rho = (k / npq) * ((k * (k / 3. + 0.625) + 1. / 6.) / npq + 0.5);
        let t = -0.5 * k * k / npq;
        let alpha = v.ln();
        if alpha < t - rho {
            break;
        }
        if alpha > t + rho {
            continue;
        }

        // Step 5.3: Final acceptance/rejection test.
        let x1 = (y + 1) as f64;
        let f1 = (m + 1) as f64;
        let z = (f64_to_i64(n) + 1 - m) as f64;
        let w = (f64_to_i64(n) - y + 1) as f64;

        fn stirling(a: f64) -> f64 {
            let a2 = a * a;
            (13860. - (462. - (132. - (99. - 140. / a2) / a2) / a2) / a2) / a / 166320.
        }

        if alpha
            > x_m * (f1 / x1).ln()
                + (n - (m as f64) + 0.5) * (z / w).ln()
                + ((y - m) as f64) * (w * btpe.p / (x1 * q)).ln()
                // We use the signs from the GSL implementation, which are
                // different than the ones in the reference. According to
                // the GSL authors, the new signs were verified to be
                // correct by one of the original designers of the
                // algorithm.
                + stirling(f1)
                + stirling(z)
                - stirling(x1)
                - stirling(w)
        {
            continue;
        }

        break;
    }
    assert!(y >= 0);
    let y = y as u64;

    if flipped {
        btpe.n - y
    } else {
        y
    }
}

impl Distribution<u64> for Binomial {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u64 {
        match self.method {
            Method::Binv(binv_para, flipped) => binv(binv_para, flipped, rng),
            Method::Btpe(btpe_para, flipped) => btpe(btpe_para, flipped, rng),
            Method::Poisson(poisson) => poisson.sample(rng) as u64,
            Method::Constant(c) => c,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Binomial;
    use crate::Distribution;
    use rand::Rng;

    fn test_binomial_mean_and_variance<R: Rng>(n: u64, p: f64, rng: &mut R) {
        let binomial = Binomial::new(n, p).unwrap();

        let expected_mean = n as f64 * p;
        let expected_variance = n as f64 * p * (1.0 - p);

        let mut results = [0.0; 1000];
        for i in results.iter_mut() {
            *i = binomial.sample(rng) as f64;
        }

        let mean = results.iter().sum::<f64>() / results.len() as f64;
        assert!((mean - expected_mean).abs() < expected_mean / 50.0);

        let variance =
            results.iter().map(|x| (x - mean) * (x - mean)).sum::<f64>() / results.len() as f64;
        assert!((variance - expected_variance).abs() < expected_variance / 10.0);
    }

    #[test]
    fn test_binomial() {
        let mut rng = crate::test::rng(351);
        test_binomial_mean_and_variance(150, 0.1, &mut rng);
        test_binomial_mean_and_variance(70, 0.6, &mut rng);
        test_binomial_mean_and_variance(40, 0.5, &mut rng);
        test_binomial_mean_and_variance(20, 0.7, &mut rng);
        test_binomial_mean_and_variance(20, 0.5, &mut rng);
        test_binomial_mean_and_variance(1 << 61, 1e-17, &mut rng);
        test_binomial_mean_and_variance(u64::MAX, 1e-19, &mut rng);
    }

    #[test]
    fn test_binomial_end_points() {
        let mut rng = crate::test::rng(352);
        assert_eq!(rng.sample(Binomial::new(20, 0.0).unwrap()), 0);
        assert_eq!(rng.sample(Binomial::new(20, 1.0).unwrap()), 20);
    }

    #[test]
    #[should_panic]
    fn test_binomial_invalid_lambda_neg() {
        Binomial::new(20, -10.0).unwrap();
    }

    #[test]
    fn binomial_distributions_can_be_compared() {
        assert_eq!(Binomial::new(1, 1.0), Binomial::new(1, 1.0));
    }

    #[test]
    fn binomial_avoid_infinite_loop() {
        let dist = Binomial::new(16000000, 3.1444753148558566e-10).unwrap();
        let mut sum: u64 = 0;
        let mut rng = crate::test::rng(742);
        for _ in 0..100_000 {
            sum = sum.wrapping_add(dist.sample(&mut rng));
        }
        assert_ne!(sum, 0);
    }
}
