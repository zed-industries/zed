// Protocol Buffers - Google's data interchange format
// Copyright 2015 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#import "GPBArray.h"

#import "GPBMessage.h"

//%PDDM-DEFINE DECLARE_ARRAY_EXTRAS()
//%ARRAY_INTERFACE_EXTRAS(Int32, int32_t)
//%ARRAY_INTERFACE_EXTRAS(UInt32, uint32_t)
//%ARRAY_INTERFACE_EXTRAS(Int64, int64_t)
//%ARRAY_INTERFACE_EXTRAS(UInt64, uint64_t)
//%ARRAY_INTERFACE_EXTRAS(Float, float)
//%ARRAY_INTERFACE_EXTRAS(Double, double)
//%ARRAY_INTERFACE_EXTRAS(Bool, BOOL)
//%ARRAY_INTERFACE_EXTRAS(Enum, int32_t)

//%PDDM-DEFINE ARRAY_INTERFACE_EXTRAS(NAME, TYPE)
//%#pragma mark - NAME
//%
//%@interface GPB##NAME##Array () {
//% @package
//%  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
//%}
//%@end
//%

//%PDDM-EXPAND DECLARE_ARRAY_EXTRAS()
// This block of code is generated, do not edit it directly.

#pragma mark - Int32

@interface GPBInt32Array () {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

#pragma mark - UInt32

@interface GPBUInt32Array () {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

#pragma mark - Int64

@interface GPBInt64Array () {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

#pragma mark - UInt64

@interface GPBUInt64Array () {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

#pragma mark - Float

@interface GPBFloatArray () {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

#pragma mark - Double

@interface GPBDoubleArray () {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

#pragma mark - Bool

@interface GPBBoolArray () {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

#pragma mark - Enum

@interface GPBEnumArray () {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end

//%PDDM-EXPAND-END DECLARE_ARRAY_EXTRAS()

#pragma mark - NSArray Subclass

@interface GPBAutocreatedArray : NSMutableArray {
 @package
  GPB_UNSAFE_UNRETAINED GPBMessage *_autocreator;
}
@end
