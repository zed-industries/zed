//! Traits and functions used in Ok* color spaces

#[cfg(feature = "approx")]
#[cfg(test)]
use crate::{angle::RealAngle, num::Trigonometry, OklabHue};

use crate::{
    convert::IntoColorUnclamped,
    num::{Arithmetics, Cbrt, MinMax, One, Powi, Real, Sqrt, Zero},
    HasBoolMask, LinSrgb, Oklab,
};

/// Finds intersection of the line defined by
///
/// L = l0 * (1 - t) + t * l1;
///
/// C = t * c1;
///
/// a and b must be normalized so a² + b² == 1
fn find_gamut_intersection<T>(a: T, b: T, l1: T, c1: T, l0: T, cusp: LC<T>) -> T
where
    T: Real + One + Zero + Arithmetics + MinMax + HasBoolMask<Mask = bool> + PartialOrd + Clone,
{
    // Find the intersection for upper and lower half separately
    if ((l1.clone() - &l0) * &cusp.chroma - (cusp.lightness.clone() - &l0) * &c1) <= T::zero() {
        // Lower half

        cusp.chroma.clone() * &l0 / (c1 * cusp.lightness + cusp.chroma * (l0 - l1))
    } else {
        // Upper half

        // First intersect with triangle
        let t = cusp.chroma.clone() * (l0.clone() - T::one())
            / (c1.clone() * (cusp.lightness - T::one()) + cusp.chroma * (l0.clone() - &l1));

        // Then one step Halley's method
        {
            let dl = l1.clone() - &l0;
            let dc = c1.clone();

            let k_l = T::from_f64(0.3963377774) * &a + T::from_f64(0.2158037573) * &b;
            let k_m = -T::from_f64(0.1055613458) * &a - T::from_f64(0.0638541728) * &b;
            let k_s = -T::from_f64(0.0894841775) * a - T::from_f64(1.2914855480) * b;

            let l_dt = dl.clone() + dc.clone() * &k_l;
            let m_dt = dl.clone() + dc.clone() * &k_m;
            let s_dt = dl + dc * &k_s;

            // If higher accuracy is required, 2 or 3 iterations of the following block can be used:
            {
                let lightness = l0 * (T::one() - &t) + t.clone() * l1;
                let chroma = t.clone() * c1;

                let l_ = lightness.clone() + chroma.clone() * k_l;
                let m_ = lightness.clone() + chroma.clone() * k_m;
                let s_ = lightness + chroma * k_s;

                let l = l_.clone() * &l_ * &l_;
                let m = m_.clone() * &m_ * &m_;
                let s = s_.clone() * &s_ * &s_;

                let ldt = T::from_f64(3.0) * &l_dt * &l_ * &l_;
                let mdt = T::from_f64(3.0) * &m_dt * &m_ * &m_;
                let sdt = T::from_f64(3.0) * &s_dt * &s_ * &s_;

                let ldt2 = T::from_f64(6.0) * &l_dt * l_dt * l_;
                let mdt2 = T::from_f64(6.0) * &m_dt * m_dt * m_;
                let sdt2 = T::from_f64(6.0) * &s_dt * s_dt * s_;

                let r = T::from_f64(4.0767416621) * &l - T::from_f64(3.3077115913) * &m
                    + T::from_f64(0.2309699292) * &s
                    - T::one();
                let r1 = T::from_f64(4.0767416621) * &ldt - T::from_f64(3.3077115913) * &mdt
                    + T::from_f64(0.2309699292) * &sdt;
                let r2 = T::from_f64(4.0767416621) * &ldt2 - T::from_f64(3.3077115913) * &mdt2
                    + T::from_f64(0.2309699292) * &sdt2;

                let u_r = r1.clone() / (r1.clone() * r1 - T::from_f64(0.5) * &r * r2);
                let mut t_r = -r * &u_r;

                let g = -T::from_f64(1.2684380046) * &l + T::from_f64(2.6097574011) * &m
                    - T::from_f64(0.3413193965) * &s
                    - T::one();
                let g1 = -T::from_f64(1.2684380046) * &ldt + T::from_f64(2.6097574011) * &mdt
                    - T::from_f64(0.3413193965) * &sdt;
                let g2 = -T::from_f64(1.2684380046) * &ldt2 + T::from_f64(2.6097574011) * &mdt2
                    - T::from_f64(0.3413193965) * &sdt2;

                let u_g = g1.clone() / (g1.clone() * g1 - T::from_f64(0.5) * &g * g2);
                let mut t_g = -g * &u_g;

                let b = -T::from_f64(0.0041960863) * l - T::from_f64(0.7034186147) * m
                    + T::from_f64(1.7076147010) * s
                    - T::one();
                let b1 = -T::from_f64(0.0041960863) * ldt - T::from_f64(0.7034186147) * mdt
                    + T::from_f64(1.7076147010) * sdt;
                let b2 = -T::from_f64(0.0041960863) * ldt2 - T::from_f64(0.7034186147) * mdt2
                    + T::from_f64(1.7076147010) * sdt2;

                let u_b = b1.clone() / (b1.clone() * b1 - T::from_f64(0.5) * &b * b2);
                let mut t_b = -b * &u_b;

                // flt_max really is a constant, but cannot be defined as one due to the T::from_f64 function
                let flt_max = T::from_f64(10e5);

                t_r = if u_r >= T::zero() {
                    t_r
                } else {
                    flt_max.clone()
                };
                t_g = if u_g >= T::zero() {
                    t_g
                } else {
                    flt_max.clone()
                };
                t_b = if u_b >= T::zero() { t_b } else { flt_max };

                t + T::min(t_r, T::min(t_g, t_b))
            }
        }
    }
}

