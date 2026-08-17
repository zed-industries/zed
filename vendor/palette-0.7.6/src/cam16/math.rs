use core::{
    marker::PhantomData,
    ops::{Div, Mul},
};

use crate::{
    angle::{RealAngle, SignedAngle},
    bool_mask::{LazySelect, Select},
    clamp,
    hues::Cam16Hue,
    num::{
        Abs, Arithmetics, Clamp, Exp, FromScalar, One, PartialCmp, Powf, Real, Signum, Sqrt,
        Trigonometry, Zero,
    },
    white_point,
    xyz::Xyz,
};

use super::{Cam16, Parameters};

use self::{chromaticity::ChromaticityType, luminance::LuminanceType};

pub(crate) mod chromaticity;
pub(crate) mod luminance;

// This module is originally based on these sources:
// - https://observablehq.com/@jrus/cam16
// - "Comprehensive color solutions: CAM16, CAT16, and CAM16-UCS" by Li C, Li Z,
//   Wang Z, et al.
//   (https://www.researchgate.net/publication/318152296_Comprehensive_color_solutions_CAM16_CAT16_and_CAM16-UCS)
// - "Algorithmic improvements for the CIECAM02 and CAM16 color appearance
//   models" by Nico Schlömer (https://arxiv.org/pdf/1802.06067.pdf)
// - https://rawpedia.rawtherapee.com/CIECAM02.
// - "Usage Guidelines for CIECAM97s" by Nathan Moroney
//   (https://www.imaging.org/common/uploaded%20files/pdfs/Papers/2000/PICS-0-81/1611.pdf)
// - "CIECAM02 and Its Recent Developments" by Ming Ronnier Luo and Changjun Li
//   (https://cielab.xyz/pdf/CIECAM02_and_Its_Recent_Developments.pdf)
// - https://en.wikipedia.org/wiki/CIECAM02

pub(crate) fn xyz_to_cam16<T>(
    xyz: Xyz<white_point::Any, T>,
    parameters: DependentParameters<T::Scalar>,
) -> Cam16<T>
where
    T: Real
        + FromScalar
        + Arithmetics
        + Powf
        + Sqrt
        + Abs
        + Signum
        + Trigonometry
        + RealAngle
        + Clone,
    T::Scalar: Clone,
{
    let xyz = xyz.with_white_point() * T::from_f64(100.0); // The reference uses 0.0 to 100.0 instead of 0.0 to 1.0.
    let d_rgb = map3(parameters.d_rgb.clone(), T::from_scalar);

    let [r_a, g_a, b_a] = map3(mul3(m16(xyz), d_rgb), |component| {
        parameters.adapt.run(component)
    });
    let a = r_a.clone() + (T::from_f64(-12.0) * &g_a + &b_a) / T::from_f64(11.0); // redness-greenness
    let b = (r_a.clone() + &g_a - T::from_f64(2.0) * &b_a) / T::from_f64(9.0); // yellowness-blueness
    let h_rad = b.clone().atan2(a.clone()); // hue in radians
    let h = Cam16Hue::from_radians(h_rad.clone()); // hue in degrees
    let e_t = T::from_f64(0.25) * (T::cos(h_rad + T::from_f64(2.0)) + T::from_f64(3.8));
    let capital_a = T::from_scalar(parameters.n_bb)
        * (T::from_f64(2.0) * &r_a + &g_a + T::from_f64(0.05) * &b_a);
    let j_root = (capital_a / T::from_scalar(parameters.a_w.clone())).powf(
        T::from_f64(0.5) * T::from_scalar(parameters.c.clone()) * T::from_scalar(parameters.z),
    );

    let j = calculate_lightness(j_root.clone()); // lightness
    let q = calculate_brightness(
        j_root.clone(),
        T::from_scalar(parameters.c.clone()),
        T::from_scalar(parameters.a_w.clone()),
        T::from_scalar(parameters.f_l_4.clone()),
    ); // brightness

    let t = T::from_f64(5e4) / T::from_f64(13.0)
        * T::from_scalar(parameters.n_c)
        * T::from_scalar(parameters.n_cb)
        * e_t
        * (a.clone() * a + b.clone() * b).sqrt()
        / (r_a + g_a + T::from_f64(1.05) * b_a + T::from_f64(0.305));
    let alpha = t.powf(T::from_f64(0.9))
        * (T::from_f64(1.64) - T::from_f64(0.29).powf(T::from_scalar(parameters.n)))
            .powf(T::from_f64(0.73));

    let c = calculate_chroma(j_root, alpha.clone()); // chroma
    let m = calculate_colorfulness(T::from_scalar(parameters.f_l_4), c.clone()); // colorfulness
    let s = calculate_saturation(
        T::from_scalar(parameters.c),
        T::from_scalar(parameters.a_w),
        alpha,
    ); // saturation

    Cam16 {
        lightness: j,
        chroma: c,
        hue: h,
        brightness: q,
        colorfulness: m,
        saturation: s,
    }
}

