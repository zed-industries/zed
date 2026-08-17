// Copyright 2016 The CGMath Developers. For a full listing of the authors,
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

use num_traits::cast;
#[cfg(feature = "rand")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use structure::*;

use angle::Rad;
use approx;
#[cfg(feature = "mint")]
use mint;
use num::BaseFloat;
use quaternion::Quaternion;

/// A set of [Euler angles] representing a rotation in three-dimensional space.
///
/// This type is marked as `#[repr(C)]`.
///
/// The axis rotation sequence is XYZ. That is, the rotation is first around
/// the X axis, then the Y axis, and lastly the Z axis (using intrinsic
/// rotations). Since all three rotation axes are used, the angles are
/// Tait–Bryan angles rather than proper Euler angles.
///
/// # Ranges
///
/// - x: [-pi, pi]
/// - y: [-pi/2, pi/2]
/// - z: [-pi, pi]
///
/// # Defining rotations using Euler angles
///
/// Note that while [Euler angles] are intuitive to define, they are prone to
/// [gimbal lock] and are challenging to interpolate between. Instead we
/// recommend that you convert them to a more robust representation, such as a
/// quaternion or a rotation matrix. To this end, `From<Euler<A>>` conversions
/// are provided for the following types:
///
/// - [`Basis3`](struct.Basis3.html)
/// - [`Matrix3`](struct.Matrix3.html)
/// - [`Matrix4`](struct.Matrix4.html)
/// - [`Quaternion`](struct.Quaternion.html)
///
/// For example, to define a quaternion that applies the following:
///
/// 1. a 90° rotation around the _x_ axis
/// 2. a 45° rotation around the _y_ axis
/// 3. a 15° rotation around the _z_ axis
///
/// you can use the following code:
///
/// ```
/// use cgmath::{Deg, Euler, Quaternion};
///
/// let rotation = Quaternion::from(Euler {
///     x: Deg(90.0),
///     y: Deg(45.0),
///     z: Deg(15.0),
/// });
/// ```
///
/// [Euler angles]: https://en.wikipedia.org/wiki/Euler_angles
/// [gimbal lock]: https://en.wikipedia.org/wiki/Gimbal_lock#Gimbal_lock_in_applied_mathematics
/// [convert]: #defining-rotations-using-euler-angles
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Euler<A> {
    /// The angle to apply around the _x_ axis. Also known at the _pitch_.
    pub x: A,
    /// The angle to apply around the _y_ axis. Also known at the _yaw_.
    pub y: A,
    /// The angle to apply around the _z_ axis. Also known at the _roll_.
    pub z: A,
}

impl<A> Euler<A> {
    /// Construct a set of euler angles.
    ///
    /// # Arguments
    ///
    /// * `x` - The angle to apply around the _x_ axis. Also known at the _pitch_.
    /// * `y` - The angle to apply around the _y_ axis. Also known at the _yaw_.
    /// * `z` - The angle to apply around the _z_ axis. Also known at the _roll_.
    pub const fn new(x: A, y: A, z: A) -> Euler<A> {
        Euler { x: x, y: y, z: z }
    }
}

impl<S: BaseFloat> From<Quaternion<S>> for Euler<Rad<S>> {
    fn from(src: Quaternion<S>) -> Euler<Rad<S>> {
        let sig: S = cast(0.499).unwrap();
        let two: S = cast(2).unwrap();
        let one: S = cast(1).unwrap();

        let (qw, qx, qy, qz) = (src.s, src.v.x, src.v.y, src.v.z);
        let (sqw, sqx, sqy, sqz) = (qw * qw, qx * qx, qy * qy, qz * qz);

        let unit = sqx + sqz + sqy + sqw;
        let test = qx * qz + qy * qw;

        // We set x to zero and z to the value, but the other way would work too.
        if test > sig * unit {
            // x + z = 2 * atan(x / w)
            Euler {
                x: Rad::zero(),
                y: Rad::turn_div_4(),
                z: Rad::atan2(qx, qw) * two,
            }
        } else if test < -sig * unit {
            // x - z = 2 * atan(x / w)
            Euler {
                x: Rad::zero(),
                y: -Rad::turn_div_4(),
                z: -Rad::atan2(qx, qw) * two,
            }
        } else {
            // Using the quat-to-matrix equation from either
            // http://www.euclideanspace.com/maths/geometry/rotations/conversions/quaternionToMatrix/index.htm
            // or equation 15 on page 7 of
            // http://ntrs.nasa.gov/archive/nasa/casi.ntrs.nasa.gov/19770024290.pdf
            // to fill in the equations on page A-2 of the NASA document gives the below.
            Euler {
                x: Rad::atan2(two * (-qy * qz + qx * qw), one - two * (sqx + sqy)),
                y: Rad::asin(two * (qx * qz + qy * qw)),
                z: Rad::atan2(two * (-qx * qy + qz * qw), one - two * (sqy + sqz)),
            }
        }
    }
}

impl<A: Angle> approx::AbsDiffEq for Euler<A> {
    type Epsilon = A::Epsilon;

    #[inline]
    fn default_epsilon() -> A::Epsilon {
        A::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: A::Epsilon) -> bool {
        A::abs_diff_eq(&self.x, &other.x, epsilon)
            && A::abs_diff_eq(&self.y, &other.y, epsilon)
            && A::abs_diff_eq(&self.z, &other.z, epsilon)
    }
}

impl<A: Angle> approx::RelativeEq for Euler<A> {
    #[inline]
    fn default_max_relative() -> A::Epsilon {
        A::default_max_relative()
    }

    #[inline]
    fn relative_eq(&self, other: &Self, epsilon: A::Epsilon, max_relative: A::Epsilon) -> bool {
        A::relative_eq(&self.x, &other.x, epsilon, max_relative)
            && A::relative_eq(&self.y, &other.y, epsilon, max_relative)
            && A::relative_eq(&self.z, &other.z, epsilon, max_relative)
    }
}

impl<A: Angle> approx::UlpsEq for Euler<A> {
    #[inline]
    fn default_max_ulps() -> u32 {
        A::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: A::Epsilon, max_ulps: u32) -> bool {
        A::ulps_eq(&self.x, &other.x, epsilon, max_ulps)
            && A::ulps_eq(&self.y, &other.y, epsilon, max_ulps)
            && A::ulps_eq(&self.z, &other.z, epsilon, max_ulps)
    }
}

#[cfg(feature = "rand")]
impl<A> Distribution<Euler<A>> for Standard
where
    Standard: Distribution<A>,
    A: Angle,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Euler<A> {
        Euler {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen(),
        }
    }
}

#[cfg(feature = "mint")]
type MintEuler<S> = mint::EulerAngles<S, mint::IntraXYZ>;

#[cfg(feature = "mint")]
impl<S, A: Angle + From<S>> From<MintEuler<S>> for Euler<A> {
    fn from(mint: MintEuler<S>) -> Self {
        Euler {
            x: mint.a.into(),
            y: mint.b.into(),
            z: mint.c.into(),
        }
    }
}

#[cfg(feature = "mint")]
impl<S: Clone, A: Angle + Into<S>> Into<MintEuler<S>> for Euler<A> {
    fn into(self) -> MintEuler<S> {
        MintEuler::from([self.x.into(), self.y.into(), self.z.into()])
    }
}