pub struct ChromaValues<T> {
    pub zero: T,
    pub mid: T,
    pub max: T,
}

impl<T> ChromaValues<T>
where
    T: Real
        + One
        + Zero
        + Arithmetics
        + MinMax
        + Cbrt
        + Sqrt
        + Powi
        + Clone
        + HasBoolMask<Mask = bool>
        + PartialOrd,
    Oklab<T>: IntoColorUnclamped<LinSrgb<T>>,
{
    // Corresponds to `get_Cs` in the reference implementation. Assumes that
    // `lightness != 1.0` and `lightness != 0.0`.
    pub fn from_normalized(lightness: T, a_: T, b_: T) -> Self {
        let cusp = LC::find_cusp(a_.clone(), b_.clone());

        let max_chroma = find_gamut_intersection(
            a_.clone(),
            b_.clone(),
            lightness.clone(),
            T::one(),
            lightness.clone(),
            cusp.clone(),
        );
        let st_max = ST::from(cusp);

        // Scale factor to compensate for the curved part of gamut shape:
        let k = max_chroma.clone()
            / T::min(
                lightness.clone() * st_max.s,
                (T::one() - &lightness) * st_max.t,
            );

        let c_mid = {
            let st_mid = ST::mid(a_, b_);

            // Use a soft minimum function, instead of a sharp triangle shape to get a smooth value for chroma.
            let c_a = lightness.clone() * st_mid.s;
            let c_b = (T::one() - &lightness) * st_mid.t;
            T::from_f64(0.9)
                * k
                * T::sqrt(T::sqrt(
                    T::one()
                        / (T::one() / (c_a.clone() * &c_a * &c_a * &c_a)
                            + T::one() / (c_b.clone() * &c_b * &c_b * &c_b)),
                ))
        };

        let c_0 = {
            // for C_0, the shape is independent of hue, so ST are constant.
            // Values picked to roughly be the average values of ST.
            let c_a = lightness.clone() * T::from_f64(0.4);
            let c_b = (T::one() - lightness) * T::from_f64(0.8);

            // Use a soft minimum function, instead of a sharp triangle shape to get a smooth value for chroma.
            T::sqrt(T::one() / (T::one() / (c_a.clone() * c_a) + T::one() / (c_b.clone() * c_b)))
        };
        Self {
            zero: c_0,
            mid: c_mid,
            max: max_chroma,
        }
    }
}

