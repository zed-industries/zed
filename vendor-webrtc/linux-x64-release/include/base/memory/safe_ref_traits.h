// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_SAFE_REF_TRAITS_H_
#define BASE_MEMORY_SAFE_REF_TRAITS_H_

namespace base {

enum class SafeRefTraits : unsigned {
  kEmpty,
  kDanglingUntriaged,
};
inline constexpr SafeRefTraits SafeRefDanglingUntriaged =
    SafeRefTraits::kDanglingUntriaged;

template <typename T, SafeRefTraits Traits = SafeRefTraits::kEmpty>
class SafeRef;

}  // namespace base

#endif  // BASE_MEMORY_SAFE_REF_TRAITS_H_
