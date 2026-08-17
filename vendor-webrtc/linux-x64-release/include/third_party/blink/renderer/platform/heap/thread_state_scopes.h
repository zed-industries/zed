// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_THREAD_STATE_SCOPES_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_THREAD_STATE_SCOPES_H_

#include "third_party/blink/renderer/platform/heap/thread_state.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "v8/include/cppgc/heap-consistency.h"

#if defined(LEAK_SANITIZER)
#include <sanitizer/lsan_interface.h>
#endif

namespace blink {

// The NoAllocationScope class is used in debug mode to catch unwanted
// allocations. E.g. allocations during GC.
class ThreadState::NoAllocationScope final {
  STACK_ALLOCATED();

 public:
  explicit NoAllocationScope(ThreadState* state)
      : disallow_gc_(state->cpp_heap().GetHeapHandle()) {}

  NoAllocationScope(const NoAllocationScope&) = delete;
  NoAllocationScope& operator=(const NoAllocationScope&) = delete;

 private:
  const cppgc::subtle::DisallowGarbageCollectionScope disallow_gc_;
};

// The GCForbiddenScope class is used to prevent GC finalization
// when it is not safe to do so.
class ThreadState::GCForbiddenScope final {
  STACK_ALLOCATED();

 public:
  explicit GCForbiddenScope(ThreadState* state)
      : no_gc_(state->cpp_heap().GetHeapHandle()) {}

  GCForbiddenScope(const NoAllocationScope&) = delete;
  GCForbiddenScope& operator=(const NoAllocationScope&) = delete;

 private:
  const cppgc::subtle::NoGarbageCollectionScope no_gc_;
};

#if defined(LEAK_SANITIZER)
class LsanDisabledScope final {
  STACK_ALLOCATED();

 public:
  explicit LsanDisabledScope() { __lsan_disable(); }

  ~LsanDisabledScope() { __lsan_enable(); }

  LsanDisabledScope(const LsanDisabledScope&) = delete;
  LsanDisabledScope& operator=(const LsanDisabledScope&) = delete;
};

#define LEAK_SANITIZER_DISABLED_SCOPE LsanDisabledScope lsan_disabled_scope
#else
#define LEAK_SANITIZER_DISABLED_SCOPE
#endif

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_THREAD_STATE_SCOPES_H_
