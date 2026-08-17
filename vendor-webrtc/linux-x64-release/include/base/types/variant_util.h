// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TYPES_VARIANT_UTIL_H_
#define BASE_TYPES_VARIANT_UTIL_H_

#include <stddef.h>

#include <type_traits>
#include <variant>

#include "base/types/always_false.h"

namespace base {
namespace internal {

template <typename Variant, typename T>
struct VariantIndexOfTypeHelper {
  static_assert(AlwaysFalse<Variant>, "Variant must be an std::variant<...>");
};

template <typename... Ts, typename T>
struct VariantIndexOfTypeHelper<std::variant<Ts...>, T> {
  static constexpr size_t Index() {
    static_assert(std::is_constructible_v<std::variant<LiteralType<Ts>...>,
                                          LiteralType<T>>,
                  "Variant is not constructible from T");
    return std::variant<LiteralType<Ts>...>(LiteralType<T>()).index();
  }

  // Helper struct; even if `Tag` may not be usable as a literal type, a
  // `LiteralType<Tag>` will be.
  template <typename Tag>
  struct LiteralType {};
};

}  // namespace internal

// Returns the 0-based index of `T` in `Variant`'s list of alternative types,
// e.g. given `Variant` == `std::variant<A, B, C>` and `T` == `B`, returns 1.
//
// Note that this helper cannot be used if the list of alternative types
// contains duplicates.
template <typename Variant, typename T>
constexpr size_t VariantIndexOfType() {
  return internal::VariantIndexOfTypeHelper<Variant, T>::Index();
}

}  // namespace base

#endif  // BASE_TYPES_VARIANT_UTIL_H_
