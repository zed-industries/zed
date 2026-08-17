// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
// refer to the Cargo.toml file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::iter;
use std::mem;
use std::ops::*;

use num_traits::{cast, NumCast};
#[cfg(feature = "rand")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use structure::*;

use angle::Rad;
use approx;
use euler::Euler;
use matrix::{Matrix3, Matrix4};
use num::BaseFloat;
use point::Point3;
use rotation::{Basis3, Rotation, Rotation3};
use vector::Vector3;

#[cfg(feature = "mint")]
use mint;

/// A [quaternion](https://en.wikipedia.org/wiki/Quaternion) in scalar/vector
/// form.
///
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Quaternion<S> {
    /// The vector part of the quaternion.
    pub v: Vector3<S>,
    /// The scalar part of the quaternion.
    pub s: S,
}

impl<S> Quaternion<S> {
    /// Construct a new quaternion from one scalar component and three
    /// imaginary components.
    #[inline]
    pub const fn new(w: S, xi: S, yj: S, zk: S) -> Quaternion<S> {
        Quaternion::from_sv(w, Vector3::new(xi, yj, zk))
    }

    /// Construct a new quaternion from a scalar and a vector.
    #[inline]
    pub const fn from_sv(s: S, v: Vector3<S>) -> Quaternion<S> {
        Quaternion { s: s, v: v }
    }
}

impl<S: BaseFloat> Quaternion<S> {
    /// Construct a new quaternion as a closest arc between two vectors
    ///
    /// Return the closest rotation that turns `src` vector into `dst`.
    ///
    /// - [Related StackOverflow question]
    ///   (http://stackoverflow.com/questions/1171849/finding-quaternion-representing-the-rotation-from-one-vector-to-another)
    /// - [Ogre implementation for normalized vectors]
    ///   (https://bitbucket.org/sinbad/ogre/src/9db75e3ba05c/OgreMain/include/OgreVector3.h?fileviewer=file-view-default#cl-651)
    pub fn from_arc(
        src: Vector3<S>,
        dst: Vector3<S>,
        fallback: Option<Vector3<S>>,
    ) -> Quaternion<S> {
        let mag_avg = (src.magnitude2() * dst.magnitude2()).sqrt();
        let dot = src.dot(dst);
        if ulps_eq!(dot, &mag_avg) {
            Quaternion::<S>::one()
        } else if ulps_eq!(dot, &-mag_avg) {
            let axis = fallback.unwrap_or_else(|| {
                let mut v = Vector3::unit_x().cross(src);
                if ulps_eq!(v, &Zero::zero()) {
                    v = Vector3::unit_y().cross(src);
                }
                v.normalize()
            });
            Quaternion::from_axis_angle(axis, Rad::turn_div_2())
        } else {
            Quaternion::from_sv(mag_avg + dot, src.cross(dst)).normalize()
        }
    }

    /// The conjugate of the quaternion.
    #[inline]
    pub fn conjugate(self) -> Quaternion<S> {
        Quaternion::from_sv(self.s, -self.v)
    }

    /// Do a normalized linear interpolation with `other`, by `amount`.
    /// 
    /// This takes the shortest path, so if the quaternions have a negative
    /// dot product, the interpolation will be between `self` and `-other`.
    pub fn nlerp(self, mut other: Quaternion<S>, amount: S) -> Quaternion<S> {
        if self.dot(other) < S::zero() {
            other = -other;
        }

        (self * (S::one() - amount) + other * amount).normalize()
    }

    /// Spherical Linear Interpolation
    ///
    /// Return the spherical linear interpolation between the quaternion and
    /// `other`. Both quaternions should be normalized first.
    /// 
    /// This takes the shortest path, so if the quaternions have a negative
    /// dot product, the interpolation will be between `self` and `-other`.
    ///
    /// # Performance notes
    ///
    /// The `acos` operation used in `slerp` is an expensive operation, so
    /// unless your quaternions are far away from each other it's generally
    /// more advisable to use `nlerp` when you know your rotations are going
    /// to be small.
    ///
    /// - [Understanding Slerp, Then Not Using It]
    ///   (http://number-none.com/product/Understanding%20Slerp,%20Then%20Not%20Using%20It/)
    /// - [Arcsynthesis OpenGL tutorial]
    ///   (http://www.arcsynthesis.org/gltut/Positioning/Tut08%20Interpolation.html)
    pub fn slerp(self, mut other: Quaternion<S>, amount: S) -> Quaternion<S> {
        let mut dot = self.dot(other);
        let dot_threshold: S = cast(0.9995f64).unwrap();

        if dot < S::zero() {
            other = -other;
            dot = -dot;
        }

        // if quaternions are close together use `nlerp`
        if dot > dot_threshold {
            self.nlerp(other, amount)
        } else {
            // stay within the domain of acos()
            let robust_dot = dot.min(S::one()).max(-S::one());

            let theta = Rad::acos(robust_dot);

            let scale1 = Rad::sin(theta * (S::one() - amount));
            let scale2 = Rad::sin(theta * amount);

            (self * scale1 + other * scale2).normalize()
        }
    }

