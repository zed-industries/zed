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

use num_traits::cast;
use num_traits::Zero;

use structure::Angle;

use angle::Rad;
use matrix::Matrix4;
use num::BaseFloat;

/// Create a perspective projection matrix.
///
/// This is the equivalent to the [`gluPerspective`] function.
///
/// [`gluPerspective`]: https://www.opengl.org/sdk/docs/man2/xhtml/gluPerspective.xml
pub fn perspective<S: BaseFloat, A: Into<Rad<S>>>(
    fovy: A,
    aspect: S,
    near: S,
    far: S,
) -> Matrix4<S> {
    PerspectiveFov {
        fovy: fovy.into(),
        aspect: aspect,
        near: near,
        far: far,
    }
    .into()
}

/// Create a perspective matrix from a view frustum.
///
/// This is the equivalent of the now deprecated [`glFrustum`] function.
///
/// [`glFrustum`]: http://www.opengl.org/sdk/docs/man2/xhtml/glFrustum.xml
pub fn frustum<S: BaseFloat>(left: S, right: S, bottom: S, top: S, near: S, far: S) -> Matrix4<S> {
    Perspective {
        left: left,
        right: right,
        bottom: bottom,
        top: top,
        near: near,
        far: far,
    }
    .into()
}

/// Create an orthographic projection matrix.
///
/// This is the equivalent of the now deprecated [`glOrtho`] function.
///
/// [`glOrtho`]: http://www.opengl.org/sdk/docs/man2/xhtml/glOrtho.xml
pub fn ortho<S: BaseFloat>(left: S, right: S, bottom: S, top: S, near: S, far: S) -> Matrix4<S> {
    Ortho {
        left: left,
        right: right,
        bottom: bottom,
        top: top,
        near: near,
        far: far,
    }
    .into()
}

/// A perspective projection based on a vertical field-of-view angle.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "rustc-serialize", derive(RustcEncodable, RustcDecodable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PerspectiveFov<S> {
    pub fovy: Rad<S>,
    pub aspect: S,
    pub near: S,
    pub far: S,
}

impl<S: BaseFloat> PerspectiveFov<S> {
    pub fn to_perspective(&self) -> Perspective<S> {
        let two: S = cast(2).unwrap();
        let angle = self.fovy / two;
        let ymax = self.near * Rad::tan(angle);
        let xmax = ymax * self.aspect;

        Perspective {
            left: -xmax,
            right: xmax,
            bottom: -ymax,
            top: ymax,
            near: self.near.clone(),
            far: self.far.clone(),
        }
    }
}

impl<S: BaseFloat> From<PerspectiveFov<S>> for Matrix4<S> {
    fn from(persp: PerspectiveFov<S>) -> Matrix4<S> {
        assert!(
            persp.fovy > Rad::zero(),
            "The vertical field of view cannot be below zero, found: {:?}",
            persp.fovy
        );
        assert!(
            persp.fovy < Rad::turn_div_2(),
            "The vertical field of view cannot be greater than a half turn, found: {:?}",
            persp.fovy
        );

        assert!(
            abs_diff_ne!(persp.aspect.abs(), S::zero()),
            "The absolute aspect ratio cannot be zero, found: {:?}",
            persp.aspect.abs()
        );
        assert!(
            persp.near > S::zero(),
            "The near plane distance cannot be below zero, found: {:?}",
            persp.near
        );
        assert!(
            persp.far > S::zero(),
            "The far plane distance cannot be below zero, found: {:?}",
            persp.far
        );
        assert!(
            abs_diff_ne!(persp.far, persp.near),
            "The far plane and near plane are too close, found: far: {:?}, near: {:?}",
            persp.far,
            persp.near
        );

        let two: S = cast(2).unwrap();
        let f = Rad::cot(persp.fovy / two);

        let c0r0 = f / persp.aspect;
        let c0r1 = S::zero();
        let c0r2 = S::zero();
        let c0r3 = S::zero();

        let c1r0 = S::zero();
        let c1r1 = f;
        let c1r2 = S::zero();
        let c1r3 = S::zero();

        let c2r0 = S::zero();
        let c2r1 = S::zero();
        let c2r2 = (persp.far + persp.near) / (persp.near - persp.far);
        let c2r3 = -S::one();

        let c3r0 = S::zero();
        let c3r1 = S::zero();
        let c3r2 = (two * persp.far * persp.near) / (persp.near - persp.far);
        let c3r3 = S::zero();

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix4::new(
            c0r0, c0r1, c0r2, c0r3,
            c1r0, c1r1, c1r2, c1r3,
            c2r0, c2r1, c2r2, c2r3,
            c3r0, c3r1, c3r2, c3r3,
        )
    }
}

