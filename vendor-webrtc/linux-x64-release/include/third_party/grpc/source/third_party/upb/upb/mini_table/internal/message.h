// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef UPB_MINI_TABLE_INTERNAL_MESSAGE_H_
#define UPB_MINI_TABLE_INTERNAL_MESSAGE_H_

#include <stddef.h>
#include <stdint.h>

#include "upb/base/descriptor_constants.h"
#include "upb/mini_table/internal/field.h"
#include "upb/mini_table/internal/sub.h"

// Must be last.
#include "upb/port/def.inc"

struct upb_Decoder;
struct upb_Message;
typedef const char* _upb_FieldParser(struct upb_Decoder* d, const char* ptr,
                                     struct upb_Message* msg, intptr_t table,
                                     uint64_t hasbits, uint64_t data);
typedef struct {
  uint64_t field_data;
  _upb_FieldParser* field_parser;
} _upb_FastTable_Entry;

typedef enum {
  kUpb_ExtMode_NonExtendable = 0,  // Non-extendable message.
  kUpb_ExtMode_Extendable = 1,     // Normal extendable message.
  kUpb_ExtMode_IsMessageSet = 2,   // MessageSet message.
  kUpb_ExtMode_IsMessageSet_ITEM =
      3,  // MessageSet item (temporary only, see decode.c)

  // During table building we steal a bit to indicate that the message is a map
  // entry.  *Only* used during table building!
  kUpb_ExtMode_IsMapEntry = 4,
} upb_ExtMode;

// upb_MiniTable represents the memory layout of a given upb_MessageDef.
// The members are public so generated code can initialize them,
// but users MUST NOT directly read or write any of its members.

// LINT.IfChange(minitable_struct_definition)
struct upb_MiniTable {
  const upb_MiniTableSubInternal* UPB_PRIVATE(subs);
  const struct upb_MiniTableField* UPB_ONLYBITS(fields);

  // Must be aligned to sizeof(void*). Doesn't include internal members like
  // unknown fields, extension dict, pointer to msglayout, etc.
  uint16_t UPB_PRIVATE(size);

  uint16_t UPB_ONLYBITS(field_count);

  uint8_t UPB_PRIVATE(ext);  // upb_ExtMode, uint8_t here so sizeof(ext) == 1
  uint8_t UPB_PRIVATE(dense_below);
  uint8_t UPB_PRIVATE(table_mask);
  uint8_t UPB_PRIVATE(required_count);  // Required fields have the low hasbits.

#ifdef UPB_TRACING_ENABLED
  const char* UPB_PRIVATE(full_name);
#endif

#ifdef UPB_FASTTABLE_ENABLED
  // To statically initialize the tables of variable length, we need a flexible
  // array member, and we need to compile in gnu99 mode (constant initialization
  // of flexible array members is a GNU extension, not in C99 unfortunately.
  _upb_FastTable_Entry UPB_PRIVATE(fasttable)[];
#endif
};
// LINT.ThenChange(//depot/google3/third_party/upb/bits/typescript/mini_table.ts)

