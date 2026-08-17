// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TYPES_ID_TYPE_H_
#define BASE_TYPES_ID_TYPE_H_

#include <algorithm>
#include <cstdint>
#include <functional>
#include <type_traits>

#include "base/types/strong_alias.h"

namespace base {

// A specialization of StrongAlias for integer-based identifiers.
//
// IdType32<>, IdType64<>, etc. wrap an integer id in a custom, type-safe type.
//
// IdType32<Foo> is an alternative to int, for a class Foo with methods like:
//
//    int GetId() { return id_; };
//    static Foo* FromId(int id) { return g_all_foos_by_id[id]; }
//
// Such methods are a standard means of safely referring to objects across
// thread and process boundaries.  But if a nearby class Bar also represents
// its IDs as a bare int, horrific mixups are possible -- one example, of many,
// is http://crrev.com/365437.  IdType<> offers compile-time protection against
// such mishaps, since IdType32<Foo> is incompatible with IdType32<Bar>, even
// though both just compile down to an int32_t.
//
// Templates in this file:
//   IdType32<T> / IdTypeU32<T>: Signed / unsigned 32-bit IDs
//   IdType64<T> / IdTypeU64<T>: Signed / unsigned 64-bit IDs
//   IdType<>: For when you need a different underlying type or
//             default/null values other than zero.
//
// IdType32<Foo> behaves just like an int32_t in the following aspects:
// - it can be used as a key in std::map;
// - it can be used as a key in std::unordered_map
// - it can be used as an argument to DCHECK_EQ or streamed to LOG(ERROR);
// - it has the same memory footprint and runtime overhead as int32_t;
// - it can be copied by memcpy.
// - it can be used in IPC messages.
//
// IdType32<Foo> has the following differences from a bare int32_t:
// - it forces coercions to go through the explicit constructor and value()
//   getter;
// - it restricts the set of available operations (e.g. no multiplication);
// - it default-constructs to a null value and allows checking against the null
//   value via is_null method.
// - optionally it may have additional values that are all considered null.
template <typename TypeMarker,
          typename WrappedType,
          WrappedType kInvalidValue,
          WrappedType kFirstGeneratedId = kInvalidValue + 1,
          WrappedType... kExtraInvalidValues>
class IdType : public StrongAlias<TypeMarker, WrappedType> {
 public:
  static constexpr WrappedType kAllInvalidValues[] = {kInvalidValue,
                                                      kExtraInvalidValues...};

  static_assert(std::is_unsigned_v<WrappedType> ||
                    std::ranges::all_of(kAllInvalidValues,
                                        [](WrappedType v) { return v <= 0; }),
                "If signed, invalid values should be negative or equal to zero "
                "to avoid overflow issues.");

  static_assert(std::ranges::all_of(kAllInvalidValues,
                                    [](WrappedType v) {
                                      return kFirstGeneratedId != v;
                                    }),
                "The first generated ID cannot be invalid.");

  static_assert(std::is_unsigned_v<WrappedType> ||
                    std::ranges::all_of(kAllInvalidValues,
                                        [](WrappedType v) {
                                          return kFirstGeneratedId > v;
                                        }),
                "If signed, the first generated ID must be greater than all "
                "invalid values so that the monotonically increasing "
                "GenerateNextId method will never return an invalid value.");

  using StrongAlias<TypeMarker, WrappedType>::StrongAlias;

  // This class can be used to generate unique IdTypes. It keeps an internal
  // counter that is continually increased by one every time an ID is generated.
  class Generator {
   public:
    Generator() = default;

    // Generates the next unique ID.
    IdType GenerateNextId() { return FromUnsafeValue(next_id_++); }

    // Non-copyable.
    Generator(const Generator&) = delete;
    Generator& operator=(const Generator&) = delete;

   private:
    WrappedType next_id_ = kFirstGeneratedId;
  };

  // Default-construct in the null state.
  constexpr IdType()
      : StrongAlias<TypeMarker, WrappedType>::StrongAlias(kInvalidValue) {}

  constexpr bool is_null() const {
    return std::ranges::any_of(kAllInvalidValues, [this](WrappedType value) {
      return this->value() == value;
    });
  }

  constexpr explicit operator bool() const { return !is_null(); }

  // TODO(mpawlowski) Replace these with constructor/value() getter. The
  // conversions are safe as long as they're explicit (which is taken care of by
  // StrongAlias).
  constexpr static IdType FromUnsafeValue(WrappedType value) {
    return IdType(value);
  }
  constexpr WrappedType GetUnsafeValue() const { return this->value(); }
};

// Type aliases for convenience:
template <typename TypeMarker>
using IdType32 = IdType<TypeMarker, std::int32_t, 0>;
template <typename TypeMarker>
using IdTypeU32 = IdType<TypeMarker, std::uint32_t, 0>;
template <typename TypeMarker>
using IdType64 = IdType<TypeMarker, std::int64_t, 0>;
template <typename TypeMarker>
using IdTypeU64 = IdType<TypeMarker, std::uint64_t, 0>;

}  // namespace base

template <typename TypeMarker,
          typename WrappedType,
          WrappedType kInvalidValue,
          WrappedType kFirstGeneratedId,
          WrappedType... kExtraInvalidValues>
struct std::hash<base::IdType<TypeMarker,
                              WrappedType,
                              kInvalidValue,
                              kFirstGeneratedId,
                              kExtraInvalidValues...>> {
  size_t operator()(const base::IdType<TypeMarker,
                                       WrappedType,
                                       kInvalidValue,
                                       kFirstGeneratedId,
                                       kExtraInvalidValues...>& id) const {
    return std::hash<WrappedType>()(id.value());
  }
};

#endif  // BASE_TYPES_ID_TYPE_H_
