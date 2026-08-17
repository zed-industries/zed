// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef RUBY_PROTOBUF_CONVERT_H_
#define RUBY_PROTOBUF_CONVERT_H_

#include "protobuf.h"
#include "ruby-upb.h"

// Converts |ruby_val| to a upb_MessageValue according to |type_info|.
//
// The |arena| parameter indicates the lifetime of the container where this
// value will be assigned. It is used as follows:
// - If type is string or bytes, the string data will be copied into |arena|.
// - If type is message, and we need to auto-construct a message due to implicit
//   conversions (eg. Time -> Google::Protobuf::Timestamp), the new message
//   will be created in |arena|.
// - If type is message and the Ruby value is a message instance, we will fuse
//   the message's arena into |arena|, to ensure that this message outlives the
//   container.
upb_MessageValue Convert_RubyToUpb(VALUE ruby_val, const char *name,
                                   TypeInfo type_info, upb_Arena *arena);

// Converts |upb_val| to a Ruby VALUE according to |type_info|. This may involve
// creating a Ruby wrapper object.
//
// The |arena| parameter indicates the arena that owns the lifetime of
// |upb_val|. Any Ruby wrapper object that is created will reference |arena|
// and ensure it outlives the wrapper.
VALUE Convert_UpbToRuby(upb_MessageValue upb_val, TypeInfo type_info,
                        VALUE arena);

// Creates a deep copy of |msgval| in |arena|.
upb_MessageValue Msgval_DeepCopy(upb_MessageValue msgval, TypeInfo type_info,
                                 upb_Arena *arena);

// Returns true if |val1| and |val2| are equal. Their type is given by
// |type_info|.
bool Msgval_IsEqual(upb_MessageValue val1, upb_MessageValue val2,
                    TypeInfo type_info);

// Returns a hash value for the given upb_MessageValue.
uint64_t Msgval_GetHash(upb_MessageValue val, TypeInfo type_info,
                        uint64_t seed);

#endif  // RUBY_PROTOBUF_CONVERT_H_
