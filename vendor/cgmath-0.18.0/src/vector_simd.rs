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

use vector::*;

use structure::*;

use std::mem;
use std::ops::*;

use simd::f32x4 as Simdf32x4;
use simd::i32x4 as Simdi32x4;
use simd::u32x4 as Simdu32x4;

impl From<Simdf32x4> for Vector4<f32> {
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

impl Vector4<f32> {
    /// Compute and return the square root of each element.
    #[inline]
    pub fn sqrt_element_wide(self) -> Self {
        let s: Simdf32x4 = self.into();
        s.sqrt().into()
    }

    /// Compute and return the reciprocal of the square root of each element.
    #[inline]
    pub fn rsqrt_element_wide(self) -> Self {
        let s: Simdf32x4 = self.into();
        s.approx_rsqrt().into()
    }

    /// Compute and return the reciprocal of each element.
    #[inline]
    pub fn recip_element_wide(self) -> Self {
        let s: Simdf32x4 = self.into();
        s.approx_reciprocal().into()
    }
}

impl Into<Simdf32x4> for Vector4<f32> {
    #[inline]
    fn into(self) -> Simdf32x4 {
        let self_ref: &[f32; 4] = self.as_ref();
        Simdf32x4::load(self_ref.as_ref(), 0 as usize)
    }
}

impl_operator_simd! {
    [Simdf32x4]; Add<Vector4<f32>> for Vector4<f32> {
        fn add(lhs, rhs) -> Vector4<f32> {
            (lhs + rhs).into()
        }
    }
}

impl_operator_simd! {
    [Simdf32x4]; Sub<Vector4<f32>> for Vector4<f32> {
        fn sub(lhs, rhs) -> Vector4<f32> {
            (lhs - rhs).into()
        }
    }
}

impl_operator_simd! {@rs
    [Simdf32x4]; Mul<f32> for Vector4<f32> {
        fn mul(lhs, rhs) -> Vector4<f32> {
            (lhs * rhs).into()
        }
    }
}

impl_operator_simd! {@rs
    [Simdf32x4]; Div<f32> for Vector4<f32> {
        fn div(lhs, rhs) -> Vector4<f32> {
            (lhs / rhs).into()
        }
    }
}

impl_operator_simd! {
    [Simdf32x4]; Neg for Vector4<f32> {
        fn neg(lhs) -> Vector4<f32> {
            (-lhs).into()
        }
    }
}

impl AddAssign for Vector4<f32> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        let s: Simdf32x4 = (*self).into();
        let rhs: Simdf32x4 = rhs.into();
        *self = (s + rhs).into();
    }
}

impl SubAssign for Vector4<f32> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        let s: Simdf32x4 = (*self).into();
        let rhs: Simdf32x4 = rhs.into();
        *self = (s - rhs).into();
    }
}

impl MulAssign<f32> for Vector4<f32> {
    fn mul_assign(&mut self, other: f32) {
        let s: Simdf32x4 = (*self).into();
        let other = Simdf32x4::splat(other);
        *self = (s * other).into();
    }
}

impl DivAssign<f32> for Vector4<f32> {
    fn div_assign(&mut self, other: f32) {
        let s: Simdf32x4 = (*self).into();
        let other = Simdf32x4::splat(other);
        *self = (s / other).into();
    }
}

impl ElementWise for Vector4<f32> {
    #[inline]
    fn add_element_wise(self, rhs: Vector4<f32>) -> Vector4<f32> {
        self + rhs
    }
    #[inline]
    fn sub_element_wise(self, rhs: Vector4<f32>) -> Vector4<f32> {
        self - rhs
    }
    #[inline]
    fn mul_element_wise(self, rhs: Vector4<f32>) -> Vector4<f32> {
        let s: Simdf32x4 = self.into();
        let rhs: Simdf32x4 = rhs.into();
        (s * rhs).into()
    }
    #[inline]
    fn div_element_wise(self, rhs: Vector4<f32>) -> Vector4<f32> {
        let s: Simdf32x4 = self.into();
        let rhs: Simdf32x4 = rhs.into();
        (s / rhs).into()
    }

    #[inline]
    fn add_assign_element_wise(&mut self, rhs: Vector4<f32>) {
        (*self) += rhs;
    }

    #[inline]
    fn sub_assign_element_wise(&mut self, rhs: Vector4<f32>) {
        (*self) -= rhs;
    }

    #[inline]
    fn mul_assign_element_wise(&mut self, rhs: Vector4<f32>) {
        let s: Simdf32x4 = (*self).into();
        let rhs: Simdf32x4 = rhs.into();
        *self = (s * rhs).into();
    }

    #[inline]
    fn div_assign_element_wise(&mut self, rhs: Vector4<f32>) {
        let s: Simdf32x4 = (*self).into();
        let rhs: Simdf32x4 = rhs.into();
        *self = (s * rhs).into();
    }
}

impl ElementWise<f32> for Vector4<f32> {
    #[inline]
    fn add_element_wise(self, rhs: f32) -> Vector4<f32> {
        let s: Simdf32x4 = self.into();
        let rhs = Simdf32x4::splat(rhs);
        (s + rhs).into()
    }

