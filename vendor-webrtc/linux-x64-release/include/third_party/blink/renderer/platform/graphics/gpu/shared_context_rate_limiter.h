// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_SHARED_CONTEXT_RATE_LIMITER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_SHARED_CONTEXT_RATE_LIMITER_H_

#include <memory>

#include "gpu/command_buffer/client/raster_interface.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"

namespace blink {

class WebGraphicsContext3DProvider;

// Purpose: to limit the amount of worked queued for execution
//   (backlog) on the GPU by blocking the main thread to allow the GPU
//   to catch up. The Prevents unsynchronized tight animation loops
//   from cause a GPU denial of service.
//
// How it works: The rate limiter uses GPU fences to mark each tick
//   and makes sure there are never more that 'maxPendingTicks' fences
//   that are awaiting completion. On platforms that do not support
//   fences, we use glFinish instead. glFinish will only be called in
//   unsynchronized cases that submit more than maxPendingTicks animation
//   tick per compositor frame, which should be quite rare.
//
// How to use it: Each unit of work that constitutes a complete animation
//   frame must call tick(). reset() must be called when the animation
//   is consumed by committing to the compositor. Several rate limiters can
//   be used concurrently: they will each use their own sequences of
//   fences which may be interleaved. When the graphics context is lost
//   and later restored, the existing rate limiter must be destroyed and
//   a new one created.

class PLATFORM_EXPORT SharedContextRateLimiter final {
  USING_FAST_MALLOC(SharedContextRateLimiter);

 public:
  explicit SharedContextRateLimiter(unsigned max_pending_ticks);
  SharedContextRateLimiter(const SharedContextRateLimiter&) = delete;
  SharedContextRateLimiter& operator=(const SharedContextRateLimiter&) = delete;

  void Tick();
  void Reset();

 private:
  std::unique_ptr<WebGraphicsContext3DProvider> context_provider_;
  Deque<GLuint> queries_;
  unsigned max_pending_ticks_;
  bool can_use_sync_queries_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_SHARED_CONTEXT_RATE_LIMITER_H_
