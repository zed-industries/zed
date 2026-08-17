use core::ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[cfg(feature = "approx")]
use approx::{AbsDiffEq, RelativeEq, UlpsEq};

use crate::{
    cast::ArrayCast,
    clamp,
    num::{self, Arithmetics, One, Real, Zero},
    stimulus::Stimulus,
    Alpha, ArrayExt, Mix, MixAssign, NextArray,
};

use super::Premultiply;

/// Premultiplied alpha wrapper.
///
/// Premultiplied, or alpha masked, or associated alpha colors have had their
/// component values multiplied with their alpha value. They are commonly used
/// in composition algorithms and as output from computer generated graphics. It
/// may also be preferred when interpolating between colors and in other image
/// manipulation operations, such as blurring or resizing images.
///
/// ```
/// use palette::{LinSrgb, LinSrgba};
/// use palette::blend::{Blend, PreAlpha};
///
/// let a = PreAlpha::from(LinSrgba::new(0.4, 0.5, 0.5, 0.3));
/// let b = PreAlpha::from(LinSrgba::new(0.3, 0.8, 0.4, 0.4));
/// let c = PreAlpha::from(LinSrgba::new(0.7, 0.1, 0.8, 0.8));
///
/// let res: LinSrgba = a.screen(b).overlay(c).into();
/// ```
///
/// Note that converting to and from premultiplied alpha will cause the alpha
/// component to be clamped to [0.0, 1.0], and fully transparent colors will
/// become black.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PreAlpha<C: Premultiply> {
    /// The premultiplied color components (`original.color * original.alpha`).
    pub color: C,

    /// The transparency component. 0.0 is fully transparent and 1.0 is fully
    /// opaque.
    pub alpha: C::Scalar,
}

impl<C> PreAlpha<C>
where
    C: Premultiply,
{
    /// Alpha mask `color` with `alpha`.
    pub fn new(color: C, alpha: C::Scalar) -> Self {
        color.premultiply(alpha)
    }

    /// Create an opaque alpha masked color.
    pub fn new_opaque(color: C) -> Self
    where
        C::Scalar: Stimulus,
    {
        Self {
            color,
            alpha: C::Scalar::max_intensity(),
        }
    }

    /// Alpha unmask the color.
    pub fn unpremultiply(self) -> Alpha<C, C::Scalar> {
        let (color, alpha) = C::unpremultiply(self);
        Alpha { color, alpha }
    }
}

impl<C> PartialEq for PreAlpha<C>
where
    C: PartialEq + Premultiply,
    C::Scalar: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color && self.alpha == other.alpha
    }
}

impl<C> Eq for PreAlpha<C>
where
    C: Eq + Premultiply,
    C::Scalar: Eq,
{
}

impl<C> From<Alpha<C, C::Scalar>> for PreAlpha<C>
where
    C: Premultiply,
{
    #[inline]
    fn from(color: Alpha<C, C::Scalar>) -> Self {
        color.color.premultiply(color.alpha)
    }
}

impl<C> From<PreAlpha<C>> for Alpha<C, C::Scalar>
where
    C: Premultiply,
{
    #[inline]
    fn from(color: PreAlpha<C>) -> Self {
        let (color, alpha) = C::unpremultiply(color);
        Alpha { color, alpha }
    }
}

impl<C> From<C> for PreAlpha<C>
where
    C: Premultiply,
    C::Scalar: Stimulus,
{
    fn from(color: C) -> Self {
        color.premultiply(C::Scalar::max_intensity())
    }
}

impl<C, T> Mix for PreAlpha<C>
where
    C: Mix<Scalar = T> + Premultiply<Scalar = T>,
    T: Real + Zero + One + num::Clamp + Arithmetics + Clone,
{
    type Scalar = T;

    #[inline]
    fn mix(mut self, other: Self, factor: T) -> Self {
        let factor = clamp(factor, T::zero(), T::one());

        self.color = self.color.mix(other.color, factor.clone());
        self.alpha = self.alpha.clone() + factor * (other.alpha - self.alpha);

        self
    }
}

impl<C, T> MixAssign for PreAlpha<C>
where
    C: MixAssign<Scalar = T> + Premultiply<Scalar = T>,
    T: Real + Zero + One + num::Clamp + Arithmetics + AddAssign + Clone,
{
    type Scalar = T;

    #[inline]
    fn mix_assign(&mut self, other: Self, factor: T) {
        let factor = clamp(factor, T::zero(), T::one());

        self.color.mix_assign(other.color, factor.clone());
        self.alpha += factor * (other.alpha - self.alpha.clone());
    }
}

