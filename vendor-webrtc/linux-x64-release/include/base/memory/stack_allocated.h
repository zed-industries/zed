// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_STACK_ALLOCATED_H_
#define BASE_MEMORY_STACK_ALLOCATED_H_

#include <stddef.h>

#if defined(__clang__)
#define STACK_ALLOCATED_IGNORE(reason) \
  __attribute__((annotate("stack_allocated_ignore")))
#else  // !defined(__clang__)
#define STACK_ALLOCATED_IGNORE(reason)
#endif  // !defined(__clang__)

// If a class or one of its ancestor classes is annotated with STACK_ALLOCATED()
// in its class definition, then instances of the class may not be allocated on
// the heap or as a member variable of a non-stack-allocated class.
#define STACK_ALLOCATED()                                         \
 public:                                                          \
  using IsStackAllocatedTypeMarker [[maybe_unused]] = int;        \
                                                                  \
 private:                                                         \
  void* operator new(size_t) = delete;                            \
  void* operator new(size_t, ::base::NotNullTag, void*) = delete; \
  void* operator new(size_t, void*) = delete

namespace base {

// NotNullTag was originally added to WebKit here:
//     https://trac.webkit.org/changeset/103243/webkit
// ...with the stated goal of improving the performance of the placement new
// operator and potentially enabling the -fomit-frame-pointer compiler flag.
//
// TODO(szager): The placement new operator which uses this tag is currently
// defined in third_party/blink/renderer/platform/wtf/allocator/allocator.h,
// in the global namespace. It should probably move to /base.
//
// It's unknown at the time of writing whether it still provides any benefit
// (or if it ever did). It is used by placing the kNotNull tag before the
// address of the object when calling placement new.
//
// If the kNotNull tag is specified to placement new for a null pointer,
// Undefined Behaviour can result.
//
// Example:
//
// union { int i; } u;
//
// // Typically placement new looks like this.
// new (&u.i) int(3);
// // But we can promise `&u.i` is not null like this.
// new (base::NotNullTag::kNotNull, &u.i) int(3);
enum class NotNullTag { kNotNull };

}  // namespace base

#endif  // BASE_MEMORY_STACK_ALLOCATED_H_
