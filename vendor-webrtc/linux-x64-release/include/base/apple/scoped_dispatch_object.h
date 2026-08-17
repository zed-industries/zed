// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_APPLE_SCOPED_DISPATCH_OBJECT_H_
#define BASE_APPLE_SCOPED_DISPATCH_OBJECT_H_

#include <dispatch/dispatch.h>

#include "base/apple/scoped_typeref.h"

#if __OBJC__
// In Objective-C ARC, dispatch types are Objective-C types, and must be managed
// as such with __strong, etc. This header file must not be included in
// Objective-C code, nor may it be allowed to be recursively included. Use the
// pimpl pattern to isolate its use in a pure C++ file if needed.
#error Do not use this file, or allow it to be included, in Objective-C code.
#endif

namespace base::apple {

namespace internal {

template <typename T>
struct ScopedDispatchObjectTraits {
  static constexpr T InvalidValue() { return nullptr; }
  static T Retain(T object) {
    dispatch_retain(object);
    return object;
  }
  static void Release(T object) { dispatch_release(object); }
};

}  // namespace internal

template <typename T>
using ScopedDispatchObject =
    ScopedTypeRef<T, internal::ScopedDispatchObjectTraits<T>>;

}  // namespace base::apple

#endif  // BASE_APPLE_SCOPED_DISPATCH_OBJECT_H_
