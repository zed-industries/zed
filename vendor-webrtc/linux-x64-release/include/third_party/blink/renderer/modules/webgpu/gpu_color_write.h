// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_COLOR_WRITE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_COLOR_WRITE_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_gpu_color_write.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class GPUColorWrite : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  GPUColorWrite(const GPUColorWrite&) = delete;
  GPUColorWrite& operator=(const GPUColorWrite&) = delete;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_GPU_COLOR_WRITE_H_
