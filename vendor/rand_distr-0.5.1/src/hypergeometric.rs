//! The hypergeometric distribution `Hypergeometric(N, K, n)`.

use crate::Distribution;
use core::fmt;
#[allow(unused_imports)]
use num_traits::Float;
use rand::distr::uniform::Uniform;
use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum SamplingMethod {
    InverseTransform {
        initial_p: f64,
        initial_x: i64,
    },
    RejectionAcceptance {
        m: f64,
        a: f64,
        lambda_l: f64,
        lambda_r: f64,
        x_l: f64,
        x_r: f64,
        p1: f64,
        p2: f64,
        p3: f64,
    },
}

/// The [hypergeometric distribution](https://en.wikipedia.org/wiki/Hypergeometric_distribution) `Hypergeometric(N, K, n)`.
///
/// This is the distribution of successes in samples of size `n` drawn without
/// replacement from a population of size `N` containing `K` success states.
///
/// See the [binomial distribution](crate::Binomial) for the analogous distribution
/// for sampling with replacement. It is a good approximation when the population
/// size is much larger than the sample size.
///
/// # Density function
///
/// `f(k) = binomial(K, k) * binomial(N-K, n-k) / binomial(N, n)`,
/// where `binomial(a, b) = a! / (b! * (a - b)!)`.
///
/// # Plot
///
/// The following plot of the hypergeometric distribution illustrates the probability of drawing
/// `k` successes in `n = 10` draws from a population of `N = 50` items, of which either `K = 12`
/// or `K = 35` are successes.
///
/// ![Hypergeometric distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/hypergeometric.svg)
///
/// # Example
/// ```
/// use rand_distr::{Distribution, Hypergeometric};
///
/// let hypergeo = Hypergeometric::new(60, 24, 7).unwrap();
/// let v = hypergeo.sample(&mut rand::rng());
/// println!("{} is from a hypergeometric distribution", v);
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hypergeometric {
    n1: u64,
    n2: u64,
    k: u64,
    offset_x: i64,
    sign_x: i64,
    sampling_method: SamplingMethod,
}

/// Error type returned from [`Hypergeometric::new`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// `total_population_size` is too large, causing floating point underflow.
    PopulationTooLarge,
    /// `population_with_feature > total_population_size`.
    ProbabilityTooLarge,
    /// `sample_size > total_population_size`.
    SampleSizeTooLarge,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Error::PopulationTooLarge => {
                "total_population_size is too large causing underflow in geometric distribution"
            }
            Error::ProbabilityTooLarge => {
                "population_with_feature > total_population_size in geometric distribution"
            }
            Error::SampleSizeTooLarge => {
                "sample_size > total_population_size in geometric distribution"
            }
        })
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

// evaluate fact(numerator.0)*fact(numerator.1) / fact(denominator.0)*fact(denominator.1)
fn fraction_of_products_of_factorials(numerator: (u64, u64), denominator: (u64, u64)) -> f64 {
    let min_top = u64::min(numerator.0, numerator.1);
    let min_bottom = u64::min(denominator.0, denominator.1);
    // the factorial of this will cancel out:
    let min_all = u64::min(min_top, min_bottom);

    let max_top = u64::max(numerator.0, numerator.1);
    let max_bottom = u64::max(denominator.0, denominator.1);
    let max_all = u64::max(max_top, max_bottom);

    let mut result = 1.0;
    for i in (min_all + 1)..=max_all {
        if i <= min_top {
            result *= i as f64;
        }

        if i <= min_bottom {
            result /= i as f64;
        }

        if i <= max_top {
            result *= i as f64;
        }

        if i <= max_bottom {
            result /= i as f64;
        }
    }

    result
}

const LOGSQRT2PI: f64 = 0.91893853320467274178; // log(sqrt(2*pi))

fn ln_of_factorial(v: f64) -> f64 {
    // the paper calls for ln(v!), but also wants to pass in fractions,
    // so we need to use Stirling's approximation to fill in the gaps:

    // shift v by 3, because Stirling is bad for small values
    let v_3 = v + 3.0;
    let ln_fac = (v_3 + 0.5) * v_3.ln() - v_3 + LOGSQRT2PI + 1.0 / (12.0 * v_3);
    // make the correction for the shift
    ln_fac - ((v + 3.0) * (v + 2.0) * (v + 1.0)).ln()
}

