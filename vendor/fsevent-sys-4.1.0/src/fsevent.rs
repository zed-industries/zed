#![allow(non_upper_case_globals, non_camel_case_types)]

use crate::core_foundation::{
    Boolean, CFAbsoluteTime, CFAllocatorCopyDescriptionCallBack, CFAllocatorRef,
    CFAllocatorReleaseCallBack, CFAllocatorRetainCallBack, CFArrayRef, CFIndex, CFRunLoopRef,
    CFStringRef, CFTimeInterval,
};
use libc::dev_t;
use std::os::raw::{c_uint, c_void};

pub type FSEventStreamRef = *mut c_void;
pub type ConstFSEventStreamRef = *const c_void;

pub type FSEventStreamCallback = extern "C" fn(
    FSEventStreamRef,               // ConstFSEventStreamRef streamRef
    *mut c_void,                    // void *clientCallBackInfo
    usize,                          // size_t numEvents
    *mut c_void,                    // void *eventPaths
    *const FSEventStreamEventFlags, // const FSEventStreamEventFlags eventFlags[]
    *const FSEventStreamEventId,    // const FSEventStreamEventId eventIds[]
);

pub type FSEventStreamEventId = u64;

pub type FSEventStreamCreateFlags = c_uint;

pub type FSEventStreamEventFlags = c_uint;

pub const kFSEventStreamEventIdSinceNow: FSEventStreamEventId = 0xFFFFFFFFFFFFFFFF;

pub const kFSEventStreamCreateFlagNone: FSEventStreamCreateFlags = 0x00000000;
pub const kFSEventStreamCreateFlagUseCFTypes: FSEventStreamCreateFlags = 0x00000001;
pub const kFSEventStreamCreateFlagNoDefer: FSEventStreamCreateFlags = 0x00000002;
pub const kFSEventStreamCreateFlagWatchRoot: FSEventStreamCreateFlags = 0x00000004;
pub const kFSEventStreamCreateFlagIgnoreSelf: FSEventStreamCreateFlags = 0x00000008;
pub const kFSEventStreamCreateFlagFileEvents: FSEventStreamCreateFlags = 0x00000010;
pub const kFSEventStreamCreateFlagMarkSelf: FSEventStreamCreateFlags = 0x00000020;
pub const kFSEventStreamCreateFlagUseExtendedData: FSEventStreamCreateFlags = 0x00000040;
pub const kFSEventStreamCreateFlagFullHistory: FSEventStreamCreateFlags = 0x00000080;

pub const kFSEventStreamEventFlagNone: FSEventStreamEventFlags = 0x00000000;
pub const kFSEventStreamEventFlagMustScanSubDirs: FSEventStreamEventFlags = 0x00000001;
pub const kFSEventStreamEventFlagUserDropped: FSEventStreamEventFlags = 0x00000002;
pub const kFSEventStreamEventFlagKernelDropped: FSEventStreamEventFlags = 0x00000004;
pub const kFSEventStreamEventFlagEventIdsWrapped: FSEventStreamEventFlags = 0x00000008;
pub const kFSEventStreamEventFlagHistoryDone: FSEventStreamEventFlags = 0x00000010;
pub const kFSEventStreamEventFlagRootChanged: FSEventStreamEventFlags = 0x00000020;
pub const kFSEventStreamEventFlagMount: FSEventStreamEventFlags = 0x00000040;
pub const kFSEventStreamEventFlagUnmount: FSEventStreamEventFlags = 0x00000080;
pub const kFSEventStreamEventFlagItemCreated: FSEventStreamEventFlags = 0x00000100;
pub const kFSEventStreamEventFlagItemRemoved: FSEventStreamEventFlags = 0x00000200;
pub const kFSEventStreamEventFlagItemInodeMetaMod: FSEventStreamEventFlags = 0x00000400;
pub const kFSEventStreamEventFlagItemRenamed: FSEventStreamEventFlags = 0x00000800;
pub const kFSEventStreamEventFlagItemModified: FSEventStreamEventFlags = 0x00001000;
pub const kFSEventStreamEventFlagItemFinderInfoMod: FSEventStreamEventFlags = 0x00002000;
pub const kFSEventStreamEventFlagItemChangeOwner: FSEventStreamEventFlags = 0x00004000;
pub const kFSEventStreamEventFlagItemXattrMod: FSEventStreamEventFlags = 0x00008000;
pub const kFSEventStreamEventFlagItemIsFile: FSEventStreamEventFlags = 0x00010000;
pub const kFSEventStreamEventFlagItemIsDir: FSEventStreamEventFlags = 0x00020000;
pub const kFSEventStreamEventFlagItemIsSymlink: FSEventStreamEventFlags = 0x00040000;
pub const kFSEventStreamEventFlagOwnEvent: FSEventStreamEventFlags = 0x00080000;
pub const kFSEventStreamEventFlagItemIsHardlink: FSEventStreamEventFlags = 0x00100000;
pub const kFSEventStreamEventFlagItemIsLastHardlink: FSEventStreamEventFlags = 0x00200000;
pub const kFSEventStreamEventFlagItemCloned: FSEventStreamEventFlags = 0x00400000;

