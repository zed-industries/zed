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

use quaternion::*;

use structure::*;

use std::mem;
use std::ops::*;

use simd::f32x4 as Simdf32x4;

impl From<Simdf32x4> for Quaternion<f32> {
    #[inline]
    fn from(f: Simdf32x4) -> Self {
        unsafe {
            let mut ret: Self = mem::uninitialized();
            {
                let ret_mut: &mut [f32; 4] = ret.as_mut();
                f.store(ret_mut.as_mut(), 0 as usize);
            }
            ret
        }
    }
}

impl Into<Simdf32x4> for Quaternion<f32> {
    #[inline]
    fn into(self) -> Simdf32x4 {
        let self_ref: &[f32; 4] = self.as_ref();
        Simdf32x4::load(self_ref.as_ref(), 0 as usize)
    }
}

impl InnerSpace for Quaternion<f32> {
    #[inline]
    fn dot(self, other: Quaternion<f32>) -> f32 {
        let lhs: Simdf32x4 = self.into();
        let rhs: Simdf32x4 = other.into();
        let r = lhs * rhs;
        r.extract(0) + r.extract(1) + r.extract(2) + r.extract(3)
    }
}

impl_operator_simd! {
    [Simdf32x4]; Neg for Quaternion<f32> {
        fn neg(lhs) -> Quaternion<f32> {
            (-lhs).into()
        }
    }
}

impl_operator_simd! {@rs
    [Simdf32x4]; Mul<f32> for Quaternion<f32> {
        fn mul(lhs, rhs) -> Quaternion<f32> {
            (lhs * rhs).into()
        }
    }
}

impl MulAssign<f32> for Quaternion<f32> {
    fn mul_assign(&mut self, other: f32) {
        let s: Simdf32x4 = (*self).into();
        let other = Simdf32x4::splat(other);
        *self = (s * other).into();
    }
}

impl_operator_simd! {@rs
    [Simdf32x4]; Div<f32> for Quaternion<f32> {
        fn div(lhs, rhs) -> Quaternion<f32> {
            (lhs / rhs).into()
        }
    }
}

impl DivAssign<f32> for Quaternion<f32> {
    fn div_assign(&mut self, other: f32) {
        let s: Simdf32x4 = (*self).into();
        let other = Simdf32x4::splat(other);
        *self = (s / other).into();
    }
}

impl_operator_simd! {
    [Simdf32x4]; Add<Quaternion<f32>> for Quaternion<f32> {
        fn add(lhs, rhs) -> Quaternion<f32> {
            (lhs + rhs).into()
        }
    }
}

impl AddAssign for Quaternion<f32> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        let s: Simdf32x4 = (*self).into();
        let rhs: Simdf32x4 = rhs.into();
        *self = (s + rhs).into();
    }
}

impl_operator_simd! {
    [Simdf32x4]; Sub<Quaternion<f32>> for Quaternion<f32> {
        fn sub(lhs, rhs) -> Quaternion<f32> {
            (lhs - rhs).into()
        }
    }
}

impl SubAssign for Quaternion<f32> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        let s: Simdf32x4 = (*self).into();
        let rhs: Simdf32x4 = rhs.into();
        *self = (s - rhs).into();
    }
}

impl_operator_simd! {
    [Simdf32x4]; Mul<Quaternion<f32>> for Quaternion<f32> {
        fn mul(lhs, rhs) -> Quaternion<f32> {
            {
                let p0 = Simdf32x4::splat(lhs.extract(0)) * rhs;
                let p1 = Simdf32x4::splat(lhs.extract(1)) * Simdf32x4::new(
                    -rhs.extract(1), rhs.extract(0), -rhs.extract(3), rhs.extract(2)
                );
                let p2 = Simdf32x4::splat(lhs.extract(2)) * Simdf32x4::new(
                    -rhs.extract(2), rhs.extract(3), rhs.extract(0), -rhs.extract(1)
                );
                let p3 = Simdf32x4::splat(lhs.extract(3)) * Simdf32x4::new(
                    -rhs.extract(3), -rhs.extract(2), rhs.extract(1), rhs.extract(0)
                );
                (p0 + p1 + p2 + p3).into()
            }
        }
    }
}
