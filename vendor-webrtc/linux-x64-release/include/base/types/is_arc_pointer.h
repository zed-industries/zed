// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TYPES_IS_ARC_POINTER_H_
#define BASE_TYPES_IS_ARC_POINTER_H_

namespace base {

// Detects whether T is a pointer managed by Objective-C Automatic
// Reference Counting. Per
// https://clang.llvm.org/docs/AutomaticReferenceCounting.html#bridged-casts,
// a __bridge cast to void* is only allowed if the argument has
// retainable object pointer type.
#if defined(__OBJC__)
template <typename T>
concept IsArcPointer =
    requires(const T& v) { (__bridge const volatile void*)(v); };
#endif

}  // namespace base

#endif  // BASE_TYPES_IS_ARC_POINTER_H_