impl Hypergeometric {
    /// Constructs a new `Hypergeometric` with the shape parameters
    /// `N = total_population_size`,
    /// `K = population_with_feature`,
    /// `n = sample_size`.
    #[allow(clippy::many_single_char_names)] // Same names as in the reference.
    pub fn new(
        total_population_size: u64,
        population_with_feature: u64,
        sample_size: u64,
    ) -> Result<Self, Error> {
        if population_with_feature > total_population_size {
            return Err(Error::ProbabilityTooLarge);
        }

        if sample_size > total_population_size {
            return Err(Error::SampleSizeTooLarge);
        }

        // set-up constants as function of original parameters
        let n = total_population_size;
        let (mut sign_x, mut offset_x) = (1, 0);
        let (n1, n2) = {
            // switch around success and failure states if necessary to ensure n1 <= n2
            let population_without_feature = n - population_with_feature;
            if population_with_feature > population_without_feature {
                sign_x = -1;
                offset_x = sample_size as i64;
                (population_without_feature, population_with_feature)
            } else {
                (population_with_feature, population_without_feature)
            }
        };
        // when sampling more than half the total population, take the smaller
        // group as sampled instead (we can then return n1-x instead).
        //
        // Note: the boundary condition given in the paper is `sample_size < n / 2`;
        // we're deviating here, because when n is even, it doesn't matter whether
        // we switch here or not, but when n is odd `n/2 < n - n/2`, so switching
        // when `k == n/2`, we'd actually be taking the _larger_ group as sampled.
        let k = if sample_size <= n / 2 {
            sample_size
        } else {
            offset_x += n1 as i64 * sign_x;
            sign_x *= -1;
            n - sample_size
        };

        // Algorithm H2PE has bounded runtime only if `M - max(0, k-n2) >= 10`,
        // where `M` is the mode of the distribution.
        // Use algorithm HIN for the remaining parameter space.
        //
        // Voratas Kachitvichyanukul and Bruce W. Schmeiser. 1985. Computer
        // generation of hypergeometric random variates.
        // J. Statist. Comput. Simul. Vol.22 (August 1985), 127-145
        // https://www.researchgate.net/publication/233212638
        const HIN_THRESHOLD: f64 = 10.0;
        let m = ((k + 1) as f64 * (n1 + 1) as f64 / (n + 2) as f64).floor();
        let sampling_method = if m - f64::max(0.0, k as f64 - n2 as f64) < HIN_THRESHOLD {
            let (initial_p, initial_x) = if k < n2 {
                (
                    fraction_of_products_of_factorials((n2, n - k), (n, n2 - k)),
                    0,
                )
            } else {
                (
                    fraction_of_products_of_factorials((n1, k), (n, k - n2)),
                    (k - n2) as i64,
                )
            };

            if initial_p <= 0.0 || !initial_p.is_finite() {
                return Err(Error::PopulationTooLarge);
            }

            SamplingMethod::InverseTransform {
                initial_p,
                initial_x,
            }
        } else {
            let a = ln_of_factorial(m)
                + ln_of_factorial(n1 as f64 - m)
                + ln_of_factorial(k as f64 - m)
                + ln_of_factorial((n2 - k) as f64 + m);

            let numerator = (n - k) as f64 * k as f64 * n1 as f64 * n2 as f64;
            let denominator = (n - 1) as f64 * n as f64 * n as f64;
            let d = 1.5 * (numerator / denominator).sqrt() + 0.5;

            let x_l = m - d + 0.5;
            let x_r = m + d + 0.5;

            let k_l = f64::exp(
                a - ln_of_factorial(x_l)
                    - ln_of_factorial(n1 as f64 - x_l)
                    - ln_of_factorial(k as f64 - x_l)
                    - ln_of_factorial((n2 - k) as f64 + x_l),
            );
            let k_r = f64::exp(
                a - ln_of_factorial(x_r - 1.0)
                    - ln_of_factorial(n1 as f64 - x_r + 1.0)
                    - ln_of_factorial(k as f64 - x_r + 1.0)
                    - ln_of_factorial((n2 - k) as f64 + x_r - 1.0),
            );

            let numerator = x_l * ((n2 - k) as f64 + x_l);
            let denominator = (n1 as f64 - x_l + 1.0) * (k as f64 - x_l + 1.0);
            let lambda_l = -((numerator / denominator).ln());

            let numerator = (n1 as f64 - x_r + 1.0) * (k as f64 - x_r + 1.0);
            let denominator = x_r * ((n2 - k) as f64 + x_r);
            let lambda_r = -((numerator / denominator).ln());

            // the paper literally gives `p2 + kL/lambdaL` where it (probably)
            // should have been `p2 <- p1 + kL/lambdaL`; another print error?!
            let p1 = 2.0 * d;
            let p2 = p1 + k_l / lambda_l;
            let p3 = p2 + k_r / lambda_r;

            SamplingMethod::RejectionAcceptance {
                m,
                a,
                lambda_l,
                lambda_r,
                x_l,
                x_r,
                p1,
                p2,
                p3,
            }
        };

        Ok(Hypergeometric {
            n1,
            n2,
            k,
            offset_x,
            sign_x,
            sampling_method,
        })
    }
}

