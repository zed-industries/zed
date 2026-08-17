// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_GRAPHICS_SHARED_IMAGE_INTERFACE_PROVIDER_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_GRAPHICS_SHARED_IMAGE_INTERFACE_PROVIDER_H_

#include "base/functional/callback_forward.h"
#include "base/memory/weak_ptr.h"
#include "third_party/blink/public/platform/web_common.h"

namespace gpu {
class SharedImageInterface;
}  // namespace gpu

namespace blink {

class BLINK_PLATFORM_EXPORT BitmapGpuChannelLostObserver {
 public:
  BitmapGpuChannelLostObserver() = default;
  ~BitmapGpuChannelLostObserver() = default;

  virtual void OnGpuChannelLost() = 0;
};

// This class provides the ClientSharedImageInterface for Canvas Resource to
// create shared images in software rendering mode. Canvas Resource cannot get
// ClientSharedImageInterface from WebGraphicsContext3DProvider because it's
// only available in GPU mode.
class BLINK_PLATFORM_EXPORT WebGraphicsSharedImageInterfaceProvider {
 public:
  virtual ~WebGraphicsSharedImageInterfaceProvider() = default;

  // Observer for GpuChannelLost. Observer methods will be called on the same
  // thread where the shared image interface provider is created.
  virtual void AddGpuChannelLostObserver(BitmapGpuChannelLostObserver* ob) = 0;
  virtual void RemoveGpuChannelLostObserver(
      BitmapGpuChannelLostObserver* ob) = 0;

  // When the GPU channel is lost, SharedImageInterface should returns nullptr.
  virtual gpu::SharedImageInterface* SharedImageInterface() = 0;

  virtual base::WeakPtr<WebGraphicsSharedImageInterfaceProvider>
  GetWeakPtr() = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_GRAPHICS_SHARED_IMAGE_INTERFACE_PROVIDER_H_
