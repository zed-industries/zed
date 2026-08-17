// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef RUBY_PROTOBUF_MESSAGE_H_
#define RUBY_PROTOBUF_MESSAGE_H_

#include "protobuf.h"
#include "ruby-upb.h"

// Gets the underlying upb_Message* and upb_MessageDef for the given Ruby
// message wrapper. Requires that |value| is indeed a message object.
const upb_Message* Message_Get(VALUE value, const upb_MessageDef** m);

// Like Message_Get(), but checks that the object is not frozen and returns a
// mutable pointer.
upb_Message* Message_GetMutable(VALUE value, const upb_MessageDef** m);

// Returns the Arena object for this message.
VALUE Message_GetArena(VALUE value);

// Converts |value| into a upb_Message value of the expected upb_MessageDef
// type, raising an error if this is not possible. Used when assigning |value|
// to a field of another message, which means the message must be of a
// particular type.
//
// This will perform automatic conversions in some cases (for example, Time ->
// Google::Protobuf::Timestamp). If any new message is created, it will be
// created on |arena|, and any existing message will have its arena fused with
// |arena|.
const upb_Message* Message_GetUpbMessage(VALUE value, const upb_MessageDef* m,
                                         const char* name, upb_Arena* arena);

// Gets or constructs a Ruby wrapper object for the given message. The wrapper
// object will reference |arena| and ensure that it outlives this object.
VALUE Message_GetRubyWrapper(const upb_Message* msg, const upb_MessageDef* m,
                             VALUE arena);

// Gets the given field from this message.
VALUE Message_getfield(VALUE _self, const upb_FieldDef* f);

// Implements #inspect for this message, printing the text to |b|.
void Message_PrintMessage(StringBuilder* b, const upb_Message* msg,
                          const upb_MessageDef* m);

// Returns a hash value for the given message.
uint64_t Message_Hash(const upb_Message* msg, const upb_MessageDef* m,
                      uint64_t seed);

// Returns a deep copy of the given message.
upb_Message* Message_deep_copy(const upb_Message* msg, const upb_MessageDef* m,
                               upb_Arena* arena);

// Checks that this Ruby object is a message, and raises an exception if not.
void Message_CheckClass(VALUE klass);

// Returns a new Hash object containing the contents of this message.
VALUE Scalar_CreateHash(upb_MessageValue val, TypeInfo type_info);

// Creates a message class or enum module for this descriptor, respectively.
VALUE build_class_from_descriptor(VALUE descriptor);
VALUE build_module_from_enumdesc(VALUE _enumdesc);

// Returns the Descriptor/EnumDescriptor for the given message class or enum
// module, respectively. Returns nil if this is not a message class or enum
// module.
VALUE MessageOrEnum_GetDescriptor(VALUE klass);

// Decodes a Message from a byte sequence.
VALUE Message_decode_bytes(int size, const char* bytes, int options,
                           VALUE klass, bool freeze);

// Recursively freeze message
VALUE Message_freeze(VALUE _self);

// Call at startup to register all types in this module.
void Message_register(VALUE protobuf);

#endif  // RUBY_PROTOBUF_MESSAGE_H_