unsafe impl<C, T> ArrayCast for PreAlpha<C>
where
    C: ArrayCast + Premultiply<Scalar = T>,
    C::Array: NextArray + ArrayExt<Item = T>,
{
    type Array = <C::Array as NextArray>::Next;
}

impl<C> Default for PreAlpha<C>
where
    C: Default + Premultiply,
    C::Scalar: Stimulus,
{
    fn default() -> PreAlpha<C> {
        PreAlpha {
            color: C::default(),
            alpha: C::Scalar::max_intensity(),
        }
    }
}

#[cfg(feature = "approx")]
impl<C, T> AbsDiffEq for PreAlpha<C>
where
    C: AbsDiffEq<Epsilon = T::Epsilon> + Premultiply<Scalar = T>,
    T: AbsDiffEq,
    T::Epsilon: Clone,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &PreAlpha<C>, epsilon: Self::Epsilon) -> bool {
        self.color.abs_diff_eq(&other.color, epsilon.clone())
            && self.alpha.abs_diff_eq(&other.alpha, epsilon)
    }
}

#[cfg(feature = "approx")]
impl<C, T> RelativeEq for PreAlpha<C>
where
    C: RelativeEq<Epsilon = T::Epsilon> + Premultiply<Scalar = T>,
    T: RelativeEq,
    T::Epsilon: Clone,
{
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &PreAlpha<C>,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.color
            .relative_eq(&other.color, epsilon.clone(), max_relative.clone())
            && self.alpha.relative_eq(&other.alpha, epsilon, max_relative)
    }
}

#[cfg(feature = "approx")]
impl<C, T> UlpsEq for PreAlpha<C>
where
    C: UlpsEq<Epsilon = T::Epsilon> + Premultiply<Scalar = T>,
    T: UlpsEq,
    T::Epsilon: Clone,
{
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    fn ulps_eq(&self, other: &PreAlpha<C>, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.color.ulps_eq(&other.color, epsilon.clone(), max_ulps)
            && self.alpha.ulps_eq(&other.alpha, epsilon, max_ulps)
    }
}

macro_rules! impl_binop {
    (
        $op_trait:ident::$op_trait_fn:ident,
        $op_assign_trait:ident::$op_assign_trait_fn:ident
    ) => {
        impl<C> $op_trait for PreAlpha<C>
        where
            C: $op_trait<Output = C> + Premultiply,
            C::Scalar: $op_trait<Output = C::Scalar>,
        {
            type Output = PreAlpha<C>;

            fn $op_trait_fn(self, other: PreAlpha<C>) -> Self::Output {
                PreAlpha {
                    color: self.color.$op_trait_fn(other.color),
                    alpha: self.alpha.$op_trait_fn(other.alpha),
                }
            }
        }

        impl<C> $op_assign_trait for PreAlpha<C>
        where
            C: $op_assign_trait + Premultiply,
            C::Scalar: $op_assign_trait + Real,
        {
            fn $op_assign_trait_fn(&mut self, other: PreAlpha<C>) {
                self.color.$op_assign_trait_fn(other.color);
                self.alpha.$op_assign_trait_fn(other.alpha);
            }
        }
    };
}

impl_binop!(Add::add, AddAssign::add_assign);
impl_binop!(Sub::sub, SubAssign::sub_assign);
impl_binop!(Mul::mul, MulAssign::mul_assign);
impl_binop!(Div::div, DivAssign::div_assign);

macro_rules! impl_scalar_binop {
    (
        $op_trait:ident::$op_trait_fn:ident,
        $op_assign_trait:ident::$op_assign_trait_fn:ident,
        [$($ty:ident),+]
    ) => {
        $(
            impl<C> $op_trait<$ty> for PreAlpha<C>
            where
                C: $op_trait<$ty, Output = C> + Premultiply<Scalar = $ty>,
            {
                type Output = PreAlpha<C>;

                fn $op_trait_fn(self, c: $ty) -> Self::Output {
                    PreAlpha {
                        color: self.color.$op_trait_fn(c),
                        alpha: self.alpha.$op_trait_fn(c),
                    }
                }
            }

            // // Disabled as work-around for https://github.com/Ogeon/palette/issues/283
            // // Blocked by https://github.com/rust-lang/rust/issues/80542
            // impl<C> $op_trait<PreAlpha<C>> for $ty
            // where
            //     C: Premultiply<Scalar = $ty>,
            //     $ty: $op_trait<$ty, Output = $ty> + $op_trait<C, Output = C>,
            // {
            //     type Output = PreAlpha<C>;
            //
            //     fn $op_trait_fn(self, color: PreAlpha<C>) -> Self::Output {
            //         PreAlpha {
            //             color: $op_trait::<C>::$op_trait_fn(self, color.color),
            //             alpha: $op_trait::<$ty>::$op_trait_fn(self, color.alpha),
            //         }
            //     }
            // }

            impl<C> $op_assign_trait<$ty> for PreAlpha<C>
            where
                C: $op_assign_trait<$ty> + Premultiply<Scalar = $ty>,
            {
                fn $op_assign_trait_fn(&mut self, c: $ty) {
                    self.color.$op_assign_trait_fn(c);
                    self.alpha.$op_assign_trait_fn(c);
                }
            }
        )+
    };
}

