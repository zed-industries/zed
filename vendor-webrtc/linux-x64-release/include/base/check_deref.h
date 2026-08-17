// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "base/check.h"
#include "base/compiler_specific.h"

#if CHECK_WILL_STREAM()
#include "base/logging.h"
#endif  // CHECK_WILL_STREAM()

#ifndef BASE_CHECK_DEREF_H_
#define BASE_CHECK_DEREF_H_

namespace logging {

// Returns a reference to pointee of `ptr` if `ptr` is not null, or dies if
// `ptr` is null.
//
// It is useful in initializers and direct assignments, where a direct `CHECK`
// call can't be used:
//
//   MyType& type_ref = CHECK_DEREF(MethodReturningAPointer());
//
// If your raw pointer is stored in a wrapped type like `unique_ptr` or
// `raw_ptr`, you should use their `.get()` methods to get the raw pointer
// before calling `CHECK_DEREF()`:
//
//   MyType& type_ref = CHECK_DEREF(your_wrapped_pointer.get());
//
#define CHECK_DEREF(ptr) ::logging::CheckDeref(ptr, #ptr " != nullptr")

template <typename T>
[[nodiscard]] T& CheckDeref(
    T* ptr,
    const char* message,
    const base::Location& location = base::Location::Current()) {
  // Note: we can't just call `CHECK_NE(ptr, nullptr)` here, as that would
  // cause the error to be reported from this header, and we want the error
  // to be reported at the file and line of the caller.
  if (ptr == nullptr) [[unlikely]] {
#if CHECK_WILL_STREAM()
    // `CheckNoreturnError` will die with a fatal error in its destructor.
    CheckNoreturnError::Check(message, location);
#else
    CheckFailure();
#endif  // !CHECK_WILL_STREAM()
  }
  return *ptr;
}

}  // namespace logging

#endif  // BASE_CHECK_DEREF_H_
