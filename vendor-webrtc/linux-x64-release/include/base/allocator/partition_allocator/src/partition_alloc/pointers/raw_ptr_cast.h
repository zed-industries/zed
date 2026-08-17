// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// IWYU pragma: private, include "base/memory/raw_ptr_cast.h"

#ifndef PARTITION_ALLOC_POINTERS_RAW_PTR_CAST_H_
#define PARTITION_ALLOC_POINTERS_RAW_PTR_CAST_H_

#include <memory>
#include <type_traits>

// This header is explicitly allowlisted from a clang plugin rule at
// "tools/clang/plugins/FindBadRawPtrPatterns.cpp". You can bypass these checks
// by performing casts explicitly with functions here.
namespace base {

// Wrapper for |static_cast<T>(src)|.
template <typename Dest, typename Source>
inline constexpr Dest unsafe_raw_ptr_static_cast(Source&& source) noexcept {
  return static_cast<Dest>(source);
}

// Wrapper for |reinterpret_cast<T>(src)|.
template <typename Dest, typename Source>
inline constexpr Dest unsafe_raw_ptr_reinterpret_cast(
    Source&& source) noexcept {
  return reinterpret_cast<Dest>(source);
}

// Wrapper for |std::bit_cast<T>(src)|.
// Though we have similar implementations at |absl::bit_cast| and
// |base::bit_cast|, it is important to perform casting in this file to
// correctly exclude from the check.
template <typename Dest, typename Source>
inline constexpr Dest unsafe_raw_ptr_bit_cast(const Source& source) noexcept {
  static_assert(!std::is_pointer_v<Source>,
                "bit_cast must not be used on pointer types");
  static_assert(!std::is_pointer_v<Dest>,
                "bit_cast must not be used on pointer types");
  static_assert(!std::is_reference_v<Source>,
                "bit_cast must not be used on reference types");
  static_assert(!std::is_reference_v<Dest>,
                "bit_cast must not be used on reference types");
  static_assert(
      sizeof(Dest) == sizeof(Source),
      "bit_cast requires source and destination types to be the same size");
  static_assert(std::is_trivially_copyable_v<Source>,
                "bit_cast requires the source type to be trivially copyable");
  static_assert(
      std::is_trivially_copyable_v<Dest>,
      "bit_cast requires the destination type to be trivially copyable");

  return __builtin_bit_cast(Dest, source);
}

}  // namespace base

#endif  // PARTITION_ALLOC_POINTERS_RAW_PTR_CAST_H_
