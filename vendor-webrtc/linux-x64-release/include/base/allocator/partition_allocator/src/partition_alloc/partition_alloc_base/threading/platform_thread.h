// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// WARNING: You should *NOT* be using this class directly.  PlatformThread is
// the low-level platform-specific abstraction to the OS's threading interface.
// You should instead be using a message-loop driven Thread, see thread.h.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_THREADING_PLATFORM_THREAD_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_THREADING_PLATFORM_THREAD_H_

#include <cstddef>
#include <iosfwd>

#include "partition_alloc/build_config.h"
#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/partition_alloc_base/threading/platform_thread_ref.h"
#include "partition_alloc/partition_alloc_base/time/time.h"

#if PA_BUILDFLAG(IS_WIN)
#include "partition_alloc/partition_alloc_base/win/windows_types.h"
#elif PA_BUILDFLAG(IS_FUCHSIA)
#include <zircon/types.h>
#elif PA_BUILDFLAG(IS_APPLE)
#include <mach/mach_types.h>
#elif PA_BUILDFLAG(IS_POSIX)
#include <pthread.h>
#include <unistd.h>
#endif

namespace partition_alloc::internal::base {

// Used for logging. Always an integer value.
#if PA_BUILDFLAG(IS_WIN)
typedef DWORD PlatformThreadId;
#elif PA_BUILDFLAG(IS_FUCHSIA)
typedef zx_handle_t PlatformThreadId;
#elif PA_BUILDFLAG(IS_APPLE)
typedef mach_port_t PlatformThreadId;
#elif PA_BUILDFLAG(IS_POSIX)
typedef pid_t PlatformThreadId;
#endif

// Used to operate on threads.
class PlatformThreadHandle {
 public:
#if PA_BUILDFLAG(IS_WIN)
  typedef void* Handle;
#elif PA_BUILDFLAG(IS_POSIX) || PA_BUILDFLAG(IS_FUCHSIA)
  typedef pthread_t Handle;
#endif

  constexpr PlatformThreadHandle() : handle_(0) {}

  explicit constexpr PlatformThreadHandle(Handle handle) : handle_(handle) {}

  bool is_equal(const PlatformThreadHandle& other) const {
    return handle_ == other.handle_;
  }

  bool is_null() const { return !handle_; }

  Handle platform_handle() const { return handle_; }

 private:
  Handle handle_;
};

const PlatformThreadId kInvalidThreadId(0);

typedef void (*SetThreadNameProc)(const std::string&);

// A namespace for low-level thread functions.
class PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) PlatformThread {
 public:
  PlatformThread() = delete;
  PlatformThread(const PlatformThread&) = delete;
  PlatformThread& operator=(const PlatformThread&) = delete;

  // Gets the current thread id, which may be useful for logging purposes.
  static PlatformThreadId CurrentId();

  // Gets the current thread reference, which can be used to check if
  // we're on the right thread quickly.
  static PlatformThreadRef CurrentRef();

  // Get the handle representing the current thread. On Windows, this is a
  // pseudo handle constant which will always represent the thread using it and
  // hence should not be shared with other threads nor be used to differentiate
  // the current thread from another.
  static PlatformThreadHandle CurrentHandle();

  // Sleeps for the specified duration (real-time; ignores time overrides).
  // Note: The sleep duration may be in base::Time or base::TimeTicks, depending
  // on platform. If you're looking to use this in unit tests testing delayed
  // tasks, this will be unreliable - instead, use
  // base::test::TaskEnvironment with MOCK_TIME mode.
  static void Sleep(TimeDelta duration);

  // Sets the thread name visible to debuggers/tools. This will try to
  // initialize the context for current thread unless it's a WorkerThread.
  static void SetName(const std::string& name);

  static void SetThreadNameHook(SetThreadNameProc hook);
};

}  // namespace partition_alloc::internal::base

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_THREADING_PLATFORM_THREAD_H_
