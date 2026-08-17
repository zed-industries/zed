// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_POINTERS_RAW_PTR_HOOKABLE_IMPL_H_
#define PARTITION_ALLOC_POINTERS_RAW_PTR_HOOKABLE_IMPL_H_

#include <cstddef>
#include <type_traits>

#include "partition_alloc/buildflags.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/partition_alloc_base/cxx20_is_constant_evaluated.h"
#include "partition_alloc/partition_alloc_forward.h"

#if !PA_BUILDFLAG(USE_RAW_PTR_HOOKABLE_IMPL)
#error "Included under wrong build option"
#endif

namespace base::internal {

struct RawPtrHooks {
  using WrapPtr = void(uintptr_t address);
  using ReleaseWrappedPtr = void(uintptr_t address);
  using SafelyUnwrapForDereference = void(uintptr_t address);
  using SafelyUnwrapForExtraction = void(uintptr_t address);
  using UnsafelyUnwrapForComparison = void(uintptr_t address);
  using Advance = void(uintptr_t old_address, uintptr_t new_address);
  using Duplicate = void(uintptr_t address);
  using WrapPtrForDuplication = void(uintptr_t address);
  using UnsafelyUnwrapForDuplication = void(uintptr_t address);

  WrapPtr* wrap_ptr;
  ReleaseWrappedPtr* release_wrapped_ptr;
  SafelyUnwrapForDereference* safely_unwrap_for_dereference;
  SafelyUnwrapForExtraction* safely_unwrap_for_extraction;
  UnsafelyUnwrapForComparison* unsafely_unwrap_for_comparison;
  Advance* advance;
  Duplicate* duplicate;
  WrapPtrForDuplication* wrap_ptr_for_duplication;
  UnsafelyUnwrapForDuplication* unsafely_unwrap_for_duplication;
};

PA_COMPONENT_EXPORT(RAW_PTR) const RawPtrHooks* GetRawPtrHooks();
PA_COMPONENT_EXPORT(RAW_PTR) void InstallRawPtrHooks(const RawPtrHooks*);
PA_COMPONENT_EXPORT(RAW_PTR) void ResetRawPtrHooks();

template <bool EnableHooks>
struct RawPtrHookableImpl {
  // Since this Impl is used for BRP-ASan, match BRP as closely as possible.
  static constexpr bool kMustZeroOnConstruct = true;
  static constexpr bool kMustZeroOnMove = true;
  static constexpr bool kMustZeroOnDestruct = true;

  // Wraps a pointer.
  template <typename T>
  PA_ALWAYS_INLINE static constexpr T* WrapRawPtr(T* ptr) {
    if (!partition_alloc::internal::base::is_constant_evaluated()) {
      if (EnableHooks) {
        GetRawPtrHooks()->wrap_ptr(reinterpret_cast<uintptr_t>(ptr));
      }
    }
    return ptr;
  }

  // Notifies the allocator when a wrapped pointer is being removed or replaced.
  template <typename T>
  PA_ALWAYS_INLINE static constexpr void ReleaseWrappedPtr(T* ptr) {
    if (!partition_alloc::internal::base::is_constant_evaluated()) {
      if (EnableHooks) {
        GetRawPtrHooks()->release_wrapped_ptr(reinterpret_cast<uintptr_t>(ptr));
      }
    }
  }

  // Unwraps the pointer, while asserting that memory hasn't been freed. The
  // function is allowed to crash on nullptr.
  template <typename T>
  PA_ALWAYS_INLINE static constexpr T* SafelyUnwrapPtrForDereference(
      T* wrapped_ptr) {
    if (!partition_alloc::internal::base::is_constant_evaluated()) {
      if (EnableHooks) {
        GetRawPtrHooks()->safely_unwrap_for_dereference(
            reinterpret_cast<uintptr_t>(wrapped_ptr));
      }
    }
    return wrapped_ptr;
  }

  // Unwraps the pointer, while asserting that memory hasn't been freed. The
  // function must handle nullptr gracefully.
  template <typename T>
  PA_ALWAYS_INLINE static constexpr T* SafelyUnwrapPtrForExtraction(
      T* wrapped_ptr) {
    if (!partition_alloc::internal::base::is_constant_evaluated()) {
      if (EnableHooks) {
        GetRawPtrHooks()->safely_unwrap_for_extraction(
            reinterpret_cast<uintptr_t>(wrapped_ptr));
      }
    }
    return wrapped_ptr;
  }

