// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// This header is private to the ProtobolBuffers library and must NOT be
// included by any sources outside this library. The contents of this file are
// subject to change at any time without notice.

#import "GPBMessage.h"

#import "GPBBootstrap.h"

typedef struct GPBMessage_Storage {
  uint32_t _has_storage_[0];
} GPBMessage_Storage;

typedef struct GPBMessage_Storage *GPBMessage_StoragePtr;

@interface GPBMessage () {
 @package
  // NOTE: Because of the +allocWithZone code using NSAllocateObject(),
  // this structure should ideally always be kept pointer aligned where the
  // real storage starts is also pointer aligned. The compiler/runtime already
  // do this, but it may not be documented.

  // A pointer to the actual fields of the subclasses. The actual structure
  // pointed to by this pointer will depend on the subclass.
  // All of the actual structures will start the same as
  // GPBMessage_Storage with _has_storage__ as the first field.
  // Kept public because static functions need to access it.
  GPBMessage_StoragePtr messageStorage_;
}

// Gets an extension value without autocreating the result if not found. (i.e.
// returns nil if the extension is not set)
- (id)getExistingExtension:(GPBExtensionDescriptor *)extension;

// Parses a message of this type from the input and merges it with this
// message.
//
// `endingTag` should be zero if expected to consume to the end of input, but if
// the input is supposed to be a Group, it should be the endgroup tag to look for.
//
// Warning:  This does not verify that all required fields are present in
// the input message.
// NOTE: This will throw if there is an error while parsing.
- (void)mergeFromCodedInputStream:(GPBCodedInputStream *)input
                extensionRegistry:(id<GPBExtensionRegistry>)extensionRegistry
                        endingTag:(uint32_t)endingTag;

- (void)addUnknownMapEntry:(int32_t)fieldNum value:(NSData *)data;

@end

CF_EXTERN_C_BEGIN

// Returns whether |message| autocreated this message. This is NO if the message
// was not autocreated by |message| or if it has been mutated since
// autocreation.
BOOL GPBWasMessageAutocreatedBy(GPBMessage *message, GPBMessage *parent);

// Call this when you mutate a message. It will cause the message to become
// visible to its autocreator.
void GPBBecomeVisibleToAutocreator(GPBMessage *self);

// Call this when an array/dictionary is mutated so the parent message that
// autocreated it can react.
void GPBAutocreatedArrayModified(GPBMessage *self, id array);
void GPBAutocreatedDictionaryModified(GPBMessage *self, id dictionary);

// Clear the autocreator, if any. Asserts if the autocreator still has an
// autocreated reference to this message.
void GPBClearMessageAutocreator(GPBMessage *self);

// The data (or null) from the unknown fields of a message;
NSData *GPBMessageUnknownFieldsData(GPBMessage *self);

CF_EXTERN_C_END
