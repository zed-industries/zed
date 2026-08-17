// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_OUT_OF_MEMORY_ERROR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_OUT_OF_MEMORY_ERROR_H_

#include "third_party/blink/renderer/modules/webgpu/gpu_error.h"

namespace blink {

class GPUOutOfMemoryError : public GPUError {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static GPUOutOfMemoryError* Create(const String& message);
  explicit GPUOutOfMemoryError(const String& message);

  GPUOutOfMemoryError(const GPUOutOfMemoryError&) = delete;
  GPUOutOfMemoryError& operator=(const GPUOutOfMemoryError&) = delete;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_OUT_OF_MEMORY_ERROR_H_
