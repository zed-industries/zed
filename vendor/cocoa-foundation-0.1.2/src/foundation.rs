// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_upper_case_globals)]

use base::{id, nil, BOOL, NO, SEL};
use block::Block;
use libc;
use std::os::raw::c_void;
use std::ptr;

#[cfg(target_pointer_width = "32")]
pub type NSInteger = libc::c_int;
#[cfg(target_pointer_width = "32")]
pub type NSUInteger = libc::c_uint;

#[cfg(target_pointer_width = "64")]
pub type NSInteger = libc::c_long;
#[cfg(target_pointer_width = "64")]
pub type NSUInteger = libc::c_ulong;

pub const NSIntegerMax: NSInteger = NSInteger::max_value();
pub const NSNotFound: NSInteger = NSIntegerMax;

const UTF8_ENCODING: usize = 4;

#[cfg(target_os = "macos")]
mod macos {
    use base::id;
    use core_graphics_types::base::CGFloat;
    use core_graphics_types::geometry::CGRect;
    use objc;
    use std::mem;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct NSPoint {
        pub x: CGFloat,
        pub y: CGFloat,
    }

    impl NSPoint {
        #[inline]
        pub fn new(x: CGFloat, y: CGFloat) -> NSPoint {
            NSPoint { x, y }
        }
    }

