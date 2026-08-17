// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PARAMETER_PACK_H_
#define BASE_PARAMETER_PACK_H_

#include <stddef.h>

#include <initializer_list>
#include <tuple>
#include <type_traits>

#include "base/containers/contains.h"

namespace base {

// Checks if any of the elements in |ilist| is true.
inline constexpr bool any_of(std::initializer_list<bool> ilist) {
  return base::Contains(ilist, true);
}

// Checks if all of the elements in |ilist| are true.
inline constexpr bool all_of(std::initializer_list<bool> ilist) {
  return !base::Contains(ilist, false);
}

// Counts the elements in |ilist| that are equal to |value|.
// Similar to std::count for the case of constexpr initializer_list.
template <class T>
inline constexpr size_t count(std::initializer_list<T> ilist, T value) {
  size_t c = 0;
  for (const auto& v : ilist) {
    c += (v == value);
  }
  return c;
}

constexpr size_t pack_npos = static_cast<size_t>(-1);

template <typename... Ts>
struct ParameterPack {
  // Checks if |Type| occurs in the parameter pack.
  template <typename Type>
  using HasType = std::bool_constant<any_of({std::is_same_v<Type, Ts>...})>;

  // Checks if the parameter pack only contains |Type|.
  template <typename Type>
  using OnlyHasType = std::bool_constant<all_of({std::is_same_v<Type, Ts>...})>;

  // Checks if |Type| occurs only once in the parameter pack.
  template <typename Type>
  using IsUniqueInPack =
      std::bool_constant<count({std::is_same_v<Type, Ts>...}, true) == 1>;

  // Returns the zero-based index of |Type| within |Pack...| or |pack_npos| if
  // it's not within the pack.
  template <typename Type>
  static constexpr size_t IndexInPack() {
    size_t index = 0;
    for (bool value : {std::is_same_v<Type, Ts>...}) {
      if (value) {
        return index;
      }
      index++;
    }
    return pack_npos;
  }

  // Helper for extracting the Nth type from a parameter pack.
  template <size_t N>
  using NthType = std::tuple_element_t<N, std::tuple<Ts...>>;

  // Checks if every type in the parameter pack is the same.
  using IsAllSameType =
      std::bool_constant<all_of({std::is_same_v<NthType<0>, Ts>...})>;
};

}  // namespace base

#endif  // BASE_PARAMETER_PACK_H_
