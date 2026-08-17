// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_BUFFER_USAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_BUFFER_USAGE_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_gpu_buffer_usage.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class GPUBufferUsage : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  GPUBufferUsage(const GPUBufferUsage&) = delete;
  GPUBufferUsage& operator=(const GPUBufferUsage&) = delete;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_BUFFER_USAGE_H_
