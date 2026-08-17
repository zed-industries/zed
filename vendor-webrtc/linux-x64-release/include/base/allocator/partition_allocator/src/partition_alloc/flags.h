// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This header provides a type-safe way of storing OR-combinations of enum
// values.
//
// The traditional C++ approach for storing OR-combinations of enum values is to
// use an int or unsigned int variable. The inconvenience with this approach is
// that there's no type checking at all; any enum value can be OR'd with any
// other enum value and passed on to a function that takes an int or unsigned
// int.

#ifndef PARTITION_ALLOC_FLAGS_H_
#define PARTITION_ALLOC_FLAGS_H_

#include <type_traits>

namespace partition_alloc::internal {
// Returns `T` if and only if `EnumType` is a scoped enum.
template <typename EnumType, typename T = EnumType>
using IfEnum = std::enable_if_t<
    std::is_enum_v<EnumType> &&
        !std::is_convertible_v<EnumType, std::underlying_type_t<EnumType>>,
    T>;

// We assume `EnumType` defines `kMaxValue` which has the largest value and all
// powers of two are represented in `EnumType`.
template <typename EnumType>
constexpr inline EnumType kAllFlags = static_cast<IfEnum<EnumType>>(
    (static_cast<std::underlying_type_t<EnumType>>(EnumType::kMaxValue) << 1) -
    1);

template <typename EnumType>
constexpr inline IfEnum<EnumType, bool> AreValidFlags(EnumType flags) {
  const auto raw_flags = static_cast<std::underlying_type_t<EnumType>>(flags);
  const auto raw_all_flags =
      static_cast<std::underlying_type_t<EnumType>>(kAllFlags<EnumType>);
  return (raw_flags & ~raw_all_flags) == 0;
}

// Checks `subset` is a subset of `superset` or not.
template <typename EnumType>
constexpr inline IfEnum<EnumType, bool> ContainsFlags(EnumType superset,
                                                      EnumType subset) {
  return (superset & subset) == subset;
}

// A macro to define binary arithmetic over `EnumType`.
// Use inside `namespace partition_alloc::internal`.
#define PA_DEFINE_OPERATORS_FOR_FLAGS(EnumType)                              \
  [[maybe_unused]] [[nodiscard]] inline constexpr EnumType operator&(        \
      const EnumType& lhs, const EnumType& rhs) {                            \
    return static_cast<EnumType>(                                            \
        static_cast<std::underlying_type_t<EnumType>>(lhs) &                 \
        static_cast<std::underlying_type_t<EnumType>>(rhs));                 \
  }                                                                          \
  [[maybe_unused]] inline constexpr EnumType& operator&=(                    \
      EnumType& lhs, const EnumType& rhs) {                                  \
    lhs = lhs & rhs;                                                         \
    return lhs;                                                              \
  }                                                                          \
  [[maybe_unused]] [[nodiscard]] inline constexpr EnumType operator|(        \
      const EnumType& lhs, const EnumType& rhs) {                            \
    return static_cast<EnumType>(                                            \
        static_cast<std::underlying_type_t<EnumType>>(lhs) |                 \
        static_cast<std::underlying_type_t<EnumType>>(rhs));                 \
  }                                                                          \
  [[maybe_unused]] inline constexpr EnumType& operator|=(                    \
      EnumType& lhs, const EnumType& rhs) {                                  \
    lhs = lhs | rhs;                                                         \
    return lhs;                                                              \
  }                                                                          \
  [[maybe_unused]] [[nodiscard]] inline constexpr EnumType operator^(        \
      const EnumType& lhs, const EnumType& rhs) {                            \
    return static_cast<EnumType>(                                            \
        static_cast<std::underlying_type_t<EnumType>>(lhs) ^                 \
        static_cast<std::underlying_type_t<EnumType>>(rhs));                 \
  }                                                                          \
  [[maybe_unused]] inline constexpr EnumType& operator^=(                    \
      EnumType& lhs, const EnumType& rhs) {                                  \
    lhs = lhs ^ rhs;                                                         \
    return lhs;                                                              \
  }                                                                          \
  [[maybe_unused]] [[nodiscard]] inline constexpr EnumType operator~(        \
      const EnumType& val) {                                                 \
    return static_cast<EnumType>(                                            \
        static_cast<std::underlying_type_t<EnumType>>(kAllFlags<EnumType>) & \
        ~static_cast<std::underlying_type_t<EnumType>>(val));                \
  }                                                                          \
  static_assert(true) /* semicolon here */

}  // namespace partition_alloc::internal

#endif  // PARTITION_ALLOC_FLAGS_H_
