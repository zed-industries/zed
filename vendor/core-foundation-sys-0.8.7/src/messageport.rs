// Copyright 2013-2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::c_void;

use crate::base::{Boolean, CFAllocatorRef, CFIndex, CFTypeID, SInt32};
use crate::data::CFDataRef;
use crate::date::CFTimeInterval;
use crate::runloop::CFRunLoopSourceRef;
use crate::string::CFStringRef;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CFMessagePortContext {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: Option<unsafe extern "C" fn(info: *const c_void) -> *const c_void>,
    pub release: Option<unsafe extern "C" fn(info: *const c_void)>,
    pub copyDescription: Option<unsafe extern "C" fn(info: *const c_void) -> CFStringRef>,
}

pub type CFMessagePortCallBack = Option<
    unsafe extern "C" fn(
        local: CFMessagePortRef,
        msgid: i32,
        data: CFDataRef,
        info: *mut c_void,
    ) -> CFDataRef,
>;

pub type CFMessagePortInvalidationCallBack =
    Option<unsafe extern "C" fn(ms: CFMessagePortRef, info: *mut c_void)>;

/* CFMessagePortSendRequest Error Codes */
pub const kCFMessagePortSuccess: SInt32 = 0;
pub const kCFMessagePortSendTimeout: SInt32 = -1;
pub const kCFMessagePortReceiveTimeout: SInt32 = -2;
pub const kCFMessagePortIsInvalid: SInt32 = -3;
pub const kCFMessagePortTransportError: SInt32 = -4;
pub const kCFMessagePortBecameInvalidError: SInt32 = -5;

#[repr(C)]
pub struct __CFMessagePort(c_void);
pub type CFMessagePortRef = *mut __CFMessagePort;

extern "C" {
    /*
     * CFMessagePort.h
     */

    /* Creating a CFMessagePort Object */
    pub fn CFMessagePortCreateLocal(
        allocator: CFAllocatorRef,
        name: CFStringRef,
        callout: CFMessagePortCallBack,
        context: *const CFMessagePortContext,
        shouldFreeInfo: *mut Boolean,
    ) -> CFMessagePortRef;
    pub fn CFMessagePortCreateRemote(
        allocator: CFAllocatorRef,
        name: CFStringRef,
    ) -> CFMessagePortRef;

    /* Configuring a CFMessagePort Object */
    pub fn CFMessagePortCreateRunLoopSource(
        allocator: CFAllocatorRef,
        local: CFMessagePortRef,
        order: CFIndex,
    ) -> CFRunLoopSourceRef;
    pub fn CFMessagePortSetInvalidationCallBack(
        ms: CFMessagePortRef,
        callout: CFMessagePortInvalidationCallBack,
    );
    pub fn CFMessagePortSetName(ms: CFMessagePortRef, newName: CFStringRef) -> Boolean;

    /* Using a Message Port */
    pub fn CFMessagePortInvalidate(ms: CFMessagePortRef);
    pub fn CFMessagePortSendRequest(
        remote: CFMessagePortRef,
        msgid: i32,
        data: CFDataRef,
        sendTimeout: CFTimeInterval,
        rcvTimeout: CFTimeInterval,
        replyMode: CFStringRef,
        returnData: *mut CFDataRef,
    ) -> i32;
    //pub fn CFMessagePortSetDispatchQueue(ms: CFMessagePortRef, queue: dispatch_queue_t);

    /* Examining a Message Port */
    pub fn CFMessagePortGetContext(ms: CFMessagePortRef, context: *mut CFMessagePortContext);
    pub fn CFMessagePortGetInvalidationCallBack(
        ms: CFMessagePortRef,
    ) -> CFMessagePortInvalidationCallBack;
    pub fn CFMessagePortGetName(ms: CFMessagePortRef) -> CFStringRef;
    pub fn CFMessagePortIsRemote(ms: CFMessagePortRef) -> Boolean;
    pub fn CFMessagePortIsValid(ms: CFMessagePortRef) -> Boolean;

    /* Getting the CFMessagePort Type ID */
    pub fn CFMessagePortGetTypeID() -> CFTypeID;
}
