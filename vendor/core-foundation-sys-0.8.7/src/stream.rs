// Copyright 2023 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::{c_int, c_void};

use crate::base::{
    Boolean, CFAllocatorRef, CFIndex, CFOptionFlags, CFTypeID, CFTypeRef, SInt32, UInt32, UInt8,
};
use crate::error::CFErrorRef;
use crate::runloop::CFRunLoopRef;
use crate::socket::{CFSocketNativeHandle, CFSocketSignature};
use crate::string::CFStringRef;
use crate::url::CFURLRef;

#[repr(C)]
pub struct __CFReadStream(c_void);

#[repr(C)]
pub struct __CFWriteStream(c_void);

pub type CFReadStreamRef = *mut __CFReadStream;
pub type CFWriteStreamRef = *mut __CFWriteStream;
pub type CFStreamPropertyKey = CFStringRef;
pub type CFStreamStatus = CFIndex;
pub type CFStreamEventType = CFOptionFlags;
pub type CFStreamErrorDomain = CFIndex;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CFStreamError {
    pub domain: CFIndex,
    pub error: SInt32,
}

/* CFStreamStatus: Constants that describe the status of a stream */
pub const kCFStreamStatusNotOpen: CFStreamStatus = 0;
pub const kCFStreamStatusOpening: CFStreamStatus = 1;
pub const kCFStreamStatusOpen: CFStreamStatus = 2;
pub const kCFStreamStatusReading: CFStreamStatus = 3;
pub const kCFStreamStatusWriting: CFStreamStatus = 4;
pub const kCFStreamStatusAtEnd: CFStreamStatus = 5;
pub const kCFStreamStatusClosed: CFStreamStatus = 6;
pub const kCFStreamStatusError: CFStreamStatus = 7;

// deprecated
pub const kCFStreamErrorDomainCustom: CFStreamErrorDomain = -1;
pub const kCFStreamErrorDomainPOSIX: CFStreamErrorDomain = 1;
pub const kCFStreamErrorDomainMacOSStatus: CFStreamErrorDomain = 2;

/* CFStreamEventType: Defines constants for stream-related events */
pub const kCFStreamEventNone: CFStreamEventType = 0;
pub const kCFStreamEventOpenCompleted: CFStreamEventType = 1;
pub const kCFStreamEventHasBytesAvailable: CFStreamEventType = 2;
pub const kCFStreamEventCanAcceptBytes: CFStreamEventType = 4;
pub const kCFStreamEventErrorOccurred: CFStreamEventType = 8;
pub const kCFStreamEventEndEncountered: CFStreamEventType = 16;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CFStreamClientContext {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: extern "C" fn(info: *const c_void) -> *const c_void,
    pub release: extern "C" fn(info: *const c_void),
    pub copyDescription: extern "C" fn(info: *const c_void) -> CFStringRef,
}

pub type CFReadStreamClientCallBack = extern "C" fn(
    stream: CFReadStreamRef,
    _type: CFStreamEventType,
    clientCallBackInfo: *mut c_void,
);
pub type CFWriteStreamClientCallBack = extern "C" fn(
    stream: CFWriteStreamRef,
    _type: CFStreamEventType,
    clientCallBackInfo: *mut c_void,
);