#ifdef __cplusplus
extern "C" {
#endif

UPB_INLINE const struct upb_MiniTable* UPB_PRIVATE(
    _upb_MiniTable_StrongReference)(const struct upb_MiniTable* mt) {
#if defined(__GNUC__)
  __asm__("" : : "r"(mt));
#else
  const struct upb_MiniTable* volatile unused = mt;
  (void)&unused;  // Use address to avoid an extra load of "unused".
#endif
  return mt;
}

UPB_INLINE const struct upb_MiniTable* UPB_PRIVATE(_upb_MiniTable_Empty)(void) {
  extern const struct upb_MiniTable UPB_PRIVATE(_kUpb_MiniTable_Empty);

  return &UPB_PRIVATE(_kUpb_MiniTable_Empty);
}

UPB_API_INLINE int upb_MiniTable_FieldCount(const struct upb_MiniTable* m) {
  return m->UPB_ONLYBITS(field_count);
}

UPB_INLINE bool UPB_PRIVATE(_upb_MiniTable_IsEmpty)(
    const struct upb_MiniTable* m) {
  extern const struct upb_MiniTable UPB_PRIVATE(_kUpb_MiniTable_Empty);

  return m == &UPB_PRIVATE(_kUpb_MiniTable_Empty);
}

UPB_API_INLINE const struct upb_MiniTableField* upb_MiniTable_GetFieldByIndex(
    const struct upb_MiniTable* m, uint32_t i) {
  return &m->UPB_ONLYBITS(fields)[i];
}

UPB_INLINE const struct upb_MiniTable* UPB_PRIVATE(
    _upb_MiniTable_GetSubTableByIndex)(const struct upb_MiniTable* m,
                                       uint32_t i) {
  return *m->UPB_PRIVATE(subs)[i].UPB_PRIVATE(submsg);
}

UPB_API_INLINE const struct upb_MiniTable* upb_MiniTable_SubMessage(
    const struct upb_MiniTable* m, const struct upb_MiniTableField* f) {
  if (upb_MiniTableField_CType(f) != kUpb_CType_Message) {
    return NULL;
  }
  return UPB_PRIVATE(_upb_MiniTable_GetSubTableByIndex)(
      m, f->UPB_PRIVATE(submsg_index));
}

UPB_API_INLINE const struct upb_MiniTable* upb_MiniTable_GetSubMessageTable(
    const struct upb_MiniTable* m, const struct upb_MiniTableField* f) {
  UPB_ASSUME(upb_MiniTableField_CType(f) == kUpb_CType_Message);
  const struct upb_MiniTable* ret = upb_MiniTable_SubMessage(m, f);
  UPB_ASSUME(ret);
  return UPB_PRIVATE(_upb_MiniTable_IsEmpty)(ret) ? NULL : ret;
}

UPB_API_INLINE bool upb_MiniTable_FieldIsLinked(
    const struct upb_MiniTable* m, const struct upb_MiniTableField* f) {
  return upb_MiniTable_GetSubMessageTable(m, f) != NULL;
}

UPB_API_INLINE const struct upb_MiniTable* upb_MiniTable_MapEntrySubMessage(
    const struct upb_MiniTable* m, const struct upb_MiniTableField* f) {
  UPB_ASSERT(upb_MiniTable_FieldIsLinked(m, f));  // Map entries must be linked.
  UPB_ASSERT(upb_MiniTableField_IsMap(f));        // Function precondition.
  return upb_MiniTable_SubMessage(m, f);
}

UPB_API_INLINE const struct upb_MiniTableEnum* upb_MiniTable_GetSubEnumTable(
    const struct upb_MiniTable* m, const struct upb_MiniTableField* f) {
  UPB_ASSERT(upb_MiniTableField_CType(f) == kUpb_CType_Enum);
  return m->UPB_PRIVATE(subs)[f->UPB_PRIVATE(submsg_index)].UPB_PRIVATE(
      subenum);
}

UPB_API_INLINE const struct upb_MiniTableField* upb_MiniTable_MapKey(
    const struct upb_MiniTable* m) {
  UPB_ASSERT(upb_MiniTable_FieldCount(m) == 2);
  const struct upb_MiniTableField* f = upb_MiniTable_GetFieldByIndex(m, 0);
  UPB_ASSERT(upb_MiniTableField_Number(f) == 1);
  return f;
}

UPB_API_INLINE const struct upb_MiniTableField* upb_MiniTable_MapValue(
    const struct upb_MiniTable* m) {
  UPB_ASSERT(upb_MiniTable_FieldCount(m) == 2);
  const struct upb_MiniTableField* f = upb_MiniTable_GetFieldByIndex(m, 1);
  UPB_ASSERT(upb_MiniTableField_Number(f) == 2);
  return f;
}

// Computes a bitmask in which the |m->required_count| lowest bits are set.
//
// Sample output:
//    RequiredMask(1) => 0b1 (0x1)
//    RequiredMask(5) => 0b11111 (0x1f)
UPB_INLINE uint64_t
UPB_PRIVATE(_upb_MiniTable_RequiredMask)(const struct upb_MiniTable* m) {
  int n = m->UPB_PRIVATE(required_count);
  UPB_ASSERT(0 < n && n <= 64);
  return (1ULL << n) - 1;
}

#ifdef UPB_TRACING_ENABLED
UPB_INLINE const char* upb_MiniTable_FullName(
    const struct upb_MiniTable* mini_table) {
  return mini_table->UPB_PRIVATE(full_name);
}
// Initializes tracing proto name from language runtimes that construct
// mini tables dynamically at runtime. The runtime is responsible for passing
// controlling lifetime of name such as storing in same arena as mini_table.
UPB_INLINE void upb_MiniTable_SetFullName(struct upb_MiniTable* mini_table,
                                          const char* full_name) {
  mini_table->UPB_PRIVATE(full_name) = full_name;
}
#endif

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif /* UPB_MINI_TABLE_INTERNAL_MESSAGE_H_ */
