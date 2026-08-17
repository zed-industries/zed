// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TEST_FAKE_CANVAS_RESOURCE_HOST_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TEST_FAKE_CANVAS_RESOURCE_HOST_H_

#include "gpu/command_buffer/common/shared_image_usage.h"
#include "third_party/blink/renderer/platform/graphics/canvas_resource_host.h"
#include "third_party/blink/renderer/platform/graphics/canvas_resource_provider.h"
#include "third_party/blink/renderer/platform/graphics/gpu/shared_gpu_context.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_canvas.h"
#include "third_party/blink/renderer/platform/graphics/skia/skia_utils.h"
#include "third_party/blink/renderer/platform/runtime_enabled_features.h"

namespace cc {
class PaintCanvas;
}

namespace blink {

class FakeCanvasResourceHost : public CanvasResourceHost {
 public:
  explicit FakeCanvasResourceHost(gfx::Size size) : CanvasResourceHost(size) {}
  ~FakeCanvasResourceHost() override = default;
  void NotifyGpuContextLost() override {}
  bool IsContextLost() const override { return false; }
  void SetNeedsCompositingUpdate() override {}
  void InitializeForRecording(cc::PaintCanvas*) const override {}
  void UpdateMemoryUsage() override {}
  bool PrintedInCurrentTask() const override { return false; }
  bool IsPageVisible() const override { return page_visible_; }
  bool IsHibernating() const override { return is_hibernating_; }
  void SetIsHibernating(bool is_hibernating) {
    is_hibernating_ = is_hibernating;
  }
  size_t GetMemoryUsage() const override { return 0; }
  CanvasResourceProvider* GetOrCreateCanvasResourceProvider() override {
    return GetOrCreateCanvasResourceProviderImpl();
  }
  CanvasResourceProvider* GetOrCreateCanvasResourceProviderImpl() override {
    if (ResourceProvider())
      return ResourceProvider();
    constexpr auto kShouldInitialize =
        CanvasResourceProvider::ShouldInitialize::kCallClear;
    std::unique_ptr<CanvasResourceProvider> provider;
    constexpr gpu::SharedImageUsageSet kSharedImageUsageFlags =
        gpu::SHARED_IMAGE_USAGE_DISPLAY_READ | gpu::SHARED_IMAGE_USAGE_SCANOUT;
    provider = CanvasResourceProvider::CreateSharedImageProvider(
        Size(), GetN32FormatForCanvas(), kPremul_SkAlphaType,
        gfx::ColorSpace::CreateSRGB(), kShouldInitialize,
        SharedGpuContext::ContextProviderWrapper(), RasterMode::kGPU,
        kSharedImageUsageFlags, this);
    if (!provider) {
      provider = CanvasResourceProvider::
          CreateSharedImageProviderForSoftwareCompositor(
              Size(), GetN32FormatForCanvas(), kPremul_SkAlphaType,
              gfx::ColorSpace::CreateSRGB(), kShouldInitialize,
              SharedGpuContext::SharedImageInterfaceProvider(), this);
    }
    if (!provider) {
      provider = CanvasResourceProvider::CreateBitmapProvider(
          Size(), GetN32FormatForCanvas(), kPremul_SkAlphaType,
          gfx::ColorSpace::CreateSRGB(), kShouldInitialize, this);
    }

    ReplaceResourceProvider(std::move(provider));

    return ResourceProvider();
  }

  void SetPageVisible(bool visible) {
    if (page_visible_ != visible) {
      page_visible_ = visible;
      PageVisibilityChanged();
    }
  }

 private:
  bool page_visible_ = true;
  bool is_hibernating_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TEST_FAKE_CANVAS_RESOURCE_HOST_H_
