// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// WARNING: *DO NOT* use this class directly. base::PlatformThreadRef is a
// low-level platform-specific abstraction to the OS's threading interface.
// Instead, consider using a message-loop driven base::Thread, see
// base/threading/thread.h.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_THREADING_PLATFORM_THREAD_REF_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_THREADING_PLATFORM_THREAD_REF_H_

#include <iosfwd>

#include "partition_alloc/build_config.h"
#include "partition_alloc/partition_alloc_base/component_export.h"

#if PA_BUILDFLAG(IS_WIN)
#include "partition_alloc/partition_alloc_base/win/windows_types.h"
#elif PA_BUILDFLAG(IS_POSIX) || PA_BUILDFLAG(IS_FUCHSIA)
#include <pthread.h>
#endif

namespace partition_alloc::internal::base {

// Used for thread checking and debugging.
// Meant to be as fast as possible.
// These are produced by PlatformThread::CurrentRef(), and used to later
// check if we are on the same thread or not by using ==. These are safe
// to copy between threads, but can't be copied to another process as they
// have no meaning there. Also, the internal identifier can be re-used
// after a thread dies, so a PlatformThreadRef cannot be reliably used
// to distinguish a new thread from an old, dead thread.
class PlatformThreadRef {
 public:
#if PA_BUILDFLAG(IS_WIN)
  using RefType = DWORD;
#elif PA_BUILDFLAG(IS_POSIX) || PA_BUILDFLAG(IS_FUCHSIA)
  using RefType = pthread_t;
#endif

  constexpr PlatformThreadRef() = default;
  explicit constexpr PlatformThreadRef(RefType id) : id_(id) {}

  bool operator==(PlatformThreadRef other) const { return id_ == other.id_; }
  bool operator!=(PlatformThreadRef other) const { return id_ != other.id_; }

  bool is_null() const { return id_ == 0; }

 private:
  RefType id_ = 0;
};

}  // namespace partition_alloc::internal::base

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_THREADING_PLATFORM_THREAD_REF_H_
