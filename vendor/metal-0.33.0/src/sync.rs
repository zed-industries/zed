// Copyright 2016 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::ffi::CStr;

use super::*;
use block::{Block, RcBlock};

#[cfg(feature = "dispatch")]
use dispatch;

/// See <https://developer.apple.com/documentation/metal/mtlsharedeventnotificationblock>
type MTLSharedEventNotificationBlock<'a> = RcBlock<(&'a SharedEventRef, u64), ()>;

/// See <https://developer.apple.com/documentation/metal/mtlevent>
pub enum MTLEvent {}

foreign_obj_type! {
    type CType = MTLEvent;
    pub struct Event;
}

impl EventRef {
    pub fn device(&self) -> &DeviceRef {
        unsafe { msg_send![self, device] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlsharedevent>
pub enum MTLSharedEvent {}

foreign_obj_type! {
    type CType = MTLSharedEvent;
    pub struct SharedEvent;
    type ParentType = Event;
}

impl SharedEventRef {
    pub fn signaled_value(&self) -> u64 {
        unsafe { msg_send![self, signaledValue] }
    }

    pub fn set_signaled_value(&self, new_value: u64) {
        unsafe { msg_send![self, setSignaledValue: new_value] }
    }

    /// Schedules a notification handler to be called after the shareable event’s signal value
    /// equals or exceeds a given value.
    pub fn notify(
        &self,
        listener: &SharedEventListenerRef,
        value: u64,
        block: MTLSharedEventNotificationBlock,
    ) {
        unsafe {
            // If the block doesn't have a signature, this segfaults.
            // Taken from https://github.com/servo/pathfinder/blob/e858c8dc1d8ff02a5b603e21e09a64d6b3e11327/metal/src/lib.rs#L2327
            let block = mem::transmute::<
                MTLSharedEventNotificationBlock,
                *mut BlockBase<(&SharedEventRef, u64), ()>,
            >(block);
            (*block).flags |= BLOCK_HAS_SIGNATURE | BLOCK_HAS_COPY_DISPOSE;
            (*block).extra = &raw const BLOCK_EXTRA;
            let () = msg_send![self, notifyListener:listener atValue:value block:block];
        }

        extern "C" fn dtor(_: *mut BlockBase<(&SharedEventRef, u64), ()>) {}

        const SIGNATURE: &CStr = c"v16@?0Q8";
        const SIGNATURE_PTR: *const i8 = SIGNATURE.as_ptr().cast();
        static mut BLOCK_EXTRA: BlockExtra<(&SharedEventRef, u64), ()> = BlockExtra {
            unknown0: std::ptr::null_mut(),
            unknown1: std::ptr::null_mut(),
            unknown2: std::ptr::null_mut(),
            dtor,
            signature: &SIGNATURE_PTR,
        };
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlsharedeventlistener>
pub enum MTLSharedEventListener {}

foreign_obj_type! {
    type CType = MTLSharedEventListener;
    pub struct SharedEventListener;
}

impl SharedEventListener {
    pub unsafe fn from_queue_handle(queue: dispatch_queue_t) -> Self {
        let listener: SharedEventListener = msg_send![class!(MTLSharedEventListener), alloc];
        let ptr: *mut Object = msg_send![listener.as_ref(), initWithDispatchQueue: queue];
        if ptr.is_null() {
            panic!("[MTLSharedEventListener alloc] initWithDispatchQueue failed");
        }
        listener
    }

    #[cfg(feature = "dispatch")]
    pub fn from_queue(queue: &dispatch::Queue) -> Self {
        unsafe {
            let raw_queue = std::mem::transmute::<&dispatch::Queue, *const dispatch_queue_t>(queue);
            Self::from_queue_handle(*raw_queue)
        }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlfence>
pub enum MTLFence {}

foreign_obj_type! {
    type CType = MTLFence;
    pub struct Fence;
}

impl FenceRef {
    pub fn device(&self) -> &DeviceRef {
        unsafe { msg_send![self, device] }
    }

    pub fn label(&self) -> &str {
        unsafe {
            let label = msg_send![self, label];
            crate::nsstring_as_str(label)
        }
    }

    pub fn set_label(&self, label: &str) {
        unsafe {
            let nslabel = crate::nsstring_from_str(label);
            let () = msg_send![self, setLabel: nslabel];
        }
    }
}

bitflags::bitflags! {
    /// The render stages at which a synchronization command is triggered.
    ///
    /// Render stages provide finer control for specifying when synchronization must occur,
    /// allowing for vertex and fragment processing to overlap in execution.
    ///
    /// See <https://developer.apple.com/documentation/metal/mtlrenderstages>
    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct MTLRenderStages: NSUInteger {
        /// The vertex rendering stage.
        const Vertex = 1 << 0;
        /// The fragment rendering stage.
        const Fragment = 1 << 1;
        /// The tile rendering stage.
        const Tile = 1 << 2;
    }
}

const BLOCK_HAS_COPY_DISPOSE: i32 = 0x02000000;
const BLOCK_HAS_SIGNATURE: i32 = 0x40000000;

#[repr(C)]
struct BlockBase<A, R> {
    isa: *const std::ffi::c_void,                             // 0x00
    flags: i32,                                               // 0x08
    _reserved: i32,                                           // 0x0c
    invoke: unsafe extern "C" fn(*mut Block<A, R>, ...) -> R, // 0x10
    extra: *const BlockExtra<A, R>,                           // 0x18
}

type BlockExtraDtor<A, R> = extern "C" fn(*mut BlockBase<A, R>);

#[repr(C)]
struct BlockExtra<A, R> {
    unknown0: *mut i32,          // 0x00
    unknown1: *mut i32,          // 0x08
    unknown2: *mut i32,          // 0x10
    dtor: BlockExtraDtor<A, R>,  // 0x18
    signature: *const *const i8, // 0x20
}