    pub fn is_finite(&self) -> bool {
        self.s.is_finite() && self.v.is_finite()
    }
}

impl<S: BaseFloat> Zero for Quaternion<S> {
    #[inline]
    fn zero() -> Quaternion<S> {
        Quaternion::from_sv(S::zero(), Vector3::zero())
    }

    #[inline]
    fn is_zero(&self) -> bool {
        ulps_eq!(self, &Quaternion::<S>::zero())
    }
}

impl<S: BaseFloat> One for Quaternion<S> {
    #[inline]
    fn one() -> Quaternion<S> {
        Quaternion::from_sv(S::one(), Vector3::zero())
    }
}

impl<S: BaseFloat> iter::Sum<Quaternion<S>> for Quaternion<S> {
    #[inline]
    fn sum<I: Iterator<Item = Quaternion<S>>>(iter: I) -> Quaternion<S> {
        iter.fold(Quaternion::<S>::zero(), Add::add)
    }
}

impl<'a, S: 'a + BaseFloat> iter::Sum<&'a Quaternion<S>> for Quaternion<S> {
    #[inline]
    fn sum<I: Iterator<Item = &'a Quaternion<S>>>(iter: I) -> Quaternion<S> {
        iter.fold(Quaternion::<S>::zero(), Add::add)
    }
}

impl<S: BaseFloat> iter::Product<Quaternion<S>> for Quaternion<S> {
    #[inline]
    fn product<I: Iterator<Item = Quaternion<S>>>(iter: I) -> Quaternion<S> {
        iter.fold(Quaternion::<S>::one(), Mul::mul)
    }
}

impl<'a, S: 'a + BaseFloat> iter::Product<&'a Quaternion<S>> for Quaternion<S> {
    #[inline]
    fn product<I: Iterator<Item = &'a Quaternion<S>>>(iter: I) -> Quaternion<S> {
        iter.fold(Quaternion::<S>::one(), Mul::mul)
    }
}

impl<S: BaseFloat> VectorSpace for Quaternion<S> {
    type Scalar = S;
}

impl<S: BaseFloat> MetricSpace for Quaternion<S> {
    type Metric = S;

    #[inline]
    fn distance2(self, other: Self) -> S {
        (other - self).magnitude2()
    }
}

impl<S: NumCast + Copy> Quaternion<S> {
    /// Component-wise casting to another type.
    pub fn cast<T: BaseFloat>(&self) -> Option<Quaternion<T>> {
        let s = match NumCast::from(self.s) {
            Some(s) => s,
            None => return None,
        };
        let v = match self.v.cast() {
            Some(v) => v,
            None => return None,
        };
        Some(Quaternion::from_sv(s, v))
    }
}

impl<S: BaseFloat> InnerSpace for Quaternion<S> {
    #[inline]
    default_fn!( dot(self, other: Quaternion<S>) -> S {
        self.s * other.s + self.v.dot(other.v)
    } );
}

impl<A> From<Euler<A>> for Quaternion<A::Unitless>
where
    A: Angle + Into<Rad<<A as Angle>::Unitless>>,
{
    fn from(src: Euler<A>) -> Quaternion<A::Unitless> {
        // Euclidean Space has an Euler to quat equation, but it is for a different order (YXZ):
        // http://www.euclideanspace.com/maths/geometry/rotations/conversions/eulerToQuaternion/index.htm
        // Page A-2 here has the formula for XYZ:
        // http://ntrs.nasa.gov/archive/nasa/casi.ntrs.nasa.gov/19770024290.pdf

        let half = cast(0.5f64).unwrap();
        let (s_x, c_x) = Rad::sin_cos(src.x.into() * half);
        let (s_y, c_y) = Rad::sin_cos(src.y.into() * half);
        let (s_z, c_z) = Rad::sin_cos(src.z.into() * half);

        Quaternion::new(
            -s_x * s_y * s_z + c_x * c_y * c_z,
            s_x * c_y * c_z + s_y * s_z * c_x,
            -s_x * s_z * c_y + s_y * c_x * c_z,
            s_x * s_y * c_z + s_z * c_x * c_y,
        )
    }
}

