// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_VALIDATION_ERROR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_VALIDATION_ERROR_H_

#include "third_party/blink/renderer/modules/webgpu/gpu_error.h"

namespace blink {

class GPUValidationError : public GPUError {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static GPUValidationError* Create(const String& message);
  explicit GPUValidationError(const String& message);

  GPUValidationError(const GPUValidationError&) = delete;
  GPUValidationError& operator=(const GPUValidationError&) = delete;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_VALIDATION_ERROR_H_
