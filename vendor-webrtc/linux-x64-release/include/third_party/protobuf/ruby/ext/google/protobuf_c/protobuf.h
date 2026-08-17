// Protocol Buffers - Google's data interchange format
// Copyright 2014 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef __GOOGLE_PROTOBUF_RUBY_PROTOBUF_H__
#define __GOOGLE_PROTOBUF_RUBY_PROTOBUF_H__

// Ruby 3+ defines NDEBUG itself, see: https://bugs.ruby-lang.org/issues/18777
#ifdef NDEBUG
#include <ruby.h>
#else
#include <ruby.h>
#undef NDEBUG
#endif

#include <ruby/version.h>

#if RUBY_API_VERSION_CODE < 20700
#error Protobuf requires Ruby >= 2.7
#endif

#include <assert.h>  // Must be included after the NDEBUG logic above.
#include <ruby/encoding.h>
#include <ruby/vm.h>

#include "defs.h"
#include "ruby-upb.h"

// These operate on a map field (i.e., a repeated field of submessages whose
// submessage type is a map-entry msgdef).
const upb_FieldDef* map_field_key(const upb_FieldDef* field);
const upb_FieldDef* map_field_value(const upb_FieldDef* field);

// -----------------------------------------------------------------------------
// Arena
// -----------------------------------------------------------------------------

// A Ruby object that wraps an underlying upb_Arena.  Any objects that are
// allocated from this arena should reference the Arena in rb_gc_mark(), to
// ensure that the object's underlying memory outlives any Ruby object that can
// reach it.

VALUE Arena_new();
upb_Arena* Arena_get(VALUE arena);

// Fuses this arena to another, throwing a Ruby exception if this is not
// possible.
void Arena_fuse(VALUE arena, upb_Arena* other);

// -----------------------------------------------------------------------------
// ObjectCache
// -----------------------------------------------------------------------------

// Global object cache from upb array/map/message/symtab to wrapper object.
//
// This is a conceptually "weak" cache, in that it does not prevent "val" from
// being collected (though in Ruby <2.7 is it effectively strong, due to
// implementation limitations).

// Tries to add a new entry to the cache, returning the newly installed value or
// the pre-existing entry.
VALUE ObjectCache_TryAdd(const void* key, VALUE val);

// Returns the cached object for this key, if any. Otherwise returns Qnil.
VALUE ObjectCache_Get(const void* key);

// -----------------------------------------------------------------------------
// StringBuilder, for inspect
// -----------------------------------------------------------------------------

struct StringBuilder;
typedef struct StringBuilder StringBuilder;

StringBuilder* StringBuilder_New();
void StringBuilder_Free(StringBuilder* b);
void StringBuilder_Printf(StringBuilder* b, const char* fmt, ...);
VALUE StringBuilder_ToRubyString(StringBuilder* b);

void StringBuilder_PrintMsgval(StringBuilder* b, upb_MessageValue val,
                               TypeInfo info);

// -----------------------------------------------------------------------------
// Utilities.
// -----------------------------------------------------------------------------

extern VALUE cTypeError;

#ifdef NDEBUG
#define PBRUBY_ASSERT(expr) \
  do {                      \
  } while (false && (expr))
#else
#define PBRUBY_ASSERT(expr) \
  if (!(expr))              \
  rb_bug("Assertion failed at %s:%d, expr: %s", __FILE__, __LINE__, #expr)
#endif

// Raises a Ruby error if val is frozen in Ruby or upb_frozen is true.
void Protobuf_CheckNotFrozen(VALUE val, bool upb_frozen);

#define PBRUBY_MAX(x, y) (((x) > (y)) ? (x) : (y))

#define UPB_UNUSED(var) (void)var

#endif  // __GOOGLE_PROTOBUF_RUBY_PROTOBUF_H__
