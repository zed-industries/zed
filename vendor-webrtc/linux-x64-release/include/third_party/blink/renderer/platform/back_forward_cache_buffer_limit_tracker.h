// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BACK_FORWARD_CACHE_BUFFER_LIMIT_TRACKER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BACK_FORWARD_CACHE_BUFFER_LIMIT_TRACKER_H_

#include <cstddef>

#include "base/synchronization/lock.h"
#include "base/thread_annotations.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

// Singleton utility class for process-wide back-forward cache buffer limit
// tracking. All the methods are concurrent-safe.
class PLATFORM_EXPORT BackForwardCacheBufferLimitTracker {
 public:
  // Returns the singleton instance of this class.
  static BackForwardCacheBufferLimitTracker& Get();

  // Called when a network request buffered an additional `num_bytes` while in
  // back-forward cache. May be called multiple times.
  void DidBufferBytes(size_t num_bytes) LOCKS_EXCLUDED(lock_);

  void DidRemoveFrameOrWorkerFromBackForwardCache(size_t total_bytes)
      LOCKS_EXCLUDED(lock_);

  bool IsUnderPerProcessBufferLimit() LOCKS_EXCLUDED(lock_);

  size_t total_bytes_buffered_for_testing() LOCKS_EXCLUDED(lock_);

  BackForwardCacheBufferLimitTracker(BackForwardCacheBufferLimitTracker&) =
      delete;

 private:
  BackForwardCacheBufferLimitTracker();

  const size_t max_buffered_bytes_per_process_;

  base::Lock lock_;

  // The total bytes buffered by all network requests in frames or workers while
  // frozen due to back-forward cache. This number gets reset for when the
  // process gets out of the back-forward cache. As this variable is accessed
  // from frames and workers, this must be protected by `lock_`.
  size_t total_bytes_buffered_ GUARDED_BY(lock_) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BACK_FORWARD_CACHE_BUFFER_LIMIT_TRACKER_H_
