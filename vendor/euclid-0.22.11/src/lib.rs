// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(not(test), no_std)]

//! A collection of strongly typed math tools for computer graphics with an inclination
//! towards 2d graphics and layout.
//!
//! All types are generic over the scalar type of their component (`f32`, `i32`, etc.),
//! and tagged with a generic Unit parameter which is useful to prevent mixing
//! values from different spaces. For example it should not be legal to translate
//! a screen-space position by a world-space vector and this can be expressed using
//! the generic Unit parameter.
//!
//! This unit system is not mandatory and all structures have an alias
//! with the default unit: `UnknownUnit`.
//! for example ```default::Point2D<T>``` is equivalent to ```Point2D<T, UnknownUnit>```.
//! Client code typically creates a set of aliases for each type and doesn't need
//! to deal with the specifics of typed units further. For example:
//!
//! ```rust
//! use euclid::*;
//! pub struct ScreenSpace;
//! pub type ScreenPoint = Point2D<f32, ScreenSpace>;
//! pub type ScreenSize = Size2D<f32, ScreenSpace>;
//! pub struct WorldSpace;
//! pub type WorldPoint = Point3D<f32, WorldSpace>;
//! pub type ProjectionMatrix = Transform3D<f32, WorldSpace, ScreenSpace>;
//! // etc...
//! ```
//!
//! All euclid types are marked `#[repr(C)]` in order to facilitate exposing them to
//! foreign function interfaces (provided the underlying scalar type is also `repr(C)`).
//!
#![deny(unconditional_recursion)]

pub use crate::angle::Angle;
pub use crate::box2d::Box2D;
pub use crate::homogen::HomogeneousVector;
pub use crate::length::Length;
pub use crate::point::{point2, point3, Point2D, Point3D};
pub use crate::scale::Scale;
pub use crate::transform2d::Transform2D;
pub use crate::transform3d::Transform3D;
pub use crate::vector::{bvec2, bvec3, BoolVector2D, BoolVector3D};
pub use crate::vector::{vec2, vec3, Vector2D, Vector3D};

pub use crate::box3d::{box3d, Box3D};
pub use crate::rect::{rect, Rect};
pub use crate::rigid::RigidTransform3D;
pub use crate::rotation::{Rotation2D, Rotation3D};
pub use crate::side_offsets::SideOffsets2D;
pub use crate::size::{size2, size3, Size2D, Size3D};
pub use crate::translation::{Translation2D, Translation3D};
pub use crate::trig::Trig;

#[macro_use]
mod macros;

mod angle;
pub mod approxeq;
pub mod approxord;
mod box2d;
mod box3d;
mod homogen;
mod length;
pub mod num;
mod point;
mod rect;
mod rigid;
mod rotation;
mod scale;
mod side_offsets;
mod size;
mod transform2d;
mod transform3d;
mod translation;
mod trig;
mod vector;

/// The default unit.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnknownUnit;

pub mod default {
    //! A set of aliases for all types, tagged with the default unknown unit.

    use super::UnknownUnit;
    pub type Length<T> = super::Length<T, UnknownUnit>;
    pub type Point2D<T> = super::Point2D<T, UnknownUnit>;
    pub type Point3D<T> = super::Point3D<T, UnknownUnit>;
    pub type Vector2D<T> = super::Vector2D<T, UnknownUnit>;
    pub type Vector3D<T> = super::Vector3D<T, UnknownUnit>;
    pub type HomogeneousVector<T> = super::HomogeneousVector<T, UnknownUnit>;
    pub type Size2D<T> = super::Size2D<T, UnknownUnit>;
    pub type Size3D<T> = super::Size3D<T, UnknownUnit>;
    pub type Rect<T> = super::Rect<T, UnknownUnit>;
    pub type Box2D<T> = super::Box2D<T, UnknownUnit>;
    pub type Box3D<T> = super::Box3D<T, UnknownUnit>;
    pub type SideOffsets2D<T> = super::SideOffsets2D<T, UnknownUnit>;
    pub type Transform2D<T> = super::Transform2D<T, UnknownUnit, UnknownUnit>;
    pub type Transform3D<T> = super::Transform3D<T, UnknownUnit, UnknownUnit>;
    pub type Rotation2D<T> = super::Rotation2D<T, UnknownUnit, UnknownUnit>;
    pub type Rotation3D<T> = super::Rotation3D<T, UnknownUnit, UnknownUnit>;
    pub type Translation2D<T> = super::Translation2D<T, UnknownUnit, UnknownUnit>;
    pub type Translation3D<T> = super::Translation3D<T, UnknownUnit, UnknownUnit>;
    pub type Scale<T> = super::Scale<T, UnknownUnit, UnknownUnit>;
    pub type RigidTransform3D<T> = super::RigidTransform3D<T, UnknownUnit, UnknownUnit>;
}
