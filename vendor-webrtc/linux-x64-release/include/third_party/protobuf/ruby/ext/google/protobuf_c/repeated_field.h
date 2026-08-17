// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef RUBY_PROTOBUF_REPEATED_FIELD_H_
#define RUBY_PROTOBUF_REPEATED_FIELD_H_

#include "protobuf.h"
#include "ruby-upb.h"

// Returns a frozen sentinel Ruby wrapper object for an empty upb_Array of the
// type specified by the field. Creates one if it doesn't exist.
VALUE RepeatedField_EmptyFrozen(const upb_FieldDef* f);

// Returns a Ruby wrapper object for the given upb_Array, which will be created
// if one does not exist already.
VALUE RepeatedField_GetRubyWrapper(const upb_Array* msg, TypeInfo type_info,
                                   VALUE arena);

// Gets the underlying upb_Array for this Ruby RepeatedField object, which must
// have a type that matches |f|. If this is not a repeated field or the type
// doesn't match, raises an exception.
const upb_Array* RepeatedField_GetUpbArray(VALUE value, const upb_FieldDef* f,
                                           upb_Arena* arena);

// Implements #inspect for this repeated field by appending its contents to |b|.
void RepeatedField_Inspect(StringBuilder* b, const upb_Array* array,
                           TypeInfo info);

// Returns a deep copy of this RepeatedField object.
VALUE RepeatedField_deep_copy(VALUE obj);

// Ruby class of Google::Protobuf::RepeatedField.
extern VALUE cRepeatedField;

// Call at startup to register all types in this module.
void RepeatedField_register(VALUE module);

// Recursively freeze RepeatedField.
VALUE RepeatedField_freeze(VALUE _self);

#endif  // RUBY_PROTOBUF_REPEATED_FIELD_H_
