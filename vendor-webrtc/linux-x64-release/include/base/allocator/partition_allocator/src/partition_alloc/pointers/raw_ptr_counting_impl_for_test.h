// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_POINTERS_RAW_PTR_COUNTING_IMPL_FOR_TEST_H_
#define PARTITION_ALLOC_POINTERS_RAW_PTR_COUNTING_IMPL_FOR_TEST_H_

#include <climits>

#include "partition_alloc/pointers/raw_ptr.h"
#include "partition_alloc/pointers/raw_ptr_noop_impl.h"

namespace base::test {

// Provides a raw_ptr/raw_ref implementation that performs accounting for test
// purposes. It performs extra bookkeeping, e.g. to track the number of times
// the raw_ptr is wrapped, unrwapped, etc.
//
// Test only.
struct RawPtrCountingImplForTest : public base::internal::RawPtrNoOpImpl {
  using SuperImpl = base::internal::RawPtrNoOpImpl;

  static constexpr bool kMustZeroOnConstruct = false;
  static constexpr bool kMustZeroOnMove = false;
  static constexpr bool kMustZeroOnDestruct = false;

  template <typename T>
  PA_ALWAYS_INLINE static constexpr T* WrapRawPtr(T* ptr) {
    ++wrap_raw_ptr_cnt;
    return SuperImpl::WrapRawPtr(ptr);
  }

  template <typename T>
  PA_ALWAYS_INLINE static constexpr void ReleaseWrappedPtr(T* ptr) {
    ++release_wrapped_ptr_cnt;
    SuperImpl::ReleaseWrappedPtr(ptr);
  }

  template <typename T>
  PA_ALWAYS_INLINE static constexpr T* SafelyUnwrapPtrForDereference(
      T* wrapped_ptr) {
    ++get_for_dereference_cnt;
    return SuperImpl::SafelyUnwrapPtrForDereference(wrapped_ptr);
  }

  template <typename T>
  PA_ALWAYS_INLINE static constexpr T* SafelyUnwrapPtrForExtraction(
      T* wrapped_ptr) {
    ++get_for_extraction_cnt;
    return SuperImpl::SafelyUnwrapPtrForExtraction(wrapped_ptr);
  }

  template <typename T>
  PA_ALWAYS_INLINE static constexpr T* UnsafelyUnwrapPtrForComparison(
      T* wrapped_ptr) {
    ++get_for_comparison_cnt;
    return SuperImpl::UnsafelyUnwrapPtrForComparison(wrapped_ptr);
  }

  PA_ALWAYS_INLINE static void IncrementSwapCountForTest() {
    ++wrapped_ptr_swap_cnt;
    SuperImpl::IncrementSwapCountForTest();
  }

  PA_ALWAYS_INLINE static void IncrementLessCountForTest() {
    ++wrapped_ptr_less_cnt;
    SuperImpl::IncrementLessCountForTest();
  }

  template <typename T>
  PA_ALWAYS_INLINE static constexpr T* WrapRawPtrForDuplication(T* ptr) {
    ++wrap_raw_ptr_for_dup_cnt;
    return SuperImpl::WrapRawPtrForDuplication(ptr);
  }

  template <typename T>
  PA_ALWAYS_INLINE static constexpr T* UnsafelyUnwrapPtrForDuplication(
      T* wrapped_ptr) {
    ++get_for_duplication_cnt;
    return SuperImpl::UnsafelyUnwrapPtrForDuplication(wrapped_ptr);
  }

  template <typename T>
  static constexpr void Trace(uint64_t owner_id, T* wrapped_ptr) {
    SuperImpl::Trace(owner_id, wrapped_ptr);
  }

  static constexpr void Untrace(uint64_t owner_id) {
    SuperImpl::Untrace(owner_id);
  }

  static void ClearCounters() {
    wrap_raw_ptr_cnt = 0;
    release_wrapped_ptr_cnt = 0;
    get_for_dereference_cnt = 0;
    get_for_extraction_cnt = 0;
    get_for_comparison_cnt = 0;
    wrapped_ptr_swap_cnt = 0;
    wrapped_ptr_less_cnt = 0;
    pointer_to_member_operator_cnt = 0;
    wrap_raw_ptr_for_dup_cnt = 0;
    get_for_duplication_cnt = 0;
  }

  static int wrap_raw_ptr_cnt;
  static int release_wrapped_ptr_cnt;
  static int get_for_dereference_cnt;
  static int get_for_extraction_cnt;
  static int get_for_comparison_cnt;
  static int wrapped_ptr_swap_cnt;
  static int wrapped_ptr_less_cnt;
  static int pointer_to_member_operator_cnt;
  static int wrap_raw_ptr_for_dup_cnt;
  static int get_for_duplication_cnt;
};

}  // namespace base::test

#endif  // PARTITION_ALLOC_POINTERS_RAW_PTR_COUNTING_IMPL_FOR_TEST_H_