fn calculate_lightness<T>(j_root: T) -> T
where
    T: Real + Arithmetics,
{
    T::from_f64(100.0) * &j_root * j_root
}

fn calculate_brightness<T>(j_root: T, param_c: T, param_a_w: T, param_f_l_4: T) -> T
where
    T: Real + Arithmetics,
{
    T::from_f64(4.0) / param_c * j_root * (T::from_f64(4.0) + param_a_w) * param_f_l_4
}

#[inline]
pub(super) fn calculate_chroma<T>(j_root: T, alpha: T) -> T
where
    T: Mul<T, Output = T>,
{
    j_root * alpha
}

#[inline]
pub(super) fn calculate_colorfulness<T>(param_f_l_4: T, chroma: T) -> T
where
    T: Mul<T, Output = T>,
{
    param_f_l_4 * chroma
}

#[inline]
pub(super) fn calculate_saturation<T>(param_c: T, param_a_w: T, alpha: T) -> T
where
    T: Real + Arithmetics + Sqrt,
{
    T::from_f64(50.0) * (param_c * alpha / (param_a_w + T::from_f64(4.0))).sqrt()
}

#[inline]
pub(crate) fn cam16_to_xyz<T>(
    cam16: (LuminanceType<T>, ChromaticityType<T>, Cam16Hue<T>),
    parameters: DependentParameters<T::Scalar>,
) -> Xyz<white_point::Any, T>
where
    T: Real
        + FromScalar
        + One
        + Zero
        + Sqrt
        + Powf
        + Abs
        + Signum
        + Arithmetics
        + Trigonometry
        + RealAngle
        + SignedAngle
        + PartialCmp
        + Clone,
    T::Mask: LazySelect<T> + Clone,
    T::Scalar: Clone,
{
    // Weird naming, but we just want to know if it's black or not here.
    let is_black = match &cam16.0 {
        LuminanceType::Lightness(lightness) => lightness.eq(&T::zero()),
        LuminanceType::Brightness(brightness) => brightness.eq(&T::zero()),
    };

    let xyz = non_black_cam16_to_xyz(cam16, parameters);
    Xyz {
        x: is_black.clone().select(T::zero(), xyz.x),
        y: is_black.clone().select(T::zero(), xyz.y),
        z: is_black.select(T::zero(), xyz.z),
        white_point: PhantomData,
    }
}

