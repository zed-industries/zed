// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_WEBGPU_RESOURCE_PROVIDER_CACHE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_WEBGPU_RESOURCE_PROVIDER_CACHE_H_

#include "base/task/single_thread_task_runner.h"
#include "base/threading/thread_checker.h"
#include "components/viz/common/resources/shared_image_format.h"
#include "gpu/command_buffer/client/webgpu_interface.h"
#include "gpu/command_buffer/common/sync_token.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"
#include "third_party/skia/include/core/SkImageInfo.h"
#include "ui/gfx/color_space.h"
#include "ui/gfx/geometry/size.h"

namespace blink {

class CanvasResourceProvider;
class WebGPURecyclableResourceCache;
class WebGraphicsContext3DProviderWrapper;

class PLATFORM_EXPORT RecyclableCanvasResource {
 public:
  RecyclableCanvasResource(
      std::unique_ptr<CanvasResourceProvider> resource_provider,
      base::WeakPtr<WebGPURecyclableResourceCache> cache);

  ~RecyclableCanvasResource();

  CanvasResourceProvider* resource_provider() {
    return resource_provider_.get();
  }

  void SetCompletionSyncToken(const gpu::SyncToken& completion_sync_token) {
    completion_sync_token_ = completion_sync_token;
  }

 private:
  std::unique_ptr<CanvasResourceProvider> resource_provider_;
  base::WeakPtr<WebGPURecyclableResourceCache> cache_;
  gpu::SyncToken completion_sync_token_;
};

class PLATFORM_EXPORT WebGPURecyclableResourceCache {
 public:
  explicit WebGPURecyclableResourceCache(
      base::WeakPtr<WebGraphicsContext3DProviderWrapper> context_provider,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);
  ~WebGPURecyclableResourceCache() = default;

  std::unique_ptr<RecyclableCanvasResource> GetOrCreateCanvasResource(
      const SkImageInfo& info);

  // When the holder is destroyed, move the resource provider to
  // |unused_providers_| if the cache is not full.
  void OnDestroyRecyclableResource(
      std::unique_ptr<CanvasResourceProvider> resource_provider,
      const gpu::SyncToken& completion_sync_token);

  wtf_size_t CleanUpResourcesAndReturnSizeForTesting();

  int GetWaitCountBeforeDeletionForTesting() {
    return kTimerIdDeltaForDeletion;
  }

 private:
  // The maximum number of unused CanvasResourceProviders size, 128 MB.
  static constexpr int kMaxRecyclableResourceCachesInKB = 128 * 1024;
  static constexpr int kMaxRecyclableResourceCachesInBytes =
      kMaxRecyclableResourceCachesInKB * 1024;

  // A resource is deleted from the cache if it's not reused after this delay.
  static constexpr int kCleanUpDelayInSeconds = 2;

  // The duration set to the resource clean-up timer function.
  // Because the resource clean-up function runs every kCleanUpDelayInSeconds
  // and the stale resource can only be deleted in the call to
  // ReleaseStaleResources(). The actually delay could be as long as
  // (kCleanUpDelayInSeconds + kCleanUpDelayInSeconds).
  static constexpr int kTimerDurationInSeconds = 1;

  // The time it takes to increase the Timer Id by this delta is equivalent to
  // kCleanUpDelayInSeconds.
  static constexpr int kTimerIdDeltaForDeletion =
      kCleanUpDelayInSeconds / kTimerDurationInSeconds;

  struct PLATFORM_EXPORT Resource {
    Resource(std::unique_ptr<CanvasResourceProvider> resource_provider,
             unsigned int timer_id,
             int resource_size);
    Resource(Resource&& that) noexcept;
    ~Resource();

    std::unique_ptr<CanvasResourceProvider> resource_provider_;
    unsigned int timer_id_;
    int resource_size_;
  };

  using DequeResourceProvider = WTF::Deque<Resource>;

  // Search |unused_providers_| and acquire the canvas resource provider with
  // the same cache key for re-use.
  std::unique_ptr<CanvasResourceProvider> AcquireCachedProvider(
      const gfx::Size& size,
      const viz::SharedImageFormat& format,
      SkAlphaType alpha_type,
      const gfx::ColorSpace& color_space);

  // Release the stale resources which are recycled before the last clean-up.
  void ReleaseStaleResources();

  // Start the clean-up function runs when there are unused resources.
  void StartResourceCleanUpTimer();

  // This is the place to keep the unused CanvasResourceProviders. They are
  // waiting to be used. MRU is in the front of the deque.
  DequeResourceProvider unused_providers_;

  uint64_t total_unused_resources_in_bytes_ = 0;

  base::WeakPtr<WebGraphicsContext3DProviderWrapper> context_provider_;

  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;
  base::RepeatingCallback<void()> timer_func_;

  // This ensures only one timer task is scheduled.
  bool timer_is_running_ = false;

  // |current_timer_id_| increases by 1 when the clean-up timer function is
  // called. This id is saved in Resource when the resource is recycled and is
  // checked later to determine whether this resource is stale.
  unsigned int current_timer_id_ = 0;

  THREAD_CHECKER(thread_checker_);
  base::WeakPtr<WebGPURecyclableResourceCache> weak_ptr_;
  base::WeakPtrFactory<WebGPURecyclableResourceCache> weak_ptr_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_WEBGPU_RESOURCE_PROVIDER_CACHE_H_
