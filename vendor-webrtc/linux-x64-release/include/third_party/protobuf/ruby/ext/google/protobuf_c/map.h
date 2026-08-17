// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef RUBY_PROTOBUF_MAP_H_
#define RUBY_PROTOBUF_MAP_H_

#include "protobuf.h"
#include "ruby-upb.h"

// Returns a frozen sentinel Ruby wrapper object for an empty upb_Map with the
// key and value types specified by the field. Creates one if it doesn't exist.
VALUE Map_EmptyFrozen(const upb_FieldDef* f);

// Returns a Ruby wrapper object for the given map, which will be created if
// one does not exist already.
VALUE Map_GetRubyWrapper(const upb_Map *map, upb_CType key_type,
                         TypeInfo value_type, VALUE arena);

// Gets the underlying upb_Map for this Ruby map object, which must have
// key/value type that match |field|. If this is not a map or the type doesn't
// match, raises an exception.
const upb_Map *Map_GetUpbMap(VALUE val, const upb_FieldDef *field,
                             upb_Arena *arena);

// Implements #inspect for this map by appending its contents to |b|.
void Map_Inspect(StringBuilder *b, const upb_Map *map, upb_CType key_type,
                 TypeInfo val_type);

// Returns a new Hash object containing the contents of this Map.
VALUE Map_CreateHash(const upb_Map *map, upb_CType key_type, TypeInfo val_info);

// Returns a deep copy of this Map object.
VALUE Map_deep_copy(VALUE obj);

// Ruby class of Google::Protobuf::Map.
extern VALUE cMap;

// Call at startup to register all types in this module.
void Map_register(VALUE module);

// Recursively freeze map
VALUE Map_freeze(VALUE _self);

#endif  // RUBY_PROTOBUF_MAP_H_