    unsafe impl objc::Encode for NSPoint {
        fn encode() -> objc::Encoding {
            let encoding = format!(
                "{{CGPoint={}{}}}",
                CGFloat::encode().as_str(),
                CGFloat::encode().as_str()
            );
            unsafe { objc::Encoding::from_str(&encoding) }
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct NSSize {
        pub width: CGFloat,
        pub height: CGFloat,
    }

    impl NSSize {
        #[inline]
        pub fn new(width: CGFloat, height: CGFloat) -> NSSize {
            NSSize { width, height }
        }
    }

    unsafe impl objc::Encode for NSSize {
        fn encode() -> objc::Encoding {
            let encoding = format!(
                "{{CGSize={}{}}}",
                CGFloat::encode().as_str(),
                CGFloat::encode().as_str()
            );
            unsafe { objc::Encoding::from_str(&encoding) }
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct NSRect {
        pub origin: NSPoint,
        pub size: NSSize,
    }

    impl NSRect {
        #[inline]
        pub fn new(origin: NSPoint, size: NSSize) -> NSRect {
            NSRect { origin, size }
        }

        #[inline]
        pub fn as_CGRect(&self) -> &CGRect {
            unsafe { mem::transmute::<&NSRect, &CGRect>(self) }
        }

        #[inline]
        pub fn inset(&self, x: CGFloat, y: CGFloat) -> NSRect {
            unsafe { NSInsetRect(*self, x, y) }
        }
    }

    unsafe impl objc::Encode for NSRect {
        fn encode() -> objc::Encoding {
            let encoding = format!(
                "{{CGRect={}{}}}",
                NSPoint::encode().as_str(),
                NSSize::encode().as_str()
            );
            unsafe { objc::Encoding::from_str(&encoding) }
        }
    }

    // Same as CGRectEdge
    #[repr(u32)]
    pub enum NSRectEdge {
        NSRectMinXEdge,
        NSRectMinYEdge,
        NSRectMaxXEdge,
        NSRectMaxYEdge,
    }

    #[link(name = "Foundation", kind = "framework")]
    extern "C" {
        fn NSInsetRect(rect: NSRect, x: CGFloat, y: CGFloat) -> NSRect;
    }

    pub trait NSValue: Sized {
        unsafe fn valueWithPoint(_: Self, point: NSPoint) -> id {
            msg_send![class!(NSValue), valueWithPoint: point]
        }

        unsafe fn valueWithSize(_: Self, size: NSSize) -> id {
            msg_send![class!(NSValue), valueWithSize: size]
        }
    }

    impl NSValue for id {}
}

#[cfg(target_os = "macos")]
pub use self::macos::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NSRange {
    pub location: NSUInteger,
    pub length: NSUInteger,
}

impl NSRange {
    #[inline]
    pub fn new(location: NSUInteger, length: NSUInteger) -> NSRange {
        NSRange { location, length }
    }
}

#[link(name = "Foundation", kind = "framework")]
extern "C" {
    pub static NSDefaultRunLoopMode: id;
}

pub trait NSAutoreleasePool: Sized {
    unsafe fn new(_: Self) -> id {
        msg_send![class!(NSAutoreleasePool), new]
    }

    unsafe fn autorelease(self) -> Self;
    unsafe fn drain(self);
}

impl NSAutoreleasePool for id {
    unsafe fn autorelease(self) -> id {
        msg_send![self, autorelease]
    }

    unsafe fn drain(self) {
        msg_send![self, drain]
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NSOperatingSystemVersion {
    pub majorVersion: NSUInteger,
    pub minorVersion: NSUInteger,
    pub patchVersion: NSUInteger,
}

impl NSOperatingSystemVersion {
    #[inline]
    pub fn new(
        majorVersion: NSUInteger,
        minorVersion: NSUInteger,
        patchVersion: NSUInteger,
    ) -> NSOperatingSystemVersion {
        NSOperatingSystemVersion {
            majorVersion,
            minorVersion,
            patchVersion,
        }
    }
}

pub trait NSProcessInfo: Sized {
    unsafe fn processInfo(_: Self) -> id {
        msg_send![class!(NSProcessInfo), processInfo]
    }

    unsafe fn systemUptime(self) -> NSTimeInterval;
    unsafe fn processName(self) -> id;
    unsafe fn operatingSystemVersion(self) -> NSOperatingSystemVersion;
    unsafe fn isOperatingSystemAtLeastVersion(self, version: NSOperatingSystemVersion) -> bool;
}

impl NSProcessInfo for id {
    unsafe fn processName(self) -> id {
        msg_send![self, processName]
    }

    unsafe fn systemUptime(self) -> NSTimeInterval {
        msg_send![self, systemUptime]
    }

    unsafe fn operatingSystemVersion(self) -> NSOperatingSystemVersion {
        msg_send![self, operatingSystemVersion]
    }

    unsafe fn isOperatingSystemAtLeastVersion(self, version: NSOperatingSystemVersion) -> bool {
        msg_send![self, isOperatingSystemAtLeastVersion: version]
    }
}

pub type NSTimeInterval = libc::c_double;

pub trait NSArray: Sized {
    unsafe fn array(_: Self) -> id {
        msg_send![class!(NSArray), array]
    }

    unsafe fn arrayWithObjects(_: Self, objects: &[id]) -> id {
        msg_send![class!(NSArray), arrayWithObjects:objects.as_ptr()
                                    count:objects.len()]
    }

    unsafe fn arrayWithObject(_: Self, object: id) -> id {
        msg_send![class!(NSArray), arrayWithObject: object]
    }

    unsafe fn init(self) -> id;

    unsafe fn count(self) -> NSUInteger;

    unsafe fn arrayByAddingObjectFromArray(self, object: id) -> id;
    unsafe fn arrayByAddingObjectsFromArray(self, objects: id) -> id;
    unsafe fn objectAtIndex(self, index: NSUInteger) -> id;
}

impl NSArray for id {
    unsafe fn init(self) -> id {
        msg_send![self, init]
    }

    unsafe fn count(self) -> NSUInteger {
        msg_send![self, count]
    }

    unsafe fn arrayByAddingObjectFromArray(self, object: id) -> id {
        msg_send![self, arrayByAddingObjectFromArray: object]
    }

    unsafe fn arrayByAddingObjectsFromArray(self, objects: id) -> id {
        msg_send![self, arrayByAddingObjectsFromArray: objects]
    }

    unsafe fn objectAtIndex(self, index: NSUInteger) -> id {
        msg_send![self, objectAtIndex: index]
    }
}

pub trait NSDictionary: Sized {
    unsafe fn dictionary(_: Self) -> id {
        msg_send![class!(NSDictionary), dictionary]
    }

    unsafe fn dictionaryWithContentsOfFile_(_: Self, path: id) -> id {
        msg_send![class!(NSDictionary), dictionaryWithContentsOfFile: path]
    }

    unsafe fn dictionaryWithContentsOfURL_(_: Self, aURL: id) -> id {
        msg_send![class!(NSDictionary), dictionaryWithContentsOfURL: aURL]
    }

    unsafe fn dictionaryWithDictionary_(_: Self, otherDictionary: id) -> id {
        msg_send![
            class!(NSDictionary),
            dictionaryWithDictionary: otherDictionary
        ]
    }

    unsafe fn dictionaryWithObject_forKey_(_: Self, anObject: id, aKey: id) -> id {
        msg_send![class!(NSDictionary), dictionaryWithObject:anObject forKey:aKey]
    }

    unsafe fn dictionaryWithObjects_forKeys_(_: Self, objects: id, keys: id) -> id {
        msg_send![class!(NSDictionary), dictionaryWithObjects:objects forKeys:keys]
    }

    unsafe fn dictionaryWithObjects_forKeys_count_(
        _: Self,
        objects: *const id,
        keys: *const id,
        count: NSUInteger,
    ) -> id {
        msg_send![class!(NSDictionary), dictionaryWithObjects:objects forKeys:keys count:count]
    }

    unsafe fn dictionaryWithObjectsAndKeys_(_: Self, firstObject: id) -> id {
        msg_send![
            class!(NSDictionary),
            dictionaryWithObjectsAndKeys: firstObject
        ]
    }

    unsafe fn init(self) -> id;
    unsafe fn initWithContentsOfFile_(self, path: id) -> id;
    unsafe fn initWithContentsOfURL_(self, aURL: id) -> id;
    unsafe fn initWithDictionary_(self, otherDicitonary: id) -> id;
    unsafe fn initWithDictionary_copyItems_(self, otherDicitonary: id, flag: BOOL) -> id;
    unsafe fn initWithObjects_forKeys_(self, objects: id, keys: id) -> id;
    unsafe fn initWithObjects_forKeys_count_(self, objects: id, keys: id, count: NSUInteger) -> id;
    unsafe fn initWithObjectsAndKeys_(self, firstObject: id) -> id;

    unsafe fn sharedKeySetForKeys_(_: Self, keys: id) -> id {
        msg_send![class!(NSDictionary), sharedKeySetForKeys: keys]
    }

    unsafe fn count(self) -> NSUInteger;

    unsafe fn isEqualToDictionary_(self, otherDictionary: id) -> BOOL;

    unsafe fn allKeys(self) -> id;
    unsafe fn allKeysForObject_(self, anObject: id) -> id;
    unsafe fn allValues(self) -> id;
    unsafe fn objectForKey_(self, aKey: id) -> id;
    unsafe fn objectForKeyedSubscript_(self, key: id) -> id;
    unsafe fn objectsForKeys_notFoundMarker_(self, keys: id, anObject: id) -> id;
    unsafe fn valueForKey_(self, key: id) -> id;

    unsafe fn keyEnumerator(self) -> id;
    unsafe fn objectEnumerator(self) -> id;
    unsafe fn enumerateKeysAndObjectsUsingBlock_(self, block: *mut Block<(id, id, *mut BOOL), ()>);
    unsafe fn enumerateKeysAndObjectsWithOptions_usingBlock_(
        self,
        opts: NSEnumerationOptions,
        block: *mut Block<(id, id, *mut BOOL), ()>,
    );

    unsafe fn keysSortedByValueUsingSelector_(self, comparator: SEL) -> id;
    unsafe fn keysSortedByValueUsingComparator_(self, cmptr: NSComparator) -> id;
    unsafe fn keysSortedByValueWithOptions_usingComparator_(
        self,
        opts: NSEnumerationOptions,
        cmptr: NSComparator,
    ) -> id;

    unsafe fn keysOfEntriesPassingTest_(
        self,
        predicate: *mut Block<(id, id, *mut BOOL), BOOL>,
    ) -> id;
    unsafe fn keysOfEntriesWithOptions_PassingTest_(
        self,
        opts: NSEnumerationOptions,
        predicate: *mut Block<(id, id, *mut BOOL), BOOL>,
    ) -> id;

    unsafe fn writeToFile_atomically_(self, path: id, flag: BOOL) -> BOOL;
    unsafe fn writeToURL_atomically_(self, aURL: id, flag: BOOL) -> BOOL;

    unsafe fn fileCreationDate(self) -> id;
    unsafe fn fileExtensionHidden(self) -> BOOL;
    unsafe fn fileGroupOwnerAccountID(self) -> id;
    unsafe fn fileGroupOwnerAccountName(self) -> id;
    unsafe fn fileIsAppendOnly(self) -> BOOL;
    unsafe fn fileIsImmutable(self) -> BOOL;
    unsafe fn fileModificationDate(self) -> id;
    unsafe fn fileOwnerAccountID(self) -> id;
    unsafe fn fileOwnerAccountName(self) -> id;
    unsafe fn filePosixPermissions(self) -> NSUInteger;
    unsafe fn fileSize(self) -> libc::c_ulonglong;
    unsafe fn fileSystemFileNumber(self) -> NSUInteger;
    unsafe fn fileSystemNumber(self) -> NSInteger;
    unsafe fn fileType(self) -> id;

    unsafe fn description(self) -> id;
    unsafe fn descriptionInStringsFileFormat(self) -> id;
    unsafe fn descriptionWithLocale_(self, locale: id) -> id;
    unsafe fn descriptionWithLocale_indent_(self, locale: id, indent: NSUInteger) -> id;
}

impl NSDictionary for id {
    unsafe fn init(self) -> id {
        msg_send![self, init]
    }

    unsafe fn initWithContentsOfFile_(self, path: id) -> id {
        msg_send![self, initWithContentsOfFile: path]
    }

    unsafe fn initWithContentsOfURL_(self, aURL: id) -> id {
        msg_send![self, initWithContentsOfURL: aURL]
    }

    unsafe fn initWithDictionary_(self, otherDictionary: id) -> id {
        msg_send![self, initWithDictionary: otherDictionary]
    }

    unsafe fn initWithDictionary_copyItems_(self, otherDictionary: id, flag: BOOL) -> id {
        msg_send![self, initWithDictionary:otherDictionary copyItems:flag]
    }

    unsafe fn initWithObjects_forKeys_(self, objects: id, keys: id) -> id {
        msg_send![self, initWithObjects:objects forKeys:keys]
    }

    unsafe fn initWithObjects_forKeys_count_(self, objects: id, keys: id, count: NSUInteger) -> id {
        msg_send![self, initWithObjects:objects forKeys:keys count:count]
    }

    unsafe fn initWithObjectsAndKeys_(self, firstObject: id) -> id {
        msg_send![self, initWithObjectsAndKeys: firstObject]
    }

    unsafe fn count(self) -> NSUInteger {
        msg_send![self, count]
    }

    unsafe fn isEqualToDictionary_(self, otherDictionary: id) -> BOOL {
        msg_send![self, isEqualToDictionary: otherDictionary]
    }

    unsafe fn allKeys(self) -> id {
        msg_send![self, allKeys]
    }

    unsafe fn allKeysForObject_(self, anObject: id) -> id {
        msg_send![self, allKeysForObject: anObject]
    }

    unsafe fn allValues(self) -> id {
        msg_send![self, allValues]
    }

    unsafe fn objectForKey_(self, aKey: id) -> id {
        msg_send![self, objectForKey: aKey]
    }

    unsafe fn objectForKeyedSubscript_(self, key: id) -> id {
        msg_send![self, objectForKeyedSubscript: key]
    }

    unsafe fn objectsForKeys_notFoundMarker_(self, keys: id, anObject: id) -> id {
        msg_send![self, objectsForKeys:keys notFoundMarker:anObject]
    }

    unsafe fn valueForKey_(self, key: id) -> id {
        msg_send![self, valueForKey: key]
    }

    unsafe fn keyEnumerator(self) -> id {
        msg_send![self, keyEnumerator]
    }

    unsafe fn objectEnumerator(self) -> id {
        msg_send![self, objectEnumerator]
    }

    unsafe fn enumerateKeysAndObjectsUsingBlock_(self, block: *mut Block<(id, id, *mut BOOL), ()>) {
        msg_send![self, enumerateKeysAndObjectsUsingBlock: block]
    }

    unsafe fn enumerateKeysAndObjectsWithOptions_usingBlock_(
        self,
        opts: NSEnumerationOptions,
        block: *mut Block<(id, id, *mut BOOL), ()>,
    ) {
        msg_send![self, enumerateKeysAndObjectsWithOptions:opts usingBlock:block]
    }

    unsafe fn keysSortedByValueUsingSelector_(self, comparator: SEL) -> id {
        msg_send![self, keysSortedByValueUsingSelector: comparator]
    }

    unsafe fn keysSortedByValueUsingComparator_(self, cmptr: NSComparator) -> id {
        msg_send![self, keysSortedByValueUsingComparator: cmptr]
    }

    unsafe fn keysSortedByValueWithOptions_usingComparator_(
        self,
        opts: NSEnumerationOptions,
        cmptr: NSComparator,
    ) -> id {
        let rv: id = msg_send![self, keysSortedByValueWithOptions:opts usingComparator:cmptr];
        rv
    }

    unsafe fn keysOfEntriesPassingTest_(
        self,
        predicate: *mut Block<(id, id, *mut BOOL), BOOL>,
    ) -> id {
        msg_send![self, keysOfEntriesPassingTest: predicate]
    }

    unsafe fn keysOfEntriesWithOptions_PassingTest_(
        self,
        opts: NSEnumerationOptions,
        predicate: *mut Block<(id, id, *mut BOOL), BOOL>,
    ) -> id {
        msg_send![self, keysOfEntriesWithOptions:opts PassingTest:predicate]
    }

    unsafe fn writeToFile_atomically_(self, path: id, flag: BOOL) -> BOOL {
        msg_send![self, writeToFile:path atomically:flag]
    }

    unsafe fn writeToURL_atomically_(self, aURL: id, flag: BOOL) -> BOOL {
        msg_send![self, writeToURL:aURL atomically:flag]
    }

    unsafe fn fileCreationDate(self) -> id {
        msg_send![self, fileCreationDate]
    }

    unsafe fn fileExtensionHidden(self) -> BOOL {
        msg_send![self, fileExtensionHidden]
    }

    unsafe fn fileGroupOwnerAccountID(self) -> id {
        msg_send![self, fileGroupOwnerAccountID]
    }

    unsafe fn fileGroupOwnerAccountName(self) -> id {
        msg_send![self, fileGroupOwnerAccountName]
    }

    unsafe fn fileIsAppendOnly(self) -> BOOL {
        msg_send![self, fileIsAppendOnly]
    }

    unsafe fn fileIsImmutable(self) -> BOOL {
        msg_send![self, fileIsImmutable]
    }

    unsafe fn fileModificationDate(self) -> id {
        msg_send![self, fileModificationDate]
    }

    unsafe fn fileOwnerAccountID(self) -> id {
        msg_send![self, fileOwnerAccountID]
    }

    unsafe fn fileOwnerAccountName(self) -> id {
        msg_send![self, fileOwnerAccountName]
    }

    unsafe fn filePosixPermissions(self) -> NSUInteger {
        msg_send![self, filePosixPermissions]
    }

    unsafe fn fileSize(self) -> libc::c_ulonglong {
        msg_send![self, fileSize]
    }

    unsafe fn fileSystemFileNumber(self) -> NSUInteger {
        msg_send![self, fileSystemFileNumber]
    }

    unsafe fn fileSystemNumber(self) -> NSInteger {
        msg_send![self, fileSystemNumber]
    }

    unsafe fn fileType(self) -> id {
        msg_send![self, fileType]
    }

    unsafe fn description(self) -> id {
        msg_send![self, description]
    }

    unsafe fn descriptionInStringsFileFormat(self) -> id {
        msg_send![self, descriptionInStringsFileFormat]
    }

    unsafe fn descriptionWithLocale_(self, locale: id) -> id {
        msg_send![self, descriptionWithLocale: locale]
    }

    unsafe fn descriptionWithLocale_indent_(self, locale: id, indent: NSUInteger) -> id {
        msg_send![self, descriptionWithLocale:locale indent:indent]
    }
}

bitflags! {
    pub struct NSEnumerationOptions: libc::c_ulonglong {
        const NSEnumerationConcurrent = 1 << 0;
        const NSEnumerationReverse = 1 << 1;
    }
}

pub type NSComparator = *mut Block<(id, id), NSComparisonResult>;

#[repr(isize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NSComparisonResult {
    NSOrderedAscending = -1,
    NSOrderedSame = 0,
    NSOrderedDescending = 1,
}

pub trait NSString: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSString), alloc]
    }

