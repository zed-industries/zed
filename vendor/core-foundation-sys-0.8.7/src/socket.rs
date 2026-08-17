// Copyright 2023 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::c_void;

use crate::base::{Boolean, CFAllocatorRef, CFIndex, CFOptionFlags, CFTypeID, SInt32, UInt16};
use crate::data::CFDataRef;
use crate::date::CFTimeInterval;
use crate::propertylist::CFPropertyListRef;
use crate::runloop::CFRunLoopSourceRef;
use crate::string::CFStringRef;

#[repr(C)]
pub struct __CFSocket(c_void);

pub type CFSocketRef = *mut __CFSocket;

pub type CFSocketError = CFIndex;
pub type CFSocketCallBackType = CFOptionFlags;
pub type CFSocketCallBack = extern "C" fn(
    s: CFSocketRef,
    _type: CFSocketCallBackType,
    address: CFDataRef,
    cdata: *const c_void,
    info: *mut c_void,
);
#[cfg(not(target_os = "windows"))]
pub type CFSocketNativeHandle = std::os::raw::c_int;
#[cfg(target_os = "windows")]
pub type CFSocketNativeHandle = std::os::raw::c_ulong;

pub const kCFSocketSuccess: CFSocketError = 0;
pub const kCFSocketError: CFSocketError = -1;
pub const kCFSocketTimeout: CFSocketError = -2;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct CFSocketSignature {
    pub protocolFamily: SInt32,
    pub socketType: SInt32,
    pub protocol: SInt32,
    pub address: CFDataRef,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct CFSocketContext {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: extern "C" fn(info: *const c_void) -> *const c_void,
    pub release: extern "C" fn(info: *const c_void),
    pub copyDescription: extern "C" fn(info: *const c_void) -> CFStringRef,
}

pub const kCFSocketNoCallBack: CFSocketError = 0;
pub const kCFSocketReadCallBack: CFSocketError = 1;
pub const kCFSocketAcceptCallBack: CFSocketError = 2;
pub const kCFSocketDataCallBack: CFSocketError = 3;
pub const kCFSocketConnectCallBack: CFSocketError = 4;
pub const kCFSocketWriteCallBack: CFSocketError = 8;

pub const kCFSocketAutomaticallyReenableReadCallBack: CFOptionFlags = 1;
pub const kCFSocketAutomaticallyReenableAcceptCallBack: CFOptionFlags = 2;
pub const kCFSocketAutomaticallyReenableDataCallBack: CFOptionFlags = 3;
pub const kCFSocketAutomaticallyReenableWriteCallBack: CFOptionFlags = 8;
pub const kCFSocketLeaveErrors: CFOptionFlags = 64;
pub const kCFSocketCloseOnInvalidate: CFOptionFlags = 128;

extern "C" {
    /*
     * CFSocket.h
     */

    /* CFSocket Name Server Keys: Not used */
    pub static kCFSocketCommandKey: CFStringRef;
    pub static kCFSocketNameKey: CFStringRef;
    pub static kCFSocketValueKey: CFStringRef;
    pub static kCFSocketResultKey: CFStringRef;
    pub static kCFSocketErrorKey: CFStringRef;
    pub static kCFSocketRegisterCommand: CFStringRef;
    pub static kCFSocketRetrieveCommand: CFStringRef;

    /* Creating Sockets */
    pub fn CFSocketCreate(
        allocator: CFAllocatorRef,
        protocolFamily: SInt32,
        socketType: SInt32,
        protocol: SInt32,
        callBackTypes: CFOptionFlags,
        callout: CFSocketCallBack,
        context: *const CFSocketContext,
    ) -> CFSocketRef;
    pub fn CFSocketCreateConnectedToSocketSignature(
        allocator: CFAllocatorRef,
        signature: *const CFSocketSignature,
        callBackTypes: CFOptionFlags,
        callout: CFSocketCallBack,
        context: *const CFSocketContext,
        timeout: CFTimeInterval,
    ) -> CFSocketRef;
    pub fn CFSocketCreateWithNative(
        allocator: CFAllocatorRef,
        sock: CFSocketNativeHandle,
        callBackTypes: CFOptionFlags,
        callout: CFSocketCallBack,
        context: *const CFSocketContext,
    ) -> CFSocketRef;
    pub fn CFSocketCreateWithSocketSignature(
        allocator: CFAllocatorRef,
        signature: *const CFSocketSignature,
        callBackTypes: CFOptionFlags,
        callout: CFSocketCallBack,
        context: *const CFSocketContext,
    ) -> CFSocketRef;

    /* Configuring Sockets */
    pub fn CFSocketCopyAddress(s: CFSocketRef) -> CFDataRef;
    pub fn CFSocketCopyPeerAddress(s: CFSocketRef) -> CFDataRef;
    pub fn CFSocketDisableCallBacks(s: CFSocketRef, callBackTypes: CFOptionFlags);
    pub fn CFSocketEnableCallBacks(s: CFSocketRef, callBackTypes: CFOptionFlags);
    pub fn CFSocketGetContext(s: CFSocketRef, context: *mut CFSocketContext);
    pub fn CFSocketGetNative(s: CFSocketRef) -> CFSocketNativeHandle;
    pub fn CFSocketGetSocketFlags(s: CFSocketRef) -> CFOptionFlags;
    pub fn CFSocketSetAddress(s: CFSocketRef, address: CFDataRef) -> CFSocketError;
    pub fn CFSocketSetSocketFlags(s: CFSocketRef, flags: CFOptionFlags);

    /* Using Sockets */
    pub fn CFSocketConnectToAddress(
        s: CFSocketRef,
        address: CFDataRef,
        timeout: CFTimeInterval,
    ) -> CFSocketError;
    pub fn CFSocketCreateRunLoopSource(
        allocator: CFAllocatorRef,
        s: CFSocketRef,
        order: CFIndex,
    ) -> CFRunLoopSourceRef;
    pub fn CFSocketGetTypeID() -> CFTypeID;
    pub fn CFSocketInvalidate(s: CFSocketRef);
    pub fn CFSocketIsValid(s: CFSocketRef) -> Boolean;
    pub fn CFSocketSendData(
        s: CFSocketRef,
        address: CFDataRef,
        data: CFDataRef,
        timeout: CFTimeInterval,
    ) -> CFSocketError;

    /* Socket Name Server Utilities */
    pub fn CFSocketCopyRegisteredSocketSignature(
        nameServerSignature: *const CFSocketSignature,
        timeout: CFTimeInterval,
        name: CFStringRef,
        signature: *mut CFSocketSignature,
        nameServerAddress: *mut CFDataRef,
    ) -> CFSocketError;
    pub fn CFSocketCopyRegisteredValue(
        nameServerSignature: *const CFSocketSignature,
        timeout: CFTimeInterval,
        name: CFStringRef,
        value: *mut CFPropertyListRef,
        nameServerAddress: *mut CFDataRef,
    ) -> CFSocketError;
    pub fn CFSocketGetDefaultNameRegistryPortNumber() -> UInt16;
    pub fn CFSocketRegisterSocketSignature(
        nameServerSignature: *const CFSocketSignature,
        timeout: CFTimeInterval,
        name: CFStringRef,
        signature: *const CFSocketSignature,
    ) -> CFSocketError;
    pub fn CFSocketRegisterValue(
        nameServerSignature: *const CFSocketSignature,
        timeout: CFTimeInterval,
        name: CFStringRef,
        value: CFPropertyListRef,
    ) -> CFSocketError;
    pub fn CFSocketSetDefaultNameRegistryPortNumber(port: UInt16);
    pub fn CFSocketUnregister(
        nameServerSignature: *const CFSocketSignature,
        timeout: CFTimeInterval,
        name: CFStringRef,
    ) -> CFSocketError;
}
