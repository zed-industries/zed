// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TEST_TEST_WEBGRAPHICS_SHARED_IMAGE_INTERFACE_PROVIDER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TEST_TEST_WEBGRAPHICS_SHARED_IMAGE_INTERFACE_PROVIDER_H_

#include "gpu/command_buffer/client/test_shared_image_interface.h"
#include "third_party/blink/public/platform/web_graphics_shared_image_interface_provider.h"

namespace blink {

class TestWebGraphicsSharedImageInterfaceProvider
    : public WebGraphicsSharedImageInterfaceProvider {
 public:
  static std::unique_ptr<TestWebGraphicsSharedImageInterfaceProvider> Create() {
    return std::make_unique<TestWebGraphicsSharedImageInterfaceProvider>(
        base::MakeRefCounted<gpu::TestSharedImageInterface>());
  }

  explicit TestWebGraphicsSharedImageInterfaceProvider(
      scoped_refptr<gpu::TestSharedImageInterface> shared_image_interface)
      : shared_image_interface_(std::move(shared_image_interface)) {}

  gpu::SharedImageInterface* SharedImageInterface() override {
    return shared_image_interface_.get();
  }

  base::WeakPtr<blink::WebGraphicsSharedImageInterfaceProvider> GetWeakPtr()
      override {
    return weak_ptr_factory_.GetWeakPtr();
  }

  void AddGpuChannelLostObserver(BitmapGpuChannelLostObserver* ob) override {}
  void RemoveGpuChannelLostObserver(BitmapGpuChannelLostObserver* ob) override {
  }

 private:
  scoped_refptr<gpu::TestSharedImageInterface> shared_image_interface_;
  base::WeakPtrFactory<TestWebGraphicsSharedImageInterfaceProvider>
      weak_ptr_factory_{this};
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TEST_TEST_WEBGRAPHICS_SHARED_IMAGE_INTERFACE_PROVIDER_H_
