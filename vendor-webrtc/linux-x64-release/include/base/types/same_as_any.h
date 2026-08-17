// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TYPES_SAME_AS_ANY_H_
#define BASE_TYPES_SAME_AS_ANY_H_

#include <concepts>

namespace base {

// A concept that is true when `T` is any of the subsequent types.
template <typename T, typename... Ts>
concept SameAsAny = (std::same_as<T, Ts> || ...);

}  // namespace base

#endif  // BASE_TYPES_SAME_AS_ANY_H_
