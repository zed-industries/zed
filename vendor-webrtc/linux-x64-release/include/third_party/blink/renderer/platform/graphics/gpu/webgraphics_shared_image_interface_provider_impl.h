// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_WEBGRAPHICS_SHARED_IMAGE_INTERFACE_PROVIDER_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_WEBGRAPHICS_SHARED_IMAGE_INTERFACE_PROVIDER_IMPL_H_

#include "base/memory/scoped_refptr.h"
#include "base/sequence_checker.h"
#include "gpu/ipc/client/gpu_channel_observer.h"
#include "third_party/blink/public/platform/web_graphics_shared_image_interface_provider.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "base/functional/callback.h"

namespace gpu {
class ClientSharedImageInterface;
}  // namespace gpu

namespace blink {

class WebGraphicsSharedImageInterfaceProviderImpl
    : public WebGraphicsSharedImageInterfaceProvider,
      public gpu::GpuChannelLostObserver {
 public:
  explicit WebGraphicsSharedImageInterfaceProviderImpl(
      scoped_refptr<gpu::ClientSharedImageInterface> shared_image_interface);

  WebGraphicsSharedImageInterfaceProviderImpl(
      const WebGraphicsSharedImageInterfaceProviderImpl&) = delete;
  WebGraphicsSharedImageInterfaceProviderImpl& operator=(
      const WebGraphicsSharedImageInterfaceProviderImpl&) = delete;

  ~WebGraphicsSharedImageInterfaceProviderImpl() override;

  // WebGraphicsSharedImageInterfaceProvider implementation.
  void AddGpuChannelLostObserver(BitmapGpuChannelLostObserver* ob) override;
  void RemoveGpuChannelLostObserver(BitmapGpuChannelLostObserver* ob) override;
  gpu::SharedImageInterface* SharedImageInterface() override;
  base::WeakPtr<blink::WebGraphicsSharedImageInterfaceProvider> GetWeakPtr()
      override;

  // gpu::GpuChannelLostObserver implementation.
  void OnGpuChannelLost() override;
  void GpuChannelLostOnWorkerThread();

 private:
  base::OnceClosure task_gpu_channel_lost_on_worker_thread_;

  scoped_refptr<gpu::ClientSharedImageInterface> shared_image_interface_;

  // GpuChannelLost observed by CanvasResourceProviders
  WTF::Vector<BitmapGpuChannelLostObserver*> observer_list_;

  SEQUENCE_CHECKER(sequence_checker_);

  base::WeakPtrFactory<WebGraphicsSharedImageInterfaceProviderImpl>
      weak_ptr_factory_{this};
};

}  // namespace content
#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_WEBGRAPHICS_SHARED_IMAGE_INTERFACE_PROVIDER_IMPL_H_