impl_operator!(<S: BaseFloat> Neg for Quaternion<S> {
    fn neg(quat) -> Quaternion<S> {
        Quaternion::from_sv(-quat.s, -quat.v)
    }
});

impl_operator!(<S: BaseFloat> Mul<S> for Quaternion<S> {
    fn mul(lhs, rhs) -> Quaternion<S> {
        Quaternion::from_sv(lhs.s * rhs, lhs.v * rhs)
    }
});

impl_assignment_operator!(<S: BaseFloat> MulAssign<S> for Quaternion<S> {
    fn mul_assign(&mut self, scalar) { self.s *= scalar; self.v *= scalar; }
});

impl_operator!(<S: BaseFloat> Div<S> for Quaternion<S> {
    fn div(lhs, rhs) -> Quaternion<S> {
        Quaternion::from_sv(lhs.s / rhs, lhs.v / rhs)
    }
});

impl_assignment_operator!(<S: BaseFloat> DivAssign<S> for Quaternion<S> {
    fn div_assign(&mut self, scalar) { self.s /= scalar; self.v /= scalar; }
});

impl_operator!(<S: BaseFloat> Rem<S> for Quaternion<S> {
    fn rem(lhs, rhs) -> Quaternion<S> {
        Quaternion::from_sv(lhs.s % rhs, lhs.v % rhs)
    }
});

impl_assignment_operator!(<S: BaseFloat> RemAssign<S> for Quaternion<S> {
    fn rem_assign(&mut self, scalar) { self.s %= scalar; self.v %= scalar; }
});

impl_operator!(<S: BaseFloat> Mul<Vector3<S> > for Quaternion<S> {
    fn mul(lhs, rhs) -> Vector3<S> {{
        let rhs = rhs.clone();
        let two: S = cast(2i8).unwrap();
        let tmp = lhs.v.cross(rhs) + (rhs * lhs.s);
        (lhs.v.cross(tmp) * two) + rhs
    }}
});

impl_operator!(<S: BaseFloat> Add<Quaternion<S> > for Quaternion<S> {
    fn add(lhs, rhs) -> Quaternion<S> {
        Quaternion::from_sv(lhs.s + rhs.s, lhs.v + rhs.v)
    }
});

impl_assignment_operator!(<S: BaseFloat> AddAssign<Quaternion<S> > for Quaternion<S> {
    fn add_assign(&mut self, other) { self.s += other.s; self.v += other.v; }
});

impl_operator!(<S: BaseFloat> Sub<Quaternion<S> > for Quaternion<S> {
    fn sub(lhs, rhs) -> Quaternion<S> {
        Quaternion::from_sv(lhs.s - rhs.s, lhs.v - rhs.v)
    }
});

impl_assignment_operator!(<S: BaseFloat> SubAssign<Quaternion<S> > for Quaternion<S> {
    fn sub_assign(&mut self, other) { self.s -= other.s; self.v -= other.v; }
});

impl_operator!(<S: BaseFloat> Mul<Quaternion<S> > for Quaternion<S> {
    fn mul(lhs, rhs) -> Quaternion<S> {
        Quaternion::new(
            lhs.s * rhs.s - lhs.v.x * rhs.v.x - lhs.v.y * rhs.v.y - lhs.v.z * rhs.v.z,
            lhs.s * rhs.v.x + lhs.v.x * rhs.s + lhs.v.y * rhs.v.z - lhs.v.z * rhs.v.y,
            lhs.s * rhs.v.y + lhs.v.y * rhs.s + lhs.v.z * rhs.v.x - lhs.v.x * rhs.v.z,
            lhs.s * rhs.v.z + lhs.v.z * rhs.s + lhs.v.x * rhs.v.y - lhs.v.y * rhs.v.x,
        )
    }
});

