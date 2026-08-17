// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_DAWN_CONTROL_CLIENT_HOLDER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_DAWN_CONTROL_CLIENT_HOLDER_H_

#include <dawn/dawn_proc_table.h>

#include <vector>

#include "gpu/command_buffer/client/webgpu_interface.h"
#include "third_party/blink/renderer/platform/graphics/gpu/webgpu_cpp.h"
#include "third_party/blink/renderer/platform/graphics/gpu/webgpu_resource_provider_cache.h"
#include "third_party/blink/renderer/platform/graphics/web_graphics_context_3d_provider_wrapper.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"

namespace base {

class SingleThreadTaskRunner;

}  // namespace base

namespace blink {

namespace scheduler {
class EventLoop;
}  // namespace scheduler

// This class holds the WebGraphicsContext3DProviderWrapper and a strong
// reference to the WebGPU APIChannel.
// DawnControlClientHolder::Destroy() should be called to destroy the
// context which will free all command buffer and GPU resources. As long
// as the reference to the APIChannel is held, calling WebGPU procs is
// valid.
class PLATFORM_EXPORT DawnControlClientHolder
    : public RefCounted<DawnControlClientHolder> {
 public:
  static scoped_refptr<DawnControlClientHolder> Create(
      std::unique_ptr<WebGraphicsContext3DProvider> context_provider,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);

  DawnControlClientHolder(
      std::unique_ptr<WebGraphicsContext3DProvider> context_provider,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);

  void Destroy();

  // Returns a weak pointer to |context_provider_|. If the pointer is valid and
  // non-null, the WebGPU context has not been destroyed, and it is safe to use
  // the WebGPU interface.
  base::WeakPtr<WebGraphicsContext3DProviderWrapper> GetContextProviderWeakPtr()
      const;
  wgpu::Instance GetWGPUInstance() const;
  void MarkContextLost();
  bool IsContextLost() const;
  std::unique_ptr<RecyclableCanvasResource> GetOrCreateCanvasResource(
      const SkImageInfo& info);

  // Flush commands on this client immediately.
  void Flush();
  // Ensure commands on this client are flushed by the end of the task.
  void EnsureFlush(scheduler::EventLoop& event_loop);

 private:
  friend class RefCounted<DawnControlClientHolder>;
  ~DawnControlClientHolder();

  bool context_lost_ = false;
  std::unique_ptr<WebGraphicsContext3DProviderWrapper> context_provider_;
  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;
  scoped_refptr<gpu::webgpu::APIChannel> api_channel_;
  WebGPURecyclableResourceCache recyclable_resource_cache_;

  base::WeakPtrFactory<DawnControlClientHolder> weak_ptr_factory_{this};
};

// Slightly hacky way to get the wgslLanguageFeatures without accessing the
// DawnControlClient because it is initialized asynchronously on workers.
// TODO(crbug.com/1246805): Remove this hack when the DawnControlClient can be
// initialized synchronously on workers and query from its wgpu::Instance
// instead.
PLATFORM_EXPORT std::vector<wgpu::WGSLLanguageFeatureName>
GatherWGSLLanguageFeatures();

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_DAWN_CONTROL_CLIENT_HOLDER_H_
