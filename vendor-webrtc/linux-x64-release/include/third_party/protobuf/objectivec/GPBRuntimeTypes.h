// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#import <Foundation/Foundation.h>

#import "GPBBootstrap.h"

@class GPBEnumDescriptor;
@class GPBMessage;

/**
 * Verifies that a given value can be represented by an enum type.
 * */
typedef BOOL (*GPBEnumValidationFunc)(int32_t);

/**
 * Magic value used at runtime to indicate an enum value that wasn't know at
 * compile time.
 * */
enum {
  kGPBUnrecognizedEnumeratorValue = (int32_t)0xFBADBEEF,
};

/**
 * A union for storing all possible Protobuf values. Note that owner is
 * responsible for memory management of object types.
 * */
typedef union {
  BOOL valueBool;
  int32_t valueInt32;
  int64_t valueInt64;
  uint32_t valueUInt32;
  uint64_t valueUInt64;
  float valueFloat;
  double valueDouble;
  GPB_UNSAFE_UNRETAINED NSData *valueData;
  GPB_UNSAFE_UNRETAINED NSString *valueString;
  GPB_UNSAFE_UNRETAINED GPBMessage *valueMessage;
  int32_t valueEnum;
} GPBGenericValue;

/**
 * Enum listing the possible data types that a field can contain.
 *
 * @note Do not change the order of this enum (or add things to it) without
 *       thinking about it very carefully. There are several things that depend
 *       on the order.
 * */
typedef NS_ENUM(uint8_t, GPBDataType) {
  /** Field contains boolean value(s). */
  GPBDataTypeBool = 0,
  /** Field contains unsigned 4 byte value(s). */
  GPBDataTypeFixed32,
  /** Field contains signed 4 byte value(s). */
  GPBDataTypeSFixed32,
  /** Field contains float value(s). */
  GPBDataTypeFloat,
  /** Field contains unsigned 8 byte value(s). */
  GPBDataTypeFixed64,
  /** Field contains signed 8 byte value(s). */
  GPBDataTypeSFixed64,
  /** Field contains double value(s). */
  GPBDataTypeDouble,
  /**
   * Field contains variable length value(s). Inefficient for encoding negative
   * numbers – if your field is likely to have negative values, use
   * GPBDataTypeSInt32 instead.
   **/
  GPBDataTypeInt32,
  /**
   * Field contains variable length value(s). Inefficient for encoding negative
   * numbers – if your field is likely to have negative values, use
   * GPBDataTypeSInt64 instead.
   **/
  GPBDataTypeInt64,
  /** Field contains signed variable length integer value(s). */
  GPBDataTypeSInt32,
  /** Field contains signed variable length integer value(s). */
  GPBDataTypeSInt64,
  /** Field contains unsigned variable length integer value(s). */
  GPBDataTypeUInt32,
  /** Field contains unsigned variable length integer value(s). */
  GPBDataTypeUInt64,
  /** Field contains an arbitrary sequence of bytes. */
  GPBDataTypeBytes,
  /** Field contains UTF-8 encoded or 7-bit ASCII text. */
  GPBDataTypeString,
  /** Field contains message type(s). */
  GPBDataTypeMessage,
  /** Field contains message type(s). */
  GPBDataTypeGroup,
  /** Field contains enum value(s). */
  GPBDataTypeEnum,
};

/** An extension range. */
typedef struct GPBExtensionRange {
  /** Inclusive. */
  uint32_t start;
  /** Exclusive. */
  uint32_t end;
} GPBExtensionRange;
