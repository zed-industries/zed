// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#import "GPBRuntimeTypes.h"

CF_EXTERN_C_BEGIN

NS_ASSUME_NONNULL_BEGIN

typedef enum {
  GPBWireFormatVarint = 0,
  GPBWireFormatFixed64 = 1,
  GPBWireFormatLengthDelimited = 2,
  GPBWireFormatStartGroup = 3,
  GPBWireFormatEndGroup = 4,
  GPBWireFormatFixed32 = 5,
} GPBWireFormat;

enum {
  GPBWireFormatMessageSetItem = 1,
  GPBWireFormatMessageSetTypeId = 2,
  GPBWireFormatMessageSetMessage = 3
};

uint32_t GPBWireFormatMakeTag(uint32_t fieldNumber, GPBWireFormat wireType) __attribute__((const));
GPBWireFormat GPBWireFormatGetTagWireType(uint32_t tag) __attribute__((const));
uint32_t GPBWireFormatGetTagFieldNumber(uint32_t tag) __attribute__((const));
BOOL GPBWireFormatIsValidTag(uint32_t tag) __attribute__((const));

GPBWireFormat GPBWireFormatForType(GPBDataType dataType, BOOL isPacked) __attribute__((const));

#define GPBWireFormatMessageSetItemTag \
  (GPBWireFormatMakeTag(GPBWireFormatMessageSetItem, GPBWireFormatStartGroup))
#define GPBWireFormatMessageSetItemEndTag \
  (GPBWireFormatMakeTag(GPBWireFormatMessageSetItem, GPBWireFormatEndGroup))
#define GPBWireFormatMessageSetTypeIdTag \
  (GPBWireFormatMakeTag(GPBWireFormatMessageSetTypeId, GPBWireFormatVarint))
#define GPBWireFormatMessageSetMessageTag \
  (GPBWireFormatMakeTag(GPBWireFormatMessageSetMessage, GPBWireFormatLengthDelimited))

NS_ASSUME_NONNULL_END

CF_EXTERN_C_END