macro_rules! impl_scalar_mul {
    ($S:ident) => {
        impl_operator!(Mul<Quaternion<$S>> for $S {
            fn mul(scalar, quat) -> Quaternion<$S> {
                Quaternion::from_sv(scalar * quat.s, scalar * quat.v)
            }
        });
    };
}

macro_rules! impl_scalar_div {
    ($S:ident) => {
        impl_operator!(Div<Quaternion<$S>> for $S {
            fn div(scalar, quat) -> Quaternion<$S> {
                Quaternion::from_sv(scalar / quat.s, scalar / quat.v)
            }
        });
    };
}

impl_scalar_mul!(f32);
impl_scalar_mul!(f64);
impl_scalar_div!(f32);
impl_scalar_div!(f64);

impl<S: BaseFloat> approx::AbsDiffEq for Quaternion<S> {
    type Epsilon = S::Epsilon;

    #[inline]
    fn default_epsilon() -> S::Epsilon {
        S::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: S::Epsilon) -> bool {
        S::abs_diff_eq(&self.s, &other.s, epsilon)
            && Vector3::abs_diff_eq(&self.v, &other.v, epsilon)
    }
}

impl<S: BaseFloat> approx::RelativeEq for Quaternion<S> {
    #[inline]
    fn default_max_relative() -> S::Epsilon {
        S::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self, other: &Self, epsilon: S::Epsilon, max_relative: S::Epsilon) -> bool {
        S::relative_eq(&self.s, &other.s, epsilon, max_relative)
            && Vector3::relative_eq(&self.v, &other.v, epsilon, max_relative)
    }
}

impl<S: BaseFloat> approx::UlpsEq for Quaternion<S> {
    #[inline]
    fn default_max_ulps() -> u32 {
        S::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: S::Epsilon, max_ulps: u32) -> bool {
        S::ulps_eq(&self.s, &other.s, epsilon, max_ulps)
            && Vector3::ulps_eq(&self.v, &other.v, epsilon, max_ulps)
    }
}

impl<S: BaseFloat> From<Quaternion<S>> for Matrix3<S> {
    /// Convert the quaternion to a 3 x 3 rotation matrix.
    fn from(quat: Quaternion<S>) -> Matrix3<S> {
        let x2 = quat.v.x + quat.v.x;
        let y2 = quat.v.y + quat.v.y;
        let z2 = quat.v.z + quat.v.z;

        let xx2 = x2 * quat.v.x;
        let xy2 = x2 * quat.v.y;
        let xz2 = x2 * quat.v.z;

        let yy2 = y2 * quat.v.y;
        let yz2 = y2 * quat.v.z;
        let zz2 = z2 * quat.v.z;

        let sy2 = y2 * quat.s;
        let sz2 = z2 * quat.s;
        let sx2 = x2 * quat.s;

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix3::new(
            S::one() - yy2 - zz2, xy2 + sz2, xz2 - sy2,
            xy2 - sz2, S::one() - xx2 - zz2, yz2 + sx2,
            xz2 + sy2, yz2 - sx2, S::one() - xx2 - yy2,
        )
    }
}

impl<S: BaseFloat> From<Quaternion<S>> for Matrix4<S> {
    /// Convert the quaternion to a 4 x 4 rotation matrix.
    fn from(quat: Quaternion<S>) -> Matrix4<S> {
        let x2 = quat.v.x + quat.v.x;
        let y2 = quat.v.y + quat.v.y;
        let z2 = quat.v.z + quat.v.z;

        let xx2 = x2 * quat.v.x;
        let xy2 = x2 * quat.v.y;
        let xz2 = x2 * quat.v.z;

        let yy2 = y2 * quat.v.y;
        let yz2 = y2 * quat.v.z;
        let zz2 = z2 * quat.v.z;

        let sy2 = y2 * quat.s;
        let sz2 = z2 * quat.s;
        let sx2 = x2 * quat.s;

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix4::new(
            S::one() - yy2 - zz2, xy2 + sz2, xz2 - sy2, S::zero(),
            xy2 - sz2, S::one() - xx2 - zz2, yz2 + sx2, S::zero(),
            xz2 + sy2, yz2 - sx2, S::one() - xx2 - yy2, S::zero(),
            S::zero(), S::zero(), S::zero(), S::one(),
        )
    }
}

// Quaternion Rotation impls

