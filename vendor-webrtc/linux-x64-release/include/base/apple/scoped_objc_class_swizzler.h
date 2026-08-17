// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_APPLE_SCOPED_OBJC_CLASS_SWIZZLER_H_
#define BASE_APPLE_SCOPED_OBJC_CLASS_SWIZZLER_H_

#import <objc/runtime.h>

#include "base/base_export.h"

namespace base::apple {

// Within a given scope, swaps method implementations of a class interface, or
// between two class interfaces. The argument and return types must match.
class BASE_EXPORT ScopedObjCClassSwizzler {
 public:
  // Given two classes that each respond to |selector|, swap the implementations
  // of those methods.
  ScopedObjCClassSwizzler(Class target, Class source, SEL selector);

  // Given two selectors on the same class interface, |target| (e.g. via
  // inheritance or categories), swap the implementations of methods |original|
  // and |alternate|.
  ScopedObjCClassSwizzler(Class target, SEL original, SEL alternate);

  ScopedObjCClassSwizzler(const ScopedObjCClassSwizzler&) = delete;
  ScopedObjCClassSwizzler& operator=(const ScopedObjCClassSwizzler&) = delete;

  ~ScopedObjCClassSwizzler();

  // Return a callable function pointer for the replaced method. To call this
  // from the replacing function, the first two arguments should be |self| and
  // |_cmd|. These are followed by the (variadic) method arguments.
  IMP GetOriginalImplementation() const;

  // Invoke the original function directly, optionally with some arguments.
  // Prefer this to hanging onto pointers to the original implementation
  // function or to casting the result of GetOriginalImplementation() yourself.
  template <typename Ret, typename... Args>
  Ret InvokeOriginal(id receiver, SEL selector, Args... args) const {
    auto func = reinterpret_cast<Ret (*)(id, SEL, Args...)>(
        GetOriginalImplementation());
    return func(receiver, selector, args...);
  }

 private:
  // Delegated constructor.
  void Init(Class target, Class source, SEL original, SEL alternate);

  Method old_selector_impl_;
  Method new_selector_impl_;
};

}  // namespace base::apple

#endif  // BASE_APPLE_SCOPED_OBJC_CLASS_SWIZZLER_H_