/// A `lightness`-`chroma` representation of a point in the `sRGB` gamut for a fixed hue.
///
/// Gamut is the range of representable colors of a color space. In this case the
/// `sRGB` color space.
///
/// Only together are `lightness` and `chroma` guaranteed to be inside the `sRGB` gamut.
/// While a color with lower `chroma` will  always stay in the gamut, a color of raised
/// *and lowered* lightness might move the point outside the gamut.
///
///# See
/// [LC diagram samples](https://bottosson.github.io/posts/gamutclipping/#gamut-clipping)
#[derive(Debug, Copy, Clone)]
pub(crate) struct LC<T> {
    /// The lightness of the color. 0 corresponds to black. 1 corresponds to white
    pub lightness: T,
    /// The chroma of the color. 0 corresponds to totally desaturated (white, grey or black).
    /// Larger values correspond to colorful values.
    ///
    ///Note: the maximum representable value depends on the lightness and the hue.
    pub chroma: T,
}

/// The number of iterations used for optimizing the result of [`LC::max_saturation`].
///
/// Must match [`MAX_SRGB_SATURATION_INACCURACY`]
pub(crate) const MAX_SRGB_SATURATION_SEARCH_MAX_ITER: usize = 1;
/// The expected inaccuracy of the result of [`LC::max_saturation`], optimized with
/// [`MAX_SRGB_SATURATION_SEARCH_MAX_ITER`] iterations
pub(crate) const MAX_SRGB_SATURATION_INACCURACY: f64 = 1e-6;

impl<T> LC<T>
where
    T: Real + One + Arithmetics + Powi + HasBoolMask<Mask = bool> + PartialOrd + Clone,
{
    /// Returns the cusp of the geometrical shape of representable `sRGB` colors for
    /// normalized `a` and `b` values of an `OKlabHue`, where "normalized" means, `a² + b² == 1`.
    ///
    /// The cusp solely depends on the maximum saturation of the hue, but is expressed as a
    /// combination of lightness and chroma.
    pub fn find_cusp(a: T, b: T) -> Self
    where
        T: MinMax + Cbrt,
        Oklab<T>: IntoColorUnclamped<LinSrgb<T>>,
    {
        // First, find the maximum saturation (saturation S = C/L)
        let max_saturation = Self::max_saturation(a.clone(), b.clone());
        // Convert to linear sRGB to find the first point where at least one of r,g or b >= 1:
        let rgb_at_max: LinSrgb<T> = Oklab::new(
            T::one(),
            max_saturation.clone() * a,
            max_saturation.clone() * b,
        )
        .into_color_unclamped();

        let max_lightness =
            T::cbrt(T::one() / T::max(T::max(rgb_at_max.red, rgb_at_max.green), rgb_at_max.blue));
        Self {
            lightness: max_lightness.clone(),
            chroma: max_lightness * max_saturation,
        }
    }

    /// Returns the maximum `sRGB`-saturation (chroma / lightness) for the hue (`a` and `b`).
    ///
    /// # Arguments
    /// * `a` - the green/redness of the hue
    /// * `b` -  the blue/yellowness of the hue
    ///
    ///  `a` and `b` must be normalized to a chroma (`a²+b²`) of `1`.
    /// # See
    /// [Original C-Version](https://bottosson.github.io/posts/gamutclipping/#intersection-with-srgb-gamut)
    fn max_saturation(a: T, b: T) -> T {
        // Max saturation will be reached, when one of r, g or b goes below zero.
        // Select different coefficients depending on which component goes below zero first
        // wl, wm and ws are coefficients for https://en.wikipedia.org/wiki/LMS_color_space
        // -- the color space modelling human perception.
        let (k0, k1, k2, k3, k4, wl, wm, ws) =
            if T::from_f64(-1.88170328) * &a - T::from_f64(0.80936493) * &b > T::one() {
                // red component at zero first
                (
                    T::from_f64(1.19086277),
                    T::from_f64(1.76576728),
                    T::from_f64(0.59662641),
                    T::from_f64(0.75515197),
                    T::from_f64(0.56771245),
                    T::from_f64(4.0767416621),
                    T::from_f64(-3.3077115913),
                    T::from_f64(0.2309699292),
                )
            } else if T::from_f64(1.81444104) * &a - T::from_f64(1.19445276) * &b > T::one() {
                //green component at zero first
                (
                    T::from_f64(0.73956515),
                    T::from_f64(-0.45954404),
                    T::from_f64(0.08285427),
                    T::from_f64(0.12541070),
                    T::from_f64(0.14503204),
                    T::from_f64(-1.2684380046),
                    T::from_f64(2.6097574011),
                    T::from_f64(-0.3413193965),
                )
            } else {
                //blue component at zero first
                (
                    T::from_f64(1.35733652),
                    T::from_f64(-0.00915799),
                    T::from_f64(-1.15130210),
                    T::from_f64(-0.50559606),
                    T::from_f64(0.00692167),
                    T::from_f64(-0.0041960863),
                    T::from_f64(-0.7034186147),
                    T::from_f64(1.7076147010),
                )
            };

        // Approximate max saturation using a polynomial
        let mut approx_max_saturation =
            k0 + k1 * &a + k2 * &b + k3 * a.clone().powi(2) + k4 * &a * &b;
        // Get closer with Halley's method
        let k_l = T::from_f64(0.3963377774) * &a + T::from_f64(0.2158037573) * &b;
        let k_m = T::from_f64(-0.1055613458) * &a - T::from_f64(0.0638541728) * &b;
        let k_s = T::from_f64(-0.0894841775) * a - T::from_f64(1.2914855480) * b;

        for _i in 0..MAX_SRGB_SATURATION_SEARCH_MAX_ITER {
            let l_ = T::one() + approx_max_saturation.clone() * &k_l;
            let m_ = T::one() + approx_max_saturation.clone() * &k_m;
            let s_ = T::one() + approx_max_saturation.clone() * &k_s;

            let l = l_.clone().powi(3);
            let m = m_.clone().powi(3);
            let s = s_.clone().powi(3);

            // first derivative components
            let l_ds = T::from_f64(3.0) * &k_l * l_.clone().powi(2);
            let m_ds = T::from_f64(3.0) * &k_m * m_.clone().powi(2);
            let s_ds = T::from_f64(3.0) * &k_s * s_.clone().powi(2);

            // second derivative components
            let l_ds2 = T::from_f64(6.0) * k_l.clone().powi(2) * l_;
            let m_ds2 = T::from_f64(6.0) * k_m.clone().powi(2) * m_;
            let s_ds2 = T::from_f64(6.0) * k_s.clone().powi(2) * s_;

            // let x be the approximate maximum saturation and
            // i the current iteration
            // f = f(x_i), f1 = f'(x_i), f2 = f''(x_i) for
            let f = wl.clone() * l + wm.clone() * m + ws.clone() * s;
            let f1 = wl.clone() * l_ds + wm.clone() * m_ds + ws.clone() * s_ds;
            let f2 = wl.clone() * l_ds2 + wm.clone() * m_ds2 + ws.clone() * s_ds2;

            approx_max_saturation =
                approx_max_saturation - f.clone() * &f1 / (f1.powi(2) - T::from_f64(0.5) * f * f2);
        }
        approx_max_saturation
    }
}