impl<S: BaseFloat> From<Quaternion<S>> for Basis3<S> {
    #[inline]
    fn from(quat: Quaternion<S>) -> Basis3<S> {
        Basis3::from_quaternion(&quat)
    }
}

impl<S: BaseFloat> Rotation for Quaternion<S> {
    type Space = Point3<S>;

    #[inline]
    fn look_at(dir: Vector3<S>, up: Vector3<S>) -> Quaternion<S> {
        Matrix3::look_to_lh(dir, up).into()
    }

    #[inline]
    fn between_vectors(a: Vector3<S>, b: Vector3<S>) -> Quaternion<S> {
        // http://stackoverflow.com/a/11741520/2074937 see 'Half-Way Quaternion Solution'

        let k_cos_theta = a.dot(b);

        // same direction
        if ulps_eq!(k_cos_theta, S::one()) {
            return Quaternion::<S>::one();
        }

        let k = (a.magnitude2() * b.magnitude2()).sqrt();

        // opposite direction
        if ulps_eq!(k_cos_theta / k, -S::one()) {
            let mut orthogonal = a.cross(Vector3::unit_x());
            if ulps_eq!(orthogonal.magnitude2(), S::zero()) {
                orthogonal = a.cross(Vector3::unit_y());
            }
            return Quaternion::from_sv(S::zero(), orthogonal.normalize());
        }

        // any other direction
        Quaternion::from_sv(k + k_cos_theta, a.cross(b)).normalize()
    }

    /// Evaluate the conjugation of `vec` by `self`.
    ///
    /// Note that `self` should be a unit quaternion (i.e. normalized) to represent a 3D rotation.
    #[inline]
    fn rotate_vector(&self, vec: Vector3<S>) -> Vector3<S> {
        self * vec
    }

    #[inline]
    fn invert(&self) -> Quaternion<S> {
        self.conjugate() / self.magnitude2()
    }
}

impl<S: BaseFloat> Rotation3 for Quaternion<S> {
    type Scalar = S;

    #[inline]
    fn from_axis_angle<A: Into<Rad<S>>>(axis: Vector3<S>, angle: A) -> Quaternion<S> {
        let (s, c) = Rad::sin_cos(angle.into() * cast(0.5f64).unwrap());
        Quaternion::from_sv(c, axis * s)
    }
}

impl<S: BaseFloat> Into<[S; 4]> for Quaternion<S> {
    #[inline]
    fn into(self) -> [S; 4] {
        match self.into() {
            (xi, yj, zk, w) => [xi, yj, zk, w],
        }
    }
}

impl<S: BaseFloat> AsRef<[S; 4]> for Quaternion<S> {
    #[inline]
    fn as_ref(&self) -> &[S; 4] {
        unsafe { mem::transmute(self) }
    }
}

impl<S: BaseFloat> AsMut<[S; 4]> for Quaternion<S> {
    #[inline]
    fn as_mut(&mut self) -> &mut [S; 4] {
        unsafe { mem::transmute(self) }
    }
}

impl<S: BaseFloat> From<[S; 4]> for Quaternion<S> {
    #[inline]
    fn from(v: [S; 4]) -> Quaternion<S> {
        Quaternion::new(v[3], v[0], v[1], v[2])
    }
}

impl<'a, S: BaseFloat> From<&'a [S; 4]> for &'a Quaternion<S> {
    #[inline]
    fn from(v: &'a [S; 4]) -> &'a Quaternion<S> {
        unsafe { mem::transmute(v) }
    }
}

impl<'a, S: BaseFloat> From<&'a mut [S; 4]> for &'a mut Quaternion<S> {
    #[inline]
    fn from(v: &'a mut [S; 4]) -> &'a mut Quaternion<S> {
        unsafe { mem::transmute(v) }
    }
}

impl<S: BaseFloat> Into<(S, S, S, S)> for Quaternion<S> {
    #[inline]
    fn into(self) -> (S, S, S, S) {
        match self {
            Quaternion {
                s,
                v: Vector3 { x, y, z },
            } => (x, y, z, s),
        }
    }
}

impl<S: BaseFloat> AsRef<(S, S, S, S)> for Quaternion<S> {
    #[inline]
    fn as_ref(&self) -> &(S, S, S, S) {
        unsafe { mem::transmute(self) }
    }
}

