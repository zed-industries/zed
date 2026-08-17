//! Hues and hue related types.

#[cfg(any(feature = "approx", feature = "random"))]
use core::ops::Mul;

use core::ops::{Add, AddAssign, Neg, Sub, SubAssign};

#[cfg(feature = "approx")]
use approx::{AbsDiffEq, RelativeEq, UlpsEq};

#[cfg(feature = "random")]
use rand::{
    distributions::{
        uniform::{SampleBorrow, SampleUniform, Uniform, UniformSampler},
        Distribution, Standard,
    },
    Rng,
};

#[cfg(feature = "approx")]
use crate::{angle::HalfRotation, num::Zero};

#[cfg(feature = "random")]
use crate::angle::FullRotation;

use crate::{
    angle::{AngleEq, FromAngle, RealAngle, SignedAngle, UnsignedAngle},
    num::Trigonometry,
};

macro_rules! make_hues {
    ($($(#[$doc:meta])+ struct $name:ident; $iter_name:ident)+) => ($(
        $(#[$doc])+
        ///
        /// The hue is a circular type, where `0` and `360` is the same, and
        /// it's normalized to `(-180, 180]` when it's converted to a linear
        /// number (like `f32`). This makes many calculations easier, but may
        /// also have some surprising effects if it's expected to act as a
        /// linear number.
        #[derive(Clone, Copy, Debug, Default)]
        #[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
        #[repr(C)]
        pub struct $name<T = f32>(T);

        impl<T> $name<T> {
            /// Create a new hue, specified in the default unit for the angle
            /// type `T`.
            ///
            /// `f32`, `f64` and other real number types represent degrees,
            /// while `u8` simply represents the range `[0, 360]` as `[0, 256]`.
            #[inline]
            pub const fn new(angle: T) -> Self {
                Self(angle)
            }

            /// Get the internal representation without normalizing or converting it.
            ///
            /// `f32`, `f64` and other real number types represent degrees,
            /// while `u8` simply represents the range `[0, 360]` as `[0, 256]`.
            pub fn into_inner(self) -> T {
                self.0
            }

            /// Convert into another angle type.
            pub fn into_format<U>(self) -> $name<U>
            where
                U: FromAngle<T>,
            {
                $name(U::from_angle(self.0))
            }

            /// Convert from another angle type.
            pub fn from_format<U>(hue: $name<U>) -> Self
            where
                T: FromAngle<U>,
            {
                hue.into_format()
            }
        }

        impl<T: RealAngle> $name<T> {
            /// Create a new hue from degrees. This is an alias for `new`.
            #[inline]
            pub fn from_degrees(degrees: T) -> Self {
                Self::new(degrees)
            }

            /// Create a new hue from radians, instead of degrees.
            #[inline]
            pub fn from_radians(radians: T) -> Self {
                Self(T::radians_to_degrees(radians))
            }

            /// Get the internal representation as degrees, without normalizing it.
            #[inline]
            pub fn into_raw_degrees(self) -> T {
                self.0
            }

            /// Get the internal representation as radians, without normalizing it.
            #[inline]
            pub fn into_raw_radians(self) -> T {
                T::degrees_to_radians(self.0)
            }
        }

        impl<T: RealAngle + SignedAngle> $name<T> {
            /// Get the hue as degrees, in the range `(-180, 180]`.
            #[inline]
            pub fn into_degrees(self) -> T {
                self.0.normalize_signed_angle()
            }

            /// Convert the hue to radians, in the range `(-π, π]`.
            #[inline]
            pub fn into_radians(self) -> T {
                T::degrees_to_radians(self.0.normalize_signed_angle())
            }
        }

        impl<T: RealAngle + UnsignedAngle> $name<T> {
            /// Convert the hue to positive degrees, in the range `[0, 360)`.
            #[inline]
            pub fn into_positive_degrees(self) -> T {
                self.0.normalize_unsigned_angle()
            }

            /// Convert the hue to positive radians, in the range `[0, 2π)`.
            #[inline]
            pub fn into_positive_radians(self) -> T {
                T::degrees_to_radians(self.0.normalize_unsigned_angle())
            }
        }

        impl<T: RealAngle + Trigonometry> $name<T> {
            /// Returns a hue from `a` and `b`, normalized to `[0°, 360°)`.
            ///
            /// If `a` and `b` are both `0`, returns `0`,
            #[inline(always)]
            pub fn from_cartesian(a: T, b: T) -> Self where T: Add<T, Output = T> + Neg<Output = T> {
                // atan2 returns values in the interval [-π, π]
                // instead of
                //   let hue_rad = T::atan2(b,a);
                // use negative a and be and rotate, to ensure the hue is normalized,
                let hue_rad = T::from_f64(core::f64::consts::PI) + T::atan2(-b, -a);
                Self::from_radians(hue_rad)
            }

            /// Returns `a` and `b` values for this hue, normalized to `[-1,
            /// 1]`.
            ///
            /// They will have to be multiplied by a radius values, such as
            /// saturation, value, chroma, etc., to represent a specific color.
            #[inline(always)]
            pub fn into_cartesian(self) -> (T, T) {
                let (b, a) = self.into_raw_radians().sin_cos();
                (a, b) // Note the swapped order compared to above
            }
        }

        impl<T> $name<&T> {
            /// Get an owned, copied version of this hue.
            #[inline(always)]
            pub fn copied(&self) -> $name<T>
            where
                T: Copy,
            {
                $name(*self.0)
            }

            /// Get an owned, cloned version of this hue.
            #[inline(always)]
            pub fn cloned(&self) -> $name<T>
            where
                T: Clone,
            {
                $name(self.0.clone())
            }
        }

        impl<T> $name<&mut T> {
            /// Update this hue with a new value.
            #[inline(always)]
            pub fn set(&mut self, value: $name<T>) {
                *self.0 = value.0;
            }

            /// Borrow this hue's value as shared references.
            #[inline(always)]
            pub fn as_ref(&self) -> $name<&T> {
                $name(&*self.0)
            }

            /// Get an owned, copied version of this hue.
            #[inline(always)]
            pub fn copied(&self) -> $name<T>
            where
                T: Copy,
            {
                $name(*self.0)
            }

            /// Get an owned, cloned version of this hue.
            #[inline(always)]
            pub fn cloned(&self) -> $name<T>
            where
                T: Clone,
            {
                $name(self.0.clone())
            }
        }

        impl<C> $name<C> {
            /// Return an iterator over the hues in the wrapped collection.
            #[inline(always)]
            pub fn iter<'a>(&'a self) -> <&'a Self as IntoIterator>::IntoIter where &'a Self: IntoIterator {
                self.into_iter()
            }

            /// Return an iterator that allows modifying the hues in the wrapped collection.
            #[inline(always)]
            pub fn iter_mut<'a>(&'a mut self) -> <&'a mut Self as IntoIterator>::IntoIter where &'a mut Self: IntoIterator {
                self.into_iter()
            }

            /// Get a hue, or slice of hues, with references to the values at `index`. See [`slice::get`] for details.
            #[inline(always)]
            pub fn get<'a, I, T>(&'a self, index: I) -> Option<$name<&<I as core::slice::SliceIndex<[T]>>::Output>>
            where
                T: 'a,
                C: AsRef<[T]>,
                I: core::slice::SliceIndex<[T]> + Clone,
            {
                self.0.as_ref().get(index).map($name)
            }

            /// Get a hue, or slice of hues, that allows modifying the values at `index`. See [`slice::get_mut`] for details.
            #[inline(always)]
            pub fn get_mut<'a, I, T>(&'a mut self, index: I) -> Option<$name<&mut <I as core::slice::SliceIndex<[T]>>::Output>>
            where
                T: 'a,
                C: AsMut<[T]>,
                I: core::slice::SliceIndex<[T]> + Clone,
            {
                self.0.as_mut().get_mut(index).map($name)
            }
        }

        #[cfg(feature = "alloc")]
        impl<T> $name<alloc::vec::Vec<T>> {
            /// Create a struct with a vector with a minimum capacity. See [`Vec::with_capacity`] for details.
            pub fn with_capacity(capacity: usize) -> Self {
                Self(alloc::vec::Vec::with_capacity(capacity))
            }

            /// Push an additional hue onto the hue vector. See [`Vec::push`] for details.
            pub fn push(&mut self, value: $name<T>) {
                self.0.push(value.0);
            }

            /// Pop a hue from the hue vector. See [`Vec::pop`] for details.
            pub fn pop(&mut self) -> Option<$name<T>> {
                self.0.pop().map($name)
            }

            /// Clear the hue vector. See [`Vec::clear`] for details.
            pub fn clear(&mut self) {
                self.0.clear();
            }

            /// Return an iterator that moves hues out of the specified range.
            pub fn drain<R>(&mut self, range: R) -> $iter_name<alloc::vec::Drain<T>>
            where
                R: core::ops::RangeBounds<usize> + Clone,
            {
                $iter_name(self.0.drain(range))
            }
        }

        impl<T> From<T> for $name<T> {
            #[inline]
            fn from(degrees: T) -> $name<T> {
                $name(degrees)
            }
        }

        impl From<$name<f64>> for f64 {
            #[inline]
            fn from(hue: $name<f64>) -> f64 {
                hue.0.normalize_signed_angle()
            }
        }

        impl From<$name<f32>> for f64 {
            #[inline]
            fn from(hue: $name<f32>) -> f64 {
                hue.0.normalize_signed_angle() as f64
            }
        }

        impl From<$name<f32>> for f32 {
            #[inline]
            fn from(hue: $name<f32>) -> f32 {
                hue.0.normalize_signed_angle()
            }
        }

        impl From<$name<f64>> for f32 {
            #[inline]
            fn from(hue: $name<f64>) -> f32 {
                hue.0.normalize_signed_angle() as f32
            }
        }

        impl From<$name<u8>> for u8 {
            #[inline]
            fn from(hue: $name<u8>) -> u8 {
                hue.0
            }
        }

        impl<T> PartialEq for $name<T> where T: AngleEq<Mask = bool> + PartialEq {
            #[inline]
            fn eq(&self, other: &$name<T>) -> bool {
                self.0.angle_eq(&other.0)
            }
        }

        impl<T> PartialEq<T> for $name<T> where T: AngleEq<Mask = bool> + PartialEq {
            #[inline]
            fn eq(&self, other: &T) -> bool {
                self.0.angle_eq(other)
            }
        }

        impl<T> Eq for $name<T> where T: AngleEq<Mask = bool> + Eq {}


        #[cfg(feature = "approx")]
        impl<T> AbsDiffEq for $name<T>
        where
            T: RealAngle + SignedAngle + Zero + AngleEq<Mask = bool> + Sub<Output = T> + AbsDiffEq + Clone,
            T::Epsilon: HalfRotation + Mul<Output = T::Epsilon>,
        {
            type Epsilon = T::Epsilon;

            fn default_epsilon() -> Self::Epsilon {
                // For hues, angles in (normalized) degrees are compared.
                // Scaling from radians to degrees raises the order of magnitude of the
                // error by 180/PI.
                // Scale the default epsilon accordingly for absolute comparisons.
                // Scaling is not required for relative comparisons (including ulps), as
                // there the error is scaled to unit size anyway
                T::default_epsilon() * T::Epsilon::half_rotation()
            }

            fn abs_diff_eq(&self, other: &Self, epsilon: T::Epsilon) -> bool {
                T::abs_diff_eq(&self.clone().into_degrees(), &other.clone().into_degrees(), epsilon)
            }
            fn abs_diff_ne(&self, other: &Self, epsilon: T::Epsilon) -> bool {
                T::abs_diff_ne(&self.clone().into_degrees(), &other.clone().into_degrees(), epsilon)
            }
        }

        #[cfg(feature = "approx")]
        impl<T> RelativeEq for $name<T>
        where
            T: RealAngle + SignedAngle + Zero + AngleEq<Mask = bool> + Sub<Output = T> + Clone + RelativeEq,
            T::Epsilon: HalfRotation + Mul<Output = T::Epsilon>,
        {
            fn default_max_relative() -> Self::Epsilon {
                T::default_max_relative()
            }

            fn relative_eq(
                &self,
                other: &Self,
                epsilon: T::Epsilon,
                max_relative: T::Epsilon,
            ) -> bool {
                T::relative_eq(&self.clone().into_degrees(), &other.clone().into_degrees(), epsilon, max_relative)
            }
            fn relative_ne(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
                max_relative: Self::Epsilon,
            ) -> bool {
                T::relative_ne(&self.clone().into_degrees(), &other.clone().into_degrees(), epsilon, max_relative)
            }
        }

        #[cfg(feature = "approx")]
        impl<T> UlpsEq for $name<T>
        where
            T: RealAngle + SignedAngle + Zero + AngleEq<Mask = bool> + Sub<Output = T> + Clone + UlpsEq,
            T::Epsilon: HalfRotation + Mul<Output = T::Epsilon>,
        {
            fn default_max_ulps() -> u32 {
                T::default_max_ulps()
            }

            fn ulps_eq(&self, other: &Self, epsilon: T::Epsilon, max_ulps: u32) -> bool {
                T::ulps_eq(&self.clone().into_degrees(), &other.clone().into_degrees(), epsilon, max_ulps)
            }
            fn ulps_ne(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
                T::ulps_ne(&self.clone().into_degrees(), &other.clone().into_degrees(), epsilon, max_ulps)
            }
        }

        impl<T: Add<Output=T>> Add<$name<T>> for $name<T> {
            type Output = $name<T>;

            #[inline]
            fn add(self, other: $name<T>) -> $name<T> {
                $name(self.0 + other.0)
            }
        }

        impl<T: Add<Output=T>> Add<T> for $name<T> {
            type Output = $name<T>;

            #[inline]
            fn add(self, other: T) -> $name<T> {
                $name(self.0 + other)
            }
        }

        impl Add<$name<f32>> for f32 {
            type Output = $name<f32>;

            #[inline]
            fn add(self, other: $name<f32>) -> $name<f32> {
                $name(self + other.0)
            }
        }

        impl Add<$name<f64>> for f64 {
            type Output = $name<f64>;

            #[inline]
            fn add(self, other: $name<f64>) -> $name<f64> {
                $name(self + other.0)
            }
        }

        impl<T: AddAssign> AddAssign<$name<T>> for $name<T> {
            #[inline]
            fn add_assign(&mut self, other: $name<T>) {
                self.0 += other.0;
            }
        }

        impl<T: AddAssign> AddAssign<T> for $name<T> {
            #[inline]
            fn add_assign(&mut self, other: T) {
                self.0 += other;
            }
        }

        impl AddAssign<$name<f32>> for f32 {
            #[inline]
            fn add_assign(&mut self, other: $name<f32>) {
                *self += other.0;
            }
        }

        impl AddAssign<$name<f64>> for f64 {
            #[inline]
            fn add_assign(&mut self, other: $name<f64>){
                *self += other.0;
            }
        }

        impl<T: $crate::num::SaturatingAdd<Output=T>> $crate::num::SaturatingAdd<$name<T>> for $name<T> {
            type Output = $name<T>;

            #[inline]
            fn saturating_add(self, other: $name<T>) -> $name<T> {
                $name(self.0.saturating_add(other.0))
            }
        }

        impl<T: $crate::num::SaturatingAdd<Output=T>> $crate::num::SaturatingAdd<T> for $name<T> {
            type Output = $name<T>;

            #[inline]
            fn saturating_add(self, other: T) -> $name<T> {
                $name(self.0.saturating_add(other))
            }
        }

        impl<T: Sub<Output=T>> Sub<$name<T>> for $name<T> {
            type Output = $name<T>;

            #[inline]
            fn sub(self, other: $name<T>) -> $name<T> {
                $name(self.0 - other.0)
            }
        }

        impl<T: Sub<Output=T>> Sub<T> for $name<T> {
            type Output = $name<T>;

            #[inline]
            fn sub(self, other: T) -> $name<T> {
                $name(self.0 - other)
            }
        }

        impl Sub<$name<f32>> for f32 {
            type Output = $name<f32>;

            #[inline]
            fn sub(self, other: $name<f32>) -> $name<f32> {
                $name(self - other.0)
            }
        }

        impl Sub<$name<f64>> for f64 {
            type Output = $name<f64>;

            #[inline]
            fn sub(self, other: $name<f64>) -> $name<f64> {
                $name(self - other.0)
            }
        }

        impl<T: SubAssign> SubAssign<$name<T>> for $name<T> {
            #[inline]
            fn sub_assign(&mut self, other: $name<T>) {
                self.0 -= other.0;
            }
        }

        impl<T: SubAssign> SubAssign<T> for $name<T> {
            #[inline]
            fn sub_assign(&mut self, other: T) {
                self.0 -= other;
            }
        }

        impl SubAssign<$name<f32>> for f32 {
            #[inline]
            fn sub_assign(&mut self, other: $name<f32>) {
                *self -= other.0;
            }
        }

        impl SubAssign<$name<f64>> for f64 {
            #[inline]
            fn sub_assign(&mut self, other: $name<f64>){
                *self -= other.0;
            }
        }

        impl<T: $crate::num::SaturatingSub<Output=T>> $crate::num::SaturatingSub<$name<T>> for $name<T> {
            type Output = $name<T>;

            #[inline]
            fn saturating_sub(self, other: $name<T>) -> $name<T> {
                $name(self.0.saturating_sub(other.0))
            }
        }

        impl<T: $crate::num::SaturatingSub<Output=T>> $crate::num::SaturatingSub<T> for $name<T> {
            type Output = $name<T>;

            #[inline]
            fn saturating_sub(self, other: T) -> $name<T> {
                $name(self.0.saturating_sub(other))
            }
        }

        impl<C, T> Extend<T> for $name<C> where C: Extend<T> {
            #[inline(always)]
            fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
                self.0.extend(iter);
            }
        }

        impl<T, const N: usize> IntoIterator for $name<[T; N]> {
            type Item = $name<T>;
            type IntoIter = $iter_name<core::array::IntoIter<T, N>>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                $iter_name(IntoIterator::into_iter(self.0))
            }
        }

        impl<'a, T> IntoIterator for $name<&'a [T]> {
            type Item = $name<&'a T>;
            type IntoIter = $iter_name<core::slice::Iter<'a, T>>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                $iter_name(self.0.into_iter())
            }
        }

        impl<'a, T> IntoIterator for $name<&'a mut [T]> {
            type Item = $name<&'a mut T>;
            type IntoIter = $iter_name<core::slice::IterMut<'a, T>>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                $iter_name(self.0.into_iter())
            }
        }

        #[cfg(feature = "alloc")]
        impl<T> IntoIterator for $name<alloc::vec::Vec<T>> {
            type Item = $name<T>;
            type IntoIter = $iter_name<alloc::vec::IntoIter<T>>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                $iter_name(self.0.into_iter())
            }
        }

        impl<'a, T, const N: usize> IntoIterator for &'a $name<[T; N]> {
            type Item = $name<&'a T>;
            type IntoIter = $iter_name<core::slice::Iter<'a, T>>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                $iter_name((&self.0).into_iter())
            }
        }

        impl<'a, 'b, T> IntoIterator for &'a $name<&'b [T]> {
            type Item = $name<&'a T>;
            type IntoIter = $iter_name<core::slice::Iter<'a, T>>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                $iter_name(self.0.into_iter())
            }
        }

        impl<'a, 'b, T> IntoIterator for &'a $name<&'b mut [T]> {
            type Item = $name<&'a T>;
            type IntoIter = $iter_name<core::slice::Iter<'a, T>>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                $iter_name((&*self.0).into_iter())
            }
        }

        #[cfg(feature = "alloc")]
        impl<'a, T> IntoIterator for &'a $name<alloc::vec::Vec<T>> {
            type Item = $name<&'a T>;
            type IntoIter = $iter_name<core::slice::Iter<'a, T>>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                $iter_name((&self.0).into_iter())
            }
        }

        #[cfg(feature = "alloc")]
        impl<'a, T> IntoIterator for &'a $name<alloc::boxed::Box<[T]>> {
            type Item = $name<&'a T>;
            type IntoIter = $iter_name<core::slice::Iter<'a, T>>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                $iter_name((&self.0).into_iter())
            }
        }

        impl<'a, T, const N: usize> IntoIterator for &'a mut $name<[T; N]> {
            type Item = $name<&'a mut T>;
            type IntoIter = $iter_name<core::slice::IterMut<'a, T>>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                $iter_name((&mut self.0).into_iter())
            }
        }

        impl<'a, 'b, T> IntoIterator for &'a mut $name<&'b mut [T]> {
            type Item = $name<&'a mut T>;
            type IntoIter = $iter_name<core::slice::IterMut<'a, T>>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                $iter_name(self.0.into_iter())
            }
        }

        #[cfg(feature = "alloc")]
        impl<'a, T> IntoIterator for &'a mut $name<alloc::vec::Vec<T>> {
            type Item = $name<&'a mut T>;
            type IntoIter = $iter_name<core::slice::IterMut<'a, T>>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                $iter_name((&mut self.0).into_iter())
            }
        }

        #[cfg(feature = "alloc")]
        impl<'a, T> IntoIterator for &'a mut $name<alloc::boxed::Box<[T]>> {
            type Item = $name<&'a mut T>;
            type IntoIter = $iter_name<core::slice::IterMut<'a, T>>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                $iter_name((&mut *self.0).into_iter())
            }
        }

        #[doc = concat!("Iterator over [`", stringify!($name), "`] values.")]
        pub struct $iter_name<I>(I);

        impl<I> Iterator for $iter_name<I>
        where
            I: Iterator,
        {
            type Item = $name<I::Item>;

            #[inline(always)]
            fn next(&mut self) -> Option<Self::Item> {
                self.0.next().map($name)
            }

            #[inline(always)]
            fn size_hint(&self) -> (usize, Option<usize>) {
                self.0.size_hint()
            }

            #[inline(always)]
            fn count(self) -> usize {
                self.0.count()
            }
        }

        impl<I> DoubleEndedIterator for $iter_name<I>
        where
            I: DoubleEndedIterator,
        {
            #[inline(always)]
            fn next_back(&mut self) -> Option<Self::Item> {
                self.0.next_back().map($name)
            }
        }

        impl<I> ExactSizeIterator for $iter_name<I>
        where
            I: ExactSizeIterator,
        {
            #[inline(always)]
            fn len(&self) -> usize {
                self.0.len()
            }
        }

        #[cfg(feature = "random")]
        impl<T> Distribution<$name<T>> for Standard
        where
            T: RealAngle + FullRotation + Mul<Output = T>,
            Standard: Distribution<T>,
        {
            #[inline(always)]
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $name<T> {
                $name::from_degrees(rng.gen() * T::full_rotation())
            }
        }

        #[cfg(feature = "bytemuck")]
        unsafe impl<T: bytemuck::Zeroable> bytemuck::Zeroable for $name<T> {}
        #[cfg(feature = "bytemuck")]
        unsafe impl<T: bytemuck::Pod> bytemuck::Pod for $name<T> {}
    )+)
}