#[cfg(feature = "approx")]
#[cfg(test)]
impl<T> OklabHue<T>
where
    T: RealAngle
        + One
        + Arithmetics
        + Trigonometry
        + MinMax
        + Cbrt
        + Powi
        + HasBoolMask<Mask = bool>
        + PartialOrd
        + Clone,
    Oklab<T>: IntoColorUnclamped<LinSrgb<T>>,
{
    pub(crate) fn srgb_limits(self) -> (LC<T>, T, T) {
        let normalized_hue_vector = self.into_cartesian();
        let lc = LC::find_cusp(
            normalized_hue_vector.0.clone(),
            normalized_hue_vector.1.clone(),
        );
        let a = lc.chroma.clone() * normalized_hue_vector.0;
        let b = lc.chroma.clone() * normalized_hue_vector.1;
        (lc, a, b)
    }
}

/// A representation of [`LC`], that allows computing the maximum chroma `C`
/// for a given lightness `L` in the gamut triangle of a hue as
/// ```text
/// C
///   = min(S*L, T*(1-L))
///   = min(lc.chroma / lc.lightness * L, lc.chroma / (T::one() - lc.lightness) * (1-L))
/// ```
#[derive(Debug, Copy, Clone)]
pub(crate) struct ST<T> {
    /// `lc.chroma / lc.lightness`
    pub s: T,
    /// `lc.chroma / (T::one() - lc.lightness)`
    pub t: T,
}