impl<S: BaseFloat> AsMut<(S, S, S, S)> for Quaternion<S> {
    #[inline]
    fn as_mut(&mut self) -> &mut (S, S, S, S) {
        unsafe { mem::transmute(self) }
    }
}

impl<S: BaseFloat> From<(S, S, S, S)> for Quaternion<S> {
    #[inline]
    fn from(v: (S, S, S, S)) -> Quaternion<S> {
        match v {
            (xi, yj, zk, w) => Quaternion::new(w, xi, yj, zk),
        }
    }
}

impl<'a, S: BaseFloat> From<&'a (S, S, S, S)> for &'a Quaternion<S> {
    #[inline]
    fn from(v: &'a (S, S, S, S)) -> &'a Quaternion<S> {
        unsafe { mem::transmute(v) }
    }
}

impl<'a, S: BaseFloat> From<&'a mut (S, S, S, S)> for &'a mut Quaternion<S> {
    #[inline]
    fn from(v: &'a mut (S, S, S, S)) -> &'a mut Quaternion<S> {
        unsafe { mem::transmute(v) }
    }
}

macro_rules! index_operators {
    ($S:ident, $Output:ty, $I:ty) => {
        impl<$S: BaseFloat> Index<$I> for Quaternion<$S> {
            type Output = $Output;

            #[inline]
            fn index<'a>(&'a self, i: $I) -> &'a $Output {
                let v: &[$S; 4] = self.as_ref();
                &v[i]
            }
        }

        impl<$S: BaseFloat> IndexMut<$I> for Quaternion<$S> {
            #[inline]
            fn index_mut<'a>(&'a mut self, i: $I) -> &'a mut $Output {
                let v: &mut [$S; 4] = self.as_mut();
                &mut v[i]
            }
        }
    };
}

index_operators!(S, S, usize);
index_operators!(S, [S], Range<usize>);
index_operators!(S, [S], RangeTo<usize>);
index_operators!(S, [S], RangeFrom<usize>);
index_operators!(S, [S], RangeFull);

#[cfg(feature = "rand")]
impl<S> Distribution<Quaternion<S>> for Standard
where
    Standard: Distribution<S>,
    Standard: Distribution<Vector3<S>>,
    S: BaseFloat,
{
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Quaternion<S> {
        Quaternion::from_sv(rng.gen(), rng.gen())
    }
}

#[cfg(feature = "mint")]
impl<S> From<mint::Quaternion<S>> for Quaternion<S> {
    fn from(q: mint::Quaternion<S>) -> Self {
        Quaternion {
            s: q.s,
            v: q.v.into(),
        }
    }
}