    #[inline]
    fn sub_element_wise(self, rhs: f32) -> Vector4<f32> {
        let s: Simdf32x4 = self.into();
        let rhs = Simdf32x4::splat(rhs);
        (s - rhs).into()
    }

    #[inline]
    fn mul_element_wise(self, rhs: f32) -> Vector4<f32> {
        self * rhs
    }

    #[inline]
    fn div_element_wise(self, rhs: f32) -> Vector4<f32> {
        self / rhs
    }

    #[inline]
    fn add_assign_element_wise(&mut self, rhs: f32) {
        let s: Simdf32x4 = (*self).into();
        let rhs = Simdf32x4::splat(rhs);
        *self = (s + rhs).into();
    }

    #[inline]
    fn sub_assign_element_wise(&mut self, rhs: f32) {
        let s: Simdf32x4 = (*self).into();
        let rhs = Simdf32x4::splat(rhs);
        *self = (s - rhs).into();
    }

    #[inline]
    fn mul_assign_element_wise(&mut self, rhs: f32) {
        (*self) *= rhs;
    }

    #[inline]
    fn div_assign_element_wise(&mut self, rhs: f32) {
        (*self) /= rhs;
    }
}

impl From<Simdi32x4> for Vector4<i32> {
    #[inline]
    fn from(f: Simdi32x4) -> Self {
        unsafe {
            let mut ret: Self = mem::uninitialized();
            {
                let ret_mut: &mut [i32; 4] = ret.as_mut();
                f.store(ret_mut.as_mut(), 0 as usize);
            }
            ret
        }
    }
}

impl Into<Simdi32x4> for Vector4<i32> {
    #[inline]
    fn into(self) -> Simdi32x4 {
        let self_ref: &[i32; 4] = self.as_ref();
        Simdi32x4::load(self_ref.as_ref(), 0 as usize)
    }
}

impl_operator_simd! {
    [Simdi32x4]; Add<Vector4<i32>> for Vector4<i32> {
        fn add(lhs, rhs) -> Vector4<i32> {
            (lhs + rhs).into()
        }
    }
}

impl_operator_simd! {
    [Simdi32x4]; Sub<Vector4<i32>> for Vector4<i32> {
        fn sub(lhs, rhs) -> Vector4<i32> {
            (lhs - rhs).into()
        }
    }
}

impl_operator_simd! {@rs
    [Simdi32x4]; Mul<i32> for Vector4<i32> {
        fn mul(lhs, rhs) -> Vector4<i32> {
            (lhs * rhs).into()
        }
    }
}

impl_operator_simd! {
    [Simdi32x4]; Neg for Vector4<i32> {
        fn neg(lhs) -> Vector4<i32> {
            (-lhs).into()
        }
    }
}

impl AddAssign for Vector4<i32> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        let s: Simdi32x4 = (*self).into();
        let rhs: Simdi32x4 = rhs.into();
        *self = (s + rhs).into();
    }
}

impl SubAssign for Vector4<i32> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        let s: Simdi32x4 = (*self).into();
        let rhs: Simdi32x4 = rhs.into();
        *self = (s - rhs).into();
    }
}

impl MulAssign<i32> for Vector4<i32> {
    fn mul_assign(&mut self, other: i32) {
        let s: Simdi32x4 = (*self).into();
        let other = Simdi32x4::splat(other);
        *self = (s * other).into();
    }
}

impl From<Simdu32x4> for Vector4<u32> {
    #[inline]
    fn from(f: Simdu32x4) -> Self {
        unsafe {
            let mut ret: Self = mem::uninitialized();
            {
                let ret_mut: &mut [u32; 4] = ret.as_mut();
                f.store(ret_mut.as_mut(), 0 as usize);
            }
            ret
        }
    }
}

impl Into<Simdu32x4> for Vector4<u32> {
    #[inline]
    fn into(self) -> Simdu32x4 {
        let self_ref: &[u32; 4] = self.as_ref();
        Simdu32x4::load(self_ref.as_ref(), 0 as usize)
    }
}

impl_operator_simd! {
    [Simdu32x4]; Add<Vector4<u32>> for Vector4<u32> {
        fn add(lhs, rhs) -> Vector4<u32> {
            (lhs + rhs).into()
        }
    }
}

impl_operator_simd! {
    [Simdu32x4]; Sub<Vector4<u32>> for Vector4<u32> {
        fn sub(lhs, rhs) -> Vector4<u32> {
            (lhs - rhs).into()
        }
    }
}

impl_operator_simd! {@rs
    [Simdu32x4]; Mul<u32> for Vector4<u32> {
        fn mul(lhs, rhs) -> Vector4<u32> {
            (lhs * rhs).into()
        }
    }
}

impl AddAssign for Vector4<u32> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        let s: Simdu32x4 = (*self).into();
        let rhs: Simdu32x4 = rhs.into();
        *self = (s + rhs).into();
    }
}

impl SubAssign for Vector4<u32> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        let s: Simdu32x4 = (*self).into();
        let rhs: Simdu32x4 = rhs.into();
        *self = (s - rhs).into();
    }
}

impl MulAssign<u32> for Vector4<u32> {
    fn mul_assign(&mut self, other: u32) {
        let s: Simdu32x4 = (*self).into();
        let other = Simdu32x4::splat(other);
        *self = (s * other).into();
    }
}