impl<T> From<LC<T>> for ST<T>
where
    T: Arithmetics + One + Clone,
{
    fn from(lc: LC<T>) -> Self {
        ST {
            s: lc.chroma.clone() / &lc.lightness,
            t: lc.chroma / (T::one() - lc.lightness),
        }
    }
}

impl<T> ST<T>
where
    T: Real + Arithmetics + One + Clone,
{
    /// Returns a smooth approximation of the location of the cusp.
    ///
    /// This polynomial was created by an optimization process.
    /// It has been designed so that
    ///
    ///   `S_mid < S_max` and
    ///
    ///   `T_mid < T_max`
    #[rustfmt::skip]
    fn mid(a_: T, b_: T) -> ST<T> {
        let s = T::from_f64(0.11516993) + T::one() / (
            T::from_f64(7.44778970)
                + T::from_f64(4.15901240) * &b_
                + a_.clone() * (T::from_f64(-2.19557347)+ T::from_f64(1.75198401) * &b_
                + a_.clone() * (T::from_f64(-2.13704948) - T::from_f64(10.02301043) * &b_
                + a_.clone() * (T::from_f64(-4.24894561) + T::from_f64(5.38770819) * &b_+ T::from_f64(4.69891013) * &a_
            )))
        );

        let t = T::from_f64(0.11239642)+ T::one()/ (
            T::from_f64(1.61320320) - T::from_f64(0.68124379) * &b_
                + a_.clone() * (T::from_f64(0.40370612)
                + T::from_f64(0.90148123) * &b_
                + a_.clone() * (T::from_f64(-0.27087943) + T::from_f64(0.61223990) * &b_
                + a_.clone() * (T::from_f64(0.00299215) - T::from_f64(0.45399568) * b_ - T::from_f64(0.14661872) * a_
            )))
        );
        ST { s, t }
    }
}

/// Maps an `oklab_lightness` to an *sRGB* reference-white based lightness `L_r`.
///
/// The `Oklab` lightness is relative, i.e. `0` is black, `1` is pure white, but
/// `Oklab` is scale independent -- i.e. the luminosity of `luminance == 1.0` is undefined.
/// Lightness values may mean different things in different contexts (maximum display
/// luminosity, background brightness and other viewing conditions).
///
/// *sRGB* however has a well defined dynamic range and a
/// [D65](https://en.wikipedia.org/wiki/Illuminant_D65) reference white luminance.
/// Mapping `1` to that luminance is just a matter of definition. But is say `0.8` `Oklab`
/// lightness equal to `0.5` or `0.9` `sRGB` luminance?
///
/// The shape and weights of `L_r` are chosen to closely matches the lightness estimate of
/// the `CIELab` color space and be nearly equal at `0.5`.
///
/// Inverse of [`toe_inv`]
///
/// # See
/// https://bottosson.github.io/posts/colorpicker/#intermission---a-new-lightness-estimate-for-oklab
pub(crate) fn toe<T>(oklab_lightness: T) -> T
where
    T: Real + Powi + Sqrt + Arithmetics + One + Clone,
{
    let k_1 = T::from_f64(0.206);
    let k_2 = T::from_f64(0.03);
    let k_3 = (T::one() + &k_1) / (T::one() + &k_2);
    T::from_f64(0.5)
        * (k_3.clone() * &oklab_lightness - &k_1
            + T::sqrt(
                (k_3.clone() * &oklab_lightness - k_1).powi(2)
                    + T::from_f64(4.0) * k_2 * k_3 * oklab_lightness,
            ))
}

/// Maps a *sRGB* reference-white based lightness to `Oklab`s scale-independent luminance.
///
/// Inverse of [`toe`]
pub(crate) fn toe_inv<T>(l_r: T) -> T
where
    T: Real + Powi + Arithmetics + One + Clone,
{
    let k_1 = T::from_f64(0.206);
    let k_2 = T::from_f64(0.03);
    let k_3 = (T::one() + &k_1) / (T::one() + &k_2);
    (l_r.clone().powi(2) + k_1 * &l_r) / (k_3 * (l_r + k_2))
}

#[cfg(feature = "approx")]
#[cfg(test)]
mod tests {

    use super::*;
    use crate::convert::FromColorUnclamped;
    use crate::rgb::Rgb;
    use crate::{encoding, Oklab, OklabHue, Srgb};
    use core::str::FromStr;

