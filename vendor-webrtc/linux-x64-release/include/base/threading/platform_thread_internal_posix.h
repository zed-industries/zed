// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_THREADING_PLATFORM_THREAD_INTERNAL_POSIX_H_
#define BASE_THREADING_PLATFORM_THREAD_INTERNAL_POSIX_H_

#include <optional>

#include "base/base_export.h"
#include "base/threading/platform_thread.h"
#include "build/build_config.h"

namespace base {

namespace internal {

struct ThreadTypeToNiceValuePair {
  ThreadType thread_type;
  int nice_value;
};

struct ThreadPriorityToNiceValuePairForTest {
  ThreadPriorityForTest priority;
  int nice_value;
};

// The elements must be listed in the order of increasing priority (lowest
// priority first), that is, in the order of decreasing nice values (highest
// nice value first).
extern const ThreadTypeToNiceValuePair kThreadTypeToNiceValueMap[7];

// The elements must be listed in the order of decreasing priority (highest
// priority first), that is, in the order of increasing nice values (lowest nice
// value first).
extern const ThreadPriorityToNiceValuePairForTest
    kThreadPriorityToNiceValueMapForTest[7];

// Returns the nice value matching |priority| based on the platform-specific
// implementation of kThreadTypeToNiceValueMap.
int ThreadTypeToNiceValue(ThreadType thread_type);

// Returns whether SetCurrentThreadTypeForPlatform can set a thread as
// kRealtimeAudio.
bool CanSetThreadTypeToRealtimeAudio();

// Allows platform specific tweaks to the generic POSIX solution for
// SetCurrentThreadType(). Returns true if the platform-specific
// implementation handled this |thread_type| change, false if the generic
// implementation should instead proceed.
bool SetCurrentThreadTypeForPlatform(ThreadType thread_type,
                                     MessagePumpType pump_type_hint);

#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS)
// Current thread id is cached in thread local storage for performance reasons.
// In some rare cases it's important to invalidate that cache explicitly (e.g.
// after going through clone() syscall which does not call pthread_atfork()
// handlers).
// This can only be called when the process is single-threaded.
BASE_EXPORT void InvalidateTidCache();
#endif  // BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS)

// Returns the ThreadPrioirtyForTest matching |nice_value| based on the
// platform-specific implementation of kThreadPriorityToNiceValueMapForTest.
ThreadPriorityForTest NiceValueToThreadPriorityForTest(int nice_value);

std::optional<ThreadPriorityForTest>
GetCurrentThreadPriorityForPlatformForTest();

int GetCurrentThreadNiceValue();

}  // namespace internal

}  // namespace base

#endif  // BASE_THREADING_PLATFORM_THREAD_INTERNAL_POSIX_H_
