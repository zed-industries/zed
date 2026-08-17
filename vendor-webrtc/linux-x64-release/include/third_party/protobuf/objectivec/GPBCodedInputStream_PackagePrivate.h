// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// This header is private to the ProtobolBuffers library and must NOT be
// included by any sources outside this library. The contents of this file are
// subject to change at any time without notice.

#import "GPBCodedInputStream.h"

#import "GPBDescriptor.h"

typedef struct GPBCodedInputStreamState {
  const uint8_t *bytes;
  size_t bufferSize;
  size_t bufferPos;

  // For parsing subsections of an input stream you can put a hard limit on
  // how much should be read. Normally the limit is the end of the stream,
  // but you can adjust it to anywhere, and if you hit it you will be at the
  // end of the stream, until you adjust the limit.
  size_t currentLimit;
  int32_t lastTag;
  NSUInteger recursionDepth;
} GPBCodedInputStreamState;

@interface GPBCodedInputStream () {
 @package
  struct GPBCodedInputStreamState state_;
  NSData *buffer_;
}

// Group support is deprecated, so we hide this interface from users, but
// support for older data.
- (void)readGroup:(int32_t)fieldNumber
              message:(GPBMessage *)message
    extensionRegistry:(id<GPBExtensionRegistry>)extensionRegistry;

// Reads a map entry.
- (void)readMapEntry:(id)mapDictionary
    extensionRegistry:(id<GPBExtensionRegistry>)extensionRegistry
                field:(GPBFieldDescriptor *)field
        parentMessage:(GPBMessage *)parentMessage;
@end

CF_EXTERN_C_BEGIN

void GPBRaiseStreamError(NSInteger code, NSString *reason);
int32_t GPBCodedInputStreamReadTag(GPBCodedInputStreamState *state);

double GPBCodedInputStreamReadDouble(GPBCodedInputStreamState *state);
float GPBCodedInputStreamReadFloat(GPBCodedInputStreamState *state);
uint64_t GPBCodedInputStreamReadUInt64(GPBCodedInputStreamState *state);
uint32_t GPBCodedInputStreamReadUInt32(GPBCodedInputStreamState *state);
int64_t GPBCodedInputStreamReadInt64(GPBCodedInputStreamState *state);
int32_t GPBCodedInputStreamReadInt32(GPBCodedInputStreamState *state);
uint64_t GPBCodedInputStreamReadFixed64(GPBCodedInputStreamState *state);
uint32_t GPBCodedInputStreamReadFixed32(GPBCodedInputStreamState *state);
int32_t GPBCodedInputStreamReadEnum(GPBCodedInputStreamState *state);
int32_t GPBCodedInputStreamReadSFixed32(GPBCodedInputStreamState *state);
int64_t GPBCodedInputStreamReadSFixed64(GPBCodedInputStreamState *state);
int32_t GPBCodedInputStreamReadSInt32(GPBCodedInputStreamState *state);
int64_t GPBCodedInputStreamReadSInt64(GPBCodedInputStreamState *state);
BOOL GPBCodedInputStreamReadBool(GPBCodedInputStreamState *state);
NSString *GPBCodedInputStreamReadRetainedString(GPBCodedInputStreamState *state)
    __attribute((ns_returns_retained));
NSData *GPBCodedInputStreamReadRetainedBytes(GPBCodedInputStreamState *state)
    __attribute((ns_returns_retained));
NSData *GPBCodedInputStreamReadRetainedBytesNoCopy(GPBCodedInputStreamState *state)
    __attribute((ns_returns_retained));
NSData *GPBCodedInputStreamReadRetainedBytesToEndGroupNoCopy(GPBCodedInputStreamState *state,
                                                             int32_t fieldNumber)
    __attribute((ns_returns_retained));

size_t GPBCodedInputStreamPushLimit(GPBCodedInputStreamState *state, size_t byteLimit);
void GPBCodedInputStreamPopLimit(GPBCodedInputStreamState *state, size_t oldLimit);
size_t GPBCodedInputStreamBytesUntilLimit(GPBCodedInputStreamState *state);
BOOL GPBCodedInputStreamIsAtEnd(GPBCodedInputStreamState *state);
void GPBCodedInputStreamCheckLastTagWas(GPBCodedInputStreamState *state, int32_t value);

CF_EXTERN_C_END