#[cfg(feature = "mint")]
impl<S: Clone> Into<mint::Quaternion<S>> for Quaternion<S> {
    fn into(self) -> mint::Quaternion<S> {
        mint::Quaternion {
            s: self.s,
            v: self.v.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use quaternion::*;
    use vector::*;

    const QUATERNION: Quaternion<f32> = Quaternion {
        v: Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
        s: 4.0,
    };

    #[test]
    fn test_into() {
        let v = QUATERNION;
        {
            let v: [f32; 4] = v.into();
            assert_eq!(v, [1.0, 2.0, 3.0, 4.0]);
        }
        {
            let v: (f32, f32, f32, f32) = v.into();
            assert_eq!(v, (1.0, 2.0, 3.0, 4.0));
        }
    }

    #[test]
    fn test_as_ref() {
        let v = QUATERNION;
        {
            let v: &[f32; 4] = v.as_ref();
            assert_eq!(v, &[1.0, 2.0, 3.0, 4.0]);
        }
        {
            let v: &(f32, f32, f32, f32) = v.as_ref();
            assert_eq!(v, &(1.0, 2.0, 3.0, 4.0));
        }
    }

    #[test]
    fn test_as_mut() {
        let mut v = QUATERNION;
        {
            let v: &mut [f32; 4] = v.as_mut();
            assert_eq!(v, &mut [1.0, 2.0, 3.0, 4.0]);
        }
        {
            let v: &mut (f32, f32, f32, f32) = v.as_mut();
            assert_eq!(v, &mut (1.0, 2.0, 3.0, 4.0));
        }
    }

    #[test]
    fn test_from() {
        assert_eq!(Quaternion::from([1.0, 2.0, 3.0, 4.0]), QUATERNION);
        {
            let v = &[1.0, 2.0, 3.0, 4.0];
            let v: &Quaternion<_> = From::from(v);
            assert_eq!(v, &QUATERNION);
        }
        {
            let v = &mut [1.0, 2.0, 3.0, 4.0];
            let v: &mut Quaternion<_> = From::from(v);
            assert_eq!(v, &QUATERNION);
        }
        assert_eq!(Quaternion::from((1.0, 2.0, 3.0, 4.0)), QUATERNION);
        {
            let v = &(1.0, 2.0, 3.0, 4.0);
            let v: &Quaternion<_> = From::from(v);
            assert_eq!(v, &QUATERNION);
        }
        {
            let v = &mut (1.0, 2.0, 3.0, 4.0);
            let v: &mut Quaternion<_> = From::from(v);
            assert_eq!(v, &QUATERNION);
        }
    }

    #[test]
    fn test_nlerp_same() {
        let q = Quaternion::from([0.5, 0.5, 0.5, 0.5]);
        assert_ulps_eq!(q, q.nlerp(q, 0.1234));
    }

    #[test]
    fn test_nlerp_start() {
        let q = Quaternion::from([0.5f64.sqrt(), 0.0, 0.5f64.sqrt(), 0.0]);
        let r = Quaternion::from([0.5, 0.5, 0.5, 0.5]);
        assert_ulps_eq!(q, q.nlerp(r, 0.0));
    }

    #[test]
    fn test_nlerp_end() {
        let q = Quaternion::from([0.5f64.sqrt(), 0.0, 0.5f64.sqrt(), 0.0]);
        let r = Quaternion::from([0.5, 0.5, 0.5, 0.5]);
        assert_ulps_eq!(r, q.nlerp(r, 1.0));
    }

    #[test]
    fn test_nlerp_half() {
        let q = Quaternion::from([-0.5, 0.5, 0.5, 0.5]);
        let r = Quaternion::from([0.5, 0.5, 0.5, 0.5]);

        let expected =
            Quaternion::from([0.0, 1.0 / 3f64.sqrt(), 1.0 / 3f64.sqrt(), 1.0 / 3f64.sqrt()]);
        assert_ulps_eq!(expected, q.nlerp(r, 0.5));
    }

    #[test]
    fn test_nlerp_quarter() {
        let q = Quaternion::from([-0.5, 0.5, 0.5, 0.5]);
        let r = Quaternion::from([0.5, 0.5, 0.5, 0.5]);

        let expected = Quaternion::from([
            -1.0 / 13f64.sqrt(),
            2.0 / 13f64.sqrt(),
            2.0 / 13f64.sqrt(),
            2.0 / 13f64.sqrt(),
        ]);
        assert_ulps_eq!(expected, q.nlerp(r, 0.25));
    }

    #[test]
    fn test_nlerp_zero_dot() {
        let q = Quaternion::from([-0.5, -0.5, 0.5, 0.5]);
        let r = Quaternion::from([0.5, 0.5, 0.5, 0.5]);

        let expected = Quaternion::from([
            -1.0 / 10f64.sqrt(),
            -1.0 / 10f64.sqrt(),
            2.0 / 10f64.sqrt(),
            2.0 / 10f64.sqrt(),
        ]);
        assert_ulps_eq!(expected, q.nlerp(r, 0.25));
    }

    #[test]
    fn test_nlerp_negative_dot() {
        let q = Quaternion::from([-0.5, -0.5, -0.5, 0.5]);
        let r = Quaternion::from([0.5, 0.5, 0.5, 0.5]);

        let expected = Quaternion::from([
            -2.0 / 13f64.sqrt(),
            -2.0 / 13f64.sqrt(),
            -2.0 / 13f64.sqrt(),
            1.0 / 13f64.sqrt(),
        ]);
        assert_ulps_eq!(expected, q.nlerp(r, 0.25));
    }

    #[test]
    fn test_nlerp_opposite() {
        let q = Quaternion::from([-0.5, -0.5, -0.5, -0.5]);
        let r = Quaternion::from([0.5, 0.5, 0.5, 0.5]);

        assert_ulps_eq!(q, q.nlerp(r, 0.25));
        assert_ulps_eq!(q, q.nlerp(r, 0.75));
    }

    #[test]
    fn test_nlerp_extrapolate() {
        let q = Quaternion::from([-0.5, -0.5, -0.5, 0.5]);
        let r = Quaternion::from([0.5, 0.5, 0.5, 0.5]);

        let expected = Quaternion::from([
            -1.0 / 12f64.sqrt(),
            -1.0 / 12f64.sqrt(),
            -1.0 / 12f64.sqrt(),
            3.0 / 12f64.sqrt(),
        ]);
        assert_ulps_eq!(expected, q.nlerp(r, -1.0));
    }

    #[test]
    fn test_slerp_same() {
        let q = Quaternion::from([0.5, 0.5, 0.5, 0.5]);
        assert_ulps_eq!(q, q.slerp(q, 0.1234));
    }

    #[test]
    fn test_slerp_start() {
        let q = Quaternion::from([0.5f64.sqrt(), 0.0, 0.5f64.sqrt(), 0.0]);
        let r = Quaternion::from([0.5, 0.5, 0.5, 0.5]);
        assert_ulps_eq!(q, q.slerp(r, 0.0));
    }

    #[test]
    fn test_slerp_end() {
        let q = Quaternion::from([0.5f64.sqrt(), 0.0, 0.5f64.sqrt(), 0.0]);
        let r = Quaternion::from([0.5, 0.5, 0.5, 0.5]);
        assert_ulps_eq!(r, q.slerp(r, 1.0));
    }

    #[test]
    fn test_slerp_half() {
        let q = Quaternion::from([-0.5, 0.5, 0.5, 0.5]);
        let r = Quaternion::from([0.5, 0.5, 0.5, 0.5]);

        let expected =
            Quaternion::from([0.0, 1.0 / 3f64.sqrt(), 1.0 / 3f64.sqrt(), 1.0 / 3f64.sqrt()]);
        assert_ulps_eq!(expected, q.slerp(r, 0.5));
    }

    #[test]
    fn test_slerp_quarter() {
        let q = Quaternion::from([-0.5, 0.5, 0.5, 0.5]);
        let r = Quaternion::from([0.5, 0.5, 0.5, 0.5]);

        let expected = Quaternion::from([
            -0.2588190451025208,
            0.5576775358252053,
            0.5576775358252053,
            0.5576775358252053,
        ]);
        assert_ulps_eq!(expected, q.slerp(r, 0.25));
    }

    #[test]
    fn test_slerp_zero_dot() {
        let q = Quaternion::from([-0.5, -0.5, 0.5, 0.5]);
        let r = Quaternion::from([0.5, 0.5, 0.5, 0.5]);

        let expected = Quaternion::from([
            -0.27059805007309845,
            -0.27059805007309845,
            0.6532814824381883,
            0.6532814824381883,
        ]);
        assert_ulps_eq!(expected, q.slerp(r, 0.25));
    }

    #[test]
    fn test_slerp_negative_dot() {
        let q = Quaternion::from([-0.5, -0.5, -0.5, 0.5]);
        let r = Quaternion::from([0.5, 0.5, 0.5, 0.5]);

        let expected = Quaternion::from([
            -0.5576775358252053,
            -0.5576775358252053,
            -0.5576775358252053,
            0.2588190451025208
        ]);
        assert_ulps_eq!(expected, q.slerp(r, 0.25));
    }

    #[test]
    fn test_slerp_opposite() {
        let q = Quaternion::from([-0.5, -0.5, -0.5, -0.5]);
        let r = Quaternion::from([0.5, 0.5, 0.5, 0.5]);

        assert_ulps_eq!(q, q.slerp(r, 0.25));
        assert_ulps_eq!(q, q.slerp(r, 0.75));
    }

    #[test]
    fn test_slerp_extrapolate() {
        let q = Quaternion::from([-0.5, -0.5, -0.5, 0.5]);
        let r = Quaternion::from([0.5, 0.5, 0.5, 0.5]);

        let expected = Quaternion::from([0.0, 0.0, 0.0, 1.0]);
        assert_ulps_eq!(expected, q.slerp(r, -1.0));
    }

    #[test]
    fn test_slerp_regression() {
        let a = Quaternion::<f32>::new(0.00052311074, 0.9999999, 0.00014682197, -0.000016342687);
        let b = Quaternion::<f32>::new(0.019973433, -0.99980056, -0.00015678025, 0.000013882192);

        assert_ulps_eq!(a.slerp(b, 0.5).magnitude(), 1.0);
    }
}
