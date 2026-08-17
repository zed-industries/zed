// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_TYPES_SAME_AS_ANY_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_TYPES_SAME_AS_ANY_H_

#include <type_traits>

namespace partition_alloc::internal::base {

// True when `T` is any of the subsequent types.
// TODO(crbug.com/344963951): Switch to a concept when C++20 is allowed.
template <typename T, typename... Ts>
inline constexpr bool kSameAsAny = (std::is_same_v<T, Ts> || ...);

}  // namespace partition_alloc::internal::base

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_TYPES_SAME_AS_ANY_H_