extern "C" {
    /*
     * CFStream.h
     */

    /* Stream Properties */
    pub static kCFStreamPropertyAppendToFile: CFStreamPropertyKey;
    pub static kCFStreamPropertyDataWritten: CFStreamPropertyKey;
    pub static kCFStreamPropertyFileCurrentOffset: CFStreamPropertyKey;
    pub static kCFStreamPropertySocketNativeHandle: CFStreamPropertyKey;
    pub static kCFStreamPropertySocketRemoteHostName: CFStreamPropertyKey;
    pub static kCFStreamPropertySocketRemotePortNumber: CFStreamPropertyKey;
    pub static kCFStreamPropertyShouldCloseNativeSocket: CFStringRef;
    pub static kCFStreamPropertySocketSecurityLevel: CFStringRef;

    /* CFStream Socket Security Level Constants */
    pub static kCFStreamSocketSecurityLevelNone: CFStringRef;
    pub static kCFStreamSocketSecurityLevelSSLv2: CFStringRef;
    pub static kCFStreamSocketSecurityLevelSSLv3: CFStringRef;
    pub static kCFStreamSocketSecurityLevelTLSv1: CFStringRef;
    pub static kCFStreamSocketSecurityLevelNegotiatedSSL: CFStringRef;

    /* CFStream SOCKS Proxy Key Constants */
    pub static kCFStreamPropertySOCKSProxy: CFStringRef;
    pub static kCFStreamPropertySOCKSProxyHost: CFStringRef;
    pub static kCFStreamPropertySOCKSProxyPort: CFStringRef;
    pub static kCFStreamPropertySOCKSVersion: CFStringRef;
    pub static kCFStreamSocketSOCKSVersion4: CFStringRef;
    pub static kCFStreamSocketSOCKSVersion5: CFStringRef;
    pub static kCFStreamPropertySOCKSUser: CFStringRef;
    pub static kCFStreamPropertySOCKSPassword: CFStringRef;

    /* CFStream Error Domain Constants (CFHost) */
    pub static kCFStreamErrorDomainSOCKS: c_int;
    pub static kCFStreamErrorDomainSSL: c_int;

    /* CFStream: Creating Streams */
    pub fn CFStreamCreatePairWithPeerSocketSignature(
        alloc: CFAllocatorRef,
        signature: *const CFSocketSignature,
        readStream: *mut CFReadStreamRef,
        writeStream: *mut CFWriteStreamRef,
    ); // deprecated
    pub fn CFStreamCreatePairWithSocketToHost(
        alloc: CFAllocatorRef,
        host: CFStringRef,
        port: UInt32,
        readStream: *mut CFReadStreamRef,
        writeStream: *mut CFWriteStreamRef,
    ); // deprecated
    pub fn CFStreamCreatePairWithSocket(
        alloc: CFAllocatorRef,
        sock: CFSocketNativeHandle,
        readStream: *mut CFReadStreamRef,
        writeStream: *mut CFWriteStreamRef,
    ); // deprecated
    pub fn CFStreamCreateBoundPair(
        alloc: CFAllocatorRef,
        readStream: *mut CFReadStreamRef,
        writeStream: *mut CFWriteStreamRef,
        transferBufferSize: CFIndex,
    );

    //pub fn CFReadStreamSetDispatchQueue(stream: CFReadStreamRef, q: dispatch_queue_t); // macos(10.9)+
    //pub fn CFWriteStreamSetDispatchQueue(stream: CFWriteStreamRef, q: dispatch_queue_t); // macos(10.9)+
    //pub fn CFReadStreamCopyDispatchQueue(stream: CFReadStreamRef) -> dispatch_queue_t; // macos(10.9)+
    //pub fn CFWriteStreamCopyDispatchQueue(stream: CFReadStreamRef) -> dispatch_queue_t; // macos(10.9)+

    /* CFReadStream */
    /* Creating a Read Stream */
    pub fn CFReadStreamCreateWithBytesNoCopy(
        alloc: CFAllocatorRef,
        bytes: *const UInt8,
        length: CFIndex,
        bytesDeallocator: CFAllocatorRef,
    ) -> CFReadStreamRef;
    pub fn CFReadStreamCreateWithFile(alloc: CFAllocatorRef, fileURL: CFURLRef) -> CFReadStreamRef;

    /* Opening and Closing a Read Stream */
    pub fn CFReadStreamClose(stream: CFReadStreamRef);
    pub fn CFReadStreamOpen(stream: CFReadStreamRef) -> Boolean;

    /* Reading from a Stream */
    pub fn CFReadStreamRead(
        stream: CFReadStreamRef,
        buffer: *mut UInt8,
        bufferLength: CFIndex,
    ) -> CFIndex;

    /* Scheduling a Read Stream */
    pub fn CFReadStreamScheduleWithRunLoop(
        stream: CFReadStreamRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
    pub fn CFReadStreamUnscheduleFromRunLoop(
        stream: CFReadStreamRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );

    /* Examining Stream Properties */
    pub fn CFReadStreamCopyProperty(
        stream: CFReadStreamRef,
        propertyName: CFStreamPropertyKey,
    ) -> CFTypeRef;
    pub fn CFReadStreamGetBuffer(
        stream: CFReadStreamRef,
        maxBytesToRead: CFIndex,
        numBytesRead: *mut CFIndex,
    ) -> *const UInt8;
    pub fn CFReadStreamCopyError(stream: CFReadStreamRef) -> CFErrorRef;
    pub fn CFReadStreamGetError(stream: CFReadStreamRef) -> CFStreamError; // deprecated
    pub fn CFReadStreamGetStatus(stream: CFReadStreamRef) -> CFStreamStatus;
    pub fn CFReadStreamHasBytesAvailable(stream: CFReadStreamRef) -> Boolean;

    /* Setting Stream Properties */
    pub fn CFReadStreamSetClient(
        stream: CFReadStreamRef,
        streamEvents: CFOptionFlags,
        clientCB: CFReadStreamClientCallBack,
        clientContext: *mut CFStreamClientContext,
    ) -> Boolean;
    pub fn CFReadStreamSetProperty(
        stream: CFReadStreamRef,
        propertyName: CFStreamPropertyKey,
        propertyValue: CFTypeRef,
    ) -> Boolean;

    /* Getting the CFReadStream Type ID */
    pub fn CFReadStreamGetTypeID() -> CFTypeID;

    /* CFWriteStream */
    /* Creating a Write Stream */
    pub fn CFWriteStreamCreateWithAllocatedBuffers(
        alloc: CFAllocatorRef,
        bufferAllocator: CFAllocatorRef,
    ) -> CFWriteStreamRef;
    pub fn CFWriteStreamCreateWithBuffer(
        alloc: CFAllocatorRef,
        buffer: *mut UInt8,
        bufferCapacity: CFIndex,
    ) -> CFWriteStreamRef;
    pub fn CFWriteStreamCreateWithFile(
        alloc: CFAllocatorRef,
        fileURL: CFURLRef,
    ) -> CFWriteStreamRef;

    /* Opening and Closing a Stream */
    pub fn CFWriteStreamClose(stream: CFWriteStreamRef);
    pub fn CFWriteStreamOpen(stream: CFWriteStreamRef) -> Boolean;

    /* Writing to a Stream */
    pub fn CFWriteStreamWrite(
        stream: CFWriteStreamRef,
        buffer: *const UInt8,
        bufferLength: CFIndex,
    ) -> CFIndex;

    /* Scheduling a Write Stream */
    pub fn CFWriteStreamScheduleWithRunLoop(
        stream: CFWriteStreamRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );
    pub fn CFWriteStreamUnscheduleFromRunLoop(
        stream: CFWriteStreamRef,
        runLoop: CFRunLoopRef,
        runLoopMode: CFStringRef,
    );

    /* Examining Stream Properties */
    pub fn CFWriteStreamCanAcceptBytes(stream: CFWriteStreamRef) -> Boolean;
    pub fn CFWriteStreamCopyProperty(
        stream: CFWriteStreamRef,
        propertyName: CFStreamPropertyKey,
    ) -> CFTypeRef;
    pub fn CFWriteStreamCopyError(stream: CFWriteStreamRef) -> CFErrorRef;
    pub fn CFWriteStreamGetError(stream: CFWriteStreamRef) -> CFStreamError; // deprecated
    pub fn CFWriteStreamGetStatus(stream: CFWriteStreamRef) -> CFStreamStatus;

    /* Setting Stream Properties */
    pub fn CFWriteStreamSetClient(
        stream: CFWriteStreamRef,
        streamEvents: CFOptionFlags,
        clientCB: CFWriteStreamClientCallBack,
        clientContext: *mut CFStreamClientContext,
    ) -> Boolean;
    pub fn CFWriteStreamSetProperty(
        stream: CFWriteStreamRef,
        propertyName: CFStreamPropertyKey,
        propertyValue: CFTypeRef,
    ) -> Boolean;

    /* Getting the CFWriteStream Type ID */
    pub fn CFWriteStreamGetTypeID() -> CFTypeID;
}
