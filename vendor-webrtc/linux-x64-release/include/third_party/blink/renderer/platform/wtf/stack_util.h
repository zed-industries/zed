// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_STACK_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_STACK_UTIL_H_

#include <stddef.h>
#include <stdint.h>
#include "base/compiler_specific.h"
#include "build/build_config.h"
#include "third_party/blink/renderer/platform/wtf/wtf_export.h"

namespace WTF {

WTF_EXPORT size_t GetUnderestimatedStackSize();
WTF_EXPORT void* GetStackStart();

// Returns the current stack position such that it works correctly with ASAN and
// SafeStack. Must be marked noinline because it relies on compiler intrinsics
// that report the current stack frame and if inlined it could report a position
// above the current stack position.
NOINLINE WTF_EXPORT uintptr_t GetCurrentStackPosition();

namespace internal {

WTF_EXPORT extern uintptr_t g_main_thread_stack_start;
WTF_EXPORT extern uintptr_t g_main_thread_underestimated_stack_size;

WTF_EXPORT void InitializeMainThreadStackEstimate();

#if BUILDFLAG(IS_WIN) && defined(COMPILER_MSVC)
size_t ThreadStackSize();
#endif

}  // namespace internal

// Returns true if the function is not called on the main thread. May return
// false positives. Returning false guarantees execution on the main thread
// though.
ALWAYS_INLINE bool MayNotBeMainThread() {
  uintptr_t dummy;
  const uintptr_t address_diff =
      internal::g_main_thread_stack_start - reinterpret_cast<uintptr_t>(&dummy);
  // This is a fast way to judge if we are in the main thread. If |&dummy| is
  // within |g_main_thread_underestimated_stack_size| byte from the stack start
  // of the main thread, we judge that we are in the main thread.
  return address_diff >= internal::g_main_thread_underestimated_stack_size;
}

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_STACK_UTIL_H_
