// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TYPES_CXX23_TO_UNDERLYING_H_
#define BASE_TYPES_CXX23_TO_UNDERLYING_H_

#include <type_traits>

namespace base {

// Implementation of C++23's std::to_underlying.
//
// Reference: https://en.cppreference.com/w/cpp/utility/to_underlying
template <typename EnumT>
  requires(std::is_enum_v<EnumT>)
constexpr std::underlying_type_t<EnumT> to_underlying(EnumT e) noexcept {
  return static_cast<std::underlying_type_t<EnumT>>(e);
}

}  // namespace base

#endif  // BASE_TYPES_CXX23_TO_UNDERLYING_H_
