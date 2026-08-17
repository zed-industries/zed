// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_REFLECTION_DEF_BUILDER_INTERNAL_H_
#define UPB_REFLECTION_DEF_BUILDER_INTERNAL_H_

#include "upb/reflection/common.h"
#include "upb/reflection/def_type.h"
#include "upb/reflection/internal/def_pool.h"

// Must be last.
#include "upb/port/def.inc"

// We want to copy the options verbatim into the destination options proto.
// We use serialize+parse as our deep copy.
#define UPB_DEF_SET_OPTIONS(target, desc_type, options_type, proto)           \
  if (UPB_DESC(desc_type##_has_options)(proto)) {                             \
    size_t size;                                                              \
    char* pb = UPB_DESC(options_type##_serialize)(                            \
        UPB_DESC(desc_type##_options)(proto), ctx->tmp_arena, &size);         \
    if (!pb) _upb_DefBuilder_OomErr(ctx);                                     \
    target =                                                                  \
        UPB_DESC(options_type##_parse)(pb, size, _upb_DefBuilder_Arena(ctx)); \
    if (!target) _upb_DefBuilder_OomErr(ctx);                                 \
  } else {                                                                    \
    target = (const UPB_DESC(options_type)*)kUpbDefOptDefault;                \
  }

#ifdef __cplusplus
extern "C" {
#endif

struct upb_DefBuilder {
  upb_DefPool* symtab;
  upb_strtable feature_cache;             // Caches features by identity.
  UPB_DESC(FeatureSet*) legacy_features;  // For computing legacy features.
  char* tmp_buf;                          // Temporary buffer in tmp_arena.
  size_t tmp_buf_size;                    // Size of temporary buffer.
  upb_FileDef* file;                 // File we are building.
  upb_Arena* arena;                  // Allocate defs here.
  upb_Arena* tmp_arena;              // For temporary allocations.
  upb_Status* status;                // Record errors here.
  const upb_MiniTableFile* layout;   // NULL if we should build layouts.
  upb_MiniTablePlatform platform;    // Platform we are targeting.
  int enum_count;                    // Count of enums built so far.
  int msg_count;                     // Count of messages built so far.
  int ext_count;                     // Count of extensions built so far.
  jmp_buf err;                       // longjmp() on error.
};

extern const char* kUpbDefOptDefault;

// ctx->status has already been set elsewhere so just fail/longjmp()
UPB_NORETURN void _upb_DefBuilder_FailJmp(upb_DefBuilder* ctx);

UPB_NORETURN void _upb_DefBuilder_Errf(upb_DefBuilder* ctx, const char* fmt,
                                       ...) UPB_PRINTF(2, 3);
UPB_NORETURN void _upb_DefBuilder_OomErr(upb_DefBuilder* ctx);

const char* _upb_DefBuilder_MakeFullName(upb_DefBuilder* ctx,
                                         const char* prefix,
                                         upb_StringView name);

// Given a symbol and the base symbol inside which it is defined,
// find the symbol's definition.
const void* _upb_DefBuilder_ResolveAny(upb_DefBuilder* ctx,
                                       const char* from_name_dbg,
                                       const char* base, upb_StringView sym,
                                       upb_deftype_t* type);

const void* _upb_DefBuilder_Resolve(upb_DefBuilder* ctx,
                                    const char* from_name_dbg, const char* base,
                                    upb_StringView sym, upb_deftype_t type);

char _upb_DefBuilder_ParseEscape(upb_DefBuilder* ctx, const upb_FieldDef* f,
                                 const char** src, const char* end);

const char* _upb_DefBuilder_FullToShort(const char* fullname);

UPB_INLINE void* _upb_DefBuilder_Alloc(upb_DefBuilder* ctx, size_t bytes) {
  if (bytes == 0) return NULL;
  void* ret = upb_Arena_Malloc(ctx->arena, bytes);
  if (!ret) _upb_DefBuilder_OomErr(ctx);
  return ret;
}

// Adds a symbol |v| to the symtab, which must be a def pointer previously
// packed with pack_def(). The def's pointer to upb_FileDef* must be set before
// adding, so we know which entries to remove if building this file fails.
UPB_INLINE void _upb_DefBuilder_Add(upb_DefBuilder* ctx, const char* name,
                                    upb_value v) {
  upb_StringView sym = {.data = name, .size = strlen(name)};
  bool ok = _upb_DefPool_InsertSym(ctx->symtab, sym, v, ctx->status);
  if (!ok) _upb_DefBuilder_FailJmp(ctx);
}

UPB_INLINE upb_Arena* _upb_DefBuilder_Arena(const upb_DefBuilder* ctx) {
  return ctx->arena;
}

UPB_INLINE upb_FileDef* _upb_DefBuilder_File(const upb_DefBuilder* ctx) {
  return ctx->file;
}

// This version of CheckIdent() is only called by other, faster versions after
// they detect a parsing error.
void _upb_DefBuilder_CheckIdentSlow(upb_DefBuilder* ctx, upb_StringView name,
                                    bool full);

// Verify a full identifier string. This is slightly more complicated than
// verifying a relative identifier string because we must track '.' chars.
UPB_INLINE void _upb_DefBuilder_CheckIdentFull(upb_DefBuilder* ctx,
                                               upb_StringView name) {
  bool good = name.size > 0;
  bool start = true;

  for (size_t i = 0; i < name.size; i++) {
    const char c = name.data[i];
    const char d = c | 0x20;  // force lowercase
    const bool is_alpha = (('a' <= d) & (d <= 'z')) | (c == '_');
    const bool is_numer = ('0' <= c) & (c <= '9') & !start;
    const bool is_dot = (c == '.') & !start;

    good &= is_alpha | is_numer | is_dot;
    start = is_dot;
  }

  if (!good) _upb_DefBuilder_CheckIdentSlow(ctx, name, true);
}

// Returns true if the returned feature set is new and must be populated.
bool _upb_DefBuilder_GetOrCreateFeatureSet(upb_DefBuilder* ctx,
                                           const UPB_DESC(FeatureSet*) parent,
                                           upb_StringView key,
                                           UPB_DESC(FeatureSet**) set);

const UPB_DESC(FeatureSet*)
    _upb_DefBuilder_DoResolveFeatures(upb_DefBuilder* ctx,
                                      const UPB_DESC(FeatureSet*) parent,
                                      const UPB_DESC(FeatureSet*) child,
                                      bool is_implicit);

UPB_INLINE const UPB_DESC(FeatureSet*)
    _upb_DefBuilder_ResolveFeatures(upb_DefBuilder* ctx,
                                    const UPB_DESC(FeatureSet*) parent,
                                    const UPB_DESC(FeatureSet*) child) {
  return _upb_DefBuilder_DoResolveFeatures(ctx, parent, child, false);
}

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_REFLECTION_DEF_BUILDER_INTERNAL_H_ */