    unsafe fn stringByAppendingString_(self, other: id) -> id;
    unsafe fn init_str(self, string: &str) -> Self;
    unsafe fn UTF8String(self) -> *const libc::c_char;
    unsafe fn len(self) -> usize;
    unsafe fn isEqualToString(self, string: &str) -> bool;
    unsafe fn substringWithRange(self, range: NSRange) -> id;
}

impl NSString for id {
    unsafe fn isEqualToString(self, other: &str) -> bool {
        let other = NSString::alloc(nil).init_str(other);
        let rv: BOOL = msg_send![self, isEqualToString: other];
        rv != NO
    }

    unsafe fn stringByAppendingString_(self, other: id) -> id {
        msg_send![self, stringByAppendingString: other]
    }

    unsafe fn init_str(self, string: &str) -> id {
        msg_send![self,
                  initWithBytes:string.as_ptr()
                      length:string.len()
                      encoding:UTF8_ENCODING as id]
    }

    unsafe fn len(self) -> usize {
        msg_send![self, lengthOfBytesUsingEncoding: UTF8_ENCODING]
    }

    unsafe fn UTF8String(self) -> *const libc::c_char {
        msg_send![self, UTF8String]
    }

    unsafe fn substringWithRange(self, range: NSRange) -> id {
        msg_send![self, substringWithRange: range]
    }
}

pub trait NSDate: Sized {
    unsafe fn distantPast(_: Self) -> id {
        msg_send![class!(NSDate), distantPast]
    }

