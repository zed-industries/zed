// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! This module contains basic instance and class structs to be used for
//! `GObject` subclasses that don't require any additional data in these
//! structs and don't provide any new virtual methods.

use std::{fmt, ops};

use super::prelude::*;
use crate::prelude::*;

// rustdoc-stripper-ignore-next
/// A basic instance struct that does not store any additional data.
#[repr(C)]
pub struct InstanceStruct<T: ObjectSubclass> {
    parent: <T::ParentType as ObjectType>::GlibType,
}

impl<T: ObjectSubclass> fmt::Debug for InstanceStruct<T>
where
    <T::ParentType as ObjectType>::GlibType: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("InstanceStruct")
            .field("parent", &self.parent)
            .finish()
    }
}

unsafe impl<T: ObjectSubclass> super::types::InstanceStruct for InstanceStruct<T> {
    type Type = T;
}

// rustdoc-stripper-ignore-next
/// A basic class struct that does not store any additional data
/// or virtual methods.
#[repr(C)]
pub struct ClassStruct<T: ObjectSubclass> {
    parent_class: <T::ParentType as ObjectType>::GlibClassType,
}

impl<T: ObjectSubclass> fmt::Debug for ClassStruct<T>
where
    <T::ParentType as ObjectType>::GlibClassType: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("InstanceStruct")
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

unsafe impl<T: ObjectSubclass> super::types::ClassStruct for ClassStruct<T> {
    type Type = T;
}

impl<T: ObjectSubclass> ops::Deref for ClassStruct<T> {
    type Target = crate::Class<<T as ObjectSubclass>::Type>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const _ as *const Self::Target) }
    }
}

impl<T: ObjectSubclass> ops::DerefMut for ClassStruct<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut _ as *mut Self::Target) }
    }
}
