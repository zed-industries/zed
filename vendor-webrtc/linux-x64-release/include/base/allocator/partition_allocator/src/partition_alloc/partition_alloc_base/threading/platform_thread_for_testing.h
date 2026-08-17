// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// WARNING: You should *NOT* be using this class directly.  PlatformThread is
// the low-level platform-specific abstraction to the OS's threading interface.
// You should instead be using a message-loop driven Thread, see thread.h.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_THREADING_PLATFORM_THREAD_FOR_TESTING_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_THREADING_PLATFORM_THREAD_FOR_TESTING_H_

#include <cstddef>
#include <iosfwd>

#include "partition_alloc/build_config.h"
#include "partition_alloc/partition_alloc_base/threading/platform_thread.h"

namespace partition_alloc::internal::base {

// A namespace for low-level thread functions.
class PlatformThreadForTesting : public PlatformThread {
 public:
  // Implement this interface to run code on a background thread.  Your
  // ThreadMain method will be called on the newly created thread.
  class Delegate {
   public:
    virtual void ThreadMain() = 0;

   protected:
    virtual ~Delegate() = default;
  };

  PlatformThreadForTesting() = delete;
  PlatformThreadForTesting(const PlatformThreadForTesting&) = delete;
  PlatformThreadForTesting& operator=(const PlatformThreadForTesting&) = delete;

  // Yield the current thread so another thread can be scheduled.
  //
  // Note: this is likely not the right call to make in most situations. If this
  // is part of a spin loop, consider base::Lock, which likely has better tail
  // latency. Yielding the thread has different effects depending on the
  // platform, system load, etc., and can result in yielding the CPU for less
  // than 1us, or many tens of ms.
  static void YieldCurrentThread();

  // Creates a new thread.  The `stack_size` parameter can be 0 to indicate
  // that the default stack size should be used.  Upon success,
  // `*thread_handle` will be assigned a handle to the newly created thread,
  // and `delegate`'s ThreadMain method will be executed on the newly created
  // thread.
  // NOTE: When you are done with the thread handle, you must call Join to
  // release system resources associated with the thread.  You must ensure that
  // the Delegate object outlives the thread.
  static bool Create(size_t stack_size,
                     Delegate* delegate,
                     PlatformThreadHandle* thread_handle);

  // Joins with a thread created via the Create function.  This function blocks
  // the caller until the designated thread exits.  This will invalidate
  // `thread_handle`.
  static void Join(PlatformThreadHandle thread_handle);

#if PA_BUILDFLAG(IS_POSIX) || PA_BUILDFLAG(IS_FUCHSIA)
  // Returns the default thread stack size set by chrome. If we do not
  // explicitly set default size then returns 0.
  static size_t GetDefaultThreadStackSize();
#endif
};

}  // namespace partition_alloc::internal::base

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_THREADING_PLATFORM_THREAD_FOR_TESTING_H_