    unsafe fn distantFuture(_: Self) -> id {
        msg_send![class!(NSDate), distantFuture]
    }
}

impl NSDate for id {}

#[repr(C)]
struct NSFastEnumerationState {
    pub state: libc::c_ulong,
    pub items_ptr: *mut id,
    pub mutations_ptr: *mut libc::c_ulong,
    pub extra: [libc::c_ulong; 5],
}

const NS_FAST_ENUM_BUF_SIZE: usize = 16;

pub struct NSFastIterator {
    state: NSFastEnumerationState,
    buffer: [id; NS_FAST_ENUM_BUF_SIZE],
    mut_val: Option<libc::c_ulong>,
    len: usize,
    idx: usize,
    object: id,
}

impl Iterator for NSFastIterator {
    type Item = id;

    fn next(&mut self) -> Option<id> {
        if self.idx >= self.len {
            self.len = unsafe {
                msg_send![self.object, countByEnumeratingWithState:&mut self.state objects:self.buffer.as_mut_ptr() count:NS_FAST_ENUM_BUF_SIZE]
            };
            self.idx = 0;
        }

        let new_mut = unsafe { *self.state.mutations_ptr };

        if let Some(old_mut) = self.mut_val {
            assert!(
                old_mut == new_mut,
                "The collection was mutated while being enumerated"
            );
        }

        if self.idx < self.len {
            let object = unsafe { *self.state.items_ptr.add(self.idx) };
            self.mut_val = Some(new_mut);
            self.idx += 1;
            Some(object)
        } else {
            None
        }
    }
}

pub trait NSFastEnumeration: Sized {
    unsafe fn iter(self) -> NSFastIterator;
}

impl NSFastEnumeration for id {
    unsafe fn iter(self) -> NSFastIterator {
        NSFastIterator {
            state: NSFastEnumerationState {
                state: 0,
                items_ptr: ptr::null_mut(),
                mutations_ptr: ptr::null_mut(),
                extra: [0; 5],
            },
            buffer: [nil; NS_FAST_ENUM_BUF_SIZE],
            mut_val: None,
            len: 0,
            idx: 0,
            object: self,
        }
    }
}

pub trait NSRunLoop: Sized {
    unsafe fn currentRunLoop() -> Self;

    unsafe fn performSelector_target_argument_order_modes_(
        self,
        aSelector: SEL,
        target: id,
        anArgument: id,
        order: NSUInteger,
        modes: id,
    );
}

impl NSRunLoop for id {
    unsafe fn currentRunLoop() -> id {
        msg_send![class!(NSRunLoop), currentRunLoop]
    }

    unsafe fn performSelector_target_argument_order_modes_(
        self,
        aSelector: SEL,
        target: id,
        anArgument: id,
        order: NSUInteger,
        modes: id,
    ) {
        msg_send![self, performSelector:aSelector
                                 target:target
                               argument:anArgument
                                  order:order
                                  modes:modes]
    }
}

bitflags! {
    pub struct NSURLBookmarkCreationOptions: NSUInteger {
        const NSURLBookmarkCreationPreferFileIDResolution = 1 << 8;
        const NSURLBookmarkCreationMinimalBookmark = 1 << 9;
        const NSURLBookmarkCreationSuitableForBookmarkFile = 1 << 10;
        const NSURLBookmarkCreationWithSecurityScope = 1 << 11;
        const NSURLBookmarkCreationSecurityScopeAllowOnlyReadAccess = 1 << 12;
    }
}

pub type NSURLBookmarkFileCreationOptions = NSURLBookmarkCreationOptions;

bitflags! {
    pub struct NSURLBookmarkResolutionOptions: NSUInteger {
        const NSURLBookmarkResolutionWithoutUI = 1 << 8;
        const NSURLBookmarkResolutionWithoutMounting = 1 << 9;
        const NSURLBookmarkResolutionWithSecurityScope = 1 << 10;
    }
}

pub trait NSURL: Sized {
    unsafe fn alloc(_: Self) -> id;

    unsafe fn URLWithString_(_: Self, string: id) -> id;
    unsafe fn initWithString_(self, string: id) -> id;
    unsafe fn URLWithString_relativeToURL_(_: Self, string: id, url: id) -> id;
    unsafe fn initWithString_relativeToURL_(self, string: id, url: id) -> id;
    unsafe fn fileURLWithPath_isDirectory_(_: Self, path: id, is_dir: BOOL) -> id;
    unsafe fn initFileURLWithPath_isDirectory_(self, path: id, is_dir: BOOL) -> id;
    unsafe fn fileURLWithPath_relativeToURL_(_: Self, path: id, url: id) -> id;
    unsafe fn initFileURLWithPath_relativeToURL_(self, path: id, url: id) -> id;
    unsafe fn fileURLWithPath_isDirectory_relativeToURL_(
        _: Self,
        path: id,
        is_dir: BOOL,
        url: id,
    ) -> id;
    unsafe fn initFileURLWithPath_isDirectory_relativeToURL_(
        self,
        path: id,
        is_dir: BOOL,
        url: id,
    ) -> id;
    unsafe fn fileURLWithPath_(_: Self, path: id) -> id;
    unsafe fn initFileURLWithPath_(self, path: id) -> id;
    unsafe fn fileURLWithPathComponents_(
        _: Self,
        path_components: id, /* (NSArray<NSString*>*) */
    ) -> id;
    unsafe fn URLByResolvingAliasFileAtURL_options_error_(
        _: Self,
        url: id,
        options: NSURLBookmarkResolutionOptions,
        error: *mut id, /* (NSError _Nullable) */
    ) -> id;
    unsafe fn URLByResolvingBookmarkData_options_relativeToURL_bookmarkDataIsStale_error_(
        _: Self,
        data: id, /* (NSData) */
        options: NSURLBookmarkResolutionOptions,
        relative_to_url: id,
        is_stale: *mut BOOL,
        error: *mut id, /* (NSError _Nullable) */
    ) -> id;
    unsafe fn initByResolvingBookmarkData_options_relativeToURL_bookmarkDataIsStale_error_(
        self,
        data: id, /* (NSData) */
        options: NSURLBookmarkResolutionOptions,
        relative_to_url: id,
        is_stale: *mut BOOL,
        error: *mut id, /* (NSError _Nullable) */
    ) -> id;
    // unsafe fn fileURLWithFileSystemRepresentation_isDirectory_relativeToURL_
    // unsafe fn getFileSystemRepresentation_maxLength_
    // unsafe fn initFileURLWithFileSystemRepresentation_isDirectory_relativeToURL_
    unsafe fn absoluteURLWithDataRepresentation_relativeToURL_(
        _: Self,
        data: id, /* (NSData) */
        url: id,
    ) -> id;
    unsafe fn initAbsoluteURLWithDataRepresentation_relativeToURL_(
        self,
        data: id, /* (NSData) */
        url: id,
    ) -> id;
    unsafe fn URLWithDataRepresentation_relativeToURL_(
        _: Self,
        data: id, /* (NSData) */
        url: id,
    ) -> id;
    unsafe fn initWithDataRepresentation_relativeToURL_(
        self,
        data: id, /* (NSData) */
        url: id,
    ) -> id;
    unsafe fn dataRepresentation(self) -> id /* (NSData) */;

    unsafe fn isEqual_(self, id: id) -> BOOL;

    unsafe fn checkResourceIsReachableAndReturnError_(
        self,
        error: id, /* (NSError _Nullable) */
    ) -> BOOL;
    unsafe fn isFileReferenceURL(self) -> BOOL;
    unsafe fn isFileURL(self) -> BOOL;