impl_scalar_binop!(Add::add, AddAssign::add_assign, [f32, f64]);
impl_scalar_binop!(Sub::sub, SubAssign::sub_assign, [f32, f64]);
impl_scalar_binop!(Mul::mul, MulAssign::mul_assign, [f32, f64]);
impl_scalar_binop!(Div::div, DivAssign::div_assign, [f32, f64]);

impl_array_casts!([C: Premultiply, const N: usize] PreAlpha<C>, [C::Scalar; N], where PreAlpha<C>: ArrayCast<Array = [C::Scalar; N]>);

impl<C: Premultiply> Deref for PreAlpha<C> {
    type Target = C;

    fn deref(&self) -> &C {
        &self.color
    }
}

impl<C: Premultiply> DerefMut for PreAlpha<C> {
    fn deref_mut(&mut self) -> &mut C {
        &mut self.color
    }
}

#[cfg(feature = "serializing")]
impl<C> serde::Serialize for PreAlpha<C>
where
    C: Premultiply + serde::Serialize,
    C::Scalar: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.color.serialize(crate::serde::AlphaSerializer {
            inner: serializer,
            alpha: &self.alpha,
        })
    }
}

#[cfg(feature = "serializing")]
impl<'de, C> serde::Deserialize<'de> for PreAlpha<C>
where
    C: Premultiply + serde::Deserialize<'de>,
    C::Scalar: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut alpha: Option<C::Scalar> = None;

        let color = C::deserialize(crate::serde::AlphaDeserializer {
            inner: deserializer,
            alpha: &mut alpha,
        })?;

        if let Some(alpha) = alpha {
            Ok(Self { color, alpha })
        } else {
            Err(serde::de::Error::missing_field("alpha"))
        }
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<C> bytemuck::Zeroable for PreAlpha<C>
where
    C: bytemuck::Zeroable + Premultiply,
    C::Scalar: bytemuck::Zeroable,
{
}

// Safety:
//
// See `Alpha<C, T>`'s implementation of `Pod`.
#[cfg(feature = "bytemuck")]
unsafe impl<C> bytemuck::Pod for PreAlpha<C>
where
    C: bytemuck::Pod + ArrayCast + Premultiply,
    C::Scalar: bytemuck::Pod,
{
}

#[cfg(test)]
#[cfg(feature = "serializing")]
mod test {
    use super::PreAlpha;
    use crate::LinSrgb;

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let color = PreAlpha {
            color: LinSrgb::new(0.3, 0.8, 0.1),
            alpha: 0.5,
        };

        assert_eq!(
            serde_json::to_string(&color).unwrap(),
            r#"{"red":0.3,"green":0.8,"blue":0.1,"alpha":0.5}"#
        );

        assert_eq!(
            ron::to_string(&color).unwrap(),
            r#"(red:0.3,green:0.8,blue:0.1,alpha:0.5)"#
        );
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let color = PreAlpha {
            color: LinSrgb::new(0.3, 0.8, 0.1),
            alpha: 0.5,
        };

        assert_eq!(
            serde_json::from_str::<PreAlpha<LinSrgb>>(
                r#"{"alpha":0.5,"red":0.3,"green":0.8,"blue":0.1}"#
            )
            .unwrap(),
            color
        );

        assert_eq!(
            ron::from_str::<PreAlpha<LinSrgb>>(r#"(alpha:0.5,red:0.3,green:0.8,blue:0.1)"#)
                .unwrap(),
            color
        );

        assert_eq!(
            ron::from_str::<PreAlpha<LinSrgb>>(r#"Rgb(alpha:0.5,red:0.3,green:0.8,blue:0.1)"#)
                .unwrap(),
            color
        );
    }
}
