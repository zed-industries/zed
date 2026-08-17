// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_WEBGPU_CALLBACK_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_WEBGPU_CALLBACK_H_

#include "gpu/webgpu/callback.h"

namespace blink {

// Alias these into the blink namespace as the gpu namespace is typically
// disallowed from being used directly outside of
// blink/renderer/platform/graphics/gpu.
using gpu::webgpu::BindWGPUOnceCallback;
using gpu::webgpu::BindWGPURepeatingCallback;
using gpu::webgpu::MakeWGPUOnceCallback;
using gpu::webgpu::MakeWGPURepeatingCallback;
using gpu::webgpu::WGPUOnceCallback;
using gpu::webgpu::WGPURepeatingCallback;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_WEBGPU_CALLBACK_H_