    unsafe fn absoluteString(self) -> id /* (NSString) */;
    unsafe fn absoluteURL(self) -> id /* (NSURL) */;
    unsafe fn baseURL(self) -> id /* (NSURL) */;
    // unsafe fn fileSystemRepresentation
    unsafe fn fragment(self) -> id /* (NSString) */;
    unsafe fn host(self) -> id /* (NSString) */;
    unsafe fn lastPathComponent(self) -> id /* (NSString) */;
    unsafe fn parameterString(self) -> id /* (NSString) */;
    unsafe fn password(self) -> id /* (NSString) */;
    unsafe fn path(self) -> id /* (NSString) */;
    unsafe fn pathComponents(self) -> id /* (NSArray<NSString*>) */;
    unsafe fn pathExtension(self) -> id /* (NSString) */;
    unsafe fn port(self) -> id /* (NSNumber) */;
    unsafe fn query(self) -> id /* (NSString) */;
    unsafe fn relativePath(self) -> id /* (NSString) */;
    unsafe fn relativeString(self) -> id /* (NSString) */;
    unsafe fn resourceSpecifier(self) -> id /* (NSString) */;
    unsafe fn scheme(self) -> id /* (NSString) */;
    unsafe fn standardizedURL(self) -> id /* (NSURL) */;
    unsafe fn user(self) -> id /* (NSString) */;

    // unsafe fn resourceValuesForKeys_error_
    // unsafe fn getResourceValue_forKey_error_
    // unsafe fn setResourceValue_forKey_error_
    // unsafe fn setResourceValues_error_
    // unsafe fn removeAllCachedResourceValues
    // unsafe fn removeCachedResourceValueForKey_
    // unsafe fn setTemporaryResourceValue_forKey_
    unsafe fn NSURLResourceKey(self) -> id /* (NSString) */;

    unsafe fn filePathURL(self) -> id;
    unsafe fn fileReferenceURL(self) -> id;
    unsafe fn URLByAppendingPathComponent_(self, path_component: id /* (NSString) */) -> id;
    unsafe fn URLByAppendingPathComponent_isDirectory_(
        self,
        path_component: id, /* (NSString) */
        is_dir: BOOL,
    ) -> id;
    unsafe fn URLByAppendingPathExtension_(self, extension: id /* (NSString) */) -> id;
    unsafe fn URLByDeletingLastPathComponent(self) -> id;
    unsafe fn URLByDeletingPathExtension(self) -> id;
    unsafe fn URLByResolvingSymlinksInPath(self) -> id;
    unsafe fn URLByStandardizingPath(self) -> id;
    unsafe fn hasDirectoryPath(self) -> BOOL;

    unsafe fn bookmarkDataWithContentsOfURL_error_(
        _: Self,
        url: id,
        error: id, /* (NSError _Nullable) */
    ) -> id /* (NSData) */;
    unsafe fn bookmarkDataWithOptions_includingResourceValuesForKeys_relativeToURL_error_(
        self,
        options: NSURLBookmarkCreationOptions,
        resource_value_for_keys: id, /* (NSArray<NSURLResourceKey>) */
        relative_to_url: id,
        error: id, /* (NSError _Nullable) */
    ) -> id /* (NSData) */;
    // unsafe fn resourceValuesForKeys_fromBookmarkData_
    unsafe fn writeBookmarkData_toURL_options_error_(
        _: Self,
        data: id, /* (NSData) */
        to_url: id,
        options: NSURLBookmarkFileCreationOptions,
        error: id, /* (NSError _Nullable) */
    ) -> id;
    unsafe fn startAccessingSecurityScopedResource(self) -> BOOL;
    unsafe fn stopAccessingSecurityScopedResource(self);
    unsafe fn NSURLBookmarkFileCreationOptions(self) -> NSURLBookmarkFileCreationOptions;
    unsafe fn NSURLBookmarkCreationOptions(self) -> NSURLBookmarkCreationOptions;
    unsafe fn NSURLBookmarkResolutionOptions(self) -> NSURLBookmarkResolutionOptions;

    // unsafe fn checkPromisedItemIsReachableAndReturnError_
    // unsafe fn getPromisedItemResourceValue_forKey_error_
    // unsafe fn promisedItemResourceValuesForKeys_error_