make_hues! {
    /// A hue type for the CIE L\*a\*b\* family of color spaces.
    ///
    /// It's measured in degrees and it's based on the four physiological
    /// elementary colors _red_, _yellow_, _green_ and _blue_. This makes it
    /// different from the hue of RGB based color spaces.
    struct LabHue; LabHueIter

    /// A hue type for the CIE L\*u\*v\* family of color spaces.
    struct LuvHue; LuvHueIter

    /// A hue type for the RGB family of color spaces.
    ///
    /// It's measured in degrees and uses the three additive primaries _red_,
    /// _green_ and _blue_.
    struct RgbHue; RgbHueIter

    /// A hue type for the Oklab color space.
    ///
    /// It's measured in degrees.
    struct OklabHue; OklabHueIter

    /// A hue type for the CAM16 color appearance model.
    ///
    /// It's measured in degrees.
    struct Cam16Hue; Cam16HueIter
}

macro_rules! impl_uniform {
    (  $uni_ty: ident , $base_ty: ident) => {
        #[doc = concat!("Sample [`", stringify!($base_ty), "`] uniformly.")]
        #[cfg(feature = "random")]
        pub struct $uni_ty<T>
        where
            T: SampleUniform,
        {
            hue: Uniform<T>,
        }

        #[cfg(feature = "random")]
        impl<T> SampleUniform for $base_ty<T>
        where
            T: RealAngle
                + UnsignedAngle
                + FullRotation
                + Add<Output = T>
                + Mul<Output = T>
                + PartialOrd
                + Clone
                + SampleUniform,
        {
            type Sampler = $uni_ty<T>;
        }

        #[cfg(feature = "random")]
        impl<T> UniformSampler for $uni_ty<T>
        where
            T: RealAngle
                + UnsignedAngle
                + FullRotation
                + Add<Output = T>
                + Mul<Output = T>
                + PartialOrd
                + Clone
                + SampleUniform,
        {
            type X = $base_ty<T>;

            fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = low_b.borrow().clone();
                let normalized_low = $base_ty::into_positive_degrees(low.clone());
                let high = high_b.borrow().clone();
                let normalized_high = $base_ty::into_positive_degrees(high.clone());

                let normalized_high = if normalized_low >= normalized_high && low.0 < high.0 {
                    normalized_high + T::full_rotation()
                } else {
                    normalized_high
                };

                $uni_ty {
                    hue: Uniform::new(normalized_low, normalized_high),
                }
            }

            fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                let low = low_b.borrow().clone();
                let normalized_low = $base_ty::into_positive_degrees(low.clone());
                let high = high_b.borrow().clone();
                let normalized_high = $base_ty::into_positive_degrees(high.clone());

                let normalized_high = if normalized_low >= normalized_high && low.0 < high.0 {
                    normalized_high + T::full_rotation()
                } else {
                    normalized_high
                };

                $uni_ty {
                    hue: Uniform::new_inclusive(normalized_low, normalized_high),
                }
            }

            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $base_ty<T> {
                $base_ty::from(self.hue.sample(rng) * T::full_rotation())
            }
        }
    };
}