    #[cfg_attr(miri, ignore)]
    #[test]
    fn test_roundtrip_toe_is_original() {
        let n = 500;
        for i in 0..n {
            let x = i as f64 / n as f64;
            assert_ulps_eq!(toe_inv(toe(x)), x);
        }

        let x = 1000.0;
        assert_ulps_eq!(toe_inv(toe(x)), x);
    }

    #[test]
    fn test_toe() {
        assert_eq!(toe(0.0), 0.0);
        assert_eq!(toe(1.0), 1.0);
        let grey50srgb: Srgb = Rgb::<encoding::Srgb, u8>::from_str("#777777")
            .unwrap()
            .into_format();
        let grey50oklab = Oklab::from_color_unclamped(grey50srgb);
        println!("grey 50% oklab lightness: {}", grey50oklab.l);
        assert_relative_eq!(toe(grey50oklab.l), 0.5, epsilon = 1e-3);
    }

    #[cfg_attr(miri, ignore)]
    #[test]
    fn print_min_max_srgb_chroma_of_all_hues() {
        struct HueLc<T: Real> {
            hue: OklabHue<T>,
            lc: LC<T>,
        }

        let mut min_chroma: HueLc<f64> = HueLc {
            hue: OklabHue::new(f64::NAN),
            lc: LC {
                lightness: 0.0,
                chroma: f64::INFINITY,
            },
        };
        let mut max_chroma: HueLc<f64> = HueLc {
            hue: OklabHue::new(f64::NAN),
            lc: LC {
                lightness: 0.0,
                chroma: 0.0,
            },
        };
        let mut min_a = f64::INFINITY;
        let mut min_b = f64::INFINITY;
        let mut max_a = -f64::INFINITY;
        let mut max_b = -f64::INFINITY;

        // use 300000 for actually computing values (takes < 10 seconds)
        const SAMPLE_RESOLUTION: usize = 3;

        for i in 0..SAMPLE_RESOLUTION * 360 {
            let hue: OklabHue<f64> = OklabHue::new(i as f64 / (SAMPLE_RESOLUTION as f64));
            let (lc, a, b) = hue.srgb_limits();
            if lc.chroma < min_chroma.lc.chroma {
                min_chroma = HueLc { hue, lc };
            }
            if lc.chroma > max_chroma.lc.chroma {
                max_chroma = HueLc { hue, lc };
            }
            max_a = f64::max(max_a, a);
            min_a = f64::min(min_a, a);
            max_b = f64::max(max_b, b);
            min_b = f64::min(min_b, b);
        }

        let (normalized_a, normalized_b) = max_chroma.hue.into_cartesian();
        let (max_chroma_a, max_chroma_b) = (
            normalized_a * max_chroma.lc.chroma,
            normalized_b * max_chroma.lc.chroma,
        );

        println!(
            "Min chroma {} at hue {:?}°.",
            min_chroma.lc.chroma, min_chroma.hue,
        );

        println!(
            "Max chroma {} at hue {:?}° (Oklab a and b {}, {}).",
            max_chroma.lc.chroma, max_chroma.hue, max_chroma_a, max_chroma_b
        );
        println!("{} <= a <= {}", min_a, max_a);
        println!("{} <= b <= {}", min_b, max_b);
    }

    #[test]
    fn max_saturation_f64_eq_f32() {
        let lin_srgb = LinSrgb::new(0.0, 0.0, 1.0);
        let oklab_64 = Oklab::<f64>::from_color_unclamped(lin_srgb);
        let (normalized_a, normalized_b) = (
            oklab_64.a / oklab_64.get_chroma(),
            oklab_64.b / oklab_64.get_chroma(),
        );
        let saturation_64 = LC::max_saturation(normalized_a, normalized_b);
        let saturation_32 = LC::max_saturation(normalized_a as f32, normalized_b as f32);

        // EPSILON should be 1e-6. See issue https://github.com/Ogeon/palette/issues/296
        const EPSILON: f32 = 3e-1;
        assert_relative_eq!(
            saturation_32,
            saturation_64 as f32,
            epsilon = EPSILON,
            max_relative = EPSILON
        );
    }
}
