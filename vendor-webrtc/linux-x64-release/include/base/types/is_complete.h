// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TYPES_IS_COMPLETE_H_
#define BASE_TYPES_IS_COMPLETE_H_

#include <type_traits>

namespace base {

// True if `T` is completely defined.
template <typename T>
concept IsComplete = requires { sizeof(T); } ||
                     // Function types must be included explicitly since you
                     // cannot apply `sizeof()` to a function type.
                     std::is_function_v<std::remove_cvref_t<T>>;

}  // namespace base

#endif  // BASE_TYPES_IS_COMPLETE_H_