impl_uniform!(UniformLabHue, LabHue);
impl_uniform!(UniformRgbHue, RgbHue);
impl_uniform!(UniformLuvHue, LuvHue);
impl_uniform!(UniformOklabHue, OklabHue);
impl_uniform!(UniformCam16Hue, Cam16Hue);

#[cfg(test)]
mod test {
    #[cfg(feature = "approx")]
    mod math {
        use crate::{
            angle::{SignedAngle, UnsignedAngle},
            OklabHue, RgbHue,
        };

        #[test]
        fn oklabhue_ab_roundtrip() {
            for degree in [0.0_f64, 90.0, 30.0, 330.0, 120.0, 240.0] {
                let hue = OklabHue::from_degrees(degree);
                let (a, b) = hue.into_cartesian();
                let roundtrip_hue = OklabHue::from_cartesian(a * 10000.0, b * 10000.0);
                assert_abs_diff_eq!(roundtrip_hue, hue);
            }
        }

        #[test]
        fn normalize_angle_0_360() {
            let inp = [
                -1000.0_f32,
                -900.0,
                -360.5,
                -360.0,
                -359.5,
                -240.0,
                -180.5,
                -180.0,
                -179.5,
                -90.0,
                -0.5,
                0.0,
                0.5,
                90.0,
                179.5,
                180.0,
                180.5,
                240.0,
                359.5,
                360.0,
                360.5,
                900.0,
                1000.0,
            ];

            let expected = [
                80.0_f32, 180.0, 359.5, 0.0, 0.5, 120.0, 179.5, 180.0, 180.5, 270.0, 359.5, 0.0,
                0.5, 90.0, 179.5, 180.0, 180.5, 240.0, 359.5, 0.0, 0.5, 180.0, 280.0,
            ];

            let result: Vec<f32> = inp
                .iter()
                .map(|x| (*x).normalize_unsigned_angle())
                .collect();
            for (res, exp) in result.iter().zip(expected.iter()) {
                assert_relative_eq!(res, exp);
            }
        }

