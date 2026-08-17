// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TEST_GPU_MEMORY_BUFFER_TEST_PLATFORM_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TEST_GPU_MEMORY_BUFFER_TEST_PLATFORM_H_

#include "third_party/blink/renderer/platform/graphics/gpu/shared_gpu_context.h"
#include "third_party/blink/renderer/platform/testing/testing_platform_support.h"

namespace blink {
class GpuMemoryBufferTestPlatform : public blink::TestingPlatformSupport {
 public:
  GpuMemoryBufferTestPlatform() {}

  ~GpuMemoryBufferTestPlatform() override {}

  bool IsGpuCompositingDisabled() const override {
    return is_gpu_compositing_disabled_;
  }

  void SetGpuCompositingDisabled(bool disable) {
    is_gpu_compositing_disabled_ = disable;
  }

 private:
  bool is_gpu_compositing_disabled_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_TEST_GPU_MEMORY_BUFFER_TEST_PLATFORM_H_
