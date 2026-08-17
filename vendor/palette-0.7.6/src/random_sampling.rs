mod cone;

pub(crate) use self::cone::*;

#[cfg(test)]
pub(crate) mod test_utils {
    pub(crate) const BINS: usize = 10;
    pub(crate) const SAMPLES: usize = 20_000;

    /// Perform a Chi-squared goodness-of-fit test to check if the bins are
    /// uniformly distributed. Returns the p-value.
    pub(crate) fn uniform_distribution_test(bins: &[usize]) -> f64 {
        let sum = bins.iter().sum::<usize>() as f64;
        let expected = sum / bins.len() as f64;
        let critical_value = bins
            .iter()
            .map(|&bin| {
                let difference = bin as f64 - expected;
                difference * difference / expected
            })
            .sum::<f64>();

        chi_square(bins.len() - 1, critical_value)
    }

    // Shamelessly taken from https://www.codeproject.com/Articles/432194/How-to-Calculate-the-Chi-Squared-P-Value
    fn chi_square(dof: usize, critical_value: f64) -> f64 {
        if critical_value < 0.0 || dof < 1 {
            return 0.0;
        }
        let k = dof as f64 * 0.5;
        let x = critical_value * 0.5;
        if dof == 2 {
            return (-x).exp();
        }

        let mut p_value = incomplete_gamma_function(k, x);
        if p_value.is_nan() || p_value.is_infinite() || p_value <= 1e-8 {
            return 1e-14;
        }

        p_value /= approximate_gamma(k);

        1.0 - p_value
    }

    fn incomplete_gamma_function(mut s: f64, z: f64) -> f64 {
        if z < 0.0 {
            return 0.0;
        }
        let mut sc = 1.0 / s;
        sc *= z.powf(s);
        sc *= (-z).exp();

        let mut sum = 1.0;
        let mut nom = 1.0;
        let mut denom = 1.0;

        for _ in 0..200 {
            nom *= z;
            s += 1.0;
            denom *= s;
            sum += nom / denom;
        }

        sum * sc
    }

    fn approximate_gamma(z: f64) -> f64 {
        #[allow(clippy::excessive_precision)]
        const RECIP_E: f64 = 0.36787944117144232159552377016147; // RECIP_E = (E^-1) = (1.0 / E)
        const TWOPI: f64 = core::f64::consts::TAU;

        let mut d = 1.0 / (10.0 * z);
        d = 1.0 / ((12.0 * z) - d);
        d = (d + z) * RECIP_E;
        d = d.powf(z);
        d *= (TWOPI / z).sqrt();

        d
    }
}