    // unsafe fn URLFromPasteboard_
    // unsafe fn writeToPasteboard_
}

impl NSURL for id {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSURL), alloc]
    }

    unsafe fn URLWithString_(_: Self, string: id) -> id {
        msg_send![class!(NSURL), URLWithString: string]
    }
    unsafe fn initWithString_(self, string: id) -> id {
        msg_send![self, initWithString: string]
    }
    unsafe fn URLWithString_relativeToURL_(_: Self, string: id, url: id) -> id {
        msg_send![class!(NSURL), URLWithString: string relativeToURL:url]
    }
    unsafe fn initWithString_relativeToURL_(self, string: id, url: id) -> id {
        msg_send![self, initWithString:string relativeToURL:url]
    }
    unsafe fn fileURLWithPath_isDirectory_(_: Self, path: id, is_dir: BOOL) -> id {
        msg_send![class!(NSURL), fileURLWithPath:path isDirectory:is_dir]
    }
    unsafe fn initFileURLWithPath_isDirectory_(self, path: id, is_dir: BOOL) -> id {
        msg_send![self, initFileURLWithPath:path isDirectory:is_dir]
    }
    unsafe fn fileURLWithPath_relativeToURL_(_: Self, path: id, url: id) -> id {
        msg_send![class!(NSURL), fileURLWithPath:path relativeToURL:url]
    }
    unsafe fn initFileURLWithPath_relativeToURL_(self, path: id, url: id) -> id {
        msg_send![self, initFileURLWithPath:path relativeToURL:url]
    }
    unsafe fn fileURLWithPath_isDirectory_relativeToURL_(
        _: Self,
        path: id,
        is_dir: BOOL,
        url: id,
    ) -> id {
        msg_send![class!(NSURL), fileURLWithPath:path isDirectory:is_dir relativeToURL:url]
    }
    unsafe fn initFileURLWithPath_isDirectory_relativeToURL_(
        self,
        path: id,
        is_dir: BOOL,
        url: id,
    ) -> id {
        msg_send![self, initFileURLWithPath:path isDirectory:is_dir relativeToURL:url]
    }
    unsafe fn fileURLWithPath_(_: Self, path: id) -> id {
        msg_send![class!(NSURL), fileURLWithPath: path]
    }
    unsafe fn initFileURLWithPath_(self, path: id) -> id {
        msg_send![self, initFileURLWithPath: path]
    }
    unsafe fn fileURLWithPathComponents_(
        _: Self,
        path_components: id, /* (NSArray<NSString*>*) */
    ) -> id {
        msg_send![class!(NSURL), fileURLWithPathComponents: path_components]
    }
    unsafe fn URLByResolvingAliasFileAtURL_options_error_(
        _: Self,
        url: id,
        options: NSURLBookmarkResolutionOptions,
        error: *mut id, /* (NSError _Nullable) */
    ) -> id {
        msg_send![class!(NSURL), URLByResolvingAliasFileAtURL:url options:options error:error]
    }
    unsafe fn URLByResolvingBookmarkData_options_relativeToURL_bookmarkDataIsStale_error_(
        _: Self,
        data: id, /* (NSData) */
        options: NSURLBookmarkResolutionOptions,
        relative_to_url: id,
        is_stale: *mut BOOL,
        error: *mut id, /* (NSError _Nullable) */
    ) -> id {
        msg_send![class!(NSURL), URLByResolvingBookmarkData:data options:options relativeToURL:relative_to_url bookmarkDataIsStale:is_stale error:error]
    }
    unsafe fn initByResolvingBookmarkData_options_relativeToURL_bookmarkDataIsStale_error_(
        self,
        data: id, /* (NSData) */
        options: NSURLBookmarkResolutionOptions,
        relative_to_url: id,
        is_stale: *mut BOOL,
        error: *mut id, /* (NSError _Nullable) */
    ) -> id {
        msg_send![self, initByResolvingBookmarkData:data options:options relativeToURL:relative_to_url bookmarkDataIsStale:is_stale error:error]
    }
    // unsafe fn fileURLWithFileSystemRepresentation_isDirectory_relativeToURL_
    // unsafe fn getFileSystemRepresentation_maxLength_
    // unsafe fn initFileURLWithFileSystemRepresentation_isDirectory_relativeToURL_
    unsafe fn absoluteURLWithDataRepresentation_relativeToURL_(
        _: Self,
        data: id, /* (NSData) */
        url: id,
    ) -> id {
        msg_send![class!(NSURL), absoluteURLWithDataRepresentation:data relativeToURL:url]
    }
    unsafe fn initAbsoluteURLWithDataRepresentation_relativeToURL_(
        self,
        data: id, /* (NSData) */
        url: id,
    ) -> id {
        msg_send![self, initAbsoluteURLWithDataRepresentation:data relativeToURL:url]
    }
    unsafe fn URLWithDataRepresentation_relativeToURL_(
        _: Self,
        data: id, /* (NSData) */
        url: id,
    ) -> id {
        msg_send![class!(NSURL), URLWithDataRepresentation:data relativeToURL:url]
    }
    unsafe fn initWithDataRepresentation_relativeToURL_(
        self,
        data: id, /* (NSData) */
        url: id,
    ) -> id {
        msg_send![self, initWithDataRepresentation:data relativeToURL:url]
    }
    unsafe fn dataRepresentation(self) -> id /* (NSData) */ {
        msg_send![self, dataRepresentation]
    }

    unsafe fn isEqual_(self, id: id) -> BOOL {
        msg_send![self, isEqual: id]
    }

    unsafe fn checkResourceIsReachableAndReturnError_(
        self,
        error: id, /* (NSError _Nullable) */
    ) -> BOOL {
        msg_send![self, checkResourceIsReachableAndReturnError: error]
    }
    unsafe fn isFileReferenceURL(self) -> BOOL {
        msg_send![self, isFileReferenceURL]
    }
    unsafe fn isFileURL(self) -> BOOL {
        msg_send![self, isFileURL]
    }

    unsafe fn absoluteString(self) -> id /* (NSString) */ {
        msg_send![self, absoluteString]
    }
    unsafe fn absoluteURL(self) -> id /* (NSURL) */ {
        msg_send![self, absoluteURL]
    }
    unsafe fn baseURL(self) -> id /* (NSURL) */ {
        msg_send![self, baseURL]
    }
    // unsafe fn fileSystemRepresentation
    unsafe fn fragment(self) -> id /* (NSString) */ {
        msg_send![self, fragment]
    }
    unsafe fn host(self) -> id /* (NSString) */ {
        msg_send![self, host]
    }
    unsafe fn lastPathComponent(self) -> id /* (NSString) */ {
        msg_send![self, lastPathComponent]
    }
    unsafe fn parameterString(self) -> id /* (NSString) */ {
        msg_send![self, parameterString]
    }
    unsafe fn password(self) -> id /* (NSString) */ {
        msg_send![self, password]
    }
    unsafe fn path(self) -> id /* (NSString) */ {
        msg_send![self, path]
    }
    unsafe fn pathComponents(self) -> id /* (NSArray<NSString*>) */ {
        msg_send![self, pathComponents]
    }
    unsafe fn pathExtension(self) -> id /* (NSString) */ {
        msg_send![self, pathExtension]
    }
    unsafe fn port(self) -> id /* (NSNumber) */ {
        msg_send![self, port]
    }
    unsafe fn query(self) -> id /* (NSString) */ {
        msg_send![self, query]
    }
    unsafe fn relativePath(self) -> id /* (NSString) */ {
        msg_send![self, relativePath]
    }
    unsafe fn relativeString(self) -> id /* (NSString) */ {
        msg_send![self, relativeString]
    }
    unsafe fn resourceSpecifier(self) -> id /* (NSString) */ {
        msg_send![self, resourceSpecifier]
    }
    unsafe fn scheme(self) -> id /* (NSString) */ {
        msg_send![self, scheme]
    }
    unsafe fn standardizedURL(self) -> id /* (NSURL) */ {
        msg_send![self, standardizedURL]
    }
    unsafe fn user(self) -> id /* (NSString) */ {
        msg_send![self, user]
    }

    // unsafe fn resourceValuesForKeys_error_
    // unsafe fn getResourceValue_forKey_error_
    // unsafe fn setResourceValue_forKey_error_
    // unsafe fn setResourceValues_error_
    // unsafe fn removeAllCachedResourceValues
    // unsafe fn removeCachedResourceValueForKey_
    // unsafe fn setTemporaryResourceValue_forKey_
    unsafe fn NSURLResourceKey(self) -> id /* (NSString) */ {
        msg_send![self, NSURLResourceKey]
    }

    unsafe fn filePathURL(self) -> id {
        msg_send![self, filePathURL]
    }
    unsafe fn fileReferenceURL(self) -> id {
        msg_send![self, fileReferenceURL]
    }
    unsafe fn URLByAppendingPathComponent_(self, path_component: id /* (NSString) */) -> id {
        msg_send![self, URLByAppendingPathComponent: path_component]
    }
    unsafe fn URLByAppendingPathComponent_isDirectory_(
        self,
        path_component: id, /* (NSString) */
        is_dir: BOOL,
    ) -> id {
        msg_send![self, URLByAppendingPathComponent:path_component isDirectory:is_dir]
    }
    unsafe fn URLByAppendingPathExtension_(self, extension: id /* (NSString) */) -> id {
        msg_send![self, URLByAppendingPathExtension: extension]
    }
    unsafe fn URLByDeletingLastPathComponent(self) -> id {
        msg_send![self, URLByDeletingLastPathComponent]
    }
    unsafe fn URLByDeletingPathExtension(self) -> id {
        msg_send![self, URLByDeletingPathExtension]
    }
    unsafe fn URLByResolvingSymlinksInPath(self) -> id {
        msg_send![self, URLByResolvingSymlinksInPath]
    }
    unsafe fn URLByStandardizingPath(self) -> id {
        msg_send![self, URLByStandardizingPath]
    }
    unsafe fn hasDirectoryPath(self) -> BOOL {
        msg_send![self, hasDirectoryPath]
    }

    unsafe fn bookmarkDataWithContentsOfURL_error_(
        _: Self,
        url: id,
        error: id, /* (NSError _Nullable) */
    ) -> id /* (NSData) */ {
        msg_send![class!(NSURL), bookmarkDataWithContentsOfURL:url error:error]
    }
    unsafe fn bookmarkDataWithOptions_includingResourceValuesForKeys_relativeToURL_error_(
        self,
        options: NSURLBookmarkCreationOptions,
        resource_value_for_keys: id, /* (NSArray<NSURLResourceKey>) */
        relative_to_url: id,
        error: id, /* (NSError _Nullable) */
    ) -> id /* (NSData) */ {
        msg_send![self, bookmarkDataWithOptions:options includingResourceValuesForKeys:resource_value_for_keys relativeToURL:relative_to_url error:error]
    }
    // unsafe fn resourceValuesForKeys_fromBookmarkData_
    unsafe fn writeBookmarkData_toURL_options_error_(
        _: Self,
        data: id, /* (NSData) */
        to_url: id,
        options: NSURLBookmarkFileCreationOptions,
        error: id, /* (NSError _Nullable) */
    ) -> id {
        msg_send![class!(NSURL), writeBookmarkData:data toURL:to_url options:options error:error]
    }
    unsafe fn startAccessingSecurityScopedResource(self) -> BOOL {
        msg_send![self, startAccessingSecurityScopedResource]
    }
    unsafe fn stopAccessingSecurityScopedResource(self) {
        msg_send![self, stopAccessingSecurityScopedResource]
    }
    unsafe fn NSURLBookmarkFileCreationOptions(self) -> NSURLBookmarkFileCreationOptions {
        msg_send![self, NSURLBookmarkFileCreationOptions]
    }
    unsafe fn NSURLBookmarkCreationOptions(self) -> NSURLBookmarkCreationOptions {
        msg_send![self, NSURLBookmarkCreationOptions]
    }
    unsafe fn NSURLBookmarkResolutionOptions(self) -> NSURLBookmarkResolutionOptions {
        msg_send![self, NSURLBookmarkResolutionOptions]
    }

    // unsafe fn checkPromisedItemIsReachableAndReturnError_
    // unsafe fn getPromisedItemResourceValue_forKey_error_
    // unsafe fn promisedItemResourceValuesForKeys_error_

    // unsafe fn URLFromPasteboard_
    // unsafe fn writeToPasteboard_
}