#[repr(C)]
pub struct FSEventStreamContext {
    pub version: CFIndex,
    pub info: *mut c_void,
    pub retain: Option<CFAllocatorRetainCallBack>,
    pub release: Option<CFAllocatorReleaseCallBack>,
    pub copy_description: Option<CFAllocatorCopyDescriptionCallBack>,
}

// https://developer.apple.com/documentation/coreservices/file_system_events
#[link(name = "CoreServices", kind = "framework")]
extern "C" {
    pub fn FSEventStreamCopyDescription(stream_ref: ConstFSEventStreamRef) -> CFStringRef;
    pub fn FSEventStreamCopyPathsBeingWatched(streamRef: ConstFSEventStreamRef) -> CFArrayRef;
    pub fn FSEventStreamCreate(
        allocator: CFAllocatorRef,
        callback: FSEventStreamCallback,
        context: *const FSEventStreamContext,
        pathsToWatch: CFArrayRef,
        sinceWhen: FSEventStreamEventId,
        latency: CFTimeInterval,
        flags: FSEventStreamCreateFlags,
    ) -> FSEventStreamRef;
    pub fn FSEventStreamCreateRelativeToDevice(
        allocator: CFAllocatorRef,
        callback: FSEventStreamCallback,
        context: *const FSEventStreamContext,
        deviceToWatch: dev_t,
        pathsToWatchRelativeToDevice: CFArrayRef,
        sinceWhen: FSEventStreamEventId,
        latency: CFTimeInterval,
        flags: FSEventStreamCreateFlags,
    ) -> FSEventStreamRef;
    pub fn FSEventStreamFlushAsync(stream_ref: FSEventStreamRef) -> FSEventStreamEventId;
    pub fn FSEventStreamFlushSync(streamRef: FSEventStreamRef);
    pub fn FSEventStreamGetDeviceBeingWatched(stream_ref: ConstFSEventStreamRef) -> dev_t;
    pub fn FSEventStreamGetLatestEventId(stream_ref: ConstFSEventStreamRef)
        -> FSEventStreamEventId;
    pub fn FSEventStreamInvalidate(stream_ref: FSEventStreamRef);
    pub fn FSEventStreamRelease(stream_ref: FSEventStreamRef);
    pub fn FSEventStreamRetain(stream_ref: FSEventStreamRef);
    pub fn FSEventStreamScheduleWithRunLoop(
        stream_ref: FSEventStreamRef,
        run_loop: CFRunLoopRef,
        run_loop_mode: CFStringRef,
    );
    // pub fn FSEventStreamSetDispatchQueue(streamRef: FSEventStreamRef, q: DispatchQueue);
    pub fn FSEventStreamSetExclusionPaths(
        stream_ref: FSEventStreamRef,
        paths_to_exclude: CFArrayRef,
    ) -> Boolean;
    pub fn FSEventStreamShow(stream_ref: FSEventStreamRef);
    pub fn FSEventStreamStart(stream_ref: FSEventStreamRef) -> Boolean;
    pub fn FSEventStreamStop(stream_ref: FSEventStreamRef);
    pub fn FSEventStreamUnscheduleFromRunLoop(
        stream_ref: FSEventStreamRef,
        run_loop: CFRunLoopRef,
        run_loop_mode: CFStringRef,
    );
    // pub fn FSEventsCopyUUIDForDevice(dev: dev_t) -> CFUUID;
    pub fn FSEventsGetCurrentEventId() -> FSEventStreamEventId;
    pub fn FSEventsGetLastEventIdForDeviceBeforeTime(
        dev: dev_t,
        time: CFAbsoluteTime,
    ) -> FSEventStreamEventId;
    pub fn FSEventsPurgeEventsForDeviceUpToEventId(
        dev: dev_t,
        eventId: FSEventStreamEventId,
    ) -> Boolean;
}
