// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROFILER_STACK_BASE_ADDRESS_POSIX_H_
#define BASE_PROFILER_STACK_BASE_ADDRESS_POSIX_H_

#include <pthread.h>
#include <stdint.h>

#include <optional>

#include "base/base_export.h"
#include "base/threading/platform_thread.h"

namespace base {

// Returns the base address of the stack for the given thread. (The address of
// the start of the stack, the highest addressable byte, where the frame of the
// first function on the thread is.)
//
// |id| and |pthread_id| must refer to the same thread.
//
// On Linux & ChromeOS, if the sandbox has been engaged, this may crash if
// |id| and |pthread_id| refer to any thread except the current one.
//
// May return nullopt on Android if the thread's memory range is not found in
// /proc/self/maps.
BASE_EXPORT std::optional<uintptr_t> GetThreadStackBaseAddress(
    PlatformThreadId id,
    pthread_t pthread_id);

}  // namespace base

#endif  // BASE_PROFILER_STACK_BASE_ADDRESS_POSIX_H_