pub trait NSBundle: Sized {
    unsafe fn mainBundle() -> Self;

    unsafe fn loadNibNamed_owner_topLevelObjects_(
        self,
        name: id, /* NSString */
        owner: id,
        topLevelObjects: *mut id, /* NSArray */
    ) -> BOOL;

    unsafe fn bundleIdentifier(self) -> id /* NSString */;

    unsafe fn resourcePath(self) -> id /* NSString */;
}

impl NSBundle for id {
    unsafe fn mainBundle() -> id {
        msg_send![class!(NSBundle), mainBundle]
    }

    unsafe fn loadNibNamed_owner_topLevelObjects_(
        self,
        name: id, /* NSString */
        owner: id,
        topLevelObjects: *mut id, /* NSArray* */
    ) -> BOOL {
        msg_send![self, loadNibNamed:name
                               owner:owner
                     topLevelObjects:topLevelObjects]
    }

    unsafe fn bundleIdentifier(self) -> id /* NSString */ {
        msg_send![self, bundleIdentifier]
    }

    unsafe fn resourcePath(self) -> id /* NSString */ {
        msg_send![self, resourcePath]
    }
}

pub trait NSData: Sized {
    unsafe fn data(_: Self) -> id {
        msg_send![class!(NSData), data]
    }

    unsafe fn dataWithBytes_length_(_: Self, bytes: *const c_void, length: NSUInteger) -> id {
        msg_send![class!(NSData), dataWithBytes:bytes length:length]
    }

    unsafe fn dataWithBytesNoCopy_length_(_: Self, bytes: *const c_void, length: NSUInteger) -> id {
        msg_send![class!(NSData), dataWithBytesNoCopy:bytes length:length]
    }

    unsafe fn dataWithBytesNoCopy_length_freeWhenDone_(
        _: Self,
        bytes: *const c_void,
        length: NSUInteger,
        freeWhenDone: BOOL,
    ) -> id {
        msg_send![class!(NSData), dataWithBytesNoCopy:bytes length:length freeWhenDone:freeWhenDone]
    }

    unsafe fn dataWithContentsOfFile_(_: Self, path: id) -> id {
        msg_send![class!(NSData), dataWithContentsOfFile: path]
    }

    unsafe fn dataWithContentsOfFile_options_error_(
        _: Self,
        path: id,
        mask: NSDataReadingOptions,
        errorPtr: *mut id,
    ) -> id {
        msg_send![class!(NSData), dataWithContentsOfFile:path options:mask error:errorPtr]
    }

    unsafe fn dataWithContentsOfURL_(_: Self, aURL: id) -> id {
        msg_send![class!(NSData), dataWithContentsOfURL: aURL]
    }

    unsafe fn dataWithContentsOfURL_options_error_(
        _: Self,
        aURL: id,
        mask: NSDataReadingOptions,
        errorPtr: *mut id,
    ) -> id {
        msg_send![class!(NSData), dataWithContentsOfURL:aURL options:mask error:errorPtr]
    }

    unsafe fn dataWithData_(_: Self, aData: id) -> id {
        msg_send![class!(NSData), dataWithData: aData]
    }

    unsafe fn initWithBase64EncodedData_options_(
        self,
        base64Data: id,
        options: NSDataBase64DecodingOptions,
    ) -> id;
    unsafe fn initWithBase64EncodedString_options_(
        self,
        base64String: id,
        options: NSDataBase64DecodingOptions,
    ) -> id;
    unsafe fn initWithBytes_length_(self, bytes: *const c_void, length: NSUInteger) -> id;
    unsafe fn initWithBytesNoCopy_length_(self, bytes: *const c_void, length: NSUInteger) -> id;
    unsafe fn initWithBytesNoCopy_length_deallocator_(
        self,
        bytes: *const c_void,
        length: NSUInteger,
        deallocator: *mut Block<(*const c_void, NSUInteger), ()>,
    ) -> id;
    unsafe fn initWithBytesNoCopy_length_freeWhenDone_(
        self,
        bytes: *const c_void,
        length: NSUInteger,
        freeWhenDone: BOOL,
    ) -> id;
    unsafe fn initWithContentsOfFile_(self, path: id) -> id;
    unsafe fn initWithContentsOfFile_options_error(
        self,
        path: id,
        mask: NSDataReadingOptions,
        errorPtr: *mut id,
    ) -> id;
    unsafe fn initWithContentsOfURL_(self, aURL: id) -> id;
    unsafe fn initWithContentsOfURL_options_error_(
        self,
        aURL: id,
        mask: NSDataReadingOptions,
        errorPtr: *mut id,
    ) -> id;
    unsafe fn initWithData_(self, data: id) -> id;

    unsafe fn bytes(self) -> *const c_void;
    unsafe fn description(self) -> id;
    unsafe fn enumerateByteRangesUsingBlock_(
        self,
        block: *mut Block<(*const c_void, NSRange, *mut BOOL), ()>,
    );
    unsafe fn getBytes_length_(self, buffer: *mut c_void, length: NSUInteger);
    unsafe fn getBytes_range_(self, buffer: *mut c_void, range: NSRange);
    unsafe fn subdataWithRange_(self, range: NSRange) -> id;
    unsafe fn rangeOfData_options_range_(
        self,
        dataToFind: id,
        options: NSDataSearchOptions,
        searchRange: NSRange,
    ) -> NSRange;

    unsafe fn base64EncodedDataWithOptions_(self, options: NSDataBase64EncodingOptions) -> id;
    unsafe fn base64EncodedStringWithOptions_(self, options: NSDataBase64EncodingOptions) -> id;

    unsafe fn isEqualToData_(self, otherData: id) -> id;
    unsafe fn length(self) -> NSUInteger;

