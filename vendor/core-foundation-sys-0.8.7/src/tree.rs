// Copyright 2023 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::c_void;

use crate::base::{CFAllocatorRef, CFComparatorFunction, CFIndex, CFTypeID};
use crate::string::CFStringRef;

#[repr(C)]
pub struct __CFTree(c_void);
pub type CFTreeRef = *mut __CFTree;

pub type CFTreeRetainCallBack = extern "C" fn(info: *const c_void) -> *const c_void;
pub type CFTreeReleaseCallBack = extern "C" fn(info: *const c_void);
pub type CFTreeCopyDescriptionCallBack = extern "C" fn(info: *const c_void) -> CFStringRef;
pub type CFTreeApplierFunction = extern "C" fn(value: *const c_void, context: *mut c_void);

#[repr(C)]
pub struct CFTreeContext {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: CFTreeRetainCallBack,
    pub release: CFTreeReleaseCallBack,
    pub copyDescription: CFTreeCopyDescriptionCallBack,
}

extern "C" {
    /*
     * CFTree.h
     */
    /* Creating Trees */
    pub fn CFTreeCreate(allocator: CFAllocatorRef, context: *const CFTreeContext) -> CFTreeRef;

    /* Modifying a Tree */
    pub fn CFTreeAppendChild(tree: CFTreeRef, newChild: CFTreeRef);
    pub fn CFTreeInsertSibling(tree: CFTreeRef, newSibling: CFTreeRef);
    pub fn CFTreeRemoveAllChildren(tree: CFTreeRef);
    pub fn CFTreePrependChild(tree: CFTreeRef, newChild: CFTreeRef);
    pub fn CFTreeRemove(tree: CFTreeRef);
    pub fn CFTreeSetContext(tree: CFTreeRef, context: *const CFTreeContext);

    /* Sorting a Tree */
    pub fn CFTreeSortChildren(
        tree: CFTreeRef,
        comparator: CFComparatorFunction,
        context: *mut c_void,
    );

    /* Examining a Tree */
    pub fn CFTreeFindRoot(tree: CFTreeRef) -> CFTreeRef;
    pub fn CFTreeGetChildAtIndex(tree: CFTreeRef, idx: CFIndex) -> CFTreeRef;
    pub fn CFTreeGetChildCount(tree: CFTreeRef) -> CFIndex;
    pub fn CFTreeGetChildren(tree: CFTreeRef, children: *mut CFTreeRef);
    pub fn CFTreeGetContext(tree: CFTreeRef, context: *mut CFTreeContext);
    pub fn CFTreeGetFirstChild(tree: CFTreeRef) -> CFTreeRef;
    pub fn CFTreeGetNextSibling(tree: CFTreeRef) -> CFTreeRef;
    pub fn CFTreeGetParent(tree: CFTreeRef) -> CFTreeRef;

    /* Performing an Operation on Tree Elements */
    pub fn CFTreeApplyFunctionToChildren(
        tree: CFTreeRef,
        applier: CFTreeApplierFunction,
        context: *mut c_void,
    );

    /* Getting the Tree Type ID */
    pub fn CFTreeGetTypeID() -> CFTypeID;
}
