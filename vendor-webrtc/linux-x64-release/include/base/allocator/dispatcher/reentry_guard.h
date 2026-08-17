// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ALLOCATOR_DISPATCHER_REENTRY_GUARD_H_
#define BASE_ALLOCATOR_DISPATCHER_REENTRY_GUARD_H_

#include "base/base_export.h"
#include "base/check.h"
#include "base/compiler_specific.h"
#include "build/build_config.h"

#if BUILDFLAG(IS_APPLE) || BUILDFLAG(IS_ANDROID)
#include <pthread.h>
#endif

namespace base::allocator::dispatcher {

#if BUILDFLAG(IS_APPLE) || BUILDFLAG(IS_ANDROID)

// The macOS implementation of libmalloc sometimes calls malloc recursively,
// delegating allocations between zones. That causes our hooks being called
// twice. The scoped guard allows us to detect that.
//
// Besides that the implementations of thread_local on macOS and Android
// seem to allocate memory lazily on the first access to thread_local variables
// (and on Android at least thread_local is implemented on top of pthread so is
// strictly worse for performance). Make use of pthread TLS instead of C++
// thread_local there.
struct BASE_EXPORT ReentryGuard {
  ALWAYS_INLINE ReentryGuard() : allowed_(!pthread_getspecific(entered_key_)) {
    pthread_setspecific(entered_key_, reinterpret_cast<void*>(true));
  }

  ALWAYS_INLINE ~ReentryGuard() {
    if (allowed_) [[likely]] {
      pthread_setspecific(entered_key_, nullptr);
    }
  }

  explicit operator bool() const noexcept { return allowed_; }

  // This function must be called before installing any allocator hooks because
  // some TLS implementations may allocate (eg. glibc will require a malloc call
  // to allocate storage for a higher slot number (>= PTHREAD_KEY_2NDLEVEL_SIZE
  // == 32). This touches the thread-local storage so that any malloc happens
  // before installing the hooks.
  static void InitTLSSlot();

  // InitTLSSlot() is called before crash keys are available. At some point
  // after SetCrashKeyImplementation() is called, this function should be
  // called to record `entered_key_` to a crash key for debugging. This may
  // allocate so it must not be called from inside an allocator hook.
  static void RecordTLSSlotToCrashKey();

 private:
  static pthread_key_t entered_key_;
  const bool allowed_;
};

#else

// Use [[maybe_unused]] as this lightweight stand-in for the more heavyweight
// ReentryGuard above will otherwise trigger the "unused code" warnings.
struct [[maybe_unused]] BASE_EXPORT ReentryGuard {
  constexpr explicit operator bool() const noexcept { return true; }

  static void InitTLSSlot();
  static void RecordTLSSlotToCrashKey();
};

#endif

}  // namespace base::allocator::dispatcher

#endif  // BASE_ALLOCATOR_DISPATCHER_REENTRY_GUARD_H_
