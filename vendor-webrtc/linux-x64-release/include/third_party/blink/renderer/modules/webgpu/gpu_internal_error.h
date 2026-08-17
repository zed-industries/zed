// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_INTERNAL_ERROR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_INTERNAL_ERROR_H_

#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/modules/webgpu/gpu_error.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class GPUInternalError : public GPUError {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static GPUInternalError* Create(const String& message);

  explicit GPUInternalError(const String& message);

  GPUInternalError(const GPUInternalError&) = delete;
  GPUInternalError& operator=(const GPUInternalError&) = delete;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_INTERNAL_ERROR_H_