/// A perspective projection with arbitrary left/right/bottom/top distances
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Perspective<S> {
    pub left: S,
    pub right: S,
    pub bottom: S,
    pub top: S,
    pub near: S,
    pub far: S,
}

impl<S: BaseFloat> From<Perspective<S>> for Matrix4<S> {
    fn from(persp: Perspective<S>) -> Matrix4<S> {
        assert!(
            persp.left <= persp.right,
            "`left` cannot be greater than `right`, found: left: {:?} right: {:?}",
            persp.left,
            persp.right
        );
        assert!(
            persp.bottom <= persp.top,
            "`bottom` cannot be greater than `top`, found: bottom: {:?} top: {:?}",
            persp.bottom,
            persp.top
        );
        assert!(
            persp.near <= persp.far,
            "`near` cannot be greater than `far`, found: near: {:?} far: {:?}",
            persp.near,
            persp.far
        );

        let two: S = cast(2i8).unwrap();

        let c0r0 = (two * persp.near) / (persp.right - persp.left);
        let c0r1 = S::zero();
        let c0r2 = S::zero();
        let c0r3 = S::zero();

        let c1r0 = S::zero();
        let c1r1 = (two * persp.near) / (persp.top - persp.bottom);
        let c1r2 = S::zero();
        let c1r3 = S::zero();

        let c2r0 = (persp.right + persp.left) / (persp.right - persp.left);
        let c2r1 = (persp.top + persp.bottom) / (persp.top - persp.bottom);
        let c2r2 = -(persp.far + persp.near) / (persp.far - persp.near);
        let c2r3 = -S::one();

        let c3r0 = S::zero();
        let c3r1 = S::zero();
        let c3r2 = -(two * persp.far * persp.near) / (persp.far - persp.near);
        let c3r3 = S::zero();

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix4::new(
            c0r0, c0r1, c0r2, c0r3,
            c1r0, c1r1, c1r2, c1r3,
            c2r0, c2r1, c2r2, c2r3,
            c3r0, c3r1, c3r2, c3r3,
        )
    }
}

/// An orthographic projection with arbitrary left/right/bottom/top distances
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Ortho<S> {
    pub left: S,
    pub right: S,
    pub bottom: S,
    pub top: S,
    pub near: S,
    pub far: S,
}

impl<S: BaseFloat> From<Ortho<S>> for Matrix4<S> {
    fn from(ortho: Ortho<S>) -> Matrix4<S> {
        let two: S = cast(2).unwrap();

        let c0r0 = two / (ortho.right - ortho.left);
        let c0r1 = S::zero();
        let c0r2 = S::zero();
        let c0r3 = S::zero();

        let c1r0 = S::zero();
        let c1r1 = two / (ortho.top - ortho.bottom);
        let c1r2 = S::zero();
        let c1r3 = S::zero();

        let c2r0 = S::zero();
        let c2r1 = S::zero();
        let c2r2 = -two / (ortho.far - ortho.near);
        let c2r3 = S::zero();

        let c3r0 = -(ortho.right + ortho.left) / (ortho.right - ortho.left);
        let c3r1 = -(ortho.top + ortho.bottom) / (ortho.top - ortho.bottom);
        let c3r2 = -(ortho.far + ortho.near) / (ortho.far - ortho.near);
        let c3r3 = S::one();

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix4::new(
            c0r0, c0r1, c0r2, c0r3,
            c1r0, c1r1, c1r2, c1r3,
            c2r0, c2r1, c2r2, c2r3,
            c3r0, c3r1, c3r2, c3r3,
        )
    }
}