// Assumes that lightness has been checked to be non-zero in `cam16_to_xyz`.
fn non_black_cam16_to_xyz<T>(
    cam16: (LuminanceType<T>, ChromaticityType<T>, Cam16Hue<T>),
    parameters: DependentParameters<T::Scalar>,
) -> Xyz<white_point::Any, T>
where
    T: Real
        + FromScalar
        + One
        + Sqrt
        + Powf
        + Abs
        + Signum
        + Arithmetics
        + Trigonometry
        + RealAngle
        + SignedAngle
        + Clone,
    T::Scalar: Clone,
{
    let h_rad = cam16.2.into_radians();
    let (sin_h, cos_h) = h_rad.clone().sin_cos();
    let j_root = match cam16.0 {
        LuminanceType::Lightness(j) => lightness_to_j_root(j),
        LuminanceType::Brightness(q) => brightness_to_j_root(
            q,
            T::from_scalar(parameters.c.clone()),
            T::from_scalar(parameters.a_w.clone()),
            T::from_scalar(parameters.f_l_4.clone()),
        ),
    };
    let alpha = match cam16.1 {
        ChromaticityType::Chroma(c) => c / &j_root,
        ChromaticityType::Colorfulness(m) => {
            colorfulness_to_chroma(m, T::from_scalar(parameters.f_l_4)) / &j_root
        }
        ChromaticityType::Saturation(s) => saturation_to_alpha(
            s,
            T::from_scalar(parameters.c.clone()),
            T::from_scalar(parameters.a_w.clone()),
        ),
    };
    let t = (alpha
        * (T::from_f64(1.64) - T::from_f64(0.29).powf(T::from_scalar(parameters.n)))
            .powf(T::from_f64(-0.73)))
    .powf(T::from_f64(10.0) / T::from_f64(9.0));
    let e_t = T::from_f64(0.25) * ((h_rad + T::from_f64(2.0)).cos() + T::from_f64(3.8));
    let capital_a = T::from_scalar(parameters.a_w)
        * j_root
            .powf(T::from_f64(2.0) / T::from_scalar(parameters.c) / T::from_scalar(parameters.z));
    let p_1 = T::from_f64(5e4) / T::from_f64(13.0)
        * T::from_scalar(parameters.n_c)
        * T::from_scalar(parameters.n_cb)
        * e_t;
    let p_2 = capital_a / T::from_scalar(parameters.n_bb);
    let r = T::from_f64(23.0) * (T::from_f64(0.305) + &p_2) * &t
        / (T::from_f64(23.0) * p_1
            + t * (T::from_f64(11.0) * &cos_h + T::from_f64(108.0) * &sin_h));
    let a = cos_h * &r;
    let b = sin_h * r;
    let denom = T::one() / T::from_f64(1403.0);
    let rgb_c = [
        (T::from_f64(460.0) * &p_2 + T::from_f64(451.0) * &a + T::from_f64(288.0) * &b) * &denom,
        (T::from_f64(460.0) * &p_2 - T::from_f64(891.0) * &a - T::from_f64(261.0) * &b) * &denom,
        (T::from_f64(460.0) * p_2 - T::from_f64(220.0) * a - T::from_f64(6300.0) * b) * &denom,
    ];

    let unadapt = parameters.unadapt;
    let rgb_c = map3(rgb_c, |component| unadapt.run(component));
    let d_rgb_inv = map3(parameters.d_rgb_inv, T::from_scalar);

    m16_inv(mul3(rgb_c, d_rgb_inv)) / T::from_f64(100.0) // The reference uses 0.0 to 100.0 instead of 0.0 to 1.0.
}

