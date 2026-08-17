// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TYPES_CXX23_IS_SCOPED_ENUM_H_
#define BASE_TYPES_CXX23_IS_SCOPED_ENUM_H_

#include <type_traits>

namespace base {

namespace internal {

// The indirection with std::is_enum<T> is required, because instantiating
// std::underlying_type_t<T> when T is not an enum is UB prior to C++20.
template <typename T, bool = std::is_enum_v<T>>
struct IsScopedEnumImpl : std::false_type {};

template <typename T>
struct IsScopedEnumImpl<T, /*std::is_enum_v<T>=*/true>
    : std::negation<std::is_convertible<T, std::underlying_type_t<T>>> {};

}  // namespace internal

// Implementation of C++23's std::is_scoped_enum
//
// Reference: https://en.cppreference.com/w/cpp/types/is_scoped_enum
template <typename T>
struct is_scoped_enum : internal::IsScopedEnumImpl<T> {};

}  // namespace base

#endif  // BASE_TYPES_CXX23_IS_SCOPED_ENUM_H_
