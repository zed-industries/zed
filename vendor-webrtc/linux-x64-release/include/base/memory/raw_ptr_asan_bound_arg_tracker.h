// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_RAW_PTR_ASAN_BOUND_ARG_TRACKER_H_
#define BASE_MEMORY_RAW_PTR_ASAN_BOUND_ARG_TRACKER_H_

#include "partition_alloc/buildflags.h"

#if PA_BUILDFLAG(USE_ASAN_BACKUP_REF_PTR)
#include <cstddef>
#include <cstdint>
#include <memory>
#include <vector>

#include "base/base_export.h"
#include "base/memory/raw_ptr.h"
#include "third_party/abseil-cpp/absl/container/inlined_vector.h"

namespace base {
namespace internal {
template <typename, typename, typename>
struct Invoker;

template <typename T, typename UnretainedTrait, RawPtrTraits PtrTraits>
class UnretainedWrapper;

template <typename T, typename UnretainedTrait, RawPtrTraits PtrTraits>
class UnretainedRefWrapper;
}  // namespace internal

// Tracks the lifetimes of bound pointer arguments during callback invocation.
//
// Example:
//   T* unsafe_ptr = new T();
//   PostTask(base::BindOnce(&T::DoSomething, base::Unretained(unsafe_ptr)));
//   delete unsafe_ptr;
//
// When the callback executes, the callee has no access to the raw_ptr<T> inside
// base::Unretained, so it is not possible for it to be invalidated until the
// callback finishes execution; so there is always at least one live raw_ptr<T>
// pointing to `this` for the duration of the call to T::DoSomething.
//
// This class is responsible for tracking and checking which allocations are
// currently protected in this way, and it is only intended to be used inside
// the Bind implementation. This should not be used directly.
class BASE_EXPORT RawPtrAsanBoundArgTracker {
 public:
  static constexpr size_t kInlineArgsCount = 3;
  using ProtectedArgsVector = absl::InlinedVector<uintptr_t, kInlineArgsCount>;

  // Check whether ptr is an address inside an allocation pointed to by one of
  // the currently protected callback arguments. If it is, then this function
  // returns the base address of that allocation, otherwise it returns 0.
  static uintptr_t GetProtectedArgPtr(uintptr_t ptr);

 private:
  template <typename, typename, typename>
  friend struct internal::Invoker;

  void Add(uintptr_t pointer);

  RawPtrAsanBoundArgTracker();
  ~RawPtrAsanBoundArgTracker();

  // Base case for any type that isn't base::Unretained, we do nothing.
  template <typename T>
  void AddArg(const T& arg) {}

  // No specialization for raw_ptr<T> directly, since bound raw_ptr<T>
  // arguments are stored in UnretainedWrapper.

  // When argument is base::Unretained, add the argument to the set of
  // arguments protected in this scope.
  template <typename T, typename UnretainedTrait, RawPtrTraits PtrTraits>
  void AddArg(
      const internal::UnretainedWrapper<T, UnretainedTrait, PtrTraits>& arg) {
    if constexpr (raw_ptr_traits::IsSupportedType<T>::value) {
      auto inner = arg.get();
      // The argument may unwrap into a raw_ptr or a T* depending if it is
      // allowed to dangle.
      if constexpr (IsRawPtr<decltype(inner)>) {
        Add(reinterpret_cast<uintptr_t>(inner.get()));
      } else {
        Add(reinterpret_cast<uintptr_t>(inner));
      }
    }
  }

  // When argument is a reference type that's supported by raw_ptr, add the
  // argument to the set of arguments protected in this scope.
  template <typename T, typename UnretainedTrait, RawPtrTraits PtrTraits>
  void AddArg(
      const internal::UnretainedRefWrapper<T, UnretainedTrait, PtrTraits>&
          arg) {
    if constexpr (raw_ptr_traits::IsSupportedType<T>::value) {
      Add(reinterpret_cast<uintptr_t>(&arg.get()));
    }
  }

  template <typename... Args>
  void AddArgs(Args&&... args) {
    if (enabled_) {
      (AddArg(std::forward<Args>(args)), ...);
    }
  }

  // Cache whether or not BRP-ASan is running when we enter the argument
  // tracking scope so that we ensure that our actions on leaving the scope are
  // consistent even if the runtime flags are changed.
  bool enabled_;

  // We save the previously bound arguments, so that we can restore them when
  // this callback returns. This helps with coverage while avoiding false
  // positives due to nested run loops/callback re-entrancy.
  raw_ptr<ProtectedArgsVector> prev_protected_args_;
  ProtectedArgsVector protected_args_;
};

}  // namespace base

#endif  // PA_BUILDFLAG(USE_ASAN_BACKUP_REF_PTR)
#endif  // BASE_MEMORY_RAW_PTR_ASAN_BOUND_ARG_TRACKER_H_