    unsafe fn writeToFile_atomically_(self, path: id, atomically: BOOL) -> BOOL;
    unsafe fn writeToFile_options_error_(
        self,
        path: id,
        mask: NSDataWritingOptions,
        errorPtr: *mut id,
    ) -> BOOL;
    unsafe fn writeToURL_atomically_(self, aURL: id, atomically: BOOL) -> BOOL;
    unsafe fn writeToURL_options_error_(
        self,
        aURL: id,
        mask: NSDataWritingOptions,
        errorPtr: *mut id,
    ) -> BOOL;
}

impl NSData for id {
    unsafe fn initWithBase64EncodedData_options_(
        self,
        base64Data: id,
        options: NSDataBase64DecodingOptions,
    ) -> id {
        msg_send![self, initWithBase64EncodedData:base64Data options:options]
    }

    unsafe fn initWithBase64EncodedString_options_(
        self,
        base64String: id,
        options: NSDataBase64DecodingOptions,
    ) -> id {
        msg_send![self, initWithBase64EncodedString:base64String options:options]
    }

    unsafe fn initWithBytes_length_(self, bytes: *const c_void, length: NSUInteger) -> id {
        msg_send![self,initWithBytes:bytes length:length]
    }

    unsafe fn initWithBytesNoCopy_length_(self, bytes: *const c_void, length: NSUInteger) -> id {
        msg_send![self, initWithBytesNoCopy:bytes length:length]
    }

    unsafe fn initWithBytesNoCopy_length_deallocator_(
        self,
        bytes: *const c_void,
        length: NSUInteger,
        deallocator: *mut Block<(*const c_void, NSUInteger), ()>,
    ) -> id {
        msg_send![self, initWithBytesNoCopy:bytes length:length deallocator:deallocator]
    }

    unsafe fn initWithBytesNoCopy_length_freeWhenDone_(
        self,
        bytes: *const c_void,
        length: NSUInteger,
        freeWhenDone: BOOL,
    ) -> id {
        msg_send![self, initWithBytesNoCopy:bytes length:length freeWhenDone:freeWhenDone]
    }

    unsafe fn initWithContentsOfFile_(self, path: id) -> id {
        msg_send![self, initWithContentsOfFile: path]
    }

    unsafe fn initWithContentsOfFile_options_error(
        self,
        path: id,
        mask: NSDataReadingOptions,
        errorPtr: *mut id,
    ) -> id {
        msg_send![self, initWithContentsOfFile:path options:mask error:errorPtr]
    }

    unsafe fn initWithContentsOfURL_(self, aURL: id) -> id {
        msg_send![self, initWithContentsOfURL: aURL]
    }

    unsafe fn initWithContentsOfURL_options_error_(
        self,
        aURL: id,
        mask: NSDataReadingOptions,
        errorPtr: *mut id,
    ) -> id {
        msg_send![self, initWithContentsOfURL:aURL options:mask error:errorPtr]
    }

    unsafe fn initWithData_(self, data: id) -> id {
        msg_send![self, initWithData: data]
    }

    unsafe fn bytes(self) -> *const c_void {
        msg_send![self, bytes]
    }

    unsafe fn description(self) -> id {
        msg_send![self, description]
    }

    unsafe fn enumerateByteRangesUsingBlock_(
        self,
        block: *mut Block<(*const c_void, NSRange, *mut BOOL), ()>,
    ) {
        msg_send![self, enumerateByteRangesUsingBlock: block]
    }

    unsafe fn getBytes_length_(self, buffer: *mut c_void, length: NSUInteger) {
        msg_send![self, getBytes:buffer length:length]
    }

    unsafe fn getBytes_range_(self, buffer: *mut c_void, range: NSRange) {
        msg_send![self, getBytes:buffer range:range]
    }

    unsafe fn subdataWithRange_(self, range: NSRange) -> id {
        msg_send![self, subdataWithRange: range]
    }

    unsafe fn rangeOfData_options_range_(
        self,
        dataToFind: id,
        options: NSDataSearchOptions,
        searchRange: NSRange,
    ) -> NSRange {
        msg_send![self, rangeOfData:dataToFind options:options range:searchRange]
    }

    unsafe fn base64EncodedDataWithOptions_(self, options: NSDataBase64EncodingOptions) -> id {
        msg_send![self, base64EncodedDataWithOptions: options]
    }

    unsafe fn base64EncodedStringWithOptions_(self, options: NSDataBase64EncodingOptions) -> id {
        msg_send![self, base64EncodedStringWithOptions: options]
    }

    unsafe fn isEqualToData_(self, otherData: id) -> id {
        msg_send![self, isEqualToData: otherData]
    }

    unsafe fn length(self) -> NSUInteger {
        msg_send![self, length]
    }

    unsafe fn writeToFile_atomically_(self, path: id, atomically: BOOL) -> BOOL {
        msg_send![self, writeToFile:path atomically:atomically]
    }

    unsafe fn writeToFile_options_error_(
        self,
        path: id,
        mask: NSDataWritingOptions,
        errorPtr: *mut id,
    ) -> BOOL {
        msg_send![self, writeToFile:path options:mask error:errorPtr]
    }

    unsafe fn writeToURL_atomically_(self, aURL: id, atomically: BOOL) -> BOOL {
        msg_send![self, writeToURL:aURL atomically:atomically]
    }

    unsafe fn writeToURL_options_error_(
        self,
        aURL: id,
        mask: NSDataWritingOptions,
        errorPtr: *mut id,
    ) -> BOOL {
        msg_send![self, writeToURL:aURL options:mask error:errorPtr]
    }
}

bitflags! {
    pub struct NSDataReadingOptions: libc::c_ulonglong {
       const NSDataReadingMappedIfSafe = 1 << 0;
       const NSDataReadingUncached = 1 << 1;
       const NSDataReadingMappedAlways = 1 << 3;
    }
}

bitflags! {
    pub struct NSDataBase64EncodingOptions: libc::c_ulonglong {
        const NSDataBase64Encoding64CharacterLineLength = 1 << 0;
        const NSDataBase64Encoding76CharacterLineLength = 1 << 1;
        const NSDataBase64EncodingEndLineWithCarriageReturn = 1 << 4;
        const NSDataBase64EncodingEndLineWithLineFeed = 1 << 5;
    }
}

bitflags! {
    pub struct NSDataBase64DecodingOptions: libc::c_ulonglong {
       const NSDataBase64DecodingIgnoreUnknownCharacters = 1 << 0;
    }
}

bitflags! {
    pub struct NSDataWritingOptions: libc::c_ulonglong {
        const NSDataWritingAtomic = 1 << 0;
        const NSDataWritingWithoutOverwriting = 1 << 1;
    }
}

bitflags! {
    pub struct NSDataSearchOptions: libc::c_ulonglong {
        const NSDataSearchBackwards = 1 << 0;
        const NSDataSearchAnchored = 1 << 1;
    }
}

pub trait NSUserDefaults {
    unsafe fn standardUserDefaults() -> Self;

    unsafe fn setBool_forKey_(self, value: BOOL, key: id);
    unsafe fn bool_forKey_(self, key: id) -> BOOL;

    unsafe fn removeObject_forKey_(self, key: id);
}

impl NSUserDefaults for id {
    unsafe fn standardUserDefaults() -> id {
        msg_send![class!(NSUserDefaults), standardUserDefaults]
    }

    unsafe fn setBool_forKey_(self, value: BOOL, key: id) {
        msg_send![self, setBool:value forKey:key]
    }

    unsafe fn bool_forKey_(self, key: id) -> BOOL {
        msg_send![self, boolForKey: key]
    }

    unsafe fn removeObject_forKey_(self, key: id) {
        msg_send![self, removeObjectForKey: key]
    }
}
