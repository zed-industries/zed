// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_APPLE_SCOPED_NSAUTORELEASE_POOL_H_
#define BASE_APPLE_SCOPED_NSAUTORELEASE_POOL_H_

#include "base/base_export.h"
#include "base/dcheck_is_on.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "base/memory/stack_allocated.h"
#include "base/thread_annotations.h"
#include "base/threading/thread_checker.h"

namespace base::apple {

// ScopedNSAutoreleasePool creates an autorelease pool when instantiated and
// pops it when destroyed.  This allows an autorelease pool to be maintained in
// ordinary C++ code without bringing in any direct Objective-C dependency.
//
// Before using, please be aware that the semantics of autorelease pools do not
// match the semantics of a C++ class. In particular, recycling or destructing a
// pool lower on the stack destroys all pools higher on the stack, which does
// not mesh well with the existence of C++ objects for each pool.
//
// Use this class only in C++ code; use @autoreleasepool in Obj-C(++) code.

class BASE_EXPORT ScopedNSAutoreleasePool {
  STACK_ALLOCATED();

 public:
  ScopedNSAutoreleasePool();

  ScopedNSAutoreleasePool(const ScopedNSAutoreleasePool&) = delete;
  ScopedNSAutoreleasePool& operator=(const ScopedNSAutoreleasePool&) = delete;
  ScopedNSAutoreleasePool(ScopedNSAutoreleasePool&&) = delete;
  ScopedNSAutoreleasePool& operator=(ScopedNSAutoreleasePool&&) = delete;

  ~ScopedNSAutoreleasePool();

  // Clear out the pool in case its position on the stack causes it to be alive
  // for long periods of time (such as the entire length of the app). Only use
  // then when you're certain the items currently in the pool are no longer
  // needed.
  void Recycle();

 private:
  // Pushes the autorelease pool and does all required verification.
  void PushImpl() VALID_CONTEXT_REQUIRED(thread_checker_);

  // Pops the autorelease pool and does all required verification.
  void PopImpl() VALID_CONTEXT_REQUIRED(thread_checker_);

  // This field is not a raw_ptr<> because it is a pointer to an Objective-C
  // object.
  RAW_PTR_EXCLUSION void* autorelease_pool_ GUARDED_BY_CONTEXT(thread_checker_);

  THREAD_CHECKER(thread_checker_);

#if DCHECK_IS_ON()
  unsigned long level_ = 0;
#endif
};

}  // namespace base::apple

#endif  // BASE_APPLE_SCOPED_NSAUTORELEASE_POOL_H_
