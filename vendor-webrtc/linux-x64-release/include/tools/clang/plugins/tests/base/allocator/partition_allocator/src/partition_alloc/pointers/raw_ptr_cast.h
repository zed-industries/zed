// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_PLUGINS_TESTS_BASE_ALLOCATOR_PARTITION_ALLOCATOR_SRC_PARTITION_ALLOC_POINTERS_RAW_PTR_CAST_H_
#define TOOLS_CLANG_PLUGINS_TESTS_BASE_ALLOCATOR_PARTITION_ALLOCATOR_SRC_PARTITION_ALLOC_POINTERS_RAW_PTR_CAST_H_

namespace base {

template <typename Dest, typename Source>
inline constexpr Dest unsafe_raw_ptr_static_cast(Source&& source) {
  return static_cast<Dest>(source);
}

template <typename Dest, typename Source>
inline constexpr Dest unsafe_raw_ptr_reinterpret_cast(Source&& source) {
  return reinterpret_cast<Dest>(source);
}

template <typename Dest, typename Source>
inline constexpr Dest unsafe_raw_ptr_bit_cast(Source source) {
  return __builtin_bit_cast(Dest, source);
}

}  // namespace base

#endif  // TOOLS_CLANG_PLUGINS_TESTS_BASE_ALLOCATOR_PARTITION_ALLOCATOR_SRC_PARTITION_ALLOC_POINTERS_RAW_PTR_CAST_H_