  // Unwraps the pointer, without making an assertion on whether memory was
  // freed or not.
  template <typename T>
  PA_ALWAYS_INLINE static constexpr T* UnsafelyUnwrapPtrForComparison(
      T* wrapped_ptr) {
    if (!partition_alloc::internal::base::is_constant_evaluated()) {
      if (EnableHooks) {
        GetRawPtrHooks()->unsafely_unwrap_for_comparison(
            reinterpret_cast<uintptr_t>(wrapped_ptr));
      }
    }
    return wrapped_ptr;
  }

  // Upcasts the wrapped pointer.
  template <typename To, typename From>
  PA_ALWAYS_INLINE static constexpr To* Upcast(From* wrapped_ptr) {
    static_assert(std::is_convertible_v<From*, To*>,
                  "From must be convertible to To.");
    // Note, this cast may change the address if upcasting to base that lies in
    // the middle of the derived object.
    return wrapped_ptr;
  }

  // Advance the wrapped pointer by `delta_elems`.
  template <
      typename T,
      typename Z,
      typename =
          std::enable_if_t<partition_alloc::internal::is_offset_type<Z>, void>>
  PA_ALWAYS_INLINE static constexpr T*
  Advance(T* wrapped_ptr, Z delta_elems, bool /*is_in_pointer_modification*/) {
    if (!partition_alloc::internal::base::is_constant_evaluated()) {
      if (EnableHooks) {
        GetRawPtrHooks()->advance(
            reinterpret_cast<uintptr_t>(wrapped_ptr),
            reinterpret_cast<uintptr_t>(wrapped_ptr + delta_elems));
      }
    }
    return wrapped_ptr + delta_elems;
  }

  // Retreat the wrapped pointer by `delta_elems`.
  template <
      typename T,
      typename Z,
      typename =
          std::enable_if_t<partition_alloc::internal::is_offset_type<Z>, void>>
  PA_ALWAYS_INLINE static constexpr T*
  Retreat(T* wrapped_ptr, Z delta_elems, bool /*is_in_pointer_modification*/) {
    if (!partition_alloc::internal::base::is_constant_evaluated()) {
      if (EnableHooks) {
        GetRawPtrHooks()->advance(
            reinterpret_cast<uintptr_t>(wrapped_ptr),
            reinterpret_cast<uintptr_t>(wrapped_ptr - delta_elems));
      }
    }
    return wrapped_ptr - delta_elems;
  }

  template <typename T>
  PA_ALWAYS_INLINE static constexpr ptrdiff_t GetDeltaElems(T* wrapped_ptr1,
                                                            T* wrapped_ptr2) {
    return wrapped_ptr1 - wrapped_ptr2;
  }

  // Returns a copy of a wrapped pointer, without making an assertion on whether
  // memory was freed or not.
  template <typename T>
  PA_ALWAYS_INLINE static constexpr T* Duplicate(T* wrapped_ptr) {
    if (!partition_alloc::internal::base::is_constant_evaluated()) {
      if (EnableHooks) {
        GetRawPtrHooks()->duplicate(reinterpret_cast<uintptr_t>(wrapped_ptr));
      }
    }
    return wrapped_ptr;
  }

  // `WrapRawPtrForDuplication` and `UnsafelyUnwrapPtrForDuplication` are used
  // to create a new raw_ptr<T> from another raw_ptr<T> of a different flavor.
  template <typename T>
  PA_ALWAYS_INLINE static constexpr T* WrapRawPtrForDuplication(T* ptr) {
    if (!partition_alloc::internal::base::is_constant_evaluated()) {
      if (EnableHooks) {
        GetRawPtrHooks()->wrap_ptr_for_duplication(
            reinterpret_cast<uintptr_t>(ptr));
      }
    }
    return ptr;
  }

  template <typename T>
  PA_ALWAYS_INLINE static constexpr T* UnsafelyUnwrapPtrForDuplication(
      T* wrapped_ptr) {
    if (!partition_alloc::internal::base::is_constant_evaluated()) {
      if (EnableHooks) {
        GetRawPtrHooks()->unsafely_unwrap_for_duplication(
            reinterpret_cast<uintptr_t>(wrapped_ptr));
      }
    }
    return wrapped_ptr;
  }

  template <typename T>
  static constexpr void Trace(uint64_t owner_id, T* wrapped_ptr) {}
  static constexpr void Untrace(uint64_t owner_id) {}

  // This is for accounting only, used by unit tests.
  PA_ALWAYS_INLINE static constexpr void IncrementSwapCountForTest() {}
  PA_ALWAYS_INLINE static constexpr void IncrementLessCountForTest() {}
};

}  // namespace base::internal

#endif  // PARTITION_ALLOC_POINTERS_RAW_PTR_HOOKABLE_IMPL_H_