impl Distribution<u64> for Hypergeometric {
    #[allow(clippy::many_single_char_names)] // Same names as in the reference.
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u64 {
        use SamplingMethod::*;

        let Hypergeometric {
            n1,
            n2,
            k,
            sign_x,
            offset_x,
            sampling_method,
        } = *self;
        let x = match sampling_method {
            InverseTransform {
                initial_p: mut p,
                initial_x: mut x,
            } => {
                let mut u = rng.random::<f64>();

                // the paper erroneously uses `until n < p`, which doesn't make any sense
                while u > p && x < k as i64 {
                    u -= p;
                    p *= ((n1 as i64 - x) * (k as i64 - x)) as f64;
                    p /= ((x + 1) * (n2 as i64 - k as i64 + 1 + x)) as f64;
                    x += 1;
                }
                x
            }
            RejectionAcceptance {
                m,
                a,
                lambda_l,
                lambda_r,
                x_l,
                x_r,
                p1,
                p2,
                p3,
            } => {
                let distr_region_select = Uniform::new(0.0, p3).unwrap();
                loop {
                    let (y, v) = loop {
                        let u = distr_region_select.sample(rng);
                        let v = rng.random::<f64>(); // for the accept/reject decision

                        if u <= p1 {
                            // Region 1, central bell
                            let y = (x_l + u).floor();
                            break (y, v);
                        } else if u <= p2 {
                            // Region 2, left exponential tail
                            let y = (x_l + v.ln() / lambda_l).floor();
                            if y as i64 >= i64::max(0, k as i64 - n2 as i64) {
                                let v = v * (u - p1) * lambda_l;
                                break (y, v);
                            }
                        } else {
                            // Region 3, right exponential tail
                            let y = (x_r - v.ln() / lambda_r).floor();
                            if y as u64 <= u64::min(n1, k) {
                                let v = v * (u - p2) * lambda_r;
                                break (y, v);
                            }
                        }
                    };

                    // Step 4: Acceptance/Rejection Comparison
                    if m < 100.0 || y <= 50.0 {
                        // Step 4.1: evaluate f(y) via recursive relationship
                        let mut f = 1.0;
                        if m < y {
                            for i in (m as u64 + 1)..=(y as u64) {
                                f *= (n1 - i + 1) as f64 * (k - i + 1) as f64;
                                f /= i as f64 * (n2 - k + i) as f64;
                            }
                        } else {
                            for i in (y as u64 + 1)..=(m as u64) {
                                f *= i as f64 * (n2 - k + i) as f64;
                                f /= (n1 - i + 1) as f64 * (k - i + 1) as f64;
                            }
                        }

                        if v <= f {
                            break y as i64;
                        }
                    } else {
                        // Step 4.2: Squeezing
                        let y1 = y + 1.0;
                        let ym = y - m;
                        let yn = n1 as f64 - y + 1.0;
                        let yk = k as f64 - y + 1.0;
                        let nk = n2 as f64 - k as f64 + y1;
                        let r = -ym / y1;
                        let s = ym / yn;
                        let t = ym / yk;
                        let e = -ym / nk;
                        let g = yn * yk / (y1 * nk) - 1.0;
                        let dg = if g < 0.0 { 1.0 + g } else { 1.0 };
                        let gu = g * (1.0 + g * (-0.5 + g / 3.0));
                        let gl = gu - g.powi(4) / (4.0 * dg);
                        let xm = m + 0.5;
                        let xn = n1 as f64 - m + 0.5;
                        let xk = k as f64 - m + 0.5;
                        let nm = n2 as f64 - k as f64 + xm;
                        let ub = xm * r * (1.0 + r * (-0.5 + r / 3.0))
                            + xn * s * (1.0 + s * (-0.5 + s / 3.0))
                            + xk * t * (1.0 + t * (-0.5 + t / 3.0))
                            + nm * e * (1.0 + e * (-0.5 + e / 3.0))
                            + y * gu
                            - m * gl
                            + 0.0034;
                        let av = v.ln();
                        if av > ub {
                            continue;
                        }
                        let dr = if r < 0.0 {
                            xm * r.powi(4) / (1.0 + r)
                        } else {
                            xm * r.powi(4)
                        };
                        let ds = if s < 0.0 {
                            xn * s.powi(4) / (1.0 + s)
                        } else {
                            xn * s.powi(4)
                        };
                        let dt = if t < 0.0 {
                            xk * t.powi(4) / (1.0 + t)
                        } else {
                            xk * t.powi(4)
                        };
                        let de = if e < 0.0 {
                            nm * e.powi(4) / (1.0 + e)
                        } else {
                            nm * e.powi(4)
                        };

                        if av < ub - 0.25 * (dr + ds + dt + de) + (y + m) * (gl - gu) - 0.0078 {
                            break y as i64;
                        }

                        // Step 4.3: Final Acceptance/Rejection Test
                        let av_critical = a
                            - ln_of_factorial(y)
                            - ln_of_factorial(n1 as f64 - y)
                            - ln_of_factorial(k as f64 - y)
                            - ln_of_factorial((n2 - k) as f64 + y);
                        if v.ln() <= av_critical {
                            break y as i64;
                        }
                    }
                }
            }
        };

        (offset_x + sign_x * x) as u64
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_hypergeometric_invalid_params() {
        assert!(Hypergeometric::new(100, 101, 5).is_err());
        assert!(Hypergeometric::new(100, 10, 101).is_err());
        assert!(Hypergeometric::new(100, 101, 101).is_err());
        assert!(Hypergeometric::new(100, 10, 5).is_ok());
    }

    fn test_hypergeometric_mean_and_variance<R: Rng>(n: u64, k: u64, s: u64, rng: &mut R) {
        let distr = Hypergeometric::new(n, k, s).unwrap();

        let expected_mean = s as f64 * k as f64 / n as f64;
        let expected_variance = {
            let numerator = (s * k * (n - k) * (n - s)) as f64;
            let denominator = (n * n * (n - 1)) as f64;
            numerator / denominator
        };

        let mut results = [0.0; 1000];
        for i in results.iter_mut() {
            *i = distr.sample(rng) as f64;
        }

        let mean = results.iter().sum::<f64>() / results.len() as f64;
        assert!((mean - expected_mean).abs() < expected_mean / 50.0);

        let variance =
            results.iter().map(|x| (x - mean) * (x - mean)).sum::<f64>() / results.len() as f64;
        assert!((variance - expected_variance).abs() < expected_variance / 10.0);
    }

    #[test]
    fn test_hypergeometric() {
        let mut rng = crate::test::rng(737);

        // exercise algorithm HIN:
        test_hypergeometric_mean_and_variance(500, 400, 30, &mut rng);
        test_hypergeometric_mean_and_variance(250, 200, 230, &mut rng);
        test_hypergeometric_mean_and_variance(100, 20, 6, &mut rng);
        test_hypergeometric_mean_and_variance(50, 10, 47, &mut rng);

        // exercise algorithm H2PE
        test_hypergeometric_mean_and_variance(5000, 2500, 500, &mut rng);
        test_hypergeometric_mean_and_variance(10100, 10000, 1000, &mut rng);
        test_hypergeometric_mean_and_variance(100100, 100, 10000, &mut rng);
    }

    #[test]
    fn hypergeometric_distributions_can_be_compared() {
        assert_eq!(Hypergeometric::new(1, 2, 3), Hypergeometric::new(1, 2, 3));
    }

    #[test]
    fn stirling() {
        let test = [0.5, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        for &v in test.iter() {
            let ln_fac = ln_of_factorial(v);
            assert!((special::Gamma::ln_gamma(v + 1.0).0 - ln_fac).abs() < 1e-4);
        }
    }
}