pub(super) fn prepare_parameters<T>(
    parameters: Parameters<Xyz<white_point::Any, T>, T>,
) -> DependentParameters<T>
where
    T: Real
        + FromScalar<Scalar = T>
        + One
        + Zero
        + Clamp
        + PartialCmp
        + Arithmetics
        + Powf
        + Sqrt
        + Exp
        + Abs
        + Signum
        + Clone,
    T::Mask: LazySelect<T>,
{
    // Compute dependent parameters.
    let xyz_w = parameters.white_point * T::from_f64(100.0); // The reference uses 0.0 to 100.0 instead of 0.0 to 1.0.
    let l_a = parameters.adapting_luminance;
    let y_b = parameters.background_luminance * T::from_f64(100.0); // The reference uses 0.0 to 100.0 instead of 0.0 to 1.0.
    let y_w = xyz_w.y.clone();
    let surround = parameters.surround.into_percent() * T::from_f64(0.1);
    let c = lazy_select! {
        if surround.gt_eq(&T::one()) => lerp(
            T::from_f64(0.59),
            T::from_f64(0.69),
            surround.clone() - T::one(),
        ),
        else => lerp(T::from_f64(0.525), T::from_f64(0.59), surround.clone())
    };
    let f = lazy_select! {
        if c.gt_eq(&T::from_f64(0.59)) => lerp(
            T::from_f64(0.9),
            T::one(),
            (c.clone() - T::from_f64(0.59)) / T::from_f64(0.1)),
        else => lerp(
            T::from_f64(0.8),
            T::from_f64(0.9),
            (c.clone() - T::from_f64(0.525)) / T::from_f64(0.065)
        )
    };
    let n_c = f.clone();
    let k = T::one() / (T::from_f64(5.0) * &l_a + T::one());
    let f_l = {
        // Luminance adaptation factor
        let k4 = k.clone() * &k * &k * k;
        let k4_inv = T::one() - &k4;
        let a_third = T::one() / T::from_f64(3.0);

        k4 * &l_a + T::from_f64(0.1) * &k4_inv * k4_inv * (T::from_f64(5.0) * &l_a).powf(a_third)
    };
    let f_l_4 = f_l.clone().powf(T::from_f64(0.25));
    let n = y_b / &y_w;
    let z = T::from_f64(1.48) + n.clone().sqrt(); // Lightness non-linearity exponent (modified by `c`).
    let n_bb = T::from_f64(0.725) * n.clone().powf(T::from_f64(-0.2)); // Chromatic induction factors
    let n_cb = n_bb.clone();
    // Illuminant discounting (adaptation). Fully adapted = 1
    let d = match parameters.discounting {
        super::Discounting::Auto => {
            // The default D function.
            f * (T::one()
                - T::one() / T::from_f64(3.6)
                    * Exp::exp((-l_a - T::from_f64(42.0)) / T::from_f64(92.0)))
        }
        super::Discounting::Custom(degree) => degree,
    };

    let d = clamp(d, T::zero(), T::one());

    let rgb_w = m16(xyz_w); // Cone responses of the white point
    let d_rgb = map3(rgb_w.clone(), |c_w| {
        lerp(T::one(), y_w.clone() / c_w, d.clone())
    });
    let d_rgb_inv = map3(d_rgb.clone(), |d_c| T::one() / d_c);
    let rgb_cw = mul3(rgb_w, d_rgb.clone());

    let adapt = Adapt { f_l: f_l.clone() };

    let exponent = T::one() / T::from_f64(0.42);
    let unadapt = Unadapt {
        constant: T::from_f64(100.0) / f_l * T::from_f64(27.13).powf(exponent.clone()),
        exponent,
    };

    let [rgb_aw1, rgb_aw2, rgb_aw3] = map3(rgb_cw, |component| adapt.run(component));
    let a_w = n_bb.clone() * (T::from_f64(2.0) * rgb_aw1 + rgb_aw2 + T::from_f64(0.05) * rgb_aw3);

    DependentParameters {
        d_rgb,
        d_rgb_inv,
        n,
        n_bb,
        n_c,
        n_cb,
        a_w,
        c,
        z,
        f_l_4,
        adapt,
        unadapt,
    }
}

#[inline]
pub(super) fn lightness_to_brightness<T>(
    lightness: T,
    param_c: T,
    param_a_w: T,
    param_f_l_4: T,
) -> T
where
    T: Real + Arithmetics + Sqrt,
{
    let j_root = lightness_to_j_root(lightness);
    calculate_brightness(j_root, param_c, param_a_w, param_f_l_4)
}

#[inline]
pub(super) fn brightness_to_lightness<T>(
    brightness: T,
    param_c: T,
    param_a_w: T,
    param_f_l_4: T,
) -> T
where
    T: Real + Arithmetics,
{
    let j_root = brightness_to_j_root(brightness, param_c, param_a_w, param_f_l_4);
    calculate_lightness(j_root)
}

#[inline]
pub(super) fn chroma_to_colorfulness<T>(chroma: T, param_f_l_4: T) -> T
where
    T: Mul<T, Output = T>,
{
    param_f_l_4 * chroma
}

#[inline]
pub(super) fn chroma_to_saturation<T>(chroma: T, lightness: T, param_c: T, param_a_w: T) -> T
where
    T: Real + Arithmetics + Sqrt + Clone,
{
    let j_root = lightness_to_j_root(lightness);
    let alpha = chroma / &j_root;

    calculate_saturation(param_c, param_a_w, alpha)
}

#[inline]
pub(super) fn colorfulness_to_chroma<T>(colorfulness: T, param_f_l_4: T) -> T
where
    T: Div<T, Output = T>,
{
    colorfulness / param_f_l_4
}

#[inline]
pub(super) fn saturation_to_chroma<T>(saturation: T, lightness: T, param_c: T, param_a_w: T) -> T
where
    T: Real + Arithmetics + Sqrt,
{
    let j_root = lightness_to_j_root(lightness);
    let alpha = saturation_to_alpha(saturation, param_c, param_a_w);

    calculate_chroma(j_root, alpha)
}

#[inline]
fn lightness_to_j_root<T>(lightness: T) -> T
where
    T: Real + Mul<T, Output = T> + Sqrt,
{
    lightness.sqrt() * T::from_f64(0.1)
}

