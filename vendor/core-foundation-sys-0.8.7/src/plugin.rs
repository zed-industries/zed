// Copyright 2023 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::c_void;

use crate::array::CFArrayRef;
use crate::base::{Boolean, CFAllocatorRef, CFIndex, CFTypeID};
use crate::bundle::{CFBundleRef, CFPlugInRef};
use crate::string::CFStringRef;
use crate::url::CFURLRef;
use crate::uuid::CFUUIDRef;

#[repr(C)]
pub struct __CFPlugInInstance(c_void);
pub type CFPlugInInstanceRef = *mut __CFPlugInInstance;

pub type CFPlugInDynamicRegisterFunction = extern "C" fn(plugIn: CFPlugInRef);
pub type CFPlugInUnloadFunction = extern "C" fn(plugIn: CFPlugInRef);
pub type CFPlugInFactoryFunction =
    extern "C" fn(allocator: CFAllocatorRef, typeUUID: CFUUIDRef) -> *mut c_void;

pub type CFPlugInInstanceGetInterfaceFunction = extern "C" fn(
    instance: CFPlugInInstanceRef,
    interfaceName: CFStringRef,
    ftbl: *mut *mut c_void,
) -> Boolean;
pub type CFPlugInInstanceDeallocateInstanceDataFunction = extern "C" fn(instanceData: *mut c_void);

extern "C" {
    /*
     * CFPlugIn.h
     */

    /* CFPlugIn */
    /* Information Property List Keys */
    pub static kCFPlugInDynamicRegistrationKey: CFStringRef;
    pub static kCFPlugInDynamicRegisterFunctionKey: CFStringRef;
    pub static kCFPlugInUnloadFunctionKey: CFStringRef;
    pub static kCFPlugInFactoriesKey: CFStringRef;
    pub static kCFPlugInTypesKey: CFStringRef;

    /* Creating Plug-ins */
    pub fn CFPlugInCreate(allocator: CFAllocatorRef, plugInURL: CFURLRef) -> CFPlugInRef;
    pub fn CFPlugInInstanceCreate(
        allocator: CFAllocatorRef,
        factoryUUID: CFUUIDRef,
        typeUUID: CFUUIDRef,
    ) -> *mut c_void;

    /* Registration */
    pub fn CFPlugInRegisterFactoryFunction(
        factoryUUID: CFUUIDRef,
        func: CFPlugInFactoryFunction,
    ) -> Boolean;
    pub fn CFPlugInRegisterFactoryFunctionByName(
        CfactoryUUID: CFUUIDRef,
        plugIn: CFPlugInRef,
        functionName: CFStringRef,
    ) -> Boolean;
    pub fn CFPlugInRegisterPlugInType(factoryUUID: CFUUIDRef, typeUUID: CFUUIDRef) -> Boolean;
    pub fn CFPlugInUnregisterFactory(factoryUUID: CFUUIDRef) -> Boolean;
    pub fn CFPlugInUnregisterPlugInType(factoryUUID: CFUUIDRef, typeUUID: CFUUIDRef) -> Boolean;

    /* CFPlugIn Miscellaneous Functions */
    pub fn CFPlugInAddInstanceForFactory(factoryID: CFUUIDRef);
    pub fn CFPlugInFindFactoriesForPlugInType(typeUUID: CFUUIDRef) -> CFArrayRef;
    pub fn CFPlugInFindFactoriesForPlugInTypeInPlugIn(
        typeUUID: CFUUIDRef,
        plugIn: CFPlugInRef,
    ) -> CFArrayRef;
    pub fn CFPlugInGetBundle(plugIn: CFPlugInRef) -> CFBundleRef;
    pub fn CFPlugInGetTypeID() -> CFTypeID;
    pub fn CFPlugInIsLoadOnDemand(plugIn: CFPlugInRef) -> Boolean;
    pub fn CFPlugInRemoveInstanceForFactory(factoryID: CFUUIDRef);
    pub fn CFPlugInSetLoadOnDemand(plugIn: CFPlugInRef, flag: Boolean);

    /* CFPlugInInstance: deprecated */
    pub fn CFPlugInInstanceCreateWithInstanceDataSize(
        allocator: CFAllocatorRef,
        instanceDataSize: CFIndex,
        deallocateInstanceFunction: CFPlugInInstanceDeallocateInstanceDataFunction,
        factoryName: CFStringRef,
        getInterfaceFunction: CFPlugInInstanceGetInterfaceFunction,
    ) -> CFPlugInInstanceRef;
    pub fn CFPlugInInstanceGetFactoryName(instance: CFPlugInInstanceRef) -> CFStringRef;
    pub fn CFPlugInInstanceGetInstanceData(instance: CFPlugInInstanceRef) -> *mut c_void;
    pub fn CFPlugInInstanceGetInterfaceFunctionTable(
        instance: CFPlugInInstanceRef,
        interfaceName: CFStringRef,
        ftbl: *mut *mut c_void,
    ) -> Boolean;
    pub fn CFPlugInInstanceGetTypeID() -> CFTypeID;
}
