// Copyright 2013-2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::c_void;

use crate::array::CFArrayRef;
use crate::base::{
    mach_port_t, Boolean, CFAllocatorRef, CFHashCode, CFIndex, CFOptionFlags, CFTypeID,
};
use crate::date::{CFAbsoluteTime, CFTimeInterval};
use crate::string::CFStringRef;

#[repr(C)]
pub struct __CFRunLoop(c_void);

pub type CFRunLoopRef = *mut __CFRunLoop;

#[repr(C)]
pub struct __CFRunLoopSource(c_void);

pub type CFRunLoopSourceRef = *mut __CFRunLoopSource;

#[repr(C)]
pub struct __CFRunLoopObserver(c_void);

pub type CFRunLoopObserverRef = *mut __CFRunLoopObserver;

// Reasons for CFRunLoopRunInMode() to Return
pub const kCFRunLoopRunFinished: i32 = 1;
pub const kCFRunLoopRunStopped: i32 = 2;
pub const kCFRunLoopRunTimedOut: i32 = 3;
pub const kCFRunLoopRunHandledSource: i32 = 4;

// Run Loop Observer Activities
//typedef CF_OPTIONS(CFOptionFlags, CFRunLoopActivity) {
pub type CFRunLoopActivity = CFOptionFlags;
pub const kCFRunLoopEntry: CFOptionFlags = 1 << 0;
pub const kCFRunLoopBeforeTimers: CFOptionFlags = 1 << 1;
pub const kCFRunLoopBeforeSources: CFOptionFlags = 1 << 2;
pub const kCFRunLoopBeforeWaiting: CFOptionFlags = 1 << 5;
pub const kCFRunLoopAfterWaiting: CFOptionFlags = 1 << 6;
pub const kCFRunLoopExit: CFOptionFlags = 1 << 7;
pub const kCFRunLoopAllActivities: CFOptionFlags = 0x0FFFFFFF;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CFRunLoopSourceContext {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: Option<extern "C" fn(info: *const c_void) -> *const c_void>,
    pub release: Option<extern "C" fn(info: *const c_void)>,
    pub copyDescription: Option<extern "C" fn(info: *const c_void) -> CFStringRef>,
    pub equal: Option<extern "C" fn(info1: *const c_void, info2: *const c_void) -> Boolean>,
    pub hash: Option<extern "C" fn(info: *const c_void) -> CFHashCode>,
    pub schedule: Option<extern "C" fn(info: *const c_void, rl: CFRunLoopRef, mode: CFStringRef)>,
    pub cancel: Option<extern "C" fn(info: *const c_void, rl: CFRunLoopRef, mode: CFStringRef)>,
    pub perform: extern "C" fn(info: *const c_void),
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CFRunLoopSourceContext1 {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: Option<extern "C" fn(info: *const c_void) -> *const c_void>,
    pub release: Option<extern "C" fn(info: *const c_void)>,
    pub copyDescription: Option<extern "C" fn(info: *const c_void) -> CFStringRef>,
    pub equal: Option<extern "C" fn(info1: *const c_void, info2: *const c_void) -> Boolean>,
    pub hash: Option<extern "C" fn(info: *const c_void) -> CFHashCode>,

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    pub getPort: extern "C" fn(info: *mut c_void) -> mach_port_t,
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    pub perform: extern "C" fn(
        msg: *mut c_void,
        size: CFIndex,
        allocator: CFAllocatorRef,
        info: *mut c_void,
    ) -> *mut c_void,

    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
    pub getPort: extern "C" fn(info: *mut c_void) -> *mut c_void,
    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
    pub perform: extern "C" fn(info: *mut c_void) -> *mut c_void,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CFRunLoopObserverContext {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: Option<extern "C" fn(info: *const c_void) -> *const c_void>,
    pub release: Option<extern "C" fn(info: *const c_void)>,
    pub copyDescription: Option<extern "C" fn(info: *const c_void) -> CFStringRef>,
}

pub type CFRunLoopObserverCallBack =
    extern "C" fn(observer: CFRunLoopObserverRef, activity: CFRunLoopActivity, info: *mut c_void);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CFRunLoopTimerContext {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: Option<extern "C" fn(info: *const c_void) -> *const c_void>,
    pub release: Option<extern "C" fn(info: *const c_void)>,
    pub copyDescription: Option<extern "C" fn(info: *const c_void) -> CFStringRef>,
}

pub type CFRunLoopTimerCallBack = extern "C" fn(timer: CFRunLoopTimerRef, info: *mut c_void);

#[repr(C)]
pub struct __CFRunLoopTimer(c_void);

pub type CFRunLoopTimerRef = *mut __CFRunLoopTimer;

extern "C" {
    /*
     * CFRunLoop.h
     */

    pub static kCFRunLoopDefaultMode: CFStringRef;
    pub static kCFRunLoopCommonModes: CFStringRef;

    /* CFRunLoop */
    /* Getting a Run Loop */
    pub fn CFRunLoopGetCurrent() -> CFRunLoopRef;
    pub fn CFRunLoopGetMain() -> CFRunLoopRef;

    /* Starting and Stopping a Run Loop */
    pub fn CFRunLoopRun();
    pub fn CFRunLoopRunInMode(
        mode: CFStringRef,
        seconds: CFTimeInterval,
        returnAfterSourceHandled: Boolean,
    ) -> i32;
    pub fn CFRunLoopWakeUp(rl: CFRunLoopRef);
    pub fn CFRunLoopStop(rl: CFRunLoopRef);
    pub fn CFRunLoopIsWaiting(rl: CFRunLoopRef) -> Boolean;

    /* Managing Sources */
    pub fn CFRunLoopAddSource(rl: CFRunLoopRef, source: CFRunLoopSourceRef, mode: CFStringRef);
    pub fn CFRunLoopContainsSource(
        rl: CFRunLoopRef,
        source: CFRunLoopSourceRef,
        mode: CFStringRef,
    ) -> Boolean;
    pub fn CFRunLoopRemoveSource(rl: CFRunLoopRef, source: CFRunLoopSourceRef, mode: CFStringRef);

    /* Managing Observers */
    pub fn CFRunLoopAddObserver(
        rl: CFRunLoopRef,
        observer: CFRunLoopObserverRef,
        mode: CFStringRef,
    );
    pub fn CFRunLoopContainsObserver(
        rl: CFRunLoopRef,
        observer: CFRunLoopObserverRef,
        mode: CFStringRef,
    ) -> Boolean;
    pub fn CFRunLoopRemoveObserver(
        rl: CFRunLoopRef,
        observer: CFRunLoopObserverRef,
        mode: CFStringRef,
    );

    /* Managing Run Loop Modes */
    pub fn CFRunLoopAddCommonMode(rl: CFRunLoopRef, mode: CFStringRef);
    pub fn CFRunLoopCopyAllModes(rl: CFRunLoopRef) -> CFArrayRef;
    pub fn CFRunLoopCopyCurrentMode(rl: CFRunLoopRef) -> CFStringRef;

    /* Managing Timers */
    pub fn CFRunLoopAddTimer(rl: CFRunLoopRef, timer: CFRunLoopTimerRef, mode: CFStringRef);
    pub fn CFRunLoopGetNextTimerFireDate(rl: CFRunLoopRef, mode: CFStringRef) -> CFAbsoluteTime;
    pub fn CFRunLoopRemoveTimer(rl: CFRunLoopRef, timer: CFRunLoopTimerRef, mode: CFStringRef);
    pub fn CFRunLoopContainsTimer(
        rl: CFRunLoopRef,
        timer: CFRunLoopTimerRef,
        mode: CFStringRef,
    ) -> Boolean;

    /* Scheduling Blocks */
    // fn CFRunLoopPerformBlock(rl: CFRunLoopRef, mode: CFTypeRef, block: void (^)(void));

    /* Getting the CFRunLoop Type ID */
    pub fn CFRunLoopGetTypeID() -> CFTypeID;

    /* CFRunLoopSource */
    /* CFRunLoopSource Miscellaneous Functions */
    pub fn CFRunLoopSourceCreate(
        allocator: CFAllocatorRef,
        order: CFIndex,
        context: *mut CFRunLoopSourceContext,
    ) -> CFRunLoopSourceRef;
    pub fn CFRunLoopSourceGetContext(
        source: CFRunLoopSourceRef,
        context: *mut CFRunLoopSourceContext,
    );
    pub fn CFRunLoopSourceGetOrder(source: CFRunLoopSourceRef) -> CFIndex;
    pub fn CFRunLoopSourceGetTypeID() -> CFTypeID;
    pub fn CFRunLoopSourceInvalidate(source: CFRunLoopSourceRef);
    pub fn CFRunLoopSourceIsValid(source: CFRunLoopSourceRef) -> Boolean;
    pub fn CFRunLoopSourceSignal(source: CFRunLoopSourceRef);

    /* CFRunLoopObserver */
    /* CFRunLoopObserver Miscellaneous Functions */
    // fn CFRunLoopObserverCreateWithHandler(allocator: CFAllocatorRef, activities: CFOptionFlags, repeats: Boolean, order: CFIndex, block: void (^) (CFRunLoopObserverRef observer, CFRunLoopActivity activity)) -> CFRunLoopObserverRef;
    pub fn CFRunLoopObserverCreate(
        allocator: CFAllocatorRef,
        activities: CFOptionFlags,
        repeats: Boolean,
        order: CFIndex,
        callout: CFRunLoopObserverCallBack,
        context: *mut CFRunLoopObserverContext,
    ) -> CFRunLoopObserverRef;
    pub fn CFRunLoopObserverDoesRepeat(observer: CFRunLoopObserverRef) -> Boolean;
    pub fn CFRunLoopObserverGetActivities(observer: CFRunLoopObserverRef) -> CFOptionFlags;
    pub fn CFRunLoopObserverGetContext(
        observer: CFRunLoopObserverRef,
        context: *mut CFRunLoopObserverContext,
    );
    pub fn CFRunLoopObserverGetOrder(observer: CFRunLoopObserverRef) -> CFIndex;
    pub fn CFRunLoopObserverGetTypeID() -> CFTypeID;
    pub fn CFRunLoopObserverInvalidate(observer: CFRunLoopObserverRef);
    pub fn CFRunLoopObserverIsValid(observer: CFRunLoopObserverRef) -> Boolean;

    /* CFRunLoopTimer */
    /* CFRunLoopTimer Miscellaneous Functions */
    // fn CFRunLoopTimerCreateWithHandler(allocator: CFAllocatorRef, fireDate: CFAbsoluteTime, interval: CFTimeInterval, flags: CFOptionFlags, order: CFIndex, block: void (^) (CFRunLoopTimerRef timer)) -> CFRunLoopTimerRef;
    pub fn CFRunLoopTimerCreate(
        allocator: CFAllocatorRef,
        fireDate: CFAbsoluteTime,
        interval: CFTimeInterval,
        flags: CFOptionFlags,
        order: CFIndex,
        callout: CFRunLoopTimerCallBack,
        context: *mut CFRunLoopTimerContext,
    ) -> CFRunLoopTimerRef;
    pub fn CFRunLoopTimerDoesRepeat(timer: CFRunLoopTimerRef) -> Boolean;
    pub fn CFRunLoopTimerGetContext(timer: CFRunLoopTimerRef, context: *mut CFRunLoopTimerContext);
    pub fn CFRunLoopTimerGetInterval(timer: CFRunLoopTimerRef) -> CFTimeInterval;
    pub fn CFRunLoopTimerGetNextFireDate(timer: CFRunLoopTimerRef) -> CFAbsoluteTime;
    pub fn CFRunLoopTimerGetOrder(timer: CFRunLoopTimerRef) -> CFIndex;
    pub fn CFRunLoopTimerGetTypeID() -> CFTypeID;
    pub fn CFRunLoopTimerInvalidate(timer: CFRunLoopTimerRef);
    pub fn CFRunLoopTimerIsValid(timer: CFRunLoopTimerRef) -> Boolean;
    pub fn CFRunLoopTimerSetNextFireDate(timer: CFRunLoopTimerRef, fireDate: CFAbsoluteTime);
    pub fn CFRunLoopTimerGetTolerance(timer: CFRunLoopTimerRef) -> CFTimeInterval; //macos(10.9)+
    pub fn CFRunLoopTimerSetTolerance(timer: CFRunLoopTimerRef, tolerance: CFTimeInterval); //macos(10.9)+
}