#[inline]
fn brightness_to_j_root<T>(brightness: T, param_c: T, param_a_w: T, param_f_l_4: T) -> T
where
    T: Real + Arithmetics,
{
    T::from_f64(0.25) * param_c * brightness / ((T::from_f64(4.0) + param_a_w) * param_f_l_4)
}

#[inline]
fn saturation_to_alpha<T>(saturation: T, param_c: T, param_a_w: T) -> T
where
    T: Real + Arithmetics,
{
    T::from_f64(0.0004) * &saturation * saturation * (T::from_f64(4.0) + param_a_w) / param_c
}

#[derive(Clone, Copy)]
pub(crate) struct DependentParameters<T> {
    d_rgb: [T; 3],
    d_rgb_inv: [T; 3],
    n: T,
    n_bb: T,
    n_c: T,
    n_cb: T,
    pub(super) a_w: T,
    pub(super) c: T,
    z: T,
    pub(super) f_l_4: T,
    adapt: Adapt<T>,
    unadapt: Unadapt<T>,
}

#[derive(Clone, Copy)]
struct Adapt<T> {
    f_l: T,
}

impl<T> Adapt<T> {
    fn run<V>(&self, component: V) -> V
    where
        V: Real + FromScalar<Scalar = T> + Abs + Signum + Powf + Arithmetics + Clone,
        T: Clone,
    {
        let x = (V::from_scalar(self.f_l.clone()) * component.clone().abs() * V::from_f64(0.01))
            .powf(V::from_f64(0.42));
        component.signum() * V::from_f64(400.0) * &x / (x + V::from_f64(27.13))
    }
}

#[derive(Clone, Copy)]
struct Unadapt<T> {
    constant: T,
    exponent: T,
}

impl<T> Unadapt<T> {
    fn run<V>(&self, component: V) -> V
    where
        V: Real + FromScalar<Scalar = T> + Abs + Signum + Powf + Arithmetics + Clone,
        T: Clone,
    {
        let c_abs = component.clone().abs();
        component.signum()
            * V::from_scalar(self.constant.clone())
            * (c_abs.clone() / (V::from_f64(400.0) - c_abs))
                .powf(V::from_scalar(self.exponent.clone()))
    }
}

fn lerp<T>(from: T, to: T, factor: T) -> T
where
    T: One + Arithmetics,
{
    (T::one() - &factor) * from + factor * to
}

fn m16<T>(xyz: Xyz<white_point::Any, T>) -> [T; 3]
where
    T: Real + Arithmetics,
{
    let Xyz { x, y, z, .. } = xyz;

    #[rustfmt::skip]
    let rgb = [
        T::from_f64( 0.401288) * &x + T::from_f64(0.650173) * &y - T::from_f64(0.051461) * &z,
        T::from_f64(-0.250268) * &x + T::from_f64(1.204414) * &y + T::from_f64(0.045854) * &z,
        T::from_f64(-0.002079) *  x + T::from_f64(0.048952) *  y + T::from_f64(0.953127) *  z,
    ];

    rgb
}

fn m16_inv<T>(rgb: [T; 3]) -> Xyz<white_point::Any, T>
where
    T: Real + Arithmetics,
{
    let [r, g, b] = rgb;

    #[rustfmt::skip]
    #[allow(clippy::excessive_precision)] // Clippy didn't like the e+0
    let xyz = Xyz {
        x: T::from_f64( 1.862067855087233e+0) * &r - T::from_f64(1.011254630531685e+0) * &g + T::from_f64(1.491867754444518e-1) * &b,
        y: T::from_f64( 3.875265432361372e-1) * &r + T::from_f64(6.214474419314753e-1) * &g - T::from_f64(8.973985167612518e-3) * &b,
        z: T::from_f64(-1.584149884933386e-2) *  r - T::from_f64(3.412293802851557e-2) *  g + T::from_f64(1.049964436877850e+0) *  b,
        white_point: PhantomData
    };

    xyz
}

fn map3<T, U>(array: [T; 3], mut map: impl FnMut(T) -> U) -> [U; 3] {
    let [a1, a2, a3] = array;
    [map(a1), map(a2), map(a3)]
}

fn mul3<T>(lhs: [T; 3], rhs: [T; 3]) -> [T; 3]
where
    T: Mul<T, Output = T>,
{
    let [l1, l2, l3] = lhs;
    let [r1, r2, r3] = rhs;

    [l1 * r1, l2 * r2, l3 * r3]
}
