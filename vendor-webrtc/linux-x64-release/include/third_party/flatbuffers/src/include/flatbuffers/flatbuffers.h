/*
 * Copyright 2014 Google Inc. All rights reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef FLATBUFFERS_H_
#define FLATBUFFERS_H_

#include <algorithm>

// TODO: These includes are for mitigating the pains of users editing their
// source because they relied on flatbuffers.h to include everything for them.
#include "flatbuffers/array.h"
#include "flatbuffers/base.h"
#include "flatbuffers/buffer.h"
#include "flatbuffers/buffer_ref.h"
#include "flatbuffers/detached_buffer.h"
#include "flatbuffers/flatbuffer_builder.h"
#include "flatbuffers/stl_emulation.h"
#include "flatbuffers/string.h"
#include "flatbuffers/struct.h"
#include "flatbuffers/table.h"
#include "flatbuffers/vector.h"
#include "flatbuffers/vector_downward.h"
#include "flatbuffers/verifier.h"

namespace flatbuffers {

/// @brief This can compute the start of a FlatBuffer from a root pointer, i.e.
/// it is the opposite transformation of GetRoot().
/// This may be useful if you want to pass on a root and have the recipient
/// delete the buffer afterwards.
inline const uint8_t *GetBufferStartFromRootPointer(const void *root) {
  auto table = reinterpret_cast<const Table *>(root);
  auto vtable = table->GetVTable();
  // Either the vtable is before the root or after the root.
  auto start = (std::min)(vtable, reinterpret_cast<const uint8_t *>(root));
  // Align to at least sizeof(uoffset_t).
  start = reinterpret_cast<const uint8_t *>(reinterpret_cast<uintptr_t>(start) &
                                            ~(sizeof(uoffset_t) - 1));
  // Additionally, there may be a file_identifier in the buffer, and the root
  // offset. The buffer may have been aligned to any size between
  // sizeof(uoffset_t) and FLATBUFFERS_MAX_ALIGNMENT (see "force_align").
  // Sadly, the exact alignment is only known when constructing the buffer,
  // since it depends on the presence of values with said alignment properties.
  // So instead, we simply look at the next uoffset_t values (root,
  // file_identifier, and alignment padding) to see which points to the root.
  // None of the other values can "impersonate" the root since they will either
  // be 0 or four ASCII characters.
  static_assert(flatbuffers::kFileIdentifierLength == sizeof(uoffset_t),
                "file_identifier is assumed to be the same size as uoffset_t");
  for (auto possible_roots = FLATBUFFERS_MAX_ALIGNMENT / sizeof(uoffset_t) + 1;
       possible_roots; possible_roots--) {
    start -= sizeof(uoffset_t);
    if (ReadScalar<uoffset_t>(start) + start ==
        reinterpret_cast<const uint8_t *>(root))
      return start;
  }
  // We didn't find the root, either the "root" passed isn't really a root,
  // or the buffer is corrupt.
  // Assert, because calling this function with bad data may cause reads
  // outside of buffer boundaries.
  FLATBUFFERS_ASSERT(false);
  return nullptr;
}

/// @brief This return the prefixed size of a FlatBuffer.
template<typename SizeT = uoffset_t>
inline SizeT GetPrefixedSize(const uint8_t *buf) {
  return ReadScalar<SizeT>(buf);
}

// Gets the total length of the buffer given a sized prefixed FlatBuffer.
//
// This includes the size of the prefix as well as the buffer:
//
//  [size prefix][flatbuffer]
//  |---------length--------|
template<typename SizeT = uoffset_t>
inline SizeT GetSizePrefixedBufferLength(const uint8_t *const buf) {
  return ReadScalar<SizeT>(buf) + sizeof(SizeT);
}

// Base class for native objects (FlatBuffer data de-serialized into native
// C++ data structures).
// Contains no functionality, purely documentative.
struct NativeTable {};

/// @brief Function types to be used with resolving hashes into objects and
/// back again. The resolver gets a pointer to a field inside an object API
/// object that is of the type specified in the schema using the attribute
/// `cpp_type` (it is thus important whatever you write to this address
/// matches that type). The value of this field is initially null, so you
/// may choose to implement a delayed binding lookup using this function
/// if you wish. The resolver does the opposite lookup, for when the object
/// is being serialized again.
typedef uint64_t hash_value_t;
typedef std::function<void(void **pointer_adr, hash_value_t hash)>
    resolver_function_t;
typedef std::function<hash_value_t(void *pointer)> rehasher_function_t;

// Helper function to test if a field is present, using any of the field
// enums in the generated code.
// `table` must be a generated table type. Since this is a template parameter,
// this is not typechecked to be a subclass of Table, so beware!
// Note: this function will return false for fields equal to the default
// value, since they're not stored in the buffer (unless force_defaults was
// used).
template<typename T>
bool IsFieldPresent(const T *table, typename T::FlatBuffersVTableOffset field) {
  // Cast, since Table is a private baseclass of any table types.
  return reinterpret_cast<const Table *>(table)->CheckField(
      static_cast<voffset_t>(field));
}

// Utility function for reverse lookups on the EnumNames*() functions
// (in the generated C++ code)
// names must be NULL terminated.
inline int LookupEnum(const char **names, const char *name) {
  for (const char **p = names; *p; p++)
    if (!strcmp(*p, name)) return static_cast<int>(p - names);
  return -1;
}

// These macros allow us to layout a struct with a guarantee that they'll end
// up looking the same on different compilers and platforms.
// It does this by disallowing the compiler to do any padding, and then
// does padding itself by inserting extra padding fields that make every
// element aligned to its own size.
// Additionally, it manually sets the alignment of the struct as a whole,
// which is typically its largest element, or a custom size set in the schema
// by the force_align attribute.
// These are used in the generated code only.

// clang-format off
#if defined(_MSC_VER)
  #define FLATBUFFERS_MANUALLY_ALIGNED_STRUCT(alignment) \
    __pragma(pack(1)) \
    struct __declspec(align(alignment))
  #define FLATBUFFERS_STRUCT_END(name, size) \
    __pragma(pack()) \
    static_assert(sizeof(name) == size, "compiler breaks packing rules")
#elif defined(__GNUC__) || defined(__clang__) || defined(__ICCARM__)
  #define FLATBUFFERS_MANUALLY_ALIGNED_STRUCT(alignment) \
    _Pragma("pack(1)") \
    struct __attribute__((aligned(alignment)))
  #define FLATBUFFERS_STRUCT_END(name, size) \
    _Pragma("pack()") \
    static_assert(sizeof(name) == size, "compiler breaks packing rules")
#else
  #error Unknown compiler, please define structure alignment macros
#endif
// clang-format on

// Minimal reflection via code generation.
// Besides full-fat reflection (see reflection.h) and parsing/printing by
// loading schemas (see idl.h), we can also have code generation for minimal
// reflection data which allows pretty-printing and other uses without needing
// a schema or a parser.
// Generate code with --reflect-types (types only) or --reflect-names (names
// also) to enable.
// See minireflect.h for utilities using this functionality.

// These types are organized slightly differently as the ones in idl.h.
enum SequenceType { ST_TABLE, ST_STRUCT, ST_UNION, ST_ENUM };

// Scalars have the same order as in idl.h
// clang-format off
#define FLATBUFFERS_GEN_ELEMENTARY_TYPES(ET) \
  ET(ET_UTYPE) \
  ET(ET_BOOL) \
  ET(ET_CHAR) \
  ET(ET_UCHAR) \
  ET(ET_SHORT) \
  ET(ET_USHORT) \
  ET(ET_INT) \
  ET(ET_UINT) \
  ET(ET_LONG) \
  ET(ET_ULONG) \
  ET(ET_FLOAT) \
  ET(ET_DOUBLE) \
  ET(ET_STRING) \
  ET(ET_SEQUENCE)  // See SequenceType.

enum ElementaryType {
  #define FLATBUFFERS_ET(E) E,
    FLATBUFFERS_GEN_ELEMENTARY_TYPES(FLATBUFFERS_ET)
  #undef FLATBUFFERS_ET
};

inline const char * const *ElementaryTypeNames() {
  static const char * const names[] = {
    #define FLATBUFFERS_ET(E) #E,
      FLATBUFFERS_GEN_ELEMENTARY_TYPES(FLATBUFFERS_ET)
    #undef FLATBUFFERS_ET
  };
  return names;
}
// clang-format on

// Basic type info cost just 16bits per field!
// We're explicitly defining the signedness since the signedness of integer
// bitfields is otherwise implementation-defined and causes warnings on older
// GCC compilers.
struct TypeCode {
  // ElementaryType
  unsigned short base_type : 4;
  // Either vector (in table) or array (in struct)
  unsigned short is_repeating : 1;
  // Index into type_refs below, or -1 for none.
  signed short sequence_ref : 11;
};

static_assert(sizeof(TypeCode) == 2, "TypeCode");

struct TypeTable;

// Signature of the static method present in each type.
typedef const TypeTable *(*TypeFunction)();

struct TypeTable {
  SequenceType st;
  size_t num_elems;  // of type_codes, values, names (but not type_refs).
  const TypeCode *type_codes;     // num_elems count
  const TypeFunction *type_refs;  // less than num_elems entries (see TypeCode).
  const int16_t *array_sizes;     // less than num_elems entries (see TypeCode).
  const int64_t *values;  // Only set for non-consecutive enum/union or structs.
  const char *const *names;  // Only set if compiled with --reflect-names.
};

// String which identifies the current version of FlatBuffers.
inline const char *flatbuffers_version_string() {
  return "FlatBuffers " FLATBUFFERS_STRING(FLATBUFFERS_VERSION_MAJOR) "."
      FLATBUFFERS_STRING(FLATBUFFERS_VERSION_MINOR) "."
      FLATBUFFERS_STRING(FLATBUFFERS_VERSION_REVISION);
}

// clang-format off
#define FLATBUFFERS_DEFINE_BITMASK_OPERATORS(E, T)\
    inline FLATBUFFERS_CONSTEXPR_CPP11 E operator | (E lhs, E rhs){\
        return E(T(lhs) | T(rhs));\
    }\
    inline FLATBUFFERS_CONSTEXPR_CPP11 E operator & (E lhs, E rhs){\
        return E(T(lhs) & T(rhs));\
    }\
    inline FLATBUFFERS_CONSTEXPR_CPP11 E operator ^ (E lhs, E rhs){\
        return E(T(lhs) ^ T(rhs));\
    }\
    inline FLATBUFFERS_CONSTEXPR_CPP11 E operator ~ (E lhs){\
        return E(~T(lhs));\
    }\
    inline FLATBUFFERS_CONSTEXPR_CPP11 E operator |= (E &lhs, E rhs){\
        lhs = lhs | rhs;\
        return lhs;\
    }\
    inline FLATBUFFERS_CONSTEXPR_CPP11 E operator &= (E &lhs, E rhs){\
        lhs = lhs & rhs;\
        return lhs;\
    }\
    inline FLATBUFFERS_CONSTEXPR_CPP11 E operator ^= (E &lhs, E rhs){\
        lhs = lhs ^ rhs;\
        return lhs;\
    }\
    inline FLATBUFFERS_CONSTEXPR_CPP11 bool operator !(E rhs) \
    {\
        return !bool(T(rhs)); \
    }
/// @endcond
}  // namespace flatbuffers

// clang-format on

#endif  // FLATBUFFERS_H_