        #[test]
        fn normalize_angle_180_180() {
            let inp = [
                -1000.0_f32,
                -900.0,
                -360.5,
                -360.0,
                -359.5,
                -240.0,
                -180.5,
                -180.0,
                -179.5,
                -90.0,
                -0.5,
                0.0,
                0.5,
                90.0,
                179.5,
                180.0,
                180.5,
                240.0,
                359.5,
                360.0,
                360.5,
                900.0,
                1000.0,
            ];

            let expected = [
                80.0, 180.0, -0.5, 0.0, 0.5, 120.0, 179.5, 180.0, -179.5, -90.0, -0.5, 0.0, 0.5,
                90.0, 179.5, 180.0, -179.5, -120.0, -0.5, 0.0, 0.5, 180.0, -80.0,
            ];

            let result: Vec<f32> = inp.iter().map(|x| (*x).normalize_signed_angle()).collect();
            for (res, exp) in result.iter().zip(expected.iter()) {
                assert_relative_eq!(res, exp);
            }
        }

        #[test]
        fn float_conversion() {
            for i in -180..180 {
                let hue = RgbHue::from(4.0 * i as f32);

                let degs = hue.into_degrees();
                assert!(degs > -180.0 && degs <= 180.0);

                let pos_degs = hue.into_positive_degrees();
                assert!((0.0..360.0).contains(&pos_degs));

                assert_relative_eq!(RgbHue::from(degs), RgbHue::from(pos_degs));
            }
        }
    }

    #[cfg(feature = "serializing")]
    mod serde {
        use crate::RgbHue;

        #[test]
        fn serialize() {
            let serialized = ::serde_json::to_string(&RgbHue::from_degrees(10.2)).unwrap();

            assert_eq!(serialized, "10.2");
        }

        #[test]
        fn deserialize() {
            let deserialized: RgbHue = ::serde_json::from_str("10.2").unwrap();

            assert_eq!(deserialized, RgbHue::from_degrees(10.2));
        }
    }
}
