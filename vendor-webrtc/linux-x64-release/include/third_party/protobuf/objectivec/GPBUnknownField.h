// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#import <Foundation/Foundation.h>

#import "GPBArray.h"
#import "GPBUnknownFields.h"

@class GPBUnknownFields;

NS_ASSUME_NONNULL_BEGIN

typedef NS_ENUM(uint8_t, GPBUnknownFieldType) {
  GPBUnknownFieldTypeVarint,
  GPBUnknownFieldTypeFixed32,
  GPBUnknownFieldTypeFixed64,
  GPBUnknownFieldTypeLengthDelimited,  // Length prefixed
  GPBUnknownFieldTypeGroup,            // Tag delimited
};

/**
 * Store an unknown field. These are used in conjunction with GPBUnknownFields.
 **/
__attribute__((objc_subclassing_restricted))
@interface GPBUnknownField : NSObject<NSCopying>

/** The field number the data is stored under. */
@property(nonatomic, readonly, assign) int32_t number;

/** The type of the field. */
@property(nonatomic, readonly, assign) GPBUnknownFieldType type;

/**
 * Fetch the varint value.
 *
 * It is a programming error to call this when the `type` is not a varint.
 */
@property(nonatomic, readonly, assign) uint64_t varint;

/**
 * Fetch the fixed32 value.
 *
 * It is a programming error to call this when the `type` is not a fixed32.
 */
@property(nonatomic, readonly, assign) uint32_t fixed32;

/**
 * Fetch the fixed64 value.
 *
 * It is a programming error to call this when the `type` is not a fixed64.
 */
@property(nonatomic, readonly, assign) uint64_t fixed64;

/**
 * Fetch the length delimited (length prefixed) value.
 *
 * It is a programming error to call this when the `type` is not a length
 * delimited.
 */
@property(nonatomic, readonly, strong, nonnull) NSData *lengthDelimited;

/**
 * Fetch the group (tag delimited) value.
 *
 * It is a programming error to call this when the `type` is not a group.
 */
@property(nonatomic, readonly, strong, nonnull) GPBUnknownFields *group;

@end

NS_ASSUME_NONNULL_END
