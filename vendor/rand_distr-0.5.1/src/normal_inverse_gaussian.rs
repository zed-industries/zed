use crate::{Distribution, InverseGaussian, StandardNormal, StandardUniform};
use core::fmt;
use num_traits::Float;
use rand::Rng;

/// Error type returned from [`NormalInverseGaussian::new`]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// `alpha <= 0` or `nan`.
    AlphaNegativeOrNull,
    /// `|beta| >= alpha` or `nan`.
    AbsoluteBetaNotLessThanAlpha,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Error::AlphaNegativeOrNull => {
                "alpha <= 0 or is NaN in normal inverse Gaussian distribution"
            }
            Error::AbsoluteBetaNotLessThanAlpha => {
                "|beta| >= alpha or is NaN in normal inverse Gaussian distribution"
            }
        })
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// The [normal-inverse Gaussian distribution](https://en.wikipedia.org/wiki/Normal-inverse_Gaussian_distribution) `NIG(α, β)`.
///
/// This is a continuous probability distribution with two parameters,
/// `α` (`alpha`) and `β` (`beta`), defined in `(-∞, ∞)`.
/// It is also known as the normal-Wald distribution.
///
/// # Plot
///
/// The following plot shows the normal-inverse Gaussian distribution with various values of `α` and `β`.
///
/// ![Normal-inverse Gaussian distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/normal_inverse_gaussian.svg)
///
/// # Example
/// ```
/// use rand_distr::{NormalInverseGaussian, Distribution};
///
/// let norm_inv_gauss = NormalInverseGaussian::new(2.0, 1.0).unwrap();
/// let v = norm_inv_gauss.sample(&mut rand::rng());
/// println!("{} is from a normal-inverse Gaussian(2, 1) distribution", v);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NormalInverseGaussian<F>
where
    F: Float,
    StandardNormal: Distribution<F>,
    StandardUniform: Distribution<F>,
{
    beta: F,
    inverse_gaussian: InverseGaussian<F>,
}

impl<F> NormalInverseGaussian<F>
where
    F: Float,
    StandardNormal: Distribution<F>,
    StandardUniform: Distribution<F>,
{
    /// Construct a new `NormalInverseGaussian` distribution with the given alpha (tail heaviness) and
    /// beta (asymmetry) parameters.
    pub fn new(alpha: F, beta: F) -> Result<NormalInverseGaussian<F>, Error> {
        if !(alpha > F::zero()) {
            return Err(Error::AlphaNegativeOrNull);
        }

        if !(beta.abs() < alpha) {
            return Err(Error::AbsoluteBetaNotLessThanAlpha);
        }

        let gamma = (alpha * alpha - beta * beta).sqrt();

        let mu = F::one() / gamma;

        let inverse_gaussian = InverseGaussian::new(mu, F::one()).unwrap();

        Ok(Self {
            beta,
            inverse_gaussian,
        })
    }
}

impl<F> Distribution<F> for NormalInverseGaussian<F>
where
    F: Float,
    StandardNormal: Distribution<F>,
    StandardUniform: Distribution<F>,
{
    fn sample<R>(&self, rng: &mut R) -> F
    where
        R: Rng + ?Sized,
    {
        let inv_gauss = rng.sample(self.inverse_gaussian);

        self.beta * inv_gauss + inv_gauss.sqrt() * rng.sample(StandardNormal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_inverse_gaussian() {
        let norm_inv_gauss = NormalInverseGaussian::new(2.0, 1.0).unwrap();
        let mut rng = crate::test::rng(210);
        for _ in 0..1000 {
            norm_inv_gauss.sample(&mut rng);
        }
    }

    #[test]
    fn test_normal_inverse_gaussian_invalid_param() {
        assert!(NormalInverseGaussian::new(-1.0, 1.0).is_err());
        assert!(NormalInverseGaussian::new(-1.0, -1.0).is_err());
        assert!(NormalInverseGaussian::new(1.0, 2.0).is_err());
        assert!(NormalInverseGaussian::new(2.0, 1.0).is_ok());
    }

    #[test]
    fn normal_inverse_gaussian_distributions_can_be_compared() {
        assert_eq!(
            NormalInverseGaussian::new(1.0, 2.0),
            NormalInverseGaussian::new(1.0, 2.0)
        );
    }
}
