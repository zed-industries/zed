// Copyright 2019 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::base::{Boolean, CFAllocatorRef, CFIndex, CFRange, CFTypeID, UTF32Char};
use crate::data::CFDataRef;
use crate::string::{CFStringRef, UniChar};
use std::os::raw::c_void;

pub type CFCharacterSetPredefinedSet = CFIndex;

// Members of CFCharacterSetPredefinedSet enum
pub static kCFCharacterSetControl: CFCharacterSetPredefinedSet = 1;
pub static kCFCharacterSetWhitespace: CFCharacterSetPredefinedSet = 2;
pub static kCFCharacterSetWhitespaceAndNewline: CFCharacterSetPredefinedSet = 3;
pub static kCFCharacterSetDecimalDigit: CFCharacterSetPredefinedSet = 4;
pub static kCFCharacterSetLetter: CFCharacterSetPredefinedSet = 5;
pub static kCFCharacterSetLowercaseLetter: CFCharacterSetPredefinedSet = 6;
pub static kCFCharacterSetUppercaseLetter: CFCharacterSetPredefinedSet = 7;
pub static kCFCharacterSetNonBase: CFCharacterSetPredefinedSet = 8;
pub static kCFCharacterSetDecomposable: CFCharacterSetPredefinedSet = 9;
pub static kCFCharacterSetAlphaNumeric: CFCharacterSetPredefinedSet = 10;
pub static kCFCharacterSetPunctuation: CFCharacterSetPredefinedSet = 11;
pub static kCFCharacterSetIllegal: CFCharacterSetPredefinedSet = 12;
pub static kCFCharacterSetCapitalizedLetter: CFCharacterSetPredefinedSet = 13;
pub static kCFCharacterSetSymbol: CFCharacterSetPredefinedSet = 14;
pub static kCFCharacterSetNewline: CFCharacterSetPredefinedSet = 15;

#[repr(C)]
pub struct __CFCharacterSet(c_void);

pub type CFCharacterSetRef = *const __CFCharacterSet;
pub type CFMutableCharacterSetRef = *mut __CFCharacterSet;

extern "C" {
    /*
     * CFCharacterSet.h
     */

    /* CFCharacterSet */
    /* Creating Character Sets */
    pub fn CFCharacterSetCreateCopy(
        alloc: CFAllocatorRef,
        theSet: CFCharacterSetRef,
    ) -> CFCharacterSetRef;
    pub fn CFCharacterSetCreateInvertedSet(
        alloc: CFAllocatorRef,
        theSet: CFCharacterSetRef,
    ) -> CFCharacterSetRef;
    pub fn CFCharacterSetCreateWithCharactersInRange(
        alloc: CFAllocatorRef,
        theRange: CFRange,
    ) -> CFCharacterSetRef;
    pub fn CFCharacterSetCreateWithCharactersInString(
        alloc: CFAllocatorRef,
        theString: CFStringRef,
    ) -> CFCharacterSetRef;
    pub fn CFCharacterSetCreateWithBitmapRepresentation(
        alloc: CFAllocatorRef,
        theData: CFDataRef,
    ) -> CFCharacterSetRef;

    /* Getting Predefined Character Sets */
    pub fn CFCharacterSetGetPredefined(
        theSetIdentifier: CFCharacterSetPredefinedSet,
    ) -> CFCharacterSetRef;

    /* Querying Character Sets */
    pub fn CFCharacterSetCreateBitmapRepresentation(
        alloc: CFAllocatorRef,
        theSet: CFCharacterSetRef,
    ) -> CFDataRef;
    pub fn CFCharacterSetHasMemberInPlane(theSet: CFCharacterSetRef, thePlane: CFIndex) -> Boolean;
    pub fn CFCharacterSetIsCharacterMember(theSet: CFCharacterSetRef, theChar: UniChar) -> Boolean;
    pub fn CFCharacterSetIsLongCharacterMember(
        theSet: CFCharacterSetRef,
        theChar: UTF32Char,
    ) -> Boolean;
    pub fn CFCharacterSetIsSupersetOfSet(
        theSet: CFCharacterSetRef,
        theOtherset: CFCharacterSetRef,
    ) -> Boolean;

    /* Getting the Character Set Type Identifier */
    pub fn CFCharacterSetGetTypeID() -> CFTypeID;

    /* CFMutableCharacterSet */
    /* Creating a Mutable Character Set */
    pub fn CFCharacterSetCreateMutable(alloc: CFAllocatorRef) -> CFMutableCharacterSetRef;
    pub fn CFCharacterSetCreateMutableCopy(
        alloc: CFAllocatorRef,
        theSet: CFCharacterSetRef,
    ) -> CFMutableCharacterSetRef;

    /* Adding Characters */
    pub fn CFCharacterSetAddCharactersInRange(theSet: CFMutableCharacterSetRef, theRange: CFRange);
    pub fn CFCharacterSetAddCharactersInString(
        theSet: CFMutableCharacterSetRef,
        theString: CFStringRef,
    );

    /* Removing Characters */
    pub fn CFCharacterSetRemoveCharactersInRange(
        theSet: CFMutableCharacterSetRef,
        theRange: CFRange,
    );
    pub fn CFCharacterSetRemoveCharactersInString(
        theSet: CFMutableCharacterSetRef,
        theString: CFStringRef,
    );

    /* Logical Operations */
    pub fn CFCharacterSetIntersect(
        theSet: CFMutableCharacterSetRef,
        theOtherSet: CFCharacterSetRef,
    );
    pub fn CFCharacterSetInvert(theSet: CFMutableCharacterSetRef);
    pub fn CFCharacterSetUnion(theSet: CFMutableCharacterSetRef, theOtherSet: CFCharacterSetRef);
}
